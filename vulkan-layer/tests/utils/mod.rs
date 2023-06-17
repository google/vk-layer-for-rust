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
use once_cell::sync::Lazy;
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{c_char, c_void, CStr},
    pin::Pin,
    ptr::{null, null_mut, NonNull},
    sync::Arc,
};
use vulkan_layer::{
    fill_vk_out_array,
    test_utils::{
        Del, TestLayerWrapper, VkLayerDeviceCreateInfo, VkLayerDeviceLink, VkLayerFunction,
        VkLayerInstanceCreateInfo, VkLayerInstanceLink,
    },
    ApiVersion, Extension, Feature, Global, IsCommandEnabled, Layer, LayerVulkanCommand,
};

pub type MockLayer<T> = Arc<TestLayerWrapper<T>>;

pub fn create_entry<T: Layer>() -> ash::Entry {
    unsafe {
        ash::Entry::from_static_fn(vk::StaticFn {
            get_instance_proc_addr: Global::<T>::get_instance_proc_addr,
        })
    }
}

pub struct InstanceContext {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub next_entry: ash::Entry,
}

pub trait DelInstanceContextExt {
    fn default_device<U: Layer>(self) -> Del<DeviceContext>;
}

impl DelInstanceContextExt for Del<InstanceContext> {
    fn default_device<U: Layer>(self) -> Del<DeviceContext> {
        let physical_device = unsafe {
            self.instance
                .enumerate_physical_devices()
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
        };
        assert!(
            unsafe {
                self.instance
                    .get_physical_device_queue_family_properties(physical_device)
            }
            .len()
                >= 1
        );
        let queue_priorities = [1.0];
        let queue_create_infos = [vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(0)
            .queue_priorities(&queue_priorities)
            .build()];
        let get_device_proc_addr = unsafe {
            self.next_entry.get_instance_proc_addr(
                self.instance.handle(),
                std::mem::transmute(b"vkGetDeviceProcAddr\0".as_ptr()),
            )
        }
        .unwrap();
        let get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr =
            unsafe { std::mem::transmute(get_device_proc_addr) };
        let mut layer_link = VkLayerDeviceLink {
            pNext: null_mut(),
            pfnNextGetInstanceProcAddr: self.next_entry.static_fn().get_instance_proc_addr,
            pfnNextGetDeviceProcAddr: get_device_proc_addr,
        };
        let mut layer_create_info = VkLayerDeviceCreateInfo {
            sType: vk::StructureType::LOADER_DEVICE_CREATE_INFO,
            pNext: null(),
            function: VkLayerFunction::VK_LAYER_LINK_INFO,
            u: Default::default(),
        };
        layer_create_info.u.pLayerInfo = &mut layer_link as *mut _;
        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .push_next(&mut layer_create_info);
        let device = unsafe {
            self.instance
                .create_device(physical_device, &device_create_info, None)
        }
        .unwrap();
        let device_context = DeviceContext {
            device,
            instance_context: self,
        };
        Del::new(device_context, move |device_context| unsafe {
            device_context.device.destroy_device(None)
        })
    }
}

pub struct DeviceContext {
    pub device: ash::Device,
    pub instance_context: Del<InstanceContext>,
}

pub trait InstanceCreateInfoExt {
    fn default_instance<T: Layer>(self) -> Del<InstanceContext>;
}

