// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ash::vk::{self, Handle};
use bytemuck::cast_slice;
use log::{error, warn};
use num_traits::Zero;
use std::{
    borrow::Borrow,
    collections::{BTreeMap, BTreeSet},
    ffi::{c_char, c_void, CStr, CString},
    fmt::Debug,
    ptr::{null, null_mut, NonNull},
    sync::{Arc, Mutex},
};
pub use vk_utils::{VulkanBaseInStructChain, VulkanBaseOutStructChain};
extern crate self as vulkan_layer;

mod bindings;
mod global_simple_intercept;
mod layer_trait;
#[cfg(any(feature = "_test", test))]
pub mod test_utils;
cfg_if::cfg_if! {
    if #[cfg(feature = "_test")] {
        pub mod lazy_collection;
    } else {
        mod lazy_collection;
    }
}
mod vk_utils;

use bindings::vk_layer::{VkLayerDeviceCreateInfo, VkLayerFunction, VkLayerInstanceCreateInfo};
pub use bindings::vk_layer::{VkLayerDeviceLink, VkLayerInstanceLink};
pub use global_simple_intercept::{ApiVersion, Extension, Feature};
use global_simple_intercept::{DeviceDispatchTable, InstanceDispatchTable, VulkanCommand};
pub use layer_trait::{
    DeviceHooks, DeviceInfo, ExtensionProperties, GlobalHooks, GlobalHooksInfo, InstanceHooks,
    InstanceInfo, Layer, LayerManifest, LayerResult, VulkanCommand as LayerVulkanCommand,
};
use lazy_collection::LazyCollection;
pub use vulkan_layer_macros::{
    auto_deviceinfo_impl, auto_globalhooksinfo_impl, auto_instanceinfo_impl,
    declare_introspection_queries,
};

pub trait IsCommandEnabled {
    #[allow(clippy::wrong_self_convention)]
    fn is_command_enabled(
        self,
        api_version: &ApiVersion,
        enabled_extensions: &BTreeSet<Extension>,
    ) -> bool;
}

impl<'a, T: IntoIterator<Item = &'a Feature>> IsCommandEnabled for T {
    fn is_command_enabled(
        self,
        api_version: &ApiVersion,
        enabled_extensions: &BTreeSet<Extension>,
    ) -> bool {
        self.into_iter().any(|feature| match feature {
            Feature::Extension(extension) => enabled_extensions.contains(extension),
            Feature::Core(version) => version <= api_version,
        })
    }
}

/// # Safety
/// Follow the safety requirements for `std::slice::from_raw_parts` with one major difference: if
/// `len` is `0`, `data` can be a null pointer, unlike `std::slice::from_raw_parts`.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn slice_from_raw_parts<'a, T>(data: *const T, len: u32) -> &'a [T] {
    if len == 0 {
        return &[];
    }
    let len = len.try_into().unwrap();
    // Safety: since `len` isn't 0 at this point, the caller guarantees that the safety requirement
    // for `std::slice::from_raw_parts` is met.
    unsafe { std::slice::from_raw_parts(data, len) }
}

/// # Safety
/// Every entry of `cstr_ptrs` must meet the safety requirement of `CStr::from_ptr`.
///
/// # Panic
/// When iterating the returned iterator, it will panic if the pointer doesn't point to a valid UTF8
/// string.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn slice_to_owned_strings(cstr_ptrs: &[*const i8]) -> impl Iterator<Item = String> + '_ {
    cstr_ptrs.iter().map(|cstr_ptr| {
        let cstr = unsafe { CStr::from_ptr(*cstr_ptr) };
        cstr.to_str()
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to decode the string {}: {}",
                    cstr.to_string_lossy(),
                    e
                )
            })
            .to_owned()
    })
}

/// # Safety
/// p_count must be a valid pointer to U. If p_count doesn't reference 0, and p_out is not null,
/// p_out must be a valid pointer to *p_count number of T's.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn fill_vk_out_array<T, U>(
    out: &[T],
    mut p_count: NonNull<U>,
    p_out: *mut T,
) -> vk::Result
where
    T: Clone,
    U: TryFrom<usize> + TryInto<usize> + Zero + Copy,
    <U as TryFrom<usize>>::Error: Debug,
    <U as TryInto<usize>>::Error: Debug,
{
    let count = unsafe { p_count.as_mut() };
    let mut p_out = match NonNull::new(p_out) {
        Some(p_out) => p_out,
        None => {
            *count = out.len().try_into().unwrap();
            return vk::Result::SUCCESS;
        }
    };
    if count.is_zero() {
        if out.is_empty() {
            return vk::Result::SUCCESS;
        } else {
            return vk::Result::INCOMPLETE;
        }
    }
    let out_slice =
        unsafe { std::slice::from_raw_parts_mut(p_out.as_mut(), (*count).try_into().unwrap()) };
    if out_slice.len() < out.len() {
        *count = out_slice.len().try_into().unwrap();
        out_slice.clone_from_slice(&out[..out_slice.len()]);
        vk::Result::INCOMPLETE
    } else {
        *count = out.len().try_into().unwrap();
        out_slice[..out.len()].clone_from_slice(out);
        vk::Result::SUCCESS
    }
}

trait DispatchableObject: vk::Handle + Copy {
    type DispatchKey: From<usize>;

