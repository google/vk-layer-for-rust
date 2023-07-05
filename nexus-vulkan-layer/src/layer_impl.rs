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

use std::{
    collections::{BTreeSet, HashMap},
    ffi::{CStr, CString},
    mem::MaybeUninit,
    ptr::{null, null_mut},
    sync::{Arc, Mutex},
};

use ash::{
    prelude::VkResult,
    vk::{self, Handle},
};
use bumpalo::Bump;

use crate::{
    bindings::{
        cros_gralloc_handle, AHardwareBuffer, AHardwareBuffer_Format,
        AHardwareBuffer_getNativeHandle,
    },
    cros_gralloc_helpers::cros_gralloc_convert_handle,
};
use log::{error, info, warn};
use vulkan_layer::{
    auto_deviceinfo_impl, auto_globalhooksinfo_impl, auto_instanceinfo_impl, DeviceHooks,
    Extension, ExtensionProperties, GlobalHooks, InstanceHooks, Layer, LayerResult,
    VkLayerDeviceLink, VulkanBaseInStructChain, VulkanBaseOutStructChain,
};

mod minigbm_bindings {
    #![allow(dead_code)]
    /* Use flags */
    pub const BO_USE_NONE: u64 = 0;
    pub const BO_USE_SCANOUT: u64 = 1u64 << 0;
    pub const BO_USE_CURSOR: u64 = 1u64 << 1;
    pub const BO_USE_CURSOR_64X64: u64 = BO_USE_CURSOR;
    pub const BO_USE_RENDERING: u64 = 1u64 << 2;
    pub const BO_USE_LINEAR: u64 = 1u64 << 4;
    pub const BO_USE_TEXTURE: u64 = 1u64 << 5;
    pub const BO_USE_CAMERA_WRITE: u64 = 1u64 << 6;
    pub const BO_USE_CAMERA_READ: u64 = 1u64 << 7;
    pub const BO_USE_PROTECTED: u64 = 1u64 << 8;
    pub const BO_USE_SW_READ_OFTEN: u64 = 1u64 << 9;
    pub const BO_USE_SW_READ_RARELY: u64 = 1u64 << 10;
    pub const BO_USE_SW_WRITE_OFTEN: u64 = 1u64 << 11;
    pub const BO_USE_SW_WRITE_RARELY: u64 = 1u64 << 12;
    pub const BO_USE_HW_VIDEO_DECODER: u64 = 1u64 << 13;
    pub const BO_USE_HW_VIDEO_ENCODER: u64 = 1u64 << 14;
    pub const BO_USE_TEST_ALLOC: u64 = 1u64 << 15;
    pub const BO_USE_FRONT_RENDERING: u64 = 1u64 << 16;
    pub const BO_USE_RENDERSCRIPT: u64 = 1u64 << 17;
    pub const BO_USE_GPU_DATA_BUFFER: u64 = 1u64 << 18;
}

trait CrosGrallocHandleExt {
    fn get_vk_format(&self) -> vk::Format;
    fn get_vk_allocation_size(&self) -> u64;
}

impl CrosGrallocHandleExt for cros_gralloc_handle {
    fn get_vk_format(&self) -> vk::Format {
        match AHardwareBuffer_Format(self.droid_format as u32) {
            AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT => {
                vk::Format::R16G16B16A16_SFLOAT
            }
            AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM => {
                vk::Format::R8G8B8A8_UNORM
            }
            _ => unimplemented!("Unsupported AHardwareBuffer format: {}", self.droid_format),
        }
    }

    fn get_vk_allocation_size(&self) -> u64 {
        self.total_size - self.reserved_region_size
    }
}

fn find_in_struct<T: vk::TaggedStructure>(
    chain: Option<&vk::BaseInStructure>,
) -> impl Iterator<Item = &T> {
    let iter: VulkanBaseInStructChain = chain.into();
    iter.filter_map(|base_in_structure| {
        if base_in_structure.s_type == T::STRUCTURE_TYPE {
            Some(unsafe { std::mem::transmute(base_in_structure) })
        } else {
            None
        }
    })
}

fn find_out_struct<T: vk::TaggedStructure>(
    chain: Option<&mut vk::BaseOutStructure>,
) -> impl Iterator<Item = &mut T> {
    let iter: VulkanBaseOutStructChain = chain.into();
    iter.filter_map(|base_out_structure| {
        if base_out_structure.s_type == T::STRUCTURE_TYPE {
            Some(unsafe { std::mem::transmute(base_out_structure) })
        } else {
            None
        }
    })
}

