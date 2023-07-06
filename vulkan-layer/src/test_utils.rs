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
    generated::GlobalHooksInfo, DeviceInfo, InstanceInfo, Layer, LayerManifest, LayerVulkanCommand,
};
use ash::vk;
use mockall::mock;
use once_cell::sync::OnceCell;
use std::{
    borrow::Borrow,
    collections::HashMap,
    ops::Deref,
    sync::{Arc, Mutex, MutexGuard, Weak},
};

pub use crate::bindings::{
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

const MAX_TEST_LAYER_SLOTS: usize = 2;
#[allow(clippy::declare_interior_mutable_const)]
const TEST_LAYER_INIT_VALUE: Mutex<OnceCell<TestLayer>> = Mutex::new(OnceCell::new());
static TEST_LAYERS: [Mutex<OnceCell<TestLayer>>; MAX_TEST_LAYER_SLOTS] =
    [TEST_LAYER_INIT_VALUE; MAX_TEST_LAYER_SLOTS];

#[derive(Default)]
pub struct MockInstanceInfo<const SLOT: usize> {
    pub mock_hooks: Mutex<MockInstanceHooks>,
    mock_drop: Mutex<Option<MockDrop>>,
}

impl<const SLOT: usize> MockInstanceInfo<SLOT> {
    pub fn with_mock_drop(&self, f: impl FnOnce(&mut MockDrop)) {
        let mut mock_drop = self.mock_drop.lock().unwrap();
        let mock_drop = mock_drop.get_or_insert_with(Default::default);
        f(mock_drop);
    }
}

impl<const SLOT: usize> InstanceInfo for MockInstanceInfo<SLOT> {
    type HooksType = MockInstanceHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockInstanceHooks>;
    fn hooked_commands() -> &'static [LayerVulkanCommand] {
        TEST_LAYERS[SLOT]
            .lock()
            .unwrap()
            .get()
            .unwrap()
            .hooked_instance_commands
    }

    fn hooks(&self) -> Self::HooksRefType<'_> {
        self.mock_hooks.lock().unwrap()
    }
}

#[derive(Default)]
pub struct MockDeviceInfo<const SLOT: usize> {
    pub mock_hooks: Mutex<MockDeviceHooks>,
    mock_drop: Mutex<Option<MockDrop>>,
}

impl<const SLOT: usize> MockDeviceInfo<SLOT> {
    pub fn with_mock_drop(&self, f: impl FnOnce(&mut MockDrop)) {
        let mut mock_drop = self.mock_drop.lock().unwrap();
        let mock_drop = mock_drop.get_or_insert_with(Default::default);
        f(mock_drop);
    }
}

impl<const SLOT: usize> DeviceInfo for MockDeviceInfo<SLOT> {
    type HooksType = MockDeviceHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockDeviceHooks>;
    fn hooked_commands() -> &'static [LayerVulkanCommand] {
        TEST_LAYERS[SLOT]
            .lock()
            .unwrap()
            .get()
            .unwrap()
            .hooked_device_commands
    }

    fn hooks(&self) -> Self::HooksRefType<'_> {
        self.mock_hooks.lock().unwrap()
    }
}

#[derive(Default)]
pub struct MockGlobalHooksInfo<const SLOT: usize> {
    pub mock_hooks: Mutex<MockGlobalHooks>,
}

impl<const SLOT: usize> GlobalHooksInfo for MockGlobalHooksInfo<SLOT> {
    type HooksType = MockGlobalHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockGlobalHooks>;
    fn hooked_commands() -> &'static [LayerVulkanCommand] {
        TEST_LAYERS[SLOT]
            .lock()
            .unwrap()
            .get()
            .unwrap()
            .hooked_global_commands
    }

    fn hooks(&self) -> Self::HooksRefType<'_> {
        self.mock_hooks.lock().unwrap()
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

struct TestLayer {
    layer_manifest: LayerManifest,
    hooked_global_commands: &'static [LayerVulkanCommand],
    hooked_instance_commands: &'static [LayerVulkanCommand],
    hooked_device_commands: &'static [LayerVulkanCommand],
}

impl Default for TestLayer {
    fn default() -> Self {
        Self {
            layer_manifest: LayerManifest::test_default(),
            hooked_global_commands: &[],
            hooked_instance_commands: &[],
            hooked_device_commands: &[],
        }
    }
}

pub struct TestLayerContext {
    slot: usize,
}

impl TestLayerContext {
    fn new(slot: usize) -> Self {
        TEST_LAYERS[slot]
            .lock()
            .unwrap()
            .set(Default::default())
            .unwrap_or_else(|_| panic!("Trying to create overlapped test contexts."));
        Self { slot }
    }

