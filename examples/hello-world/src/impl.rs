// Copyright 2024 Google LLC
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

use ash::{self, vk};
use once_cell::sync::Lazy;
use std::sync::Arc;
use vulkan_layer::{
    declare_introspection_queries, Global, Layer, LayerManifest, StubDeviceInfo, StubGlobalHooks,
    StubInstanceInfo,
};

#[derive(Default)]
struct MyLayer(StubGlobalHooks);

impl Layer for MyLayer {
    type GlobalHooksInfo = StubGlobalHooks;
    type InstanceInfo = StubInstanceInfo;
    type DeviceInfo = StubDeviceInfo;
    type InstanceInfoContainer = StubInstanceInfo;
    type DeviceInfoContainer = StubDeviceInfo;

    fn global_instance() -> impl std::ops::Deref<Target = Global<Self>> + 'static {
        static GLOBAL: Lazy<Global<MyLayer>> = Lazy::new(Default::default);
        &*GLOBAL
    }

    fn manifest() -> LayerManifest {
        let mut manifest = LayerManifest::default();
        // Must match with the binary name on Android.
        manifest.name = "VK_LAYER_VENDOR_rust_example_hello_world";
        manifest.spec_version = vk::API_VERSION_1_1;
        manifest.description = "Rust test layer hello world";
        manifest
    }

    fn global_hooks_info(&self) -> &Self::GlobalHooksInfo {
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
        println!("Hello from the Rust Vulkan layer!");
        Default::default()
    }
}

declare_introspection_queries!(Global::<MyLayer>);