// Add wrapper to some Vulkan structs with #[repr(transparent)] so that we can add trait to those
// structs, but can still use them as the wrapped struct.

#[repr(transparent)]
struct NativeBufferANDROIDWrapper(vk::NativeBufferANDROID);

unsafe impl vk::ExtendsImageCreateInfo for NativeBufferANDROIDWrapper {}

impl From<vk::NativeBufferANDROID> for NativeBufferANDROIDWrapper {
    fn from(value: vk::NativeBufferANDROID) -> Self {
        Self(value)
    }
}

#[repr(transparent)]
struct SwapchainImageCreateInfoANDROIDWrapper(vk::SwapchainImageCreateInfoANDROID);

unsafe impl vk::ExtendsImageCreateInfo for SwapchainImageCreateInfoANDROIDWrapper {}

impl From<vk::SwapchainImageCreateInfoANDROID> for SwapchainImageCreateInfoANDROIDWrapper {
    fn from(value: vk::SwapchainImageCreateInfoANDROID) -> Self {
        Self(value)
    }
}

pub(crate) struct NexusLayer;

pub(crate) struct NexusInstanceInfo {
    instance: vk::Instance,
}

#[auto_instanceinfo_impl]
impl InstanceHooks for NexusInstanceInfo {
    fn get_physical_device_image_format_properties2(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_image_format_info: &vk::PhysicalDeviceImageFormatInfo2,
        image_format_properties: &mut vk::ImageFormatProperties2,
    ) -> LayerResult<VkResult<()>> {
        let p_next =
            unsafe { (image_format_properties.p_next as *mut vk::BaseOutStructure).as_mut() };
        for _ in find_out_struct::<vk::AndroidHardwareBufferUsageANDROID>(p_next) {
            unimplemented!("vk::AndroidHardwareBufferUsageANDROID in get_physical_device_image_format_properties2 unimplemented");
        }
        LayerResult::Unhandled
    }

    fn get_physical_device_properties2(
        &self,
        _physical_device: vk::PhysicalDevice,
        properties: &mut vk::PhysicalDeviceProperties2,
    ) -> LayerResult<()> {
        let p_next = unsafe { (properties.p_next as *mut vk::BaseOutStructure).as_mut() };
        for _ in find_out_struct::<vk::PhysicalDevicePresentationPropertiesANDROID>(p_next) {
            // TODO(nexus): required for VK_ANDROID_native_buffer
            error!("vk::PhysicalDevicePresentationPropertiesANDROID in get_physical_device_properties2 unimplemented");
        }
        LayerResult::Unhandled
    }

    fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
        layer_device_link: &VkLayerDeviceLink,
        allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Device>> {
        let mut create_info = *create_info;
        if create_info.pp_enabled_extension_names == null() {
            return LayerResult::Unhandled;
        }
        let mut extensions = unsafe {
            std::slice::from_raw_parts(
                create_info.pp_enabled_extension_names,
                create_info.enabled_extension_count.try_into().unwrap(),
            )
        }
        .iter()
        .map(|extension_name| unsafe { CStr::from_ptr(*extension_name) })
        .collect::<BTreeSet<_>>();
        if !extensions.contains(vk::AndroidNativeBufferFn::name()) {
            return LayerResult::Unhandled;
        }
        extensions.insert(vk::KhrExternalMemoryFdFn::name());
        let extensions = extensions
            .iter()
            .map(|extension_name| extension_name.as_ptr())
            .collect::<Vec<_>>();
        create_info.pp_enabled_extension_names = extensions.as_ptr();
        create_info.enabled_extension_count = extensions.len().try_into().unwrap();

        let create_device_name: CString = CString::new("vkCreateDevice").unwrap();
        let create_device = unsafe {
            (layer_device_link.pfnNextGetInstanceProcAddr)(
                self.instance,
                create_device_name.as_ptr(),
            )
        }
        .unwrap();
        let create_device: vk::PFN_vkCreateDevice = unsafe { std::mem::transmute(create_device) };
        let mut device = MaybeUninit::<vk::Device>::uninit();
        let allocator = allocator.map_or_else(null, |allocator| allocator as *const _);
        let res = unsafe {
            create_device(
                physical_device,
                &create_info,
                allocator,
                device.as_mut_ptr(),
            )
        };
        if res != vk::Result::SUCCESS {
            LayerResult::Handled(Err(res))
        } else {
            LayerResult::Handled(Ok(unsafe { device.assume_init() }))
        }
    }
}

const VK_GET_SWAPCHAIN_GRALLOC_USAGE2_ANDROID_NAME: &str = "vkGetSwapchainGrallocUsage2ANDROID";