    fn with_mut_test_layer(&self, f: impl FnOnce(&mut TestLayer)) {
        let mut test_layer = TEST_LAYERS[self.slot].lock().unwrap();
        let test_layer = test_layer
            .get_mut()
            .unwrap_or_else(|| panic!("TestLayer not initialized yet."));
        f(test_layer);
    }

    pub fn set_layer_manifest(&self, layer_manifest: LayerManifest) -> &Self {
        self.with_mut_test_layer(move |test_layer| test_layer.layer_manifest = layer_manifest);
        self
    }

    pub fn set_hooked_global_commands(
        &self,
        hooked_global_commands: &'static [LayerVulkanCommand],
    ) -> &Self {
        self.with_mut_test_layer(move |test_layer| {
            test_layer.hooked_global_commands = hooked_global_commands
        });
        self
    }

    pub fn set_hooked_instance_commands(
        &self,
        hooked_instance_commands: &'static [LayerVulkanCommand],
    ) -> &Self {
        self.with_mut_test_layer(move |test_layer| {
            test_layer.hooked_instance_commands = hooked_instance_commands
        });
        self
    }

    pub fn set_hooked_device_commands(
        &self,
        hooked_device_commands: &'static [LayerVulkanCommand],
    ) -> &Self {
        self.with_mut_test_layer(move |test_layer| {
            test_layer.hooked_device_commands = hooked_device_commands
        });
        self
    }
}

impl Drop for TestLayerContext {
    fn drop(&mut self) {
        TEST_LAYERS[self.slot].lock().unwrap().take().unwrap();
    }
}

pub struct TestLayerWrapper<
    const SLOT: usize = 0,
    U = MockGlobalHooksInfo<SLOT>,
    V = MockInstanceInfo<SLOT>,
    W = MockDeviceInfo<SLOT>,
> where
    U: GlobalHooksInfo + 'static,
    V: InstanceInfo + 'static,
    W: DeviceInfo + 'static,
{
    global_hooks_info: U,
    instances: Mutex<HashMap<vk::Instance, Weak<Del<V>>>>,
    devices: Mutex<HashMap<vk::Device, Weak<Del<W>>>>,
}

impl<const SLOT: usize, U, V, W> TestLayerWrapper<SLOT, U, V, W>
where
    U: GlobalHooksInfo + 'static,
    V: InstanceInfo + 'static,
    W: DeviceInfo + 'static,
{
    pub fn get_instance_info(&self, instance: vk::Instance) -> Option<Arc<impl Deref<Target = V>>> {
        self.instances
            .lock()
            .unwrap()
            .get(&instance)
            .and_then(Weak::upgrade)
    }

    pub fn get_device_info(&self, device: vk::Device) -> Option<Arc<impl Deref<Target = W>>> {
        self.devices
            .lock()
            .unwrap()
            .get(&device)
            .and_then(Weak::upgrade)
    }

    pub fn context() -> TestLayerContext {
        TestLayerContext::new(SLOT)
    }
}

impl<const SLOT: usize, U, V, W> Default for TestLayerWrapper<SLOT, U, V, W>
where
    U: GlobalHooksInfo + Default + 'static,
    V: InstanceInfo + 'static,
    W: DeviceInfo + 'static,
{
    fn default() -> Self {
        Self {
            global_hooks_info: Default::default(),
            instances: Default::default(),
            devices: Default::default(),
        }
    }
}

impl<const SLOT: usize, U, V, W> Layer for Arc<TestLayerWrapper<SLOT, U, V, W>>
where
    U: GlobalHooksInfo + Default + 'static,
    V: InstanceInfo + Default + 'static,
    W: DeviceInfo + Default + 'static,
{
    type GlobalHooksInfo = U;
    type InstanceInfo = V;
    type DeviceInfo = W;
    type InstanceInfoContainer = ArcDel<Self::InstanceInfo>;
    type DeviceInfoContainer = ArcDel<Self::DeviceInfo>;

    fn manifest() -> LayerManifest {
        TEST_LAYERS[SLOT]
            .lock()
            .unwrap()
            .get()
            .unwrap()
            .layer_manifest
            .clone()
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
        let weak_layer = Arc::downgrade(self);
        let device_handle = device.handle();
        let device_info = ArcDel::new(Default::default(), move |_| {
            let layer = match weak_layer.upgrade() {
                Some(layer) => layer,
                None => return,
            };
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
        let weak_layer = Arc::downgrade(self);
        let instance_handle = instance.handle();
        let instance_info = ArcDel::new(Default::default(), move |_| {
            let layer = match weak_layer.upgrade() {
                Some(layer) => layer,
                None => return,
            };
            layer.instances.lock().unwrap().remove(&instance_handle);
        });
        self.instances
            .lock()
            .unwrap()
            .insert(instance.handle(), Arc::downgrade(&instance_info.0));
        instance_info
    }
}
