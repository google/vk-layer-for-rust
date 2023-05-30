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

use std::{ffi::CString, sync::Arc};

use ash::{prelude::VkResult, vk};

use log::{error, info};
use vulkan_layer::{DeviceInfo, InstanceInfo, Layer, LayerResult};

// TODO: find a way to share these functions with vulkan-layer.
struct VulkanBaseInStructChain<'a> {
    next: Option<&'a vk::BaseInStructure>,
}

impl<'a> Iterator for VulkanBaseInStructChain<'a> {
    type Item = &'a vk::BaseInStructure;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|element| {
            self.next = unsafe { element.p_next.as_ref() };
            element
        })
    }
}

fn find_in_struct<T: vk::TaggedStructure>(
    chain: Option<&vk::BaseInStructure>,
) -> impl Iterator<Item = &T> {
    let iter = VulkanBaseInStructChain { next: chain };
    iter.filter_map(|base_out_structure| {
        if base_out_structure.s_type == T::STRUCTURE_TYPE {
            Some(unsafe { std::mem::transmute(base_out_structure) })
        } else {
            None
        }
    })
}

struct VulkanBaseOutStructChain<'a> {
    next: Option<&'a mut vk::BaseOutStructure>,
}

impl<'a> Iterator for VulkanBaseOutStructChain<'a> {
    type Item = &'a mut vk::BaseOutStructure;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|element| {
            self.next = unsafe { element.p_next.as_mut() };
            element
        })
    }
}

pub(crate) fn find_out_struct<T: vk::TaggedStructure>(
    chain: Option<&mut vk::BaseOutStructure>,
) -> impl Iterator<Item = &mut T> {
    let iter = VulkanBaseOutStructChain { next: chain };
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

impl InstanceInfo for NexusInstanceInfo {
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
            // TODO(nexus): required
            error!("vk::PhysicalDevicePresentationPropertiesANDROID in get_physical_device_properties2 unimplemented");
        }
        LayerResult::Unhandled
    }
}

const VK_GET_SWAPCHAIN_GRALLOC_USAGE2_ANDROID_NAME: &str = "vkGetSwapchainGrallocUsage2ANDROID";

pub(crate) struct NexusDeviceInfo {
    get_swapchain_gralloc_usage2_android: Option<vk::PFN_vkGetSwapchainGrallocUsage2ANDROID>,
}

impl DeviceInfo for NexusDeviceInfo {
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
        _: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Image>> {
        info!("Create image from {}", NexusLayer::LAYER_NAME);
        let p_next_chain = unsafe { (create_info.p_next as *const vk::BaseInStructure).as_ref() };
        for _external_format_android in find_in_struct::<vk::ExternalFormatANDROID>(p_next_chain) {
            // TODO(nexus): required
            error!("vk::ExternalFormatANDROID in create_image unimplemented");
        }
        for _native_buffer_android in find_in_struct::<vk::NativeBufferANDROID>(p_next_chain) {
            // TODO(nexus): required
            error!("vk::NativeBufferANDROID in create_image unimplemented");
        }
        for _swapchain_image_create_info in
            find_in_struct::<vk::SwapchainImageCreateInfoANDROID>(p_next_chain)
        {
            // TODO(nexus): required
            error!("vk::SwapchainImageCreateInfoANDROID in create_image unimplemented");
        }
        LayerResult::Unhandled
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
            unimplemented!("vk::ExternalFormatANDROID in create_sampler_ycbcr_conversion unimplemented");
        }
        LayerResult::Unhandled
    }

    fn get_android_hardware_buffer_properties_android(
        &self,
        _buffer: *const vk::AHardwareBuffer,
        _p_properties: &mut vk::AndroidHardwareBufferPropertiesANDROID,
    ) -> LayerResult<VkResult<()>> {
        // TODO(nexus): required
        error!("get_android_hardware_buffer_properties_android unimplemented");
        LayerResult::Unhandled
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
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::DeviceMemory>> {
        let p_next_chain = unsafe { (allocate_info.p_next as *const vk::BaseInStructure).as_ref() };
        for _ in find_in_struct::<vk::ImportAndroidHardwareBufferInfoANDROID>(p_next_chain) {
            // TODO(nexus): required
            error!("vk::ImportAndroidHardwareBufferInfoANDROID in allocate_memory unimplemented");
        }
        LayerResult::Unhandled
    }

    // VK_ANDROID_native_buffer commands
    fn get_swapchain_gralloc_usage_android(
        &self,
        _format: vk::Format,
        _image_usage: vk::ImageUsageFlags,
    ) -> LayerResult<VkResult<std::ffi::c_int>> {
        // TODO(nexus): required
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

impl Layer for NexusLayer {
    const LAYER_NAME: &'static str = "VK_LAYER_GOOGLE_nexus";
    const LAYER_DESCRIPTION: &'static str =
        "Emulate some Android Vulkan extensions support where the ICD doesn't support them.";
    const IMPLEMENTATION_VERSION: u32 = 1;
    const SPEC_VERSION: u32 = vk::API_VERSION_1_1;

    type InstanceInfo = NexusInstanceInfo;
    type DeviceInfo = NexusDeviceInfo;

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
        NexusDeviceInfo {
            get_swapchain_gralloc_usage2_android: unsafe {
                next_get_device_proc_addr(device.handle(), func_name_c_str.as_ptr())
            }
            .map(|src| unsafe { std::mem::transmute(src) }),
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
