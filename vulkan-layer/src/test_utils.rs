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

use crate::{
    DeviceInfo, Global, GlobalHooksInfo, InstanceInfo, Layer, LayerManifest, LayerVulkanCommand,
};
use ash::vk;
use mockall::mock;
use once_cell::sync::Lazy;
use std::{
    any::Any,
    borrow::Borrow,
    collections::HashMap,
    marker::PhantomData,
    ops::Deref,
    sync::{Arc, Mutex, MutexGuard, Weak},
};

pub use crate::bindings::vk_layer::{
    VkLayerDeviceCreateInfo, VkLayerDeviceLink, VkLayerFunction, VkLayerInstanceCreateInfo,
};

mod device_hooks_mock;
mod global_hooks_mock;
mod instance_hooks_mock;

use device_hooks_mock::MockDeviceHooks;
use global_hooks_mock::MockGlobalHooks;
use instance_hooks_mock::MockInstanceHooks;

type Deleter<T> = Box<dyn FnOnce(&mut T) + Send + Sync>;

pub struct Del<T> {
    data: T,
    deleter: Option<Deleter<T>>,
}

impl<T> Del<T> {
    pub fn new(data: T, deleter: impl FnOnce(&mut T) + Send + Sync + 'static) -> Self {
        Del {
            data,
            deleter: Some(Box::new(deleter)),
        }
    }
}

impl<T> Deref for Del<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Drop for Del<T> {
    fn drop(&mut self) {
        (self.deleter.take().unwrap())(&mut self.data);
    }
}

impl<T> AsRef<T> for Del<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

pub struct ArcDel<T>(pub Arc<Del<T>>);

impl<T> ArcDel<T> {
    pub fn new(data: T, deleter: impl FnOnce(&mut T) + Send + Sync + 'static) -> Self {
        ArcDel(Arc::new(Del::new(data, deleter)))
    }
}

impl<T> Borrow<T> for ArcDel<T> {
    fn borrow(&self) -> &T {
        &self.0.data
    }
}

impl<T: Default> Default for ArcDel<T> {
    fn default() -> Self {
        Self(Arc::new(Del::new(Default::default(), |_| {})))
    }
}

impl<T> From<Del<T>> for ArcDel<T> {
    fn from(value: Del<T>) -> Self {
        Self(Arc::new(value))
    }
}

impl<T> Deref for ArcDel<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<T> AsRef<T> for ArcDel<T> {
    fn as_ref(&self) -> &T {
        self.0.as_ref()
    }
}

impl<T> Clone for ArcDel<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

mock! {
    pub Drop {}
    impl Drop for Drop {
        fn drop(&mut self);
    }
}

pub trait TestLayerMock: 'static {
    type TestLayerType: Layer;
    fn instance() -> &'static Global<Self::TestLayerType>;
    fn mock() -> &'static Self;

    fn manifest(&self) -> LayerManifest;
    fn hooked_global_commands(&self) -> &[LayerVulkanCommand];
    fn hooked_instance_commands(&self) -> &[LayerVulkanCommand];
    fn hooked_device_commands(&self) -> &[LayerVulkanCommand];
}

#[derive(Default)]
pub struct MockGlobalHooksInfo<T: TestLayerMock> {
    pub mock_hooks: Mutex<MockGlobalHooks>,
    _marker: PhantomData<fn(T)>,
}

impl<T: TestLayerMock> GlobalHooksInfo for MockGlobalHooksInfo<T> {
    type HooksType = MockGlobalHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockGlobalHooks>;
    fn hooked_commands() -> &'static [LayerVulkanCommand] {
        T::mock().hooked_global_commands()
    }

    fn hooks(&self) -> Self::HooksRefType<'_> {
        self.mock_hooks.lock().unwrap()
    }
}

#[derive(Default)]
pub struct MockInstanceInfo<T: TestLayerMock> {
    pub mock_hooks: Mutex<MockInstanceHooks>,
    mock_drop: Mutex<Option<MockDrop>>,
    _marker: PhantomData<fn(T)>,
}

impl<T: TestLayerMock> MockInstanceInfo<T> {
    pub fn with_mock_drop(&self, f: impl FnOnce(&mut MockDrop)) {
        let mut mock_drop = self.mock_drop.lock().unwrap();
        let mock_drop = mock_drop.get_or_insert_with(Default::default);
        f(mock_drop);
    }
}