    fn get_dispatch_key(&self) -> Self::DispatchKey {
        let key = self.as_raw() as *const *const c_void;
        // Safe, because all dispatchable objects can be cast to void **. See details at
        // https://github.com/KhronosGroup/Vulkan-Loader/blob/35b005a5792f6e4c2931d62a37324923f1a71c79/docs/LoaderDriverInterface.md#driver-dispatchable-object-creation.
        (*(unsafe { key.as_ref() }.unwrap()) as usize).into()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InstanceDispatchKey(usize);

impl From<usize> for InstanceDispatchKey {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl DispatchableObject for vk::Instance {
    type DispatchKey = InstanceDispatchKey;
}

impl DispatchableObject for vk::PhysicalDevice {
    type DispatchKey = InstanceDispatchKey;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DeviceDispatchKey(usize);

impl From<usize> for DeviceDispatchKey {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

trait DeviceDispatchableObject: vk::Handle {}

impl DispatchableObject for vk::Device {
    type DispatchKey = DeviceDispatchKey;
}

impl DispatchableObject for vk::CommandBuffer {
    type DispatchKey = DeviceDispatchKey;
}

impl DispatchableObject for vk::Queue {
    type DispatchKey = DeviceDispatchKey;
}

struct InstanceInfoWrapper<T: Layer> {
    get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    dispatch_table: InstanceDispatchTable,
    api_version: ApiVersion,
    enabled_extensions: BTreeSet<Extension>,
    // instance_commands and device_commands are recalculated on every vkCreateInstance, so that
    // the layer can decide which commands to intercept dynamically.
    instance_commands: Box<[VulkanCommand]>,
    device_commands: Box<[VulkanCommand]>,
    is_create_device_hooked: bool,
    customized_info: T::InstanceInfoContainer,
}

struct PhysicalDeviceInfoWrapper {
    owner_instance: vk::Instance,
    properties: vk::PhysicalDeviceProperties,
}

struct DeviceInfoWrapper<T: Layer> {
    dispatch_table: DeviceDispatchTable,
    get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    api_version: ApiVersion,
    enabled_extensions: BTreeSet<Extension>,
    // device_commands are recalculated on every vkCreateDevice, so that the layer can decide
    // which commands to intercept dynamically.
    device_commands: Box<[VulkanCommand]>,
    customized_info: T::DeviceInfoContainer,
}

pub struct Global<T: Layer> {
    instance_map: Mutex<LazyCollection<BTreeMap<InstanceDispatchKey, Arc<InstanceInfoWrapper<T>>>>>,
    physical_device_map:
        Mutex<LazyCollection<BTreeMap<vk::PhysicalDevice, Arc<PhysicalDeviceInfoWrapper>>>>,
    device_map: Mutex<LazyCollection<BTreeMap<DeviceDispatchKey, Arc<DeviceInfoWrapper<T>>>>>,
    // layer_info won't be moved into InstanceInfoWrapper like instance_commands and
    // device_commands. For instance_commands and device_commands, although they should be shared
    // globally, we allow multiple existence of them - we only care about the value. T can be
    // different, because we want to guarantee that there is only one unique layer_info, so that
    // the client can put global state in T, and can perform global initialization(e.g. the
    // global logger) in T::default. If the client needs Global to be trivially destructible,
    // the client must guarantee that T is also trivially destructible.
    pub layer_info: T,
    get_instance_addr_proc_hooked: bool,
}

impl<T: Layer> Global<T> {
    pub fn instance() -> &'static Self {
        T::global_instance()
    }

    fn get_instance_info(
        &self,
        instance: impl DispatchableObject<DispatchKey = InstanceDispatchKey>,
    ) -> Option<Arc<InstanceInfoWrapper<T>>> {
        self.instance_map
            .lock()
            .unwrap()
            .get()
            .get(&instance.get_dispatch_key())
            .map(Arc::clone)
    }

    fn create_physical_device_infos<U: Borrow<vk::PhysicalDevice>>(
        &self,
        instance: vk::Instance,
        physical_devices: impl IntoIterator<Item = U>,
    ) {
        for physical_device in physical_devices {
            let mut physical_device_map = self.physical_device_map.lock().unwrap();
            let mut physical_device_map = physical_device_map.get_mut_or_default();
            if let Some(physical_device_info) = physical_device_map.get(physical_device.borrow()) {
                assert_eq!(physical_device_info.owner_instance, instance);
                continue;
            }
            let instance_info = self.get_instance_info(instance).unwrap_or_else(|| {
                panic!("Unknown VkInstance handle: {:#018x}", instance.as_raw())
            });
            let properties = unsafe {
                instance_info
                    .dispatch_table
                    .core
                    .get_physical_device_properties(*physical_device.borrow())
            };
            physical_device_map.insert(
                *physical_device.borrow(),
                Arc::new(PhysicalDeviceInfoWrapper {
                    owner_instance: instance,
                    properties,
                }),
            );
        }
    }

    fn get_device_info(
        &self,
        device: impl DispatchableObject<DispatchKey = DeviceDispatchKey>,
    ) -> Option<Arc<DeviceInfoWrapper<T>>> {
        self.device_map
            .lock()
            .unwrap()
            .get()
            .get(&device.get_dispatch_key())
            .map(Arc::clone)
    }

    fn get_physical_info(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Option<Arc<PhysicalDeviceInfoWrapper>> {
        self.physical_device_map
            .lock()
            .unwrap()
            .get()
            .get(&physical_device)
            .map(Arc::clone)
    }

    extern "system" fn create_instance(
        create_info: *const vk::InstanceCreateInfo,
        allocator: *const vk::AllocationCallbacks,
        p_instance: *mut vk::Instance,
    ) -> vk::Result {
        let p_next_chain = unsafe { create_info.as_ref() }
            .map(|create_info| create_info.p_next as *mut vk::BaseOutStructure)
            .unwrap_or(null_mut());
        let mut p_next_chain: VulkanBaseOutStructChain = unsafe { p_next_chain.as_mut() }.into();
        let layer_create_info = p_next_chain.find_map(|out_struct| {
            let out_struct = out_struct as *mut vk::BaseOutStructure;
            let layer_create_info = unsafe {
                ash::match_out_struct!(match out_struct {
                    out_struct @ VkLayerInstanceCreateInfo => {
                        out_struct
                    }
                    _ => {
                        return None;
                    }
                })
            };
            if layer_create_info.function == VkLayerFunction::VK_LAYER_LINK_INFO {
                Some(layer_create_info)
            } else {
                None
            }
        });
        let layer_create_info = match layer_create_info {
            Some(layer_create_info) => layer_create_info,
            None => {
                error!("failed to find the VkLayerInstanceCreateInfo struct in the chain.");
                return vk::Result::ERROR_INITIALIZATION_FAILED;
            }
        };
        let layer_info = unsafe { layer_create_info.u.pLayerInfo.as_ref() }.unwrap();
        layer_create_info.u.pLayerInfo = layer_info.pNext;

        let get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr =
            layer_info.pfnNextGetInstanceProcAddr;

        let global = Self::instance();
        let hooked = T::GlobalHooksInfo::hooked_commands()
            .iter()
            .any(|command| *command == LayerVulkanCommand::CreateInstance);
        let layer_result = if hooked {
            global.layer_info.get_global_hooks().create_instance(
                unsafe { create_info.as_ref() }.unwrap(),
                layer_info,
                unsafe { allocator.as_ref() },
                p_instance,
            )
        } else {
            LayerResult::Unhandled
        };

        match layer_result {
            LayerResult::Handled(Ok(())) => {}
            LayerResult::Handled(Err(e)) => return e,
            LayerResult::Unhandled => {
                let get_proc_addr = |name: &CStr| -> *const c_void {
                    unsafe {
                        std::mem::transmute(get_instance_proc_addr(
                            vk::Instance::null(),
                            name.as_ptr(),
                        ))
                    }
                };
                let entry = vk::EntryFnV1_0::load(get_proc_addr);

                let ret: vk::Result =
                    unsafe { (entry.create_instance)(create_info, allocator, p_instance) };
                if !matches!(ret, vk::Result::SUCCESS) {
                    return ret;
                }
            }
        };

        let instance = *unsafe { p_instance.as_ref() }.unwrap();
        let ash_instance = unsafe {
            ash::Instance::load(
                &ash::vk::StaticFn {
                    get_instance_proc_addr,
                },
                instance,
            )
        };

        let create_info = unsafe { create_info.as_ref() }.unwrap();
        let enabled_extensions = unsafe {
            slice_from_raw_parts(
                create_info.pp_enabled_extension_names,
                create_info.enabled_extension_count,
            )
        };
        let enabled_extensions = unsafe { slice_to_owned_strings(enabled_extensions) }
            .filter_map(|extension_name| match extension_name.as_str().try_into() {
                Ok(extension) => Some(extension),
                Err(e) => {
                    warn!(
                        "Failed to recognize the extension {}: {}",
                        extension_name, e
                    );
                    None
                }
            })
            .collect();
        let api_version = unsafe { create_info.p_application_info.as_ref() }
            .map(|app_info| app_info.api_version)
            .unwrap_or(0);
        let api_version = if api_version == 0 {
            ApiVersion { major: 1, minor: 0 }
        } else {
            api_version.into()
        };
        let ash_instance = Arc::new(ash_instance);
        let customized_info = global.layer_info.create_instance_info(
            create_info,
            unsafe { allocator.as_ref() },
            Arc::clone(&ash_instance),
            get_instance_proc_addr,
        );
        let is_create_device_hooked = global
            .layer_info
            .hooked_instance_commands(customized_info.borrow())
            .any(|command| command == LayerVulkanCommand::CreateDevice);
        let instance_commands = global.create_instance_commands(customized_info.borrow());
        let device_commands = global.create_device_commands(customized_info.borrow(), None);
        {
            let key = instance.get_dispatch_key();
            let mut instance_map = global.instance_map.lock().unwrap();
            let mut instance_map = instance_map.get_mut_or_default();
            if instance_map.contains_key(&key) {
                error!(
                    "duplicate instances: instance {:?} already exists",
                    instance
                );
                return vk::Result::ERROR_INITIALIZATION_FAILED;
            }
            instance_map.insert(
                key,
                Arc::new(InstanceInfoWrapper {
                    get_instance_proc_addr,
                    dispatch_table: InstanceDispatchTable::load(
                        get_instance_proc_addr,
                        ash_instance,
                    ),
                    api_version,
                    enabled_extensions,
                    instance_commands,
                    device_commands,
                    is_create_device_hooked,
                    customized_info,
                }),
            );
        }
        vk::Result::SUCCESS
    }

    extern "system" fn destroy_instance(
        instance: vk::Instance,
        allocator: *const vk::AllocationCallbacks,
    ) {
        if instance == vk::Instance::null() {
            return;
        }
        let dispatch_key = instance.get_dispatch_key();
        let global = Self::instance();
        let instance_info = global
            .instance_map
            .lock()
            .unwrap()
            .get_mut_or_default()
            .remove(&dispatch_key);
        let instance_info = instance_info.unwrap();
        let instance_info = match Arc::try_unwrap(instance_info) {
            Ok(instance_info) => instance_info,
            Err(_) => panic!(
                "This should be the sole instance dispatch table reference for instance {:?}.",
                instance
            ),
        };
        global
            .physical_device_map
            .lock()
            .unwrap()
            .get_mut_or_default()
            .retain(|_, physical_device_info| physical_device_info.owner_instance != instance);
        unsafe {
            (instance_info.dispatch_table.core.fp_v1_0().destroy_instance)(instance, allocator)
        };
    }

    extern "system" fn enumerate_physical_devices(
        instance: vk::Instance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut vk::PhysicalDevice,
    ) -> vk::Result {
        let global = Self::instance();
        let ash_instance = &global
            .get_instance_info(instance)
            .expect("Must be a valid VkInstance")
            .dispatch_table
            .core;
        let res = unsafe {
            (ash_instance.fp_v1_0().enumerate_physical_devices)(
                instance,
                p_physical_device_count,
                p_physical_devices,
            )
        };
        if (res == vk::Result::SUCCESS || res == vk::Result::INCOMPLETE)
            && !p_physical_devices.is_null()
        {
            let physical_device_count = unsafe { p_physical_device_count.as_ref() }.unwrap();
            let physical_devices =
                unsafe { slice_from_raw_parts(p_physical_devices, *physical_device_count) };
            global.create_physical_device_infos(instance, physical_devices);
        }
        res
    }

    extern "system" fn enumerate_physical_device_groups(
        instance: vk::Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut vk::PhysicalDeviceGroupProperties,
    ) -> vk::Result {
        let global = Self::instance();
        let ash_instance = &global
            .get_instance_info(instance)
            .expect("Must be a valid VkInstance")
            .dispatch_table
            .core;
        let res = unsafe {
            (ash_instance.fp_v1_1().enumerate_physical_device_groups)(
                instance,
                p_physical_device_group_count,
                p_physical_device_group_properties,
            )
        };
        if (res == vk::Result::SUCCESS || res == vk::Result::INCOMPLETE)
            && !p_physical_device_group_count.is_null()
        {
            let physical_device_group_count =
                *unsafe { p_physical_device_group_count.as_ref() }.unwrap();
            let physical_device_groups = unsafe {
                slice_from_raw_parts(
                    p_physical_device_group_properties,
                    physical_device_group_count,
                )
            };
            let physical_devices =
                physical_device_groups
                    .iter()
                    .flat_map(|physical_device_group| {
                        &physical_device_group.physical_devices[..physical_device_group
                            .physical_device_count
                            .try_into()
                            .unwrap()]
                    });
            global.create_physical_device_infos(instance, physical_devices);
        }
        res
    }

    extern "system" fn create_device(
        physical_device: vk::PhysicalDevice,
        create_info: *const vk::DeviceCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_device: *mut vk::Device,
    ) -> vk::Result {
        let create_info = unsafe { create_info.as_ref() }.unwrap();
        let mut p_next_chain: VulkanBaseOutStructChain =
            unsafe { (create_info.p_next as *mut vk::BaseOutStructure).as_mut() }.into();
        let layer_create_info = p_next_chain.find_map(|out_struct| {
            let out_struct = out_struct as *mut vk::BaseOutStructure;
            let layer_create_info = unsafe {
                ash::match_out_struct!(match out_struct {
                    out_struct @ VkLayerDeviceCreateInfo => {
                        out_struct
                    }
                    _ => {
                        return None;
                    }
                })
            };
            if layer_create_info.function == VkLayerFunction::VK_LAYER_LINK_INFO {
                Some(layer_create_info)
            } else {
                None
            }
        });
        let layer_create_info = match layer_create_info {
            Some(layer_create_info) => layer_create_info,
            None => {
                error!("failed to find the VkLayerDeviceCreateInfo struct in the chain.");
                return vk::Result::ERROR_INITIALIZATION_FAILED;
            }
        };
        let layer_link = unsafe { layer_create_info.u.pLayerInfo.as_ref() }.unwrap();
        layer_create_info.u.pLayerInfo = layer_link.pNext;

        let get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr =
            layer_link.pfnNextGetInstanceProcAddr;
        let get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr = layer_link.pfnNextGetDeviceProcAddr;

        let global = Self::instance();
        let physical_device_info = global
            .get_physical_info(physical_device)
            .expect("physical device must be a valid VkPhysicalDevice.");
        let instance_info = global
            .get_instance_info(physical_device_info.owner_instance)
            .expect("The owner instance of this physical device must be registered.");

        let create_device_name = CString::new("vkCreateDevice").unwrap();
        let next_create_device = unsafe {
            get_instance_proc_addr(
                instance_info.dispatch_table.core.handle(),
                create_device_name.as_ptr(),
            )
        };
        let next_create_device: vk::PFN_vkCreateDevice =
            unsafe { std::mem::transmute(next_create_device) };
        let mut create_info = *create_info;
        let requested_extensions = unsafe {
            slice_from_raw_parts(
                create_info.pp_enabled_extension_names,
                create_info.enabled_extension_count,
            )
        };
        let requested_extensions =
            unsafe { slice_to_owned_strings(requested_extensions) }.collect::<Vec<_>>();
        let create_device_res = if instance_info.is_create_device_hooked {
            instance_info
                .customized_info
                .borrow()
                .hooks()
                .create_device(
                    physical_device,
                    &create_info,
                    layer_link,
                    unsafe { p_allocator.as_ref() },
                    unsafe { p_device.as_mut() }.unwrap(),
                )
        } else {
            LayerResult::Unhandled
        };
        let layer_manifest = T::manifest();
        match create_device_res {
            LayerResult::Handled(Ok(())) => {}
            LayerResult::Handled(Err(e)) => return e,
            LayerResult::Unhandled => {
                let enabled_extensions = requested_extensions
                    .iter()
                    .filter_map(|extension_name| {
                        let extension_name_cstring = CString::new(extension_name.clone())
                            .unwrap_or_else(|e| {
                                panic!("Failed to create CString from {}: {}", extension_name, e)
                            });
                        let extension: Extension = match extension_name.as_str().try_into() {
                            Ok(extension) => extension,
                            Err(_) => return Some(extension_name_cstring),
                        };
                        if layer_manifest
                            .device_extensions
                            .iter()
                            .any(|layer_extension| layer_extension.name == extension)
                        {
                            None
                        } else {
                            Some(extension_name_cstring)
                        }
                    })
                    .collect::<Vec<_>>();
                let enabled_extensions = enabled_extensions
                    .iter()
                    .map(|extension_name| extension_name.as_ptr())
                    .collect::<Vec<_>>();
                create_info.enabled_extension_count = enabled_extensions.len().try_into().unwrap();
                create_info.pp_enabled_extension_names = if enabled_extensions.is_empty() {
                    null()
                } else {
                    enabled_extensions.as_ptr()
                };
                let res = unsafe {
                    next_create_device(physical_device, &create_info, p_allocator, p_device)
                };
                if res != vk::Result::SUCCESS {
                    return res;
                }
            }
        }

        let device = *unsafe { p_device.as_ref() }.unwrap();
        let ash_instance = Arc::clone(&instance_info.dispatch_table.core);
        let ash_device = unsafe {
            // ash will also try to load instance-level dispatchable commands with
            // vkGetDeviceProcAddr. Although that won't end up with undefined behavior, and we
            // should always expect NULL returned according to the spec, we may see Android vulkan
            // loader complaining about internal vkGetDeviceProcAddr called for <function name>,
            // which should be benign.
            ash::Device::load(
                &vk::InstanceFnV1_0 {
                    get_device_proc_addr,
                    ..ash_instance.fp_v1_0().clone()
                },
                device,
            )
        };
        let ash_device = Arc::new(ash_device);
        let enabled_extensions = requested_extensions
            .iter()
            .filter_map(|name| {
                let extension_name: Option<Extension> = name.as_str().try_into().ok();
                extension_name
            })
            .collect::<BTreeSet<_>>();
        let api_version = instance_info
            .api_version
            .min(physical_device_info.properties.api_version.into());
        let customized_info = global.layer_info.create_device_info(
            physical_device,
            &create_info,
            unsafe { p_allocator.as_ref() },
            Arc::clone(&ash_device),
            get_device_proc_addr,
        );
        let device_commands = global.create_device_commands(
            instance_info.customized_info.borrow(),
            Some(customized_info.borrow()),
        );
        {
            let mut device_map = global.device_map.lock().unwrap();
            let mut device_map = device_map.get_mut_or_default();
            assert!(
                !device_map.contains_key(&device.get_dispatch_key()),
                "duplicate VkDevice: {:?}",
                device
            );
            device_map.insert(
                device.get_dispatch_key(),
                Arc::new(DeviceInfoWrapper {
                    dispatch_table: DeviceDispatchTable::load(get_device_proc_addr, ash_device),
                    get_device_proc_addr,
                    api_version,
                    enabled_extensions,
                    device_commands,
                    customized_info,
                }),
            );
        }
        vk::Result::SUCCESS
    }

    extern "system" fn destroy_device(
        device: vk::Device,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        if device == vk::Device::null() {
            return;
        }
        let global = Self::instance();
        let device_info = match Arc::try_unwrap(
            global
                .device_map
                .lock()
                .unwrap()
                .get_mut_or_default()
                .remove(&device.get_dispatch_key())
                .expect("device must be registered"),
        ) {
            Ok(device_info) => device_info,
            Err(_) => panic!("This should be the sole owner of the device {:?}", device),
        };
        let allocation_callback = unsafe { p_allocator.as_ref() };
        unsafe {
            device_info
                .dispatch_table
                .core
                .destroy_device(allocation_callback)
        };
    }

    fn layer_properties() -> Vec<vk::LayerProperties> {
        let layer_manifest = T::manifest();
        // layer_name and description will be set later
        let mut layer_property: vk::LayerProperties = vk::LayerProperties::builder()
            .spec_version(layer_manifest.spec_version)
            .implementation_version(layer_manifest.implementation_version)
            .build();
        assert!(layer_manifest.name.len() < vk::MAX_EXTENSION_NAME_SIZE);
        let layer_name = CString::new(layer_manifest.name).unwrap();
        let layer_name = cast_slice(layer_name.as_bytes());
        layer_property.layer_name[..layer_name.len()].copy_from_slice(layer_name);

        let layer_description = CString::new(layer_manifest.description).unwrap();
        let layer_description = cast_slice(layer_description.as_bytes());
        assert!(layer_description.len() < vk::MAX_DESCRIPTION_SIZE);
        layer_property.description[..layer_description.len()].copy_from_slice(layer_description);
        vec![layer_property]
    }

    // Introspection queries required by Android, so we need to expose them as public functions so
    // that the user can futher expose them as functions exported by the dynamic link library.

    /// # Safety
    /// See valid usage of `vkEnumerateInstanceLayerProperties` at
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "system" fn enumerate_instance_layer_properties(
        property_count: *mut u32,
        properties: *mut vk::LayerProperties,
    ) -> vk::Result {
        let ret_properties = Self::layer_properties();
        // Safe, because the caller guarantees that `property_count` is a valid pointer to u32, and
        // if the value referenced by property_count is not 0, and properties is not NULL,
        // properties must be a valid pointer to an array of propert_count vk::LayerProperties
        // structures. See details in
        // VUID-vkEnumerateInstanceLayerProperties-pPropertyCount-parameter and
        // VUID-vkEnumerateInstanceLayerProperties-pProperties-parameter.
        unsafe {
            fill_vk_out_array(
                &ret_properties,
                NonNull::new(property_count).unwrap(),
                properties,
            )
        }
    }

    /// # Safety
    /// See valid usage of `vkEnumerateInstanceExtensionProperties` at
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html.
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "system" fn enumerate_instance_extension_properties(
        layer_name: *const c_char,
        property_count: *mut u32,
        _: *mut vk::ExtensionProperties,
    ) -> vk::Result {
        if !layer_name.is_null() {
            let layer_name = unsafe { CStr::from_ptr(layer_name) }
                .to_str()
                .expect(concat!(
                    "According to \
                     VUID-vkEnumerateInstanceExtensionProperties-pLayerName-parameter, ",
                    "if p_layer_name is not NULL, p_layer_name must be a null-terminated UTF-8 \
                     string."
                ));
            if layer_name == T::manifest().name {
                // Safe, because the caller guarantees that if the passed in `property_count` is not
                // null, it's a valid pointer to u32.
                if let Some(property_count) = unsafe { property_count.as_mut() } {
                    *property_count = 0;
                }
                return vk::Result::SUCCESS;
            }
        }
        vk::Result::ERROR_LAYER_NOT_PRESENT
    }

    /// # Safety
    /// See valid usage of `vkEnumerateDeviceLayerProperties` at
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html.
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "system" fn enumerate_device_layer_properties(
        _: vk::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut vk::LayerProperties,
    ) -> vk::Result {
        // TODO: call into the customized instance data with the vk::PhysicalDevice to provide more
        // flexibility.
        let ret_properties = Self::layer_properties();
        // Safe, because the caller guarantees that `p_property_count` is a valid pointer to u32,
        // and if the value referenced by `p_property_count` is not 0, and `p_properties` is not
        // NULL, `p_properties` must be a valid pointer to an array of `p_property_count`
        // vk::LayerProperties structures. See details in
        // VUID-vkEnumerateDeviceLayerProperties-pPropertyCount-parameter
        // and VUID-vkEnumerateDeviceLayerProperties-pProperties-parameter.
        unsafe {
            fill_vk_out_array(
                &ret_properties,
                NonNull::new(p_property_count).expect(concat!(
                    "p_property_count must be a valid pointer to u32 according to ",
                    "VUID-vkEnumerateDeviceLayerProperties-pPropertyCount-parameter"
                )),
                p_properties,
            )
        }
    }

    /// # Safety
    /// See valid usage of `vkEnumerateDeviceExtensionProperties` at
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html.
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "system" fn enumerate_device_extension_properties(
        physical_device: vk::PhysicalDevice,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut vk::ExtensionProperties,
    ) -> vk::Result {
        let layer_manifest = T::manifest();
        let is_this_layer = if p_layer_name.is_null() {
            false
        } else {
            unsafe { CStr::from_ptr(p_layer_name) }
                .to_str()
                .map(|layer_name| layer_name == layer_manifest.name)
                .unwrap_or(false)
        };
        if !is_this_layer {
            let instance_info = Self::instance().get_instance_info(physical_device).unwrap();
            return unsafe {
                (instance_info
                    .dispatch_table
                    .core
                    .fp_v1_0()
                    .enumerate_device_extension_properties)(
                    physical_device,
                    p_layer_name,
                    p_property_count,
                    p_properties,
                )
            };
        }
        let property_count = NonNull::new(p_property_count).expect(concat!(
            "`p_property_count` must be a valid pointer to u32 according to ",
            "VUID-vkEnumerateDeviceExtensionProperties-pPropertyCount-parameter."
        ));
        // Safe because the caller guarantees `p_property_count` is a valid pointer to u32 according
        // to VUID-vkEnumerateDeviceExtensionProperties-pPropertyCount-parameter, and if
        // `p_property_count` doesn't point to 0, p_properties is either NULL or a valid pointer to
        // an array of `p_property_count` `vk::ExtensionProperties` structures according to
        // VUID-vkEnumerateDeviceExtensionProperties-pProperties-parameter.
        let device_extensions = layer_manifest
            .device_extensions
            .iter()
            .cloned()
            .map(Into::<vk::ExtensionProperties>::into)
            .collect::<Vec<_>>();
        unsafe { fill_vk_out_array(&device_extensions, property_count, p_properties) }
    }

    /// # Safety
    /// See valid usage of `vkGetInstanceProcAddr` at
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html.
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "system" fn get_instance_proc_addr(
        instance: vk::Instance,
        p_name: *const c_char,
    ) -> vk::PFN_vkVoidFunction {
        // Make sure Global is initialized.
        let global = Self::instance();
        // Safe because the caller is expected to pass in a C string in the name parameter. See
        // VUID-vkGetInstanceProcAddr-pName-parameter.
        let name = unsafe { CStr::from_ptr(p_name) };
        let name = name.to_str().expect(concat!(
            "According to VUID-vkGetInstanceProcAddr-pName-parameter, p_name must be a null-",
            "terminated UTF-8 string."
        ));
        if instance == vk::Instance::null() {
            match name {
                "vkGetInstanceProcAddr" => {
                    return unsafe {
                        std::mem::transmute(
                            Self::get_instance_proc_addr as vk::PFN_vkGetInstanceProcAddr,
                        )
                    }
                }
                "vkEnumerateInstanceExtensionProperties" => {
                    return unsafe {
                        std::mem::transmute(
                            Self::enumerate_instance_extension_properties
                                as vk::PFN_vkEnumerateInstanceExtensionProperties,
                        )
                    }
                }
                "vkEnumerateInstanceLayerProperties" => {
                    return unsafe {
                        std::mem::transmute(
                            Self::enumerate_instance_layer_properties
                                as vk::PFN_vkEnumerateInstanceLayerProperties,
                        )
                    }
                }
                "vkCreateInstance" => {
                    return unsafe {
                        std::mem::transmute(Self::create_instance as vk::PFN_vkCreateInstance)
                    }
                }
                _ => {}
            }
            // TODO: allow inteception of vkEnumerateInstanceVersion and handle
            // vkEnumerateInstanceVersion properly. For now, we still return NULL for
            // vkEnumerateInstanceVersion. The Vulkan loader should cover us for this case.
            // Per spec, if instance is NULL, and pName is neither NULL nor a global command, NULL
            // should be returned.
            return None;
        }
        let instance_info = global.get_instance_info(instance)?;
        if global.get_instance_addr_proc_hooked {
            if let LayerResult::Handled(res) = instance_info
                .customized_info
                .borrow()
                .hooks()
                .get_instance_proc_addr(name)
            {
                return res;
            }
        }
        let get_next_proc_addr =
            || unsafe { (instance_info.get_instance_proc_addr)(instance, p_name) };
        let instance_commands = &instance_info.instance_commands;
        let instance_command = match instance_commands
            .binary_search_by_key(&name, |VulkanCommand { name, .. }| name)
        {
            Ok(index) => Some(&instance_commands[index]),
            Err(_) => None,
        };
        if let Some(instance_command) = instance_command {
            if !instance_command.hooked {
                return get_next_proc_addr();
            }
            let enabled = instance_command.features.is_command_enabled(
                &instance_info.api_version,
                &instance_info.enabled_extensions,
            );
            if !enabled {
                return get_next_proc_addr();
            }
            return instance_command.proc;
        }
        let device_commands = &instance_info.device_commands;
        let device_command =
            match device_commands.binary_search_by_key(&name, |VulkanCommand { name, .. }| name) {
                Ok(index) => Some(&device_commands[index]),
                Err(_) => None,
            };
        if let Some(device_command) = device_command {
            // If the next proc addr can't find it, this is an unavailable device command.
            let next_proc_addr = get_next_proc_addr()?;
            if !device_command.hooked {
                return Some(next_proc_addr);
            }
            return device_command.proc;
        }
        // We don't recognize this command.
        get_next_proc_addr()
    }

    /// # Safety
    /// See valid usage of `vkGetInstanceProcAddr` at
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html.
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "system" fn get_device_proc_addr(
        device: vk::Device,
        p_name: *const c_char,
    ) -> vk::PFN_vkVoidFunction {
        let name = unsafe { CStr::from_ptr(p_name) };
        let name = name.to_str().expect("name should be a valid UTF-8 string.");
        let global = Self::instance();
        let device_info = global.get_device_info(device)?;
        if let LayerResult::Handled(res) = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_device_proc_addr(name)
        {
            return res;
        }
        let get_next_device_proc_addr =
            || unsafe { (device_info.get_device_proc_addr)(device, p_name) };
        let device_commands = &device_info.device_commands;
        let command = if let Ok(index) =
            device_commands.binary_search_by_key(&name, |VulkanCommand { name, .. }| name)
        {
            &device_commands[index]
        } else {
            return get_next_device_proc_addr();
        };
        if !command
            .features
            .is_command_enabled(&device_info.api_version, &device_info.enabled_extensions)
        {
            return get_next_device_proc_addr();
        }
        if !command.hooked {
            return get_next_device_proc_addr();
        }
        command.proc
    }
}

impl<T: Layer> Default for Global<T> {
    fn default() -> Self {
        let layer_info: T = Default::default();
        let get_instance_addr_proc_hooked = T::GlobalHooksInfo::hooked_commands()
            .iter()
            .any(|command| *command == LayerVulkanCommand::GetInstanceProcAddr);
        Self {
            instance_map: Default::default(),
            physical_device_map: Default::default(),
            device_map: Default::default(),
            layer_info,
            get_instance_addr_proc_hooked,
        }
    }
}

#[derive(Default)]
pub struct StubGlobalHooks;

#[auto_globalhooksinfo_impl]
impl GlobalHooks for StubGlobalHooks {}

#[derive(Default)]
pub struct StubInstanceInfo;

#[auto_instanceinfo_impl]
impl InstanceHooks for StubInstanceInfo {}

#[derive(Default)]
pub struct StubDeviceInfo;

#[auto_deviceinfo_impl]
impl DeviceHooks for StubDeviceInfo {}

#[cfg(test)]
mod test {
    use once_cell::sync::Lazy;

    use crate::test_utils::LayerManifestExt;
    use std::{cmp::Ordering, mem::MaybeUninit};

    use super::*;

    #[test]
    fn commands_must_be_sorted() {
        #[derive(Default)]
        struct TestLayer(StubGlobalHooks);

        impl Layer for TestLayer {
            type GlobalHooksInfo = StubGlobalHooks;
            type InstanceInfo = StubInstanceInfo;
            type DeviceInfo = StubDeviceInfo;
            type InstanceInfoContainer = StubInstanceInfo;
            type DeviceInfoContainer = StubDeviceInfo;

            fn global_instance() -> &'static Global<Self> {
                &GLOBAL
            }

            fn manifest() -> LayerManifest {
                LayerManifest::test_default()
            }

            fn get_global_hooks_info(&self) -> &Self::GlobalHooksInfo {
                &self.0
            }

            fn create_instance_info(
                &self,
                _: &vk::InstanceCreateInfo,
                _: Option<&vk::AllocationCallbacks>,
                _: Arc<ash::Instance>,
                _next_get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
            ) -> Self::InstanceInfoContainer {
                Default::default()
            }

            fn create_device_info(
                &self,
                _: vk::PhysicalDevice,
                _: &vk::DeviceCreateInfo,
                _: Option<&vk::AllocationCallbacks>,
                _: Arc<ash::Device>,
                _next_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
            ) -> Self::DeviceInfoContainer {
                Default::default()
            }
        }

        static GLOBAL: Lazy<Global<TestLayer>> = Lazy::new(Default::default);

        #[inline]
        fn check<T: PartialOrd>(last: &mut T) -> impl FnMut(T) -> bool + '_ {
            move |curr| {
                if let Some(Ordering::Greater) | None = (*last).partial_cmp(&curr) {
                    return false;
                }
                *last = curr;
                true
            }
        }

        let instance_info = Default::default();
        let device_commands =
            Global::<TestLayer>::instance().create_device_commands(&instance_info, None);
        let mut name_iter = device_commands
            .iter()
            .map(|VulkanCommand { name, .. }| *name);
        let mut last = name_iter.next().unwrap();

        assert!(name_iter.all(check(&mut last)));

        let instance_commands =
            Global::<TestLayer>::instance().create_instance_commands(&instance_info);
        let mut name_iter = instance_commands
            .iter()
            .map(|VulkanCommand { name, .. }| *name);
        let mut last = name_iter.next().unwrap();

        assert!(name_iter.all(check(&mut last)));
    }

    #[test]
    fn fill_vk_out_array_should_handle_zero_count_correctly() {
        let extensions = [ExtensionProperties {
            name: Extension::KHRSwapchain,
            spec_version: 1,
        }]
        .into_iter()
        .map(Into::<vk::ExtensionProperties>::into)
        .collect::<Vec<_>>();

        let mut count = 0;
        assert_eq!(
            unsafe { fill_vk_out_array(&extensions, (&mut count).into(), null_mut()) },
            vk::Result::SUCCESS
        );
        assert_eq!(count, extensions.len());

        count = 0;
        let mut extension_property = MaybeUninit::<vk::ExtensionProperties>::uninit();
        assert_eq!(
            unsafe {
                fill_vk_out_array(
                    &extensions,
                    (&mut count).into(),
                    extension_property.as_mut_ptr(),
                )
            },
            vk::Result::INCOMPLETE
        );
        assert_eq!(count, 0);

        count = 0;
        assert_eq!(
            unsafe { fill_vk_out_array(&[], (&mut count).into(), extension_property.as_mut_ptr()) },
            vk::Result::SUCCESS
        );
    }

    #[test]
    fn fill_vk_out_array_should_return_incomplete_with_short_out_array() {
        let extensions = [
            ExtensionProperties {
                name: Extension::KHRSwapchain,
                spec_version: 1,
            },
            ExtensionProperties {
                name: Extension::KHRSamplerYcbcrConversion,
                spec_version: 1,
            },
        ]
        .into_iter()
        .map(Into::<vk::ExtensionProperties>::into)
        .collect::<Vec<_>>();

        let mut count = 1;
        let mut extension_property = MaybeUninit::<vk::ExtensionProperties>::uninit();
        assert_eq!(
            unsafe {
                fill_vk_out_array(
                    &extensions,
                    (&mut count).into(),
                    extension_property.as_mut_ptr(),
                )
            },
            vk::Result::INCOMPLETE
        );
        assert_eq!(count, 1);
    }
}
