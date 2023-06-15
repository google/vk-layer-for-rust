use crate::{
    DeviceInfo, GlobalHooks, InstanceHooks, InstanceInfo, Layer, LayerResult, LayerVulkanCommand,
};
use ash::vk;
use mockall::mock;
use std::{
    borrow::Borrow,
    collections::HashMap,
    marker::PhantomData,
    ops::Deref,
    sync::{Arc, Mutex, MutexGuard, Weak},
};

pub use crate::bindings::{
    VkLayerDeviceCreateInfo, VkLayerFunction, VkLayerInstanceCreateInfo, VkLayerInstanceLink,
};

pub struct Del<T> {
    data: T,
    deleter: Option<Box<dyn FnOnce(&mut T) + Send + Sync>>,
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

pub struct ArcDel<T>(pub Arc<Del<T>>);

impl<T> ArcDel<T> {
    fn new(data: T, deleter: impl FnOnce(&mut T) + Send + Sync + 'static) -> Self {
        ArcDel(Arc::new(Del {
            data,
            deleter: Some(Box::new(deleter)),
        }))
    }
}

impl<T> Borrow<T> for ArcDel<T> {
    fn borrow(&self) -> &T {
        &self.0.data
    }
}

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

#[derive(Default)]
pub struct MockInstanceInfo<T: TestLayer> {
    pub mock_hooks: Mutex<MockInstanceHooks>,
    _marker: PhantomData<T>,
}

impl<T: TestLayer> InstanceInfo for MockInstanceInfo<T> {
    type LayerInfo = Arc<TestLayerWrapper<T, Self>>;
    type HooksType = MockInstanceHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockInstanceHooks>;
    fn hooked_commands(layer: &Self::LayerInfo) -> &[LayerVulkanCommand] {
        layer.inner.hooked_instance_commands()
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

    // Will only be used when work with MockInstanceInfo<Self> and TestLayerWrapper so that we can
    // use an actual InstanceInfo with TestLayer and TestLayerWrapper.
    fn hooked_instance_commands(&self) -> &[LayerVulkanCommand] {
        &[]
    }
}

pub struct TestLayerWrapper<T, U = MockInstanceInfo<T>>
where
    T: TestLayer,
    U: InstanceInfo + 'static,
{
    inner: T,
    instances: Mutex<HashMap<vk::Instance, Weak<Del<U>>>>,
}

impl<T, U> TestLayerWrapper<T, U>
where
    T: TestLayer,
    U: InstanceInfo + 'static,
{
    pub fn get_instance_info(&self, instance: vk::Instance) -> Option<Arc<impl Deref<Target = U>>> {
        self.instances
            .lock()
            .unwrap()
            .get(&instance)
            .and_then(Weak::upgrade)
    }
}

impl<T, U> Default for TestLayerWrapper<T, U>
where
    T: TestLayer,
    U: InstanceInfo + 'static,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
            instances: Default::default(),
        }
    }
}

impl<T, U> GlobalHooks for Arc<TestLayerWrapper<T, U>>
where
    T: TestLayer,
    U: InstanceInfo + 'static,
{
}

impl<T, U> Layer for Arc<TestLayerWrapper<T, U>>
where
    T: TestLayer,
    U: InstanceInfo<LayerInfo = Self> + Default + 'static,
{
    type DeviceInfo = MockDeviceInfo<Self>;
    type InstanceInfo = U;
    type DeviceInfoContainer = Self::DeviceInfo;
    type InstanceInfoContainer = ArcDel<Self::InstanceInfo>;

    const LAYER_NAME: &'static str = <T as TestLayer>::LAYER_NAME;
    const LAYER_DESCRIPTION: &'static str = <T as TestLayer>::LAYER_DESCRIPTION;
    const IMPLEMENTATION_VERSION: u32 = <T as TestLayer>::IMPLEMENTATION_VERSION;
    const SPEC_VERSION: u32 = <T as TestLayer>::SPEC_VERSION;

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