struct NativeBufferImage {
    memory: vk::DeviceMemory,
}

pub(crate) struct NexusDeviceInfo {
    device: Arc<ash::Device>,
    get_swapchain_gralloc_usage2_android: Option<vk::PFN_vkGetSwapchainGrallocUsage2ANDROID>,
    native_buffer_images: Mutex<HashMap<vk::Image, NativeBufferImage>>,
}

impl NexusDeviceInfo {
    fn create_native_buffer_image(
        &self,
        create_info: vk::ImageCreateInfoBuilder<'_>,
        native_buffer: &vk::NativeBufferANDROID,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::Image> {
        use std::collections::hash_map::Entry;
        let cros_gralloc_handle =
            match unsafe { cros_gralloc_convert_handle(native_buffer.handle as *const _) } {
                Some(handle) => handle,
                None => {
                    error!("Failed to convert the native buffer to cros gralloc handle.");
                    return Err(vk::Result::ERROR_INVALID_EXTERNAL_HANDLE_KHR);
                }
            };

        let p_next_chain = unsafe { (create_info.p_next as *const vk::BaseInStructure).as_ref() };
        let next_chain: VulkanBaseInStructChain = p_next_chain.into();
        for next_element in next_chain {
            error!(
                "Unsupported native buffer image pNext element: {}",
                next_element.s_type.as_raw()
            );
        }
        let mut external_memory_image_create_info = vk::ExternalMemoryImageCreateInfo::builder()
            .handle_types(vk::ExternalMemoryHandleTypeFlags::OPAQUE_FD);
        let mut create_info = create_info.push_next(&mut external_memory_image_create_info);
        let expected_format = cros_gralloc_handle.get_vk_format();
        if create_info.format != expected_format {
            warn!("Native buffer image format mismatch: the gralloc buffer is created with {:?}, the client wants to create the VkImage with {:?}. Will use the expected format {:?}.", expected_format, create_info.format, expected_format);
            create_info = create_info.format(expected_format);
        }
        let image = match unsafe { self.device.create_image(&create_info, allocation_callbacks) } {
            Ok(image) => image,
            Err(e) => {
                error!(
                    "Failed to create the VkImage from the driver for a native buffer VkImage: {}",
                    e
                );
                return Err(e);
            }
        };
        let mut allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(cros_gralloc_handle.get_vk_allocation_size())
            .memory_type_index(cros_gralloc_handle.memory_type_index);
        let mut import_memory_fd = vk::ImportMemoryFdInfoKHR::builder()
            .handle_type(vk::ExternalMemoryHandleTypeFlags::OPAQUE_FD)
            .fd(cros_gralloc_handle.fds[0]);
        allocate_info = allocate_info.push_next(&mut import_memory_fd);
        let mut dedicated_allocation = vk::MemoryDedicatedAllocateInfo::builder().image(image);
        if cros_gralloc_handle.dedicated_allocation != 0 {
            allocate_info = allocate_info.push_next(&mut dedicated_allocation);
        }
        let memory = match unsafe {
            self.device
                .allocate_memory(&allocate_info, allocation_callbacks)
        } {
            Ok(memory) => memory,
            Err(e) => {
                error!(
                    "Failed to allocate VkDeviceMemory for the native buffer image: {}",
                    e
                );
                unsafe { self.device.destroy_image(image, allocation_callbacks) };
                return Err(e);
            }
        };
        if let Err(e) = unsafe { self.device.bind_image_memory(image, memory, 0) } {
            error!(
                "Failed to bind the native buffer image to the memory: {}",
                e
            );
            unsafe { self.device.free_memory(memory, allocation_callbacks) };
            unsafe { self.device.destroy_image(image, allocation_callbacks) };
            return Err(e);
        }
        let mut native_buffer_images = self.native_buffer_images.lock().unwrap();
        let entry = match native_buffer_images.entry(image) {
            Entry::Occupied(_) => panic!("Image {:#010x} already existed.", image.as_raw()),
            Entry::Vacant(e) => e,
        };
        entry.insert(NativeBufferImage { memory });
        Ok(image)
    }
}

#[auto_deviceinfo_impl]
impl DeviceHooks for NexusDeviceInfo {
    fn get_device_proc_addr(&self, name: &str) -> LayerResult<vk::PFN_vkVoidFunction> {
        if name == VK_GET_SWAPCHAIN_GRALLOC_USAGE2_ANDROID_NAME
            && self.get_swapchain_gralloc_usage2_android.is_none()
        {
            return LayerResult::Handled(None);
        }
        LayerResult::Unhandled
    }

