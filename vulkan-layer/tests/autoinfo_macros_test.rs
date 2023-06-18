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

use ash::vk;
use std::sync::Arc;
use vulkan_layer::{
    auto_deviceinfo_impl, auto_instanceinfo_impl,
    test_utils::{MockGlobalHooksInfo, MockInstanceInfo, TestLayer, TestLayerWrapper},
    DeviceHooks, Global, InstanceHooks, Layer, LayerResult, LayerVulkanCommand,
};

#[test]
fn test_auto_instanceinfo_should_intercept_hooked_proc() {
    #[derive(Default)]
    struct Tag;
    impl TestLayer for Tag {}

    type MockLayer = Arc<TestLayerWrapper<Tag, MockGlobalHooksInfo<Tag>, TestInstanceInfo>>;
    #[derive(Default)]
    struct TestInstanceInfo;
    #[auto_instanceinfo_impl]
    impl InstanceHooks for TestInstanceInfo {
        fn destroy_surface_khr(
            &self,
            _: vk::SurfaceKHR,
            _: Option<&'static vk::AllocationCallbacks>,
        ) -> LayerResult<()> {
            unimplemented!()
        }
    }
    let hooked_commands = &Global::<MockLayer>::instance()
        .layer_info
        .hooked_commands()
        .collect::<Vec<_>>();
    assert_eq!(hooked_commands, &[LayerVulkanCommand::DestroySurfaceKhr]);
}

#[test]
fn test_auto_deviceinfo_should_intercept_hooked_proc() {
    #[derive(Default)]
    struct Tag;
    impl TestLayer for Tag {}

    type MockLayer =
        Arc<TestLayerWrapper<Tag, MockGlobalHooksInfo<Tag>, MockInstanceInfo<Tag>, TestDeviceInfo>>;
    #[derive(Default)]
    struct TestDeviceInfo;
    #[auto_deviceinfo_impl]
    impl DeviceHooks for TestDeviceInfo {
        fn destroy_image(
            &self,
            _image: vk::Image,
            _p_allocator: Option<&'static vk::AllocationCallbacks>,
        ) -> LayerResult<()> {
            unimplemented!()
        }
    }
    let hooked_commands = &Global::<MockLayer>::instance()
        .layer_info
        .hooked_commands()
        .collect::<Vec<_>>();
    assert_eq!(hooked_commands, &[LayerVulkanCommand::DestroyImage]);
}
