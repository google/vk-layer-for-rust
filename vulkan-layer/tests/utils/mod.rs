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

use ash::{
    prelude::VkResult,
    vk::{self, Handle},
};
use once_cell::sync::Lazy;
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{c_char, c_void, CStr, CString},
    marker::PhantomData,
    pin::Pin,
    ptr::{null, null_mut, NonNull},
    sync::{Arc, Mutex, Weak},
};
use vulkan_layer::{
    fill_vk_out_array,
    test_utils::{
        ArcDel, Del, TestLayerWrapper, VkLayerDeviceCreateInfo, VkLayerDeviceLink, VkLayerFunction,
        VkLayerInstanceCreateInfo,
    },
    ApiVersion, Extension, ExtensionProperties, Feature, Global, IsCommandEnabled, Layer,
    LayerVulkanCommand, VkLayerInstanceLink,
};

#[derive(Clone)]
enum DispatchKind {
    Global,
    Instance,
    Device,
}

#[derive(Clone)]
struct VulkanCommand {
    proc: vk::PFN_vkVoidFunction,
    dispatch_kind: DispatchKind,
    features: BTreeSet<Feature>,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
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

trait ToVulkanHandle: Sized {
    type Handle: vk::Handle;
    fn into_vulkan_handle(self: Arc<Self>) -> Self::Handle {
        Self::Handle::from_raw((Arc::into_raw(self) as usize).try_into().unwrap())
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn destroy(handle: Self::Handle) {
        let raw_handle: usize = handle.as_raw().try_into().unwrap();
        let ptr = raw_handle as *const Self;
        let last_object = unsafe { Arc::from_raw(ptr) };
        let last_object = Arc::into_inner(last_object).unwrap_or_else(|| {
            panic!("Object {:?} {:#20x} leaks.", Self::Handle::TYPE, raw_handle)
        });
        drop(last_object)
    }
}

pub trait FromVulkanHandle<T: vk::Handle>: Sized {
    /// # Safety
    /// The raw Vulkan handle must be created from the Self type.
    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn from_handle(handle: T) -> Arc<Self> {
        let raw_handle: usize = handle.as_raw().try_into().unwrap();
        let ptr = raw_handle as *const Self;
        unsafe {
            Arc::increment_strong_count(ptr);
            Arc::from_raw(ptr)
        }
    }
}

impl<T: vk::Handle, U: ToVulkanHandle<Handle = T>> FromVulkanHandle<T> for U {}

#[repr(C)]
struct InstanceDispatchTable {
    _magic_number: u32,
    commands: Mutex<BTreeMap<VulkanCommandName, VulkanCommand>>,
}

impl Default for InstanceDispatchTable {
    fn default() -> Self {
        let commands: &BTreeMap<VulkanCommandName, VulkanCommand> = &VULKAN_COMMANDS;
        Self {
            _magic_number: 0x10d0,
            commands: Mutex::new(commands.clone()),
        }
    }
}

#[repr(C)]
pub struct InstanceData {
    // Pointer to dispatch_table, this is the ABI guaranteed by the Vulkan loader.
    _dispatch_table: *const c_void,
    dispatch_table: Pin<Box<InstanceDispatchTable>>,
    version: ApiVersion,
    supported_device_version: Mutex<ApiVersion>,
    enabled_extensions: BTreeSet<Extension>,
    physical_devices: Box<[Del<vk::PhysicalDevice>]>,
    available_device_extensions: Mutex<Option<BTreeSet<Extension>>>,
}
impl InstanceData {
    pub fn set_available_device_extensions(&self, available_device_extensions: &[Extension]) {
        let mut self_available_device_extensions = self.available_device_extensions.lock().unwrap();
        *self_available_device_extensions =
            Some(available_device_extensions.iter().cloned().collect());
    }