    fn create_image(
        &self,
        create_info: &vk::ImageCreateInfo,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Image>> {
        info!("Create image from {}", NexusLayer::LAYER_NAME);
        let p_next_chain = unsafe { (create_info.p_next as *const vk::BaseInStructure).as_ref() };
        let next_chain: VulkanBaseInStructChain = p_next_chain.into();
        let queue_family_indices = unsafe {
            std::slice::from_raw_parts(
                create_info.p_queue_family_indices,
                create_info.queue_family_index_count.try_into().unwrap(),
            )
        };
        let mut native_buffer: Option<&vk::NativeBufferANDROID> = None;
        let bump = Bump::new();
        let mut local_create_info = vk::ImageCreateInfo::builder()
            .flags(create_info.flags)
            .image_type(create_info.image_type)
            .format(create_info.format)
            .extent(create_info.extent)
            .mip_levels(create_info.mip_levels)
            .array_layers(create_info.array_layers)
            .samples(create_info.samples)
            .tiling(create_info.tiling)
            .usage(create_info.usage)
            .sharing_mode(create_info.sharing_mode)
            .queue_family_indices(queue_family_indices)
            .initial_layout(create_info.initial_layout);
        for p_next in next_chain {
            let s_type = p_next.s_type;
            let p_next = p_next as *const vk::BaseInStructure;
            unsafe {
                ash::match_in_struct!(match p_next {
                    p_next @ vk::ExternalFormatANDROID => {
                        assert_eq!(
                            p_next.external_format, 0,
                            "Unhandled external format: {}",
                            p_next.external_format
                        );
                        assert_ne!(local_create_info.format, vk::Format::UNDEFINED);
                    }
                    p_next @ vk::BufferCollectionImageCreateInfoFUCHSIA => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::DedicatedAllocationImageCreateInfoNV => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ExportMetalObjectCreateInfoEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ExternalFormatANDROID => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null_mut();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ExternalMemoryImageCreateInfo => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ExternalMemoryImageCreateInfoNV => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ImageCompressionControlEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ImageDrmFormatModifierExplicitCreateInfoEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ImageDrmFormatModifierListCreateInfoEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ImageFormatListCreateInfo => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ImageStencilUsageCreateInfo => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ImageSwapchainCreateInfoKHR => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ImportMetalIOSurfaceInfoEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::ImportMetalTextureInfoEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::OpaqueCaptureDescriptorDataCreateInfoEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::OpticalFlowImageFormatInfoNV => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::VideoProfileListInfoKHR => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                    }
                    p_next @ vk::NativeBufferANDROID => {
                        native_buffer = Some(p_next);
                    }
                    p_next @ vk::SwapchainImageCreateInfoANDROID => {
                        let p_next: &mut SwapchainImageCreateInfoANDROIDWrapper =
                            bump.alloc((*p_next).into());
                        p_next.0.p_next = null();
                        local_create_info = local_create_info.push_next(p_next);
                        // TODO(nexus): required for VK_ANDROID_native_buffer
                        error!("vk::SwapchainImageCreateInfoANDROID in create_image unimplemented");
                    }
                    _ => {
                        error!(
                            "Dropped unknown pNext element in VkImageCreateInfo: {}",
                            s_type.as_raw()
                        );
                    }
                })
            }
        }
        if let Some(native_buffer) = native_buffer {
            return LayerResult::Handled(self.create_native_buffer_image(
                local_create_info,
                native_buffer,
                allocation_callbacks,
            ));
        }
        LayerResult::Handled(unsafe {
            self.device
                .create_image(&local_create_info, allocation_callbacks)
        })
    }

    fn get_device_image_memory_requirements(
        &self,
        info: &vk::DeviceImageMemoryRequirements,
        _p_memory_requirements: &mut vk::MemoryRequirements2,
    ) -> LayerResult<()> {
        let image_create_info = unsafe { info.p_create_info.as_ref() }.unwrap();
        let image_create_info_chain =
            unsafe { (image_create_info.p_next as *const vk::BaseInStructure).as_ref() };
        for _external_format_android in
            find_in_struct::<vk::ExternalFormatANDROID>(image_create_info_chain)
        {
            // TODO(nexus): required for VK_ANDROID_external_memory_android_hardware_buffer
            unimplemented!(
                "vk::ExternalFormatANDROID in get_device_image_memory_requirements unimplemented"
            );
        }
        LayerResult::Unhandled
    }