impl<T: TestLayerMock> InstanceInfo for MockInstanceInfo<T> {
    type HooksType = MockInstanceHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockInstanceHooks>;
    fn hooked_commands() -> &'static [LayerVulkanCommand] {
        T::mock().hooked_instance_commands()
    }

    fn hooks(&self) -> Self::HooksRefType<'_> {
        self.mock_hooks.lock().unwrap()
    }
}

#[derive(Default)]
pub struct MockDeviceInfo<T: TestLayerMock> {
    pub mock_hooks: Mutex<MockDeviceHooks>,
    mock_drop: Mutex<Option<MockDrop>>,
    _marker: PhantomData<fn(T)>,
}

impl<T: TestLayerMock> MockDeviceInfo<T> {
    pub fn with_mock_drop(&self, f: impl FnOnce(&mut MockDrop)) {
        let mut mock_drop = self.mock_drop.lock().unwrap();
        let mock_drop = mock_drop.get_or_insert_with(Default::default);
        f(mock_drop);
    }
}

impl<T: TestLayerMock> DeviceInfo for MockDeviceInfo<T> {
    type HooksType = MockDeviceHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockDeviceHooks>;

    fn hooked_commands() -> &'static [LayerVulkanCommand] {
        T::mock().hooked_device_commands()
    }

    fn hooks(&self) -> Self::HooksRefType<'_> {
        self.mock_hooks.lock().unwrap()
    }
}

pub struct Tag<const I: usize>;

pub trait TestLayerTag: Sync + Send + 'static {}

impl<const I: usize> TestLayerTag for Tag<I> {}

pub struct TestLayer<T: TestLayerTag = Tag<0>> {
    global_hooks_info: MockGlobalHooksInfo<MockTestLayer<T>>,
    instances: Mutex<HashMap<vk::Instance, Weak<Del<<Self as Layer>::InstanceInfo>>>>,
    devices: Mutex<HashMap<vk::Device, Weak<Del<<Self as Layer>::DeviceInfo>>>>,
    _marker: PhantomData<fn(T)>,
}

impl<T: TestLayerTag> TestLayer<T> {
    pub fn get_instance_info(
        &self,
        instance: vk::Instance,
    ) -> Option<Arc<impl Deref<Target = <Self as Layer>::InstanceInfo>>> {
        self.instances
            .lock()
            .unwrap()
            .get(&instance)
            .and_then(Weak::upgrade)
    }

    pub fn get_device_info(
        &self,
        device: vk::Device,
    ) -> Option<Arc<impl Deref<Target = <Self as Layer>::DeviceInfo>>> {
        self.devices
            .lock()
            .unwrap()
            .get(&device)
            .and_then(Weak::upgrade)
    }
}

