use crate::{
    DeviceInfo, GlobalHooks, InstanceHooks, InstanceInfo, Layer, LayerResult, LayerVulkanCommand,
};
use ash::vk;
use mockall::mock;
use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex, MutexGuard, Weak},
};

pub use crate::bindings::{
    VkLayerDeviceCreateInfo, VkLayerFunction, VkLayerInstanceCreateInfo, VkLayerInstanceLink,
};

#[derive(Default)]
pub struct MockDeviceHooks {}
impl crate::DeviceHooks for MockDeviceHooks {}
pub struct MockGlobalHooks {}
impl crate::GlobalHooks for MockGlobalHooks {}
// We don't automock the original trait, because that hurts compilation speed significantly.
mock! {
    pub InstanceHooks {}
    impl InstanceHooks for InstanceHooks {
        fn destroy_surface_khr(
            &self,
            surface: vk::SurfaceKHR,
            p_allocator: Option<&'static vk::AllocationCallbacks>,
        ) -> LayerResult<()>;
    }
}

pub trait Tag: 'static + Sync + Send + Default {}
impl<T: 'static + Sync + Send + Default> Tag for T {}

pub struct MockInstanceInfo<T: TestLayer> {
    layer: Weak<TestLayerWrapper<T>>,
    instance: vk::Instance,
    mock_hooks: Arc<Mutex<MockInstanceHooks>>,
}

impl<T: TestLayer> Drop for MockInstanceInfo<T> {
    fn drop(&mut self) {
        let layer = match self.layer.upgrade() {
            Some(layer) => layer,
            None => return,
        };
        layer
            .instance_hook_mocks
            .lock()
            .unwrap()
            .remove(&self.instance);
    }
}

impl<T: TestLayer> InstanceInfo for MockInstanceInfo<T> {
    type LayerInfo = Arc<TestLayerWrapper<T>>;
    type HooksType = MockInstanceHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockInstanceHooks>;
    fn hooked_commands(_: &Self::LayerInfo) -> &[LayerVulkanCommand] {
        &[]
    }

    fn hooks(&self) -> Self::HooksRefType<'_> {
        self.mock_hooks.lock().unwrap()
    }
}

#[derive(Default)]
pub struct MockDeviceInfo<T: Layer<DeviceInfo = Self> + Send> {
    mock_hooks: Arc<Mutex<MockDeviceHooks>>,
    _marker: PhantomData<T>,
}

impl<T: Layer<DeviceInfo = Self> + Send> DeviceInfo for MockDeviceInfo<T> {
    type LayerInfo = T;
    type HooksType = MockDeviceHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockDeviceHooks>;
    fn hooked_commands(_: &Self::LayerInfo) -> &[LayerVulkanCommand] {
        &[]
    }

    fn hooks(&self) -> Self::HooksRefType<'_> {
        self.mock_hooks.lock().unwrap()
    }
}

pub trait TestLayer: Send + Sync + Default + 'static {
    const LAYER_NAME: &'static str = "VK_LAYER_GOOGLE_test";
    const LAYER_DESCRIPTION: &'static str = "";
    const IMPLEMENTATION_VERSION: u32 = 1;
    const SPEC_VERSION: u32 = vk::API_VERSION_1_1;

    fn hooked_commands(&self) -> Box<dyn Iterator<Item = LayerVulkanCommand> + '_> {
        Box::new([].into_iter())
    }
}

#[derive(Default)]
pub struct TestLayerWrapper<T: TestLayer> {
    inner: T,
    instance_hook_mocks: Mutex<HashMap<vk::Instance, Weak<Mutex<MockInstanceHooks>>>>,
}

impl<T: TestLayer> TestLayerWrapper<T> {
    pub fn get_instance_hooks_mock(
        &self,
        instance: vk::Instance,
    ) -> Option<Arc<Mutex<MockInstanceHooks>>> {
        self.instance_hook_mocks
            .lock()
            .unwrap()
            .get(&instance)
            .and_then(Weak::upgrade)
    }
}

impl<T: TestLayer> GlobalHooks for Arc<TestLayerWrapper<T>> {}

impl<T: TestLayer> Layer for Arc<TestLayerWrapper<T>> {
    type DeviceInfo = MockDeviceInfo<Self>;
    type InstanceInfo = MockInstanceInfo<T>;

    const LAYER_NAME: &'static str = <T as TestLayer>::LAYER_NAME;
    const LAYER_DESCRIPTION: &'static str = <T as TestLayer>::LAYER_DESCRIPTION;
    const IMPLEMENTATION_VERSION: u32 = <T as TestLayer>::IMPLEMENTATION_VERSION;
    const SPEC_VERSION: u32 = <T as TestLayer>::SPEC_VERSION;

    fn hooked_commands(&self) -> Box<dyn Iterator<Item = LayerVulkanCommand> + '_> {
        self.inner.hooked_commands()
    }

    fn create_device_info(
        &self,
        _physical_device: vk::PhysicalDevice,
        _create_info: &vk::DeviceCreateInfo,
        _allocator: Option<&vk::AllocationCallbacks>,
        _device: Arc<ash::Device>,
        _next_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    ) -> Self::DeviceInfo {
        Default::default()
    }

    fn create_instance_info(
        &self,
        _create_info: &vk::InstanceCreateInfo,
        _allocator: Option<&vk::AllocationCallbacks>,
        instance: Arc<ash::Instance>,
        _next_get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    ) -> Self::InstanceInfo {
        let mock_hooks: Arc<Mutex<MockInstanceHooks>> = Default::default();
        self.instance_hook_mocks
            .lock()
            .unwrap()
            .insert(instance.handle(), Arc::downgrade(&mock_hooks));
        Self::InstanceInfo {
            layer: Arc::downgrade(self),
            instance: instance.handle(),
            mock_hooks,
        }
    }
}