    fn get_device_image_sparse_memory_requirements(
        &self,
        info: &vk::DeviceImageMemoryRequirements,
        _p_sparse_memory_requirements: Option<&mut [vk::SparseImageMemoryRequirements2]>,
    ) -> LayerResult<()> {
        let image_create_info = unsafe { info.p_create_info.as_ref() }.unwrap();
        let image_create_info_chain =
            unsafe { (image_create_info.p_next as *const vk::BaseInStructure).as_ref() };
        for _external_format_android in
            find_in_struct::<vk::ExternalFormatANDROID>(image_create_info_chain)
        {
            unimplemented!("vk::ExternalFormatANDROID in get_device_image_sparse_memory_requirements unimplemented");
        }
        LayerResult::Unhandled
    }

    fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &vk::SamplerYcbcrConversionCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SamplerYcbcrConversion>> {
        let p_next_chain = unsafe { (create_info.p_next as *const vk::BaseInStructure).as_ref() };
        for _external_format_android in find_in_struct::<vk::ExternalFormatANDROID>(p_next_chain) {
            unimplemented!(
                "vk::ExternalFormatANDROID in create_sampler_ycbcr_conversion unimplemented"
            );
        }
        LayerResult::Unhandled
    }

    fn get_android_hardware_buffer_properties_android(
        &self,
        buffer: *const vk::AHardwareBuffer,
        properties: &mut vk::AndroidHardwareBufferPropertiesANDROID,
    ) -> LayerResult<VkResult<()>> {
        let native_buffer = unsafe { AHardwareBuffer_getNativeHandle(buffer as *const _) };
        if native_buffer.is_null() {
            error!("Failed to get the native handle out of the AHardwareBuffer.");
            return LayerResult::Handled(Err(vk::Result::ERROR_INVALID_EXTERNAL_HANDLE_KHR));
        }
        let cros_gralloc_handle = unsafe { cros_gralloc_convert_handle(native_buffer) };
        let cros_gralloc_handle = match cros_gralloc_handle {
            Some(handle) => handle,
            None => {
                error!("Failed to convert the native buffer to cros gralloc handle.");
                return LayerResult::Handled(Err(vk::Result::ERROR_INVALID_EXTERNAL_HANDLE_KHR));
            }
        };
        assert!(cros_gralloc_handle.vulkan_backed != 0);
        let properties_p_next_chain: VulkanBaseOutStructChain =
            unsafe { (properties.p_next as *mut vk::BaseOutStructure).as_mut() }.into();
        for p_next in properties_p_next_chain {
            fn get_vk_format_features(
                cros_gralloc_handle: &cros_gralloc_handle,
            ) -> vk::FormatFeatureFlags {
                let use_flags = cros_gralloc_handle.use_flags;
                let mut features = vk::FormatFeatureFlags::default();
                use minigbm_bindings::*;
                if (use_flags & (BO_USE_SCANOUT | BO_USE_TEXTURE)) != 0 {
                    features |= vk::FormatFeatureFlags::SAMPLED_IMAGE;
                }
                if (use_flags & (BO_USE_SCANOUT | BO_USE_RENDERING)) != 0 {
                    features |= vk::FormatFeatureFlags::COLOR_ATTACHMENT;
                }
                features
            }
            fn get_vk_format_features2(
                cros_gralloc_handle: &cros_gralloc_handle,
            ) -> vk::FormatFeatureFlags2 {
                vk::FormatFeatureFlags2::from_raw(
                    get_vk_format_features(cros_gralloc_handle).as_raw().into(),
                )
            }
            let p_next_ptr = p_next as *mut vk::BaseOutStructure;
            unsafe {
                ash::match_out_struct!(match p_next_ptr {
                    format_properties2 @ vk::AndroidHardwareBufferFormatProperties2ANDROID => {
                        let p_next = format_properties2.p_next;
                        *format_properties2 =
                            *vk::AndroidHardwareBufferFormatProperties2ANDROID::builder()
                                .format(cros_gralloc_handle.get_vk_format())
                                .external_format(0)
                                .format_features(get_vk_format_features2(cros_gralloc_handle))
                                .sampler_ycbcr_conversion_components(vk::ComponentMapping {
                                    r: vk::ComponentSwizzle::IDENTITY,
                                    g: vk::ComponentSwizzle::IDENTITY,
                                    b: vk::ComponentSwizzle::IDENTITY,
                                    a: vk::ComponentSwizzle::IDENTITY,
                                })
                                .suggested_ycbcr_model(
                                    vk::SamplerYcbcrModelConversion::RGB_IDENTITY,
                                )
                                .suggested_ycbcr_range(vk::SamplerYcbcrRange::ITU_FULL)
                                .suggested_x_chroma_offset(vk::ChromaLocation::COSITED_EVEN)
                                .suggested_y_chroma_offset(vk::ChromaLocation::COSITED_EVEN);
                        format_properties2.p_next = p_next;
                    }
                    format_properties @ vk::AndroidHardwareBufferFormatPropertiesANDROID => {
                        let p_next = format_properties.p_next;
                        *format_properties =
                            *vk::AndroidHardwareBufferFormatPropertiesANDROID::builder()
                                .format(cros_gralloc_handle.get_vk_format())
                                .external_format(0)
                                .format_features(get_vk_format_features(cros_gralloc_handle))
                                .sampler_ycbcr_conversion_components(vk::ComponentMapping {
                                    r: vk::ComponentSwizzle::IDENTITY,
                                    g: vk::ComponentSwizzle::IDENTITY,
                                    b: vk::ComponentSwizzle::IDENTITY,
                                    a: vk::ComponentSwizzle::IDENTITY,
                                })
                                .suggested_ycbcr_model(
                                    vk::SamplerYcbcrModelConversion::RGB_IDENTITY,
                                )
                                .suggested_ycbcr_range(vk::SamplerYcbcrRange::ITU_FULL)
                                .suggested_x_chroma_offset(vk::ChromaLocation::COSITED_EVEN)
                                .suggested_y_chroma_offset(vk::ChromaLocation::COSITED_EVEN);
                        format_properties.p_next = p_next;
                    }
                    _ => {
                        error!(
                            concat!(
                                "Unsupported pNext chain element for ",
                                "VkAndroidHardwareBufferPropertiesANDROID: {}"
                            ),
                            p_next.s_type.as_raw()
                        );
                    }
                })
            };
        }
        properties.allocation_size = cros_gralloc_handle.get_vk_allocation_size();
        properties.memory_type_bits = 1 << cros_gralloc_handle.memory_type_index;
        LayerResult::Handled(Ok(()))
    }