    pub fn add_instance_command(&self, command_name: &str, proc: vk::PFN_vkVoidFunction) {
        assert!(self
            .dispatch_table
            .commands
            .lock()
            .unwrap()
            .insert(
                command_name.into(),
                VulkanCommand {
                    proc,
                    dispatch_kind: DispatchKind::Instance,
                    // Added as core 1.0 command, so that this command will always be enabled.
                    features: [ApiVersion::V1_0.into()].into()
                }
            )
            .is_none())
    }

    pub fn set_supported_device_version(&self, api_version: &ApiVersion) {
        *self.supported_device_version.lock().unwrap() = *api_version;
    }
}
impl ToVulkanHandle for InstanceData {
    type Handle = vk::Instance;
}

#[repr(C)]
struct PhysicalDeviceData {
    // Should be the same as the owner VkInstance.
    _dispatch_table: *const c_void,
    owner_instance: Weak<InstanceData>,
    queue_family_properties: Vec<vk::QueueFamilyProperties>,
}
impl ToVulkanHandle for PhysicalDeviceData {
    type Handle = vk::PhysicalDevice;
}

#[repr(C)]
struct DeviceDispatchTable(u32);
impl Default for DeviceDispatchTable {
    fn default() -> Self {
        Self(0xa1de)
    }
}

#[repr(C)]
pub struct DeviceData {
    // A pointer to dispatch_table.
    _dispatch_table: *const c_void,
    dispatch_table: Pin<Box<DeviceDispatchTable>>,
    owner_physical_device: Weak<PhysicalDeviceData>,
    api_version: ApiVersion,
    pub enabled_extensions: BTreeSet<Extension>,
}

impl ToVulkanHandle for DeviceData {
    type Handle = vk::Device;
}

static VULKAN_COMMANDS: Lazy<BTreeMap<VulkanCommandName, VulkanCommand>> = Lazy::new(|| {
    use LayerVulkanCommand::*;
    let commands = [
        (
            CreateInstance.into(),
            VulkanCommand {
                proc: {
                    extern "system" fn create_instance(
                        create_info: *const vk::InstanceCreateInfo,
                        _: *const vk::AllocationCallbacks,
                        instance: *mut vk::Instance,
                    ) -> vk::Result {
                        // TODO: assert that the ICD shouldn't receive a non-zero-length
                        // VkLayerInstanceLink
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
                        let dispatch_table = Box::into_pin(Box::<InstanceDispatchTable>::default());
                        let version = application_info
                            .map(|application_info| {
                                let mut api_version = application_info.api_version;
                                if api_version == 0 {
                                    api_version = vk::API_VERSION_1_0;
                                }
                                api_version.try_into().unwrap()
                            })
                            .unwrap_or(ApiVersion::V1_0);
                        let instance_data: Arc<InstanceData> = Arc::new_cyclic(|instance_data| {
                            let _dispatch_table =
                                dispatch_table.as_ref().get_ref() as *const _ as *const c_void;
                            InstanceData {
                                _dispatch_table,
                                dispatch_table,
                                version,
                                supported_device_version: Mutex::new(version),
                                enabled_extensions,
                                physical_devices: Box::new([Del::new(
                                    Arc::new(PhysicalDeviceData {
                                        _dispatch_table,
                                        owner_instance: instance_data.clone(),
                                        queue_family_properties: vec![vk::QueueFamilyProperties {
                                            queue_flags: vk::QueueFlags::GRAPHICS
                                                | vk::QueueFlags::TRANSFER,
                                            queue_count: 1,
                                            timestamp_valid_bits: 0,
                                            min_image_transfer_granularity: vk::Extent3D::builder()
                                                .width(0)
                                                .height(0)
                                                .depth(0)
                                                .build(),
                                        }],
                                    })
                                    .into_vulkan_handle(),
                                    |handle| unsafe { PhysicalDeviceData::destroy(*handle) },
                                )]),
                                available_device_extensions: Default::default(),
                            }
                        });
                        *unsafe { instance.as_mut() }.unwrap() = instance_data.into_vulkan_handle();
                        vk::Result::SUCCESS
                    }
                    unsafe { std::mem::transmute(create_instance as vk::PFN_vkCreateInstance) }
                },
                dispatch_kind: DispatchKind::Global,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            "vkDestroyDevice".into(),
            VulkanCommand {
                proc: {
                    extern "system" fn destroy_device(
                        device: vk::Device,
                        _: *const vk::AllocationCallbacks,
                    ) {
                        if device == vk::Device::null() {
                            return;
                        }
                        unsafe { DeviceData::destroy(device) };
                    }
                    unsafe { std::mem::transmute(destroy_device as vk::PFN_vkDestroyDevice) }
                },
                dispatch_kind: DispatchKind::Device,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            CreateDevice.into(),
            VulkanCommand {
                proc: {
                    extern "system" fn create_device(
                        physical_device: vk::PhysicalDevice,
                        device_create_info: *const vk::DeviceCreateInfo,
                        _: *const vk::AllocationCallbacks,
                        device: *mut vk::Device,
                    ) -> vk::Result {
                        // TODO: assert that the ICD shouldn't receive a non-zero-length
                        // VkLayerDeviceLink
                        let dispatch_table = Box::into_pin(Box::<DeviceDispatchTable>::default());
                        let physical_device =
                            unsafe { PhysicalDeviceData::from_handle(physical_device) };
                        let instance_data = physical_device.owner_instance.upgrade().unwrap();
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
                            .collect::<BTreeSet<_>>();
                        if let Some(available_device_extensions) = instance_data
                            .available_device_extensions
                            .lock()
                            .unwrap()
                            .as_ref()
                        {
                            if enabled_extensions
                                .iter()
                                .any(|extension| !available_device_extensions.contains(extension))
                            {
                                return vk::Result::ERROR_EXTENSION_NOT_PRESENT;
                            }
                        }
                        let device_data = Arc::new(DeviceData {
                            _dispatch_table: dispatch_table.as_ref().get_ref() as *const _
                                as *const c_void,
                            dispatch_table,
                            owner_physical_device: Arc::downgrade(&physical_device),
                            api_version: instance_data
                                .version
                                .min(*instance_data.supported_device_version.lock().unwrap()),
                            enabled_extensions,
                        });
                        *unsafe { device.as_mut() }.unwrap() = device_data.into_vulkan_handle();
                        vk::Result::SUCCESS
                    }
                    unsafe { std::mem::transmute(create_device as vk::PFN_vkCreateDevice) }
                },
                dispatch_kind: DispatchKind::Instance,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            "vkEnumeratePhysicalDeviceGroups".into(),
            VulkanCommand {
                proc: {
                    extern "system" fn enumerate_physical_device_groups(
                        _: vk::Instance,
                        _physical_device_group_count: *mut u32,
                        _phyiscal_device_group_properties: *mut vk::PhysicalDeviceGroupProperties,
                    ) -> vk::Result {
                        unimplemented!()
                    }
                    unsafe {
                        std::mem::transmute(
                            enumerate_physical_device_groups
                                as vk::PFN_vkEnumeratePhysicalDeviceGroups,
                        )
                    }
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
                proc: {
                    extern "system" fn enumerate_physical_devices(
                        instance: vk::Instance,
                        physical_device_count: *mut u32,
                        physical_devices: *mut vk::PhysicalDevice,
                    ) -> vk::Result {
                        let all_physical_devices = unsafe { InstanceData::from_handle(instance) }
                            .physical_devices
                            .iter()
                            .map(|physical_device_data| **physical_device_data)
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
                    unsafe {
                        std::mem::transmute(
                            enumerate_physical_devices as vk::PFN_vkEnumeratePhysicalDevices,
                        )
                    }
                },
                dispatch_kind: DispatchKind::Instance,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            GetDeviceProcAddr.into(),
            VulkanCommand {
                proc: {
                    extern "system" fn get_device_proc_addr(
                        device: vk::Device,
                        p_name: *const c_char,
                    ) -> vk::PFN_vkVoidFunction {
                        assert_ne!(device, vk::Device::null());
                        let device_data = unsafe { DeviceData::from_handle(device) };
                        let name = unsafe { CStr::from_ptr(p_name) }.to_str().unwrap();
                        let command = VULKAN_COMMANDS.get(&name.into())?;
                        if let DispatchKind::Global | DispatchKind::Instance = command.dispatch_kind
                        {
                            return None;
                        }
                        if !command.features.is_command_enabled(
                            &device_data.api_version,
                            &device_data.enabled_extensions,
                        ) {
                            return None;
                        }
                        command.proc
                    }
                    unsafe {
                        std::mem::transmute(get_device_proc_addr as vk::PFN_vkGetDeviceProcAddr)
                    }
                },
                dispatch_kind: DispatchKind::Device,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            GetInstanceProcAddr.into(),
            VulkanCommand {
                proc: unsafe {
                    std::mem::transmute(get_instance_proc_addr as vk::PFN_vkGetInstanceProcAddr)
                },
                dispatch_kind: DispatchKind::Instance,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            "vkEnumerateDeviceLayerProperties".into(),
            VulkanCommand {
                proc: {
                    extern "system" fn enumerate_device_layer_properties(
                        _: vk::PhysicalDevice,
                        _property_count: *mut u32,
                        _properties: *mut vk::LayerProperties,
                    ) -> vk::Result {
                        unimplemented!()
                    }
                    unsafe {
                        std::mem::transmute(
                            enumerate_device_layer_properties
                                as vk::PFN_vkEnumerateDeviceLayerProperties,
                        )
                    }
                },
                dispatch_kind: DispatchKind::Instance,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            "vkEnumerateDeviceExtensionProperties".into(),
            VulkanCommand {
                proc: {
                    extern "system" fn enumerate_device_extension_properties(
                        physical_device: vk::PhysicalDevice,
                        layer_name: *const c_char,
                        property_count: *mut u32,
                        properties: *mut vk::ExtensionProperties,
                    ) -> vk::Result {
                        assert_ne!(physical_device, vk::PhysicalDevice::null());
                        assert_eq!(layer_name, null());
                        let physical_device_data =
                            unsafe { PhysicalDeviceData::from_handle(physical_device) };
                        let instance_data = physical_device_data.owner_instance.upgrade().unwrap();
                        let available_extensions =
                            instance_data.available_device_extensions.lock().unwrap();
                        let available_extensions = available_extensions.as_ref().expect(concat!(
                            "The caller must set available extensions before querying the ",
                            "available extensions."
                        ));
                        let out_properties = available_extensions
                            .iter()
                            .map(|name| -> vk::ExtensionProperties {
                                ExtensionProperties {
                                    name: name.clone(),
                                    spec_version: 1,
                                }
                                .into()
                            })
                            .collect::<Vec<_>>();
                        let property_count = NonNull::new(property_count).unwrap();
                        unsafe { fill_vk_out_array(&out_properties, property_count, properties) }
                    }
                    unsafe {
                        std::mem::transmute(
                            enumerate_device_extension_properties
                                as vk::PFN_vkEnumerateDeviceExtensionProperties,
                        )
                    }
                },
                dispatch_kind: DispatchKind::Instance,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            "vkDestroyInstance".into(),
            VulkanCommand {
                proc: {
                    extern "system" fn destroy_instance(
                        instance: vk::Instance,
                        _: *const vk::AllocationCallbacks,
                    ) {
                        if instance == vk::Instance::null() {
                            return;
                        }
                        unsafe { InstanceData::destroy(instance) }
                    }
                    unsafe { std::mem::transmute(destroy_instance as vk::PFN_vkDestroyInstance) }
                },
                dispatch_kind: DispatchKind::Instance,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            GetPhysicalDeviceSparseImageFormatProperties2.into(),
            VulkanCommand {
                proc: {
                    extern "system" fn get_physical_device_sparse_image_format_properties2(
                        _: vk::PhysicalDevice,
                        _: *const vk::PhysicalDeviceSparseImageFormatInfo2,
                        _: *mut u32,
                        _: *mut vk::SparseImageFormatProperties2,
                    ) {
                        unimplemented!()
                    }
                    unsafe {
                        std::mem::transmute(
                            get_physical_device_sparse_image_format_properties2
                                as vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
                        )
                    }
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
                proc: {
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
                    unsafe {
                        std::mem::transmute(destroy_surface_khr as vk::PFN_vkDestroySurfaceKHR)
                    }
                },
                dispatch_kind: DispatchKind::Instance,
                features: [Extension::KHRSurface.into()].into(),
            },
        ),
        (
            GetPhysicalDeviceQueueFamilyProperties.into(),
            VulkanCommand {
                proc: {
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
                    unsafe {
                        std::mem::transmute(
                            get_physical_device_queue_family_properties
                                as vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties,
                        )
                    }
                },
                dispatch_kind: DispatchKind::Instance,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            DestroyImage.into(),
            VulkanCommand {
                proc: {
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
                    unsafe { std::mem::transmute(destroy_image as vk::PFN_vkDestroyImage) }
                },
                dispatch_kind: DispatchKind::Device,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
        (
            DestroySwapchainKhr.into(),
            VulkanCommand {
                proc: {
                    extern "system" fn destroy_swapchain_khr(
                        _: vk::Device,
                        _: vk::SwapchainKHR,
                        _: *const vk::AllocationCallbacks,
                    ) {
                        unimplemented!()
                    }
                    unsafe {
                        std::mem::transmute(destroy_swapchain_khr as vk::PFN_vkDestroySwapchainKHR)
                    }
                },
                dispatch_kind: DispatchKind::Device,
                features: [Extension::KHRSwapchain.into()].into(),
            },
        ),
        (
            DestroySamplerYcbcrConversion.into(),
            VulkanCommand {
                proc: {
                    extern "system" fn destroy_sampler_ycbcr_conversion(
                        _: vk::Device,
                        sampler_ycbcr_conversion: vk::SamplerYcbcrConversion,
                        _: *const vk::AllocationCallbacks,
                    ) {
                        if sampler_ycbcr_conversion == vk::SamplerYcbcrConversion::null() {
                            return;
                        }
                        unimplemented!()
                    }
                    unsafe {
                        std::mem::transmute(
                            destroy_sampler_ycbcr_conversion
                                as vk::PFN_vkDestroySamplerYcbcrConversion,
                        )
                    }
                },
                dispatch_kind: DispatchKind::Device,
                features: [ApiVersion::V1_1.into()].into(),
            },
        ),
        (
            GetPhysicalDeviceProperties.into(),
            VulkanCommand {
                proc: {
                    extern "system" fn get_physical_device_properties(
                        physical_device: vk::PhysicalDevice,
                        properties: *mut vk::PhysicalDeviceProperties,
                    ) {
                        let physical_device_data =
                            unsafe { PhysicalDeviceData::from_handle(physical_device) };
                        let instance_data = physical_device_data.owner_instance.upgrade().unwrap();
                        *unsafe { properties.as_mut() }.unwrap() = vk::PhysicalDeviceProperties {
                            api_version: (*instance_data.supported_device_version.lock().unwrap())
                                .into(),
                            driver_version: 0,
                            vendor_id: 0x1AE0,
                            device_id: physical_device_data.as_ref() as *const _ as usize as u32,
                            device_type: vk::PhysicalDeviceType::OTHER,
                            device_name: [0; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE],
                            pipeline_cache_uuid: [0; vk::UUID_SIZE],
                            limits: Default::default(),
                            sparse_properties: Default::default(),
                        };
                    }
                    unsafe {
                        std::mem::transmute(
                            get_physical_device_properties as vk::PFN_vkGetPhysicalDeviceProperties,
                        )
                    }
                },
                dispatch_kind: DispatchKind::Instance,
                features: [ApiVersion::V1_0.into()].into(),
            },
        ),
    ];
    commands.into_iter().collect()
});

unsafe extern "system" fn get_instance_proc_addr(
    instance: vk::Instance,
    p_name: *const i8,
) -> vk::PFN_vkVoidFunction {
    let name = unsafe { CStr::from_ptr(p_name) }.to_str().unwrap();
    let name = name.into();
    let instance_data = if instance == vk::Instance::null() {
        let command = VULKAN_COMMANDS.get(&name)?;
        return match command.dispatch_kind {
            DispatchKind::Global => command.proc,
            _ => None,
        };
    } else {
        unsafe { InstanceData::from_handle(instance) }
    };

    let commands = instance_data.dispatch_table.commands.lock().unwrap();
    let command = commands.get(&name)?;

    match command.dispatch_kind {
        DispatchKind::Global => None,
        DispatchKind::Instance => {
            if !command
                .features
                .is_command_enabled(&instance_data.version, &instance_data.enabled_extensions)
            {
                return None;
            }
            command.proc
        }
        DispatchKind::Device => {
            let available_extensions = instance_data.available_device_extensions.lock().unwrap();
            let enabled = if let Some(available_extensions) = available_extensions.as_ref() {
                command.features.is_command_enabled(
                    &instance_data.supported_device_version.lock().unwrap(),
                    available_extensions,
                )
            } else {
                // available_device_extensions is not set. Assume all extensions to be
                // enabled.
                command.features.iter().any(|feature| {
                    if let Feature::Core(api_version) = feature {
                        if api_version > &instance_data.supported_device_version.lock().unwrap() {
                            return false;
                        }
                    }
                    true
                })
            };
            if !enabled {
                return None;
            }
            command.proc
        }
    }
}

pub type MockLayer = TestLayerWrapper;

pub fn create_entry<T: Layer>() -> ash::Entry {
    unsafe {
        ash::Entry::from_static_fn(vk::StaticFn {
            get_instance_proc_addr: Global::<T>::get_instance_proc_addr,
        })
    }
}

pub struct InstanceContext<T: Layers> {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub icd_entry: ash::Entry,
    pub next_instance_dispatch: ash::Instance,
    _marker: PhantomData<T>,
}

pub trait ArcDelInstanceContextExt<T: Layers>: Sized {
    fn create_device(
        self,
        create_info_builder: impl FnOnce(
            vk::DeviceCreateInfoBuilder,
            &mut dyn for<'a> FnMut(vk::DeviceCreateInfoBuilder<'a>),
        ),
    ) -> VkResult<Del<DeviceContext<T>>>;

    fn default_device(self) -> VkResult<Del<DeviceContext<T>>> {
        self.create_device(|create_info, create_device| create_device(create_info))
    }
}

impl<T: Layers> ArcDelInstanceContextExt<T> for ArcDel<InstanceContext<T>> {
    fn create_device(
        self,
        create_info_builder: impl FnOnce(
            vk::DeviceCreateInfoBuilder,
            &mut dyn for<'a> FnMut(vk::DeviceCreateInfoBuilder<'a>),
        ),
    ) -> VkResult<Del<DeviceContext<T>>> {
        let physical_device = unsafe {
            self.instance
                .enumerate_physical_devices()
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
        };
        assert!(!unsafe {
            self.instance
                .get_physical_device_queue_family_properties(physical_device)
        }
        .is_empty());
        let queue_priorities = [1.0];
        let queue_create_infos = [vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(0)
            .queue_priorities(&queue_priorities)
            .build()];
        let mut layer_links = T::RestLayers::device_links(self.instance.handle());
        let mut layer_create_info = VkLayerDeviceCreateInfo {
            sType: vk::StructureType::LOADER_DEVICE_CREATE_INFO,
            pNext: null(),
            function: VkLayerFunction::VK_LAYER_LINK_INFO,
            u: Default::default(),
        };
        layer_create_info.u.pLayerInfo = layer_links[0].as_mut();
        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .push_next(&mut layer_create_info);
        let mut device: Option<VkResult<ash::Device>> = None;
        create_info_builder(device_create_info, &mut |create_info| {
            assert!(device.is_none());
            device.replace(unsafe {
                self.instance
                    .create_device(physical_device, &create_info, None)
            });
        });
        let device = device.unwrap()?;
        let next_device_dispatch =
            unsafe { ash::Device::load(self.next_instance_dispatch.fp_v1_0(), device.handle()) };
        let device_context = DeviceContext {
            device,
            next_device_dispatch,
            instance_context: self,
        };
        Ok(Del::new(device_context, move |device_context| unsafe {
            device_context.device.destroy_device(None)
        }))
    }
}

pub struct DeviceContext<T: Layers> {
    pub device: ash::Device,
    pub next_device_dispatch: ash::Device,
    pub instance_context: ArcDel<InstanceContext<T>>,
}

pub trait Layers {
    type RestLayers: Layers;
    fn create_entry() -> ash::Entry;
    fn head_instance_link() -> VkLayerInstanceLink;
    fn instance_links() -> Vec<Box<VkLayerInstanceLink>> {
        let mut head = vec![Box::new(Self::head_instance_link())];
        let mut rest = Self::RestLayers::instance_links();
        head.append(&mut rest);
        if head.len() > 1 {
            head[0].pNext = head[1].as_mut();
        }
        head
    }
    fn head_device_link(instance: vk::Instance) -> VkLayerDeviceLink {
        let instance_link = Self::head_instance_link();
        let get_instance_proc_addr = instance_link.pfnNextGetInstanceProcAddr;
        let get_device_proc_addr_name = CString::new("vkGetDeviceProcAddr").unwrap();
        let get_device_proc_addr =
            unsafe { get_instance_proc_addr(instance, get_device_proc_addr_name.as_ptr()) }
                .unwrap();
        let get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr =
            unsafe { std::mem::transmute(get_device_proc_addr) };
        VkLayerDeviceLink {
            pNext: null_mut(),
            pfnNextGetInstanceProcAddr: get_instance_proc_addr,
            pfnNextGetDeviceProcAddr: get_device_proc_addr,
        }
    }
    fn device_links(instance: vk::Instance) -> Vec<Box<VkLayerDeviceLink>> {
        let mut head = vec![Box::new(Self::head_device_link(instance))];
        let mut rest = Self::RestLayers::device_links(instance);
        head.append(&mut rest);
        if head.len() > 1 {
            head[0].pNext = head[1].as_mut();
        }
        head
    }
}

impl Layers for () {
    type RestLayers = ();
    fn create_entry() -> ash::Entry {
        unimplemented!()
    }
    fn head_instance_link() -> VkLayerInstanceLink {
        VkLayerInstanceLink {
            pNext: null_mut(),
            pfnNextGetInstanceProcAddr: get_instance_proc_addr,
            pfnNextGetPhysicalDeviceProcAddr: None,
        }
    }
    fn instance_links() -> Vec<Box<VkLayerInstanceLink>> {
        vec![Box::new(Self::head_instance_link())]
    }
    fn device_links(instance: vk::Instance) -> Vec<Box<VkLayerDeviceLink>> {
        vec![Box::new(Self::head_device_link(instance))]
    }
}

impl<T: Layer> Layers for (T,) {
    type RestLayers = ();
    fn create_entry() -> ash::Entry {
        create_entry::<T>()
    }
    fn head_instance_link() -> VkLayerInstanceLink {
        VkLayerInstanceLink {
            pNext: null_mut(),
            pfnNextGetInstanceProcAddr: Global::<T>::get_instance_proc_addr,
            pfnNextGetPhysicalDeviceProcAddr: None,
        }
    }
}

impl<T: Layer, U: Layer> Layers for (T, U) {
    type RestLayers = (U,);
    fn create_entry() -> ash::Entry {
        <(T,)>::create_entry()
    }
    fn head_instance_link() -> VkLayerInstanceLink {
        <(T,)>::head_instance_link()
    }
}

pub trait InstanceCreateInfoExt {
    fn default_instance<T: Layers>(self) -> ArcDel<InstanceContext<T>>;
}

impl InstanceCreateInfoExt for vk::InstanceCreateInfoBuilder<'_> {
    fn default_instance<T: Layers>(self) -> ArcDel<InstanceContext<T>> {
        let entry = T::create_entry();
        let mut layer_links = T::RestLayers::instance_links();
        let mut layer_create_info = VkLayerInstanceCreateInfo {
            sType: vk::StructureType::LOADER_INSTANCE_CREATE_INFO,
            pNext: null(),
            function: VkLayerFunction::VK_LAYER_LINK_INFO,
            u: Default::default(),
        };
        *unsafe { layer_create_info.u.pLayerInfo.as_mut() } = if layer_links.is_empty() {
            null_mut()
        } else {
            layer_links[0].as_mut()
        };

        let create_instance_info = self.push_next(&mut layer_create_info);
        let instance = unsafe { entry.create_instance(&create_instance_info, None) }.unwrap();
        assert_ne!(instance.handle(), vk::Instance::null());
        let icd_entry = unsafe {
            ash::Entry::from_static_fn(vk::StaticFn {
                get_instance_proc_addr,
            })
        };
        let next_instance_dispatch =
            unsafe { ash::Instance::load(icd_entry.static_fn(), instance.handle()) };
        let context = InstanceContext {
            entry,
            instance,
            icd_entry,
            next_instance_dispatch,
            _marker: Default::default(),
        };
        ArcDel::new(context, move |context| {
            unsafe { context.instance.destroy_instance(None) };
        })
    }
}

mod tests {
    use std::mem::MaybeUninit;

    use super::*;

    #[test]
    fn test_device_data_layout() {
        let ptr = MaybeUninit::<DeviceData>::uninit();
        let ptr = ptr.as_ptr();
        assert_eq!(
            unsafe { std::ptr::addr_of!((*ptr)._dispatch_table) } as usize - ptr as usize,
            0
        );
    }

    #[test]
    fn test_physical_device_data_layout() {
        let ptr = MaybeUninit::<PhysicalDeviceData>::uninit();
        let ptr = ptr.as_ptr();
        assert_eq!(
            unsafe { std::ptr::addr_of!((*ptr)._dispatch_table) } as usize - ptr as usize,
            0
        );
    }

    #[test]
    fn test_instance_data_layout() {
        let ptr = MaybeUninit::<InstanceData>::uninit();
        let ptr = ptr.as_ptr();
        assert_eq!(
            unsafe { std::ptr::addr_of!((*ptr)._dispatch_table) } as usize - ptr as usize,
            0
        );
    }

    #[test]
    fn test_vulkan_handle_layout() {
        #[derive(Default)]
        struct TestDispatchableObject {
            _dispatch_table: usize,
        }

        impl ToVulkanHandle for TestDispatchableObject {
            type Handle = vk::Instance;
        }

        let test_dispatchable_object: Arc<TestDispatchableObject> = Arc::default();
        let test_dispatchable_handle = test_dispatchable_object.clone().into_vulkan_handle();
        assert_eq!(
            test_dispatchable_object.as_ref() as *const _ as usize,
            test_dispatchable_handle.as_raw() as usize
        );
        drop(test_dispatchable_object);
        unsafe { TestDispatchableObject::destroy(test_dispatchable_handle) };
    }
}
