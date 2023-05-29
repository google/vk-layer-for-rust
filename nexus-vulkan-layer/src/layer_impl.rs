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

use std::sync::Arc;

use ash::{prelude::VkResult, vk};

use log::info;
use vulkan_layer::{DeviceInfo, InstanceInfo, Layer, LayerResult};

pub(crate) struct NexusLayer;

pub(crate) struct NexusInstanceInfo;

impl InstanceInfo for NexusInstanceInfo {}

pub(crate) struct NexusDeviceInfo;

impl DeviceInfo for NexusDeviceInfo {
    fn create_image(
        &self,
        _: &vk::ImageCreateInfo,
        _: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Image>> {
        info!("Create image from {}", NexusLayer::LAYER_NAME);
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
    ) -> Self::InstanceInfo {
        info!("create instance from {}", Self::LAYER_NAME);
        NexusInstanceInfo
    }

    fn create_device_info(
        &self,
        _: vk::PhysicalDevice,
        _: &vk::DeviceCreateInfo,
        _: Option<&vk::AllocationCallbacks>,
        _: Arc<ash::Device>,
    ) -> Self::DeviceInfo {
        info!("create device from {}", Self::LAYER_NAME);
        NexusDeviceInfo
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
                    .with_max_level(log::LevelFilter::Trace),
            );
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