    fn get_memory_android_hardware_buffer_android(
        &self,
        _p_info: &vk::MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> LayerResult<VkResult<*mut vk::AHardwareBuffer>> {
        unimplemented!("get_memory_android_hardware_buffer_android unimplemented");
    }

    fn allocate_memory(
        &self,
        allocate_info: &vk::MemoryAllocateInfo,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::DeviceMemory>> {
        let p_next_chain: VulkanBaseInStructChain =
            unsafe { (allocate_info.p_next as *const vk::BaseInStructure).as_ref() }.into();
        let bump = Bump::new();
        let mut local_allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(allocate_info.allocation_size)
            .memory_type_index(allocate_info.memory_type_index);
        for p_next in p_next_chain {
            let p_next_ptr = p_next as *const vk::BaseInStructure;
            unsafe {
                ash::match_in_struct!(match p_next_ptr {
                    p_next @ vk::DedicatedAllocationMemoryAllocateInfoNV => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ExportMemoryAllocateInfo => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ExportMemoryAllocateInfoNV => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ExportMemoryWin32HandleInfoKHR => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ExportMemoryWin32HandleInfoNV => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ExportMetalObjectCreateInfoEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ImportMemoryBufferCollectionFUCHSIA => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ImportMemoryFdInfoKHR => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ImportMemoryHostPointerInfoEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ImportMemoryWin32HandleInfoKHR => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ImportMemoryWin32HandleInfoNV => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ImportMemoryZirconHandleInfoFUCHSIA => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ImportMetalBufferInfoEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::MemoryAllocateFlagsInfo => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::MemoryDedicatedAllocateInfo => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::MemoryOpaqueCaptureAddressAllocateInfo => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::MemoryPriorityAllocateInfoEXT => {
                        let p_next = bump.alloc(*p_next);
                        p_next.p_next = null();
                        local_allocate_info = local_allocate_info.push_next(p_next);
                    }
                    p_next @ vk::ImportAndroidHardwareBufferInfoANDROID => {
                        fn get_opaque_fd_from_ahb(ahb: *const AHardwareBuffer) -> VkResult<i32> {
                            let native_buffer = unsafe { AHardwareBuffer_getNativeHandle(ahb) };
                            if native_buffer.is_null() {
                                error!(
                                    "Failed to get the native handle out of the AHardwareBuffer."
                                );
                                return Err(vk::Result::ERROR_INVALID_EXTERNAL_HANDLE_KHR);
                            }
                            let cros_gralloc_handle =
                                unsafe { cros_gralloc_convert_handle(native_buffer) };
                            let cros_gralloc_handle = match cros_gralloc_handle {
                                Some(handle) => handle,
                                None => {
                                    error!("Failed to convert the native buffer to cros gralloc handle.");
                                    return Err(vk::Result::ERROR_INVALID_EXTERNAL_HANDLE_KHR);
                                }
                            };
                            Ok(cros_gralloc_handle.fds[0])
                        }
                        let fd = match get_opaque_fd_from_ahb(p_next.buffer as *const _) {
                            Ok(fd) => fd,
                            Err(e) => return LayerResult::Handled(Err(e)),
                        };
                        local_allocate_info = local_allocate_info.push_next(
                            bump.alloc(
                                vk::ImportMemoryFdInfoKHR::builder()
                                    .handle_type(vk::ExternalMemoryHandleTypeFlags::OPAQUE_FD)
                                    .fd(fd)
                                    .build(),
                            ),
                        );
                    }
                    _ => {
                        error!(
                            "Dropped unknown pNext chain element in VkMemoryAllocateInfo: {}",
                            p_next.s_type.as_raw()
                        );
                    }
                })
            }
        }
        LayerResult::Handled(unsafe {
            self.device
                .allocate_memory(&local_allocate_info, allocation_callbacks)
        })
    }

    // VK_ANDROID_native_buffer commands
    fn get_swapchain_gralloc_usage_android(
        &self,
        _format: vk::Format,
        _image_usage: vk::ImageUsageFlags,
    ) -> LayerResult<VkResult<std::ffi::c_int>> {
        // TODO(nexus): required for VK_ANDROID_native_buffer
        error!("get_swapchain_gralloc_usage_android unimplemented");
        LayerResult::Unhandled
    }

    fn get_swapchain_gralloc_usage2_android(
        &self,
        _format: vk::Format,
        _image_usage: vk::ImageUsageFlags,
        _swapchain_image_usage: vk::SwapchainImageUsageFlagsANDROID,
        _gralloc_consumer_usage: &mut u64,
    ) -> LayerResult<VkResult<u64>> {
        unimplemented!()
    }

    // vkGetSwapchainGrallocUsage3ANDROID is not yet in the public vk.xml

    fn acquire_image_android(
        &self,
        _image: vk::Image,
        _native_fence_fd: std::ffi::c_int,
        _semaphore: vk::Semaphore,
        _fence: vk::Fence,
    ) -> LayerResult<VkResult<()>> {
        // TODO(nexus): required
        error!("acquire_image_android unimplemented");
        LayerResult::Unhandled
    }

    fn queue_signal_release_image_android(
        &self,
        _queue: vk::Queue,
        _p_wait_semaphores: &[vk::Semaphore],
        _image: vk::Image,
    ) -> LayerResult<VkResult<std::ffi::c_int>> {
        // TODO(nexus): required
        error!("queue_signal_release_image_android unimplemented");
        LayerResult::Unhandled
    }

    // vkBindImageMemory2 can be called with VkNativeBufferANDROID extended.
    fn bind_image_memory2(
        &self,
        bind_infos: &[vk::BindImageMemoryInfo],
    ) -> LayerResult<VkResult<()>> {
        for bind_info in bind_infos {
            let p_next_chain = unsafe { (bind_info.p_next as *const vk::BaseInStructure).as_ref() };
            for _native_buffer_android in find_in_struct::<vk::NativeBufferANDROID>(p_next_chain) {
                unimplemented!("vk::NativeBufferANDROID in bind_image_memory2 unimplemented");
            }
        }
        LayerResult::Unhandled
    }

    // fn create_shader_module(
    //     &self,
    //     create_info: &vk::ShaderModuleCreateInfo,
    //     _p_allocator: Option<&vk::AllocationCallbacks>,
    // ) -> LayerResult<VkResult<vk::ShaderModule>> { let code = unsafe {
    //   std::slice::from_raw_parts(create_info.p_code, create_info.code_size / 4) };
    //   info!("create_shader_module:"); info!("{:#?}", code); LayerResult::Unhandled
    // }

    fn destroy_image(
        &self,
        image: vk::Image,
        allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        if let Some(image) = self.native_buffer_images.lock().unwrap().remove(&image) {
            unsafe { self.device.free_memory(image.memory, allocator) };
        }
        LayerResult::Unhandled
    }

    fn free_memory(
        &self,
        memory: vk::DeviceMemory,
        _allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        error!("vkFreeMemory: memory = {:#010x}", memory.as_raw());
        LayerResult::Unhandled
    }
}

#[auto_globalhooksinfo_impl]
impl GlobalHooks for NexusLayer {}

impl Layer for NexusLayer {
    const LAYER_NAME: &'static str = "VK_LAYER_GOOGLE_nexus";
    const LAYER_DESCRIPTION: &'static str =
        "Emulate some Android Vulkan extensions support where the ICD doesn't support them.";
    const IMPLEMENTATION_VERSION: u32 = 1;
    const SPEC_VERSION: u32 = vk::API_VERSION_1_1;
    const DEVICE_EXTENSIONS: &'static [ExtensionProperties] = &[ExtensionProperties {
        name: Extension::ANDROIDExternalMemoryAndroidHardwareBuffer,
        spec_version: 5,
    }];