impl InstanceCreateInfoExt for vk::InstanceCreateInfoBuilder<'_> {
    fn default_instance<T: Layer>(self) -> Del<InstanceContext> {
        extern "system" fn get_instance_proc_addr(
            instance: vk::Instance,
            p_name: *const i8,
        ) -> vk::PFN_vkVoidFunction {
            enum DispatchKind {
                Global,
                Instance,
                Device,
            }
            #[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
            enum VulkanCommandName {
                LayerCommand(LayerVulkanCommand),
                Other(String),
            }
            impl From<&str> for VulkanCommandName {
                fn from(value: &str) -> Self {
                    if let Ok(layer_command) = value.try_into() {
                        return Self::LayerCommand(layer_command);
                    }
                    Self::Other(value.to_owned())
                }
            }
            impl From<LayerVulkanCommand> for VulkanCommandName {
                fn from(value: LayerVulkanCommand) -> Self {
                    Self::LayerCommand(value)
                }
            }
            struct VulkanCommand {
                proc: vk::PFN_vkVoidFunction,
                dispatch_kind: DispatchKind,
                features: BTreeSet<Feature>,
            }
            static VULKAN_COMMANDS: Lazy<BTreeMap<VulkanCommandName, VulkanCommand>> =
                Lazy::new(|| {
                    use LayerVulkanCommand::*;
                    [
                        (
                            CreateInstance.into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(create_instance as vk::PFN_vkCreateInstance)
                                },
                                dispatch_kind: DispatchKind::Global,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                        (
                            "vkDestroyDevice".into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(destroy_device as vk::PFN_vkDestroyDevice)
                                },
                                dispatch_kind: DispatchKind::Device,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                        (
                            CreateDevice.into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(create_device as vk::PFN_vkCreateDevice)
                                },
                                dispatch_kind: DispatchKind::Instance,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                        (
                            "vkEnumeratePhysicalDeviceGroups".into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(
                                        enumerate_physical_device_groups
                                            as vk::PFN_vkEnumeratePhysicalDeviceGroups,
                                    )
                                },
                                dispatch_kind: DispatchKind::Instance,
                                features: [
                                    ApiVersion::V1_1.into(),
                                    Extension::KHRDeviceGroupCreation.into(),
                                ]
                                .into(),
                            },
                        ),
                        (
                            "vkEnumeratePhysicalDevices".into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(
                                        enumerate_physical_devices
                                            as vk::PFN_vkEnumeratePhysicalDevices,
                                    )
                                },
                                dispatch_kind: DispatchKind::Instance,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                        (
                            GetDeviceProcAddr.into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(
                                        get_device_proc_addr as vk::PFN_vkGetDeviceProcAddr,
                                    )
                                },
                                dispatch_kind: DispatchKind::Device,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                        (
                            GetInstanceProcAddr.into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(
                                        get_instance_proc_addr as vk::PFN_vkGetInstanceProcAddr,
                                    )
                                },
                                dispatch_kind: DispatchKind::Instance,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                        (
                            "vkEnumerateDeviceLayerProperties".into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(
                                        enumerate_device_layer_properties
                                            as vk::PFN_vkEnumerateDeviceLayerProperties,
                                    )
                                },
                                dispatch_kind: DispatchKind::Instance,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                        (
                            "vkEnumerateDeviceExtensionProperties".into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(
                                        enumerate_device_extension_properties
                                            as vk::PFN_vkEnumerateDeviceExtensionProperties,
                                    )
                                },
                                dispatch_kind: DispatchKind::Instance,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                        (
                            "vkDestroyInstance".into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(
                                        destroy_instance as vk::PFN_vkDestroyInstance,
                                    )
                                },
                                dispatch_kind: DispatchKind::Instance,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                        (
                            GetPhysicalDeviceSparseImageFormatProperties2.into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(
                                    get_physical_device_sparse_image_format_properties2
                                        as vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
                                )
                                },
                                dispatch_kind: DispatchKind::Instance,
                                features: [
                                    ApiVersion::V1_1.into(),
                                    Extension::KHRGetPhysicalDeviceProperties2.into(),
                                ]
                                .into(),
                            },
                        ),
                        (
                            DestroySurfaceKhr.into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(
                                        destroy_surface_khr as vk::PFN_vkDestroySurfaceKHR,
                                    )
                                },
                                dispatch_kind: DispatchKind::Instance,
                                features: [Extension::KHRSurface.into()].into(),
                            },
                        ),
                        (
                            GetPhysicalDeviceQueueFamilyProperties.into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(
                                        get_physical_device_queue_family_properties
                                            as vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties,
                                    )
                                },
                                dispatch_kind: DispatchKind::Instance,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                        (
                            DestroyImage.into(),
                            VulkanCommand {
                                proc: unsafe {
                                    std::mem::transmute(destroy_image as vk::PFN_vkDestroyImage)
                                },
                                dispatch_kind: DispatchKind::Device,
                                features: [ApiVersion::V1_0.into()].into(),
                            },
                        ),
                    ]
                    .into_iter()
                    .collect()
                });
            #[repr(C)]
            struct InstanceDispatchTable(u32);
            impl Default for InstanceDispatchTable {
                fn default() -> Self {
                    Self(0x10d0)
                }
            }
            #[repr(C)]
            struct InstanceData {
                // Pointer to dispatch_table, this is the ABI guaranteed by the Vulkan loader.
                _dispatch_table: *const c_void,
                dispatch_table: Pin<Box<InstanceDispatchTable>>,
                version: ApiVersion,
                enabled_extensions: BTreeSet<Extension>,
                physical_devices: Box<[Pin<Box<PhysicalDeviceData>>]>,
            }
            impl InstanceData {
                #[deny(unsafe_op_in_unsafe_fn)]
                unsafe fn from_handle<'a>(handle: vk::Instance) -> &'a Self {
                    unsafe { (handle.as_raw() as *const InstanceData).as_ref() }.unwrap()
                }
            }
            #[repr(C)]
            struct PhysicalDeviceData {
                // Should be the same as the owner VkInstance.
                _dispatch_table: *const c_void,
                owner_instance: vk::Instance,
                queue_family_properties: Vec<vk::QueueFamilyProperties>,
            }
            impl PhysicalDeviceData {
                #[deny(unsafe_op_in_unsafe_fn)]
                unsafe fn from_handle<'a>(handle: vk::PhysicalDevice) -> &'a Self {
                    unsafe { (handle.as_raw() as *const PhysicalDeviceData).as_ref() }.unwrap()
                }
            }
            #[repr(C)]
            struct DeviceDispatchTable(u32);
            impl Default for DeviceDispatchTable {
                fn default() -> Self {
                    Self(0xa1de)
                }
            }
            #[repr(C)]
            struct DeviceData {
                // A pointer to dispatch_table.
                _dispatch_table: *const c_void,
                dispatch_table: Pin<Box<DeviceDispatchTable>>,
                owner_physical_device: vk::PhysicalDevice,
                api_veersion: ApiVersion,
                enabled_extensions: BTreeSet<Extension>,
            }
            impl DeviceData {
                #[deny(unsafe_op_in_unsafe_fn)]
                unsafe fn from_handle<'a>(handle: vk::Device) -> &'a Self {
                    unsafe { (handle.as_raw() as *const DeviceData).as_ref() }.unwrap()
                }
            }
            extern "system" fn create_instance(
                create_info: *const vk::InstanceCreateInfo,
                _: *const vk::AllocationCallbacks,
                instance: *mut vk::Instance,
            ) -> vk::Result {
                let create_info = unsafe { create_info.as_ref() }.unwrap();
                let application_info = unsafe { create_info.p_application_info.as_ref() };
                let enabled_extensions = unsafe {
                    std::slice::from_raw_parts(
                        create_info.pp_enabled_extension_names,
                        create_info.enabled_extension_count.try_into().unwrap(),
                    )
                }
                .iter()
                .filter_map(|enabled_extension| {
                    let extension = unsafe { CStr::from_ptr(*enabled_extension) }
                        .to_owned()
                        .into_string()
                        .unwrap();
                    extension.as_str().try_into().ok()
                })
                .collect::<_>();
                let dispatch_table =
                    Box::into_pin(Box::<InstanceDispatchTable>::new(Default::default()));
                let instance_data: Box<InstanceData> = Box::new(InstanceData {
                    _dispatch_table: dispatch_table.as_ref().get_ref() as *const _ as *const c_void,
                    dispatch_table,
                    version: application_info
                        .map(|application_info| application_info.api_version.try_into().unwrap())
                        .unwrap_or(ApiVersion::V1_0),
                    enabled_extensions,
                    physical_devices: Default::default(),
                });
                let instance_raw_handle = Box::into_raw(instance_data);
                let instance_handle = vk::Instance::from_raw(instance_raw_handle as _);
                let instance_data = unsafe { instance_raw_handle.as_mut() }.unwrap();
                instance_data.physical_devices = Box::new([Box::new(PhysicalDeviceData {
                    _dispatch_table: instance_data._dispatch_table,
                    owner_instance: instance_handle,
                    queue_family_properties: vec![vk::QueueFamilyProperties {
                        queue_flags: vk::QueueFlags::GRAPHICS | vk::QueueFlags::TRANSFER,
                        queue_count: 1,
                        timestamp_valid_bits: 0,
                        min_image_transfer_granularity: vk::Extent3D::builder()
                            .width(0)
                            .height(0)
                            .depth(0)
                            .build(),
                    }],
                })
                .into()]);
                *unsafe { instance.as_mut() }.unwrap() = instance_handle;
                vk::Result::SUCCESS
            }
            extern "system" fn destroy_instance(
                instance: vk::Instance,
                _: *const vk::AllocationCallbacks,
            ) {
                let inner = unsafe { Box::from_raw(instance.as_raw() as *mut InstanceData) };
                drop(inner);
            }
            extern "system" fn get_physical_device_sparse_image_format_properties2(
                _: vk::PhysicalDevice,
                _: *const vk::PhysicalDeviceSparseImageFormatInfo2,
                _: *mut u32,
                _: *mut vk::SparseImageFormatProperties2,
            ) {
                unimplemented!()
            }
            extern "system" fn destroy_surface_khr(
                _: vk::Instance,
                surface: vk::SurfaceKHR,
                _: *const vk::AllocationCallbacks,
            ) {
                if surface == vk::SurfaceKHR::null() {
                    return;
                }
                unimplemented!()
            }
            extern "system" fn enumerate_device_layer_properties(
                _: vk::PhysicalDevice,
                _property_count: *mut u32,
                _properties: *mut vk::LayerProperties,
            ) -> vk::Result {
                unimplemented!()
            }
            extern "system" fn enumerate_device_extension_properties(
                _: vk::PhysicalDevice,
                _layer_name: *const c_char,
                _property_count: *mut u32,
                _properties: *mut vk::ExtensionProperties,
            ) -> vk::Result {
                unimplemented!()
            }
            extern "system" fn get_device_proc_addr(
                device: vk::Device,
                p_name: *const c_char,
            ) -> vk::PFN_vkVoidFunction {
                assert_ne!(device, vk::Device::null());
                let device_data = unsafe { DeviceData::from_handle(device) };
                let name = unsafe { CStr::from_ptr(p_name) }.to_str().unwrap();
                let command = VULKAN_COMMANDS.get(&name.into())?;
                if !command
                    .features
                    .is_command_enabled(&device_data.api_veersion, &device_data.enabled_extensions)
                {
                    return None;
                }
                command.proc
            }
            extern "system" fn enumerate_physical_devices(
                instance: vk::Instance,
                physical_device_count: *mut u32,
                physical_devices: *mut vk::PhysicalDevice,
            ) -> vk::Result {
                let all_physical_devices = unsafe { InstanceData::from_handle(instance) }
                    .physical_devices
                    .iter()
                    .map(|physical_device_data| {
                        let physical_device_data_ptr =
                            physical_device_data.as_ref().get_ref() as *const PhysicalDeviceData;
                        vk::PhysicalDevice::from_raw(physical_device_data_ptr as _)
                    })
                    .collect::<Vec<_>>();
                let physical_device_count = NonNull::new(physical_device_count).unwrap();
                unsafe {
                    fill_vk_out_array(
                        &all_physical_devices,
                        physical_device_count,
                        physical_devices,
                    )
                }
            }
            extern "system" fn get_physical_device_queue_family_properties(
                physical_device: vk::PhysicalDevice,
                p_queue_family_property_count: *mut u32,
                p_queue_family_properties: *mut vk::QueueFamilyProperties,
            ) {
                let physical_device_data =
                    unsafe { PhysicalDeviceData::from_handle(physical_device) };
                let p_queue_family_property_count =
                    NonNull::new(p_queue_family_property_count).unwrap();
                assert!(
                    unsafe {
                        fill_vk_out_array(
                            &physical_device_data.queue_family_properties,
                            p_queue_family_property_count,
                            p_queue_family_properties,
                        )
                    }
                    .as_raw()
                        >= 0
                );
            }
            extern "system" fn enumerate_physical_device_groups(
                _: vk::Instance,
                _physical_device_group_count: *mut u32,
                _phyiscal_device_group_properties: *mut vk::PhysicalDeviceGroupProperties,
            ) -> vk::Result {
                unimplemented!()
            }
            extern "system" fn create_device(
                physical_device: vk::PhysicalDevice,
                device_create_info: *const vk::DeviceCreateInfo,
                _: *const vk::AllocationCallbacks,
                device: *mut vk::Device,
            ) -> vk::Result {
                let dispatch_table =
                    Box::into_pin(Box::<DeviceDispatchTable>::new(Default::default()));
                let physical_device_data =
                    unsafe { PhysicalDeviceData::from_handle(physical_device) };
                let instance_data =
                    unsafe { InstanceData::from_handle(physical_device_data.owner_instance) };
                let device_create_info = unsafe { device_create_info.as_ref() }.unwrap();
                let enabled_extensions = unsafe {
                    std::slice::from_raw_parts(
                        device_create_info.pp_enabled_extension_names,
                        device_create_info
                            .enabled_extension_count
                            .try_into()
                            .unwrap(),
                    )
                }
                .iter()
                .map(|extension_name| {
                    unsafe { CStr::from_ptr(*extension_name) }
                        .to_str()
                        .unwrap()
                        .try_into()
                })
                .collect::<Vec<Result<Extension, _>>>();
                if enabled_extensions.iter().any(Result::is_err) {
                    return vk::Result::ERROR_EXTENSION_NOT_PRESENT;
                }
                let enabled_extensions = enabled_extensions
                    .into_iter()
                    .filter_map(Result::ok)
                    .collect();
                let device_info = DeviceData {
                    _dispatch_table: dispatch_table.as_ref().get_ref() as *const _ as *const c_void,
                    dispatch_table,
                    owner_physical_device: physical_device,
                    api_veersion: instance_data.version,
                    enabled_extensions,
                };
                let device_info = Box::new(device_info);
                *unsafe { device.as_mut() }.unwrap() =
                    vk::Device::from_raw(Box::into_raw(device_info) as _);
                vk::Result::SUCCESS
            }
            extern "system" fn destroy_device(
                device: vk::Device,
                _: *const vk::AllocationCallbacks,
            ) {
                let inner = unsafe { Box::from_raw(device.as_raw() as *mut DeviceData) };
                drop(inner);
            }
            extern "system" fn destroy_image(
                _: vk::Device,
                image: vk::Image,
                _: *const vk::AllocationCallbacks,
            ) {
                if image == vk::Image::null() {
                    return;
                }
                unimplemented!()
            }
            let name = unsafe { CStr::from_ptr(p_name) }.to_str().unwrap();
            let command = VULKAN_COMMANDS.get(&name.into())?;
            let instance_data = if instance == vk::Instance::null() {
                return match command.dispatch_kind {
                    DispatchKind::Global => command.proc,
                    _ => None,
                };
            } else {
                unsafe { InstanceData::from_handle(instance) }
            };

            if let DispatchKind::Global = command.dispatch_kind {
                return None;
            }
            if !command
                .features
                .is_command_enabled(&instance_data.version, &instance_data.enabled_extensions)
            {
                return None;
            }
            command.proc
        }
        let entry = create_entry::<T>();
        let mut layer_link = VkLayerInstanceLink {
            pNext: null_mut(),
            pfnNextGetInstanceProcAddr: get_instance_proc_addr,
            pfnNextGetPhysicalDeviceProcAddr: None,
        };
        let mut layer_create_info = VkLayerInstanceCreateInfo {
            sType: vk::StructureType::LOADER_INSTANCE_CREATE_INFO,
            pNext: null(),
            function: VkLayerFunction::VK_LAYER_LINK_INFO,
            u: Default::default(),
        };
        *unsafe { layer_create_info.u.pLayerInfo.as_mut() } = &mut layer_link as *mut _;

        let create_instance_info = self.push_next(&mut layer_create_info);
        let instance = unsafe { entry.create_instance(&create_instance_info, None) }.unwrap();
        assert_ne!(instance.handle(), vk::Instance::null());
        let context = InstanceContext {
            entry,
            instance,
            next_entry: unsafe {
                ash::Entry::from_static_fn(vk::StaticFn {
                    get_instance_proc_addr,
                })
            },
        };
        Del::new(context, move |context| {
            unsafe { context.instance.destroy_instance(None) };
        })
    }
}
