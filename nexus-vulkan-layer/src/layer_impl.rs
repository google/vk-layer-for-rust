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
    ffi::{c_void, CString},
    sync::Arc,
};

use ash::{prelude::VkResult, vk};
use bumpalo::Bump;

use crate::{
    bindings::{
        cros_gralloc_handle, AHardwareBuffer, AHardwareBuffer_Format,
        AHardwareBuffer_getNativeHandle,
    },
    cros_gralloc_helpers::cros_gralloc_convert_handle,
};
use log::{error, info};
use vulkan_layer::{
    auto_deviceinfo_impl, auto_globalhooksinfo_impl, auto_instanceinfo_impl, DeviceHooks,
    Extension, ExtensionProperties, GlobalHooks, InstanceHooks, Layer, LayerResult,
    VulkanBaseInStructChain, VulkanBaseOutStructChain,
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

pub(crate) struct NexusLayer;

pub(crate) struct NexusInstanceInfo;

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
}

const VK_GET_SWAPCHAIN_GRALLOC_USAGE2_ANDROID_NAME: &str = "vkGetSwapchainGrallocUsage2ANDROID";

pub(crate) struct NexusDeviceInfo {
    device: Arc<ash::Device>,
    get_swapchain_gralloc_usage2_android: Option<vk::PFN_vkGetSwapchainGrallocUsage2ANDROID>,
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
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::DedicatedAllocationImageCreateInfoNV => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ExportMetalObjectCreateInfoEXT => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ExternalFormatANDROID => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ExternalMemoryImageCreateInfo => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ExternalMemoryImageCreateInfoNV => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImageCompressionControlEXT => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImageDrmFormatModifierExplicitCreateInfoEXT => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImageDrmFormatModifierListCreateInfoEXT => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImageFormatListCreateInfo => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImageStencilUsageCreateInfo => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImageSwapchainCreateInfoKHR => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImportMetalIOSurfaceInfoEXT => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImportMetalTextureInfoEXT => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::OpaqueCaptureDescriptorDataCreateInfoEXT => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::OpticalFlowImageFormatInfoNV => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::VideoProfileListInfoKHR => {
                        local_create_info = local_create_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::NativeBufferANDROID => {
                        let p_next = bump.alloc(p_next.clone());
                        assert!(p_next.p_next.is_null());
                        p_next.p_next = local_create_info.p_next;
                        local_create_info.p_next = p_next as *const _ as *const c_void;
                        // TODO(nexus): required for VK_ANDROID_native_buffer
                        error!("vk::NativeBufferANDROID in create_image unimplemented");
                    }
                    p_next @ vk::SwapchainImageCreateInfoANDROID => {
                        let p_next = bump.alloc(p_next.clone());
                        assert!(p_next.p_next.is_null());
                        p_next.p_next = local_create_info.p_next;
                        local_create_info.p_next = p_next as *const _ as *const c_void;
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
            fn get_vk_format(cros_gralloc_handle: &cros_gralloc_handle) -> vk::Format {
                match AHardwareBuffer_Format(cros_gralloc_handle.droid_format as u32) {
                    AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT => {
                        vk::Format::R16G16B16A16_SFLOAT
                    }
                    AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM => {
                        vk::Format::R8G8B8A8_UNORM
                    }
                    _ => unimplemented!(
                        "Unsupported AHardwareBuffer format: {}",
                        cros_gralloc_handle.droid_format
                    ),
                }
            }
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
                            vk::AndroidHardwareBufferFormatProperties2ANDROID::builder()
                                .format(get_vk_format(cros_gralloc_handle))
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
                                .suggested_y_chroma_offset(vk::ChromaLocation::COSITED_EVEN)
                                .clone();
                        format_properties2.p_next = p_next;
                    }
                    format_properties @ vk::AndroidHardwareBufferFormatPropertiesANDROID => {
                        let p_next = format_properties.p_next;
                        *format_properties =
                            vk::AndroidHardwareBufferFormatPropertiesANDROID::builder()
                                .format(get_vk_format(cros_gralloc_handle))
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
                                .suggested_y_chroma_offset(vk::ChromaLocation::COSITED_EVEN)
                                .clone();
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
        properties.allocation_size =
            cros_gralloc_handle.total_size - cros_gralloc_handle.reserved_region_size;
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
        let mut local_allocate_info = vk::MemoryAllocateInfo::builder();
        for p_next in p_next_chain {
            let p_next_ptr = p_next as *const vk::BaseInStructure;
            unsafe {
                ash::match_in_struct!(match p_next_ptr {
                    p_next @ vk::DedicatedAllocationMemoryAllocateInfoNV => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ExportMemoryAllocateInfo => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ExportMemoryAllocateInfoNV => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ExportMemoryWin32HandleInfoKHR => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ExportMemoryWin32HandleInfoNV => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ExportMetalObjectCreateInfoEXT => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImportMemoryBufferCollectionFUCHSIA => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImportMemoryFdInfoKHR => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImportMemoryHostPointerInfoEXT => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImportMemoryWin32HandleInfoKHR => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImportMemoryWin32HandleInfoNV => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImportMemoryZirconHandleInfoFUCHSIA => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::ImportMetalBufferInfoEXT => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::MemoryAllocateFlagsInfo => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::MemoryDedicatedAllocateInfo => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::MemoryOpaqueCaptureAddressAllocateInfo => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
                    }
                    p_next @ vk::MemoryPriorityAllocateInfoEXT => {
                        local_allocate_info =
                            local_allocate_info.push_next(bump.alloc(p_next.clone()));
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
        _: Arc<ash::Instance>,
        _: vk::PFN_vkGetInstanceProcAddr,
    ) -> Self::InstanceInfo {
        info!("create instance from {}", Self::LAYER_NAME);
        NexusInstanceInfo
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