    type GlobalHooksInfo = Self;
    type InstanceInfo = NexusInstanceInfo;
    type DeviceInfo = NexusDeviceInfo;
    type InstanceInfoContainer = NexusInstanceInfo;
    type DeviceInfoContainer = NexusDeviceInfo;

    fn get_global_hooks_info(&self) -> &Self::GlobalHooksInfo {
        self
    }

    fn create_instance_info(
        &self,
        _: &vk::InstanceCreateInfo,
        _: Option<&vk::AllocationCallbacks>,
        instance: Arc<ash::Instance>,
        _: vk::PFN_vkGetInstanceProcAddr,
    ) -> Self::InstanceInfo {
        info!("create instance from {}", Self::LAYER_NAME);
        NexusInstanceInfo {
            instance: instance.handle(),
        }
    }

    fn create_device_info(
        &self,
        _: vk::PhysicalDevice,
        _: &vk::DeviceCreateInfo,
        _: Option<&vk::AllocationCallbacks>,
        device: Arc<ash::Device>,
        next_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    ) -> Self::DeviceInfo {
        info!("create device from {}", Self::LAYER_NAME);
        let func_name_c_str = CString::new(VK_GET_SWAPCHAIN_GRALLOC_USAGE2_ANDROID_NAME).unwrap();
        let get_swapchain_gralloc_usage2_android =
            unsafe { next_get_device_proc_addr(device.handle(), func_name_c_str.as_ptr()) }
                .map(|src| unsafe { std::mem::transmute(src) });
        NexusDeviceInfo {
            device,
            get_swapchain_gralloc_usage2_android,
            native_buffer_images: Default::default(),
        }
    }
}

#[cfg(windows)]
mod windows {
    use std::ffi::OsStr;
    use std::io::Write;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::System::Diagnostics::Debug::OutputDebugStringW;