impl<T: TestLayerTag> Default for TestLayer<T> {
    fn default() -> Self {
        Self {
            global_hooks_info: Default::default(),
            instances: Default::default(),
            devices: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<T: TestLayerTag> Layer for TestLayer<T> {
    type GlobalHooksInfo = MockGlobalHooksInfo<MockTestLayer<T>>;
    type InstanceInfo = MockInstanceInfo<MockTestLayer<T>>;
    type DeviceInfo = MockDeviceInfo<MockTestLayer<T>>;
    type InstanceInfoContainer = ArcDel<Self::InstanceInfo>;
    type DeviceInfoContainer = ArcDel<Self::DeviceInfo>;

    fn global_instance() -> &'static Global<Self> {
        MockTestLayer::<T>::instance()
    }

    fn manifest() -> LayerManifest {
        MockTestLayer::<T>::mock().manifest()
    }

    fn get_global_hooks_info(&self) -> &Self::GlobalHooksInfo {
        &self.global_hooks_info
    }

    fn create_device_info(
        &self,
        _physical_device: vk::PhysicalDevice,
        _create_info: &vk::DeviceCreateInfo,
        _allocator: Option<&vk::AllocationCallbacks>,
        device: Arc<ash::Device>,
        _next_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    ) -> ArcDel<Self::DeviceInfo> {
        let device_handle = device.handle();
        let device_info = ArcDel::new(Default::default(), move |_| {
            let layer = &Self::global_instance().layer_info;
            layer.devices.lock().unwrap().remove(&device_handle);
        });
        self.devices
            .lock()
            .unwrap()
            .insert(device.handle(), Arc::downgrade(&device_info.0));
        device_info
    }

    fn create_instance_info(
        &self,
        _create_info: &vk::InstanceCreateInfo,
        _allocator: Option<&vk::AllocationCallbacks>,
        instance: Arc<ash::Instance>,
        _next_get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    ) -> ArcDel<Self::InstanceInfo> {
        let instance_handle = instance.handle();
        let instance_info = ArcDel::new(Default::default(), move |_| {
            let layer = &Self::global_instance().layer_info;
            layer.instances.lock().unwrap().remove(&instance_handle);
        });
        self.instances
            .lock()
            .unwrap()
            .insert(instance.handle(), Arc::downgrade(&instance_info.0));
        instance_info
    }
}

mock! {
    pub TestLayer<T: TestLayerTag = Tag<0>> {}

    impl<T: TestLayerTag> TestLayerMock for TestLayer<T> {
        type TestLayerType = TestLayer<T>;

        fn instance() -> &'static Global<TestLayer<T>>;
        fn mock() -> &'static Self;

        fn manifest(&self) -> LayerManifest;
        fn hooked_global_commands(&self) -> &[LayerVulkanCommand];
        fn hooked_instance_commands(&self) -> &[LayerVulkanCommand];
        fn hooked_device_commands(&self) -> &[LayerVulkanCommand];
    }
}

impl<T: TestLayerTag> MockTestLayer<T> {
    pub fn set_default_expections(&mut self) {
        self.expect_manifest()
            .return_const(LayerManifest::test_default());
        self.expect_hooked_global_commands().return_const(vec![]);
        self.expect_hooked_instance_commands().return_const(vec![]);
        self.expect_hooked_device_commands().return_const(vec![]);
    }
}

pub struct TestGlobal<T: TestLayerTag = Tag<0>> {
    layer_mock: Lazy<MockTestLayer<T>>,
    layer_global: Lazy<Global<TestLayer<T>>>,
}

pub struct TestGlobalBuilder<T: TestLayerTag = Tag<0>> {
    layer_mock_builder: fn() -> MockTestLayer<T>,
    layer_global_builder: fn() -> Global<TestLayer<T>>,
}

impl<T: TestLayerTag> TestGlobalBuilder<T> {
    pub const fn set_layer_mock_builder(
        self,
        layer_mock_builder: fn() -> MockTestLayer<T>,
    ) -> Self {
        Self {
            layer_mock_builder,
            ..self
        }
    }

    pub const fn set_layer_global_builder(
        self,
        layer_global_builder: fn() -> Global<TestLayer<T>>,
    ) -> Self {
        Self {
            layer_global_builder,
            ..self
        }
    }

    pub const fn build(&self) -> TestGlobal<T> {
        TestGlobal {
            layer_mock: Lazy::new(self.layer_mock_builder),
            layer_global: Lazy::new(self.layer_global_builder),
        }
    }
}

impl<T: TestLayerTag> TestGlobal<T> {
    pub const fn builder() -> TestGlobalBuilder<T> {
        TestGlobalBuilder {
            layer_mock_builder: || {
                let mut mock = MockTestLayer::<T>::default();
                mock.set_default_expections();
                mock
            },
            layer_global_builder: Default::default,
        }
    }
    pub fn create_context(&'static self) -> Box<dyn Any> {
        let layer_mock_ctx = MockTestLayer::mock_context();
        layer_mock_ctx.expect().return_const(&*self.layer_mock);
        let layer_global_ctx = MockTestLayer::instance_context();
        layer_global_ctx.expect().return_const(&*self.layer_global);
        Box::new((layer_mock_ctx, layer_global_ctx))
    }
}

pub trait LayerManifestExt {
    fn test_default() -> Self;
}

impl LayerManifestExt for LayerManifest {
    fn test_default() -> Self {
        Self {
            name: "VK_LAYER_GOOGLE_test",
            spec_version: vk::API_VERSION_1_1,
            ..Default::default()
        }
    }
}