    pub(super) struct DebugOutputWriter;
    impl Write for DebugOutputWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let out = String::from_utf8_lossy(buf);
            let out: &str = &out;
            let out = OsStr::new(&out)
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>();
            unsafe { OutputDebugStringW(out.as_ptr()) };
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}

impl Default for NexusLayer {
    fn default() -> Self {
        #[cfg(target_os = "android")]
        {
            #[cfg(feature = "aosp_build")]
            logger::init(
                logger::Config::default()
                    .with_tag_on_device(Self::LAYER_NAME)
                    .with_min_level(log::Level::Trace),
            );
            #[cfg(not(feature = "aosp_build"))]
            android_logger::init_once(
                android_logger::Config::default()
                    .with_tag(Self::LAYER_NAME)
                    .format(|f, record| {
                        write!(
                            f,
                            "{}({}:{}): {}",
                            record.module_path().unwrap_or("<Unknown module>"),
                            record.file().unwrap_or("<Unknown file>"),
                            record.line().unwrap_or(0),
                            record.args()
                        )
                    })
                    .with_max_level(log::LevelFilter::Trace),
            );
            std::panic::set_hook(Box::new(|panic_info| {
                let location = if let Some(location) = panic_info.location() {
                    format!("{}", location)
                } else {
                    "unknown location".to_owned()
                };
                let payload = panic_info
                    .payload()
                    .downcast_ref::<&str>()
                    .unwrap_or(&"<Unknwon payload>");

                log::error!("panicked at '{}', {}", location, payload);
            }));
        }
        #[cfg(not(target_os = "android"))]
        {
            let mut logger_builder = env_logger::Builder::new();
            logger_builder.filter_level(log::LevelFilter::Trace);
            #[cfg(windows)]
            logger_builder.target(env_logger::Target::Pipe(Box::new(
                windows::DebugOutputWriter,
            )));
            logger_builder.init();
        }

        Self
    }
}

#[cfg(target_os = "android")]
impl Drop for NexusLayer {
    fn drop(&mut self) {
        // drop to suppress the must use warning.
        drop(std::panic::take_hook());
    }
}
