use crate::{
    DeviceHooks, DeviceInfo, GlobalHooks, InstanceHooks, InstanceInfo, Layer, LayerResult,
    LayerVulkanCommand,
};
use ash::{prelude::VkResult, vk};
use mockall::mock;
use std::{
    borrow::Borrow,
    collections::HashMap,
    marker::PhantomData,
    ops::Deref,
    sync::{Arc, Mutex, MutexGuard, Weak},
};

pub use crate::bindings::{
    VkLayerDeviceCreateInfo, VkLayerDeviceLink, VkLayerFunction, VkLayerInstanceCreateInfo,
    VkLayerInstanceLink,
};

pub struct Del<T> {
    data: T,
    deleter: Option<Box<dyn FnOnce(&mut T) + Send + Sync>>,
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
    fn new(data: T, deleter: impl FnOnce(&mut T) + Send + Sync + 'static) -> Self {
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

mock! {
    pub GlobalHooks {}
    impl GlobalHooks for GlobalHooks {
        fn create_instance(
            &self,
            _p_create_info: &'static vk::InstanceCreateInfo,
            _p_allocator: Option<&'static vk::AllocationCallbacks>,
        ) -> LayerResult<VkResult<vk::Instance>>;
    }
}

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

mock! {
    pub DeviceHooks {}
    impl DeviceHooks for DeviceHooks {
        fn destroy_image(
            &self,
            _image: vk::Image,
            _p_allocator: Option<&'static vk::AllocationCallbacks>,
        ) -> LayerResult<()>;
    }
}

#[derive(Default)]
pub struct MockInstanceInfo<T: TestLayer> {
    pub mock_hooks: Mutex<MockInstanceHooks>,
    _marker: PhantomData<T>,
}

impl<T: TestLayer> InstanceInfo for MockInstanceInfo<T> {
    type HooksType = MockInstanceHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockInstanceHooks>;
    fn hooked_commands() -> &'static [LayerVulkanCommand] {
        T::hooked_instance_commands()
    }

    fn hooks(&self) -> Self::HooksRefType<'_> {
        self.mock_hooks.lock().unwrap()
    }
}

#[derive(Default)]
pub struct MockDeviceInfo<T: TestLayer> {
    pub mock_hooks: Mutex<MockDeviceHooks>,
    _marker: PhantomData<T>,
}

impl<T: TestLayer> DeviceInfo for MockDeviceInfo<T> {
    type HooksType = MockDeviceHooks;
    type HooksRefType<'a> = MutexGuard<'a, MockDeviceHooks>;
    fn hooked_commands() -> &'static [LayerVulkanCommand] {
        T::hooked_device_commands()
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

    fn hooked_global_commands() -> &'static [LayerVulkanCommand] {
        &[]
    }

    // Will only be used when work with MockInstanceInfo<Self> and TestLayerWrapper so that we can
    // use TestLayer and TestLayerWrapper with an actual InstanceInfo type that implements the
    // hooked_commands interface.
    fn hooked_instance_commands() -> &'static [LayerVulkanCommand] {
        &[]
    }

    // Will only be used when work with MockDeviceInfo<Self> and TestLayerWrapper so that we can
    // use TestLayer and TestLayerWrapper with an actual DeviceInfo type that implements the
    // hooked_commands interface.
    fn hooked_device_commands() -> &'static [LayerVulkanCommand] {
        &[]
    }
}

pub struct TestLayerWrapper<T, U = MockInstanceInfo<T>, V = MockDeviceInfo<T>>
where
    T: TestLayer,
    U: InstanceInfo + 'static,
    V: DeviceInfo + 'static,
{
    instances: Mutex<HashMap<vk::Instance, Weak<Del<U>>>>,
    devices: Mutex<HashMap<vk::Device, Weak<Del<V>>>>,
    pub global_hooks: Mutex<MockGlobalHooks>,
    _marker: PhantomData<T>,
}

impl<T, U, V> TestLayerWrapper<T, U, V>
where
    T: TestLayer,
    U: InstanceInfo + 'static,
    V: DeviceInfo + 'static,
{
    pub fn get_instance_info(&self, instance: vk::Instance) -> Option<Arc<impl Deref<Target = U>>> {
        self.instances
            .lock()
            .unwrap()
            .get(&instance)
            .and_then(Weak::upgrade)
    }

    pub fn get_device_info(&self, device: vk::Device) -> Option<Arc<impl Deref<Target = V>>> {
        self.devices
            .lock()
            .unwrap()
            .get(&device)
            .and_then(Weak::upgrade)
    }
}

impl<T, U, V> Default for TestLayerWrapper<T, U, V>
where
    T: TestLayer,
    U: InstanceInfo + 'static,
    V: DeviceInfo + 'static,
{
    fn default() -> Self {
        Self {
            instances: Default::default(),
            devices: Default::default(),
            global_hooks: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<T, U, V> GlobalHooks for Arc<TestLayerWrapper<T, U, V>>
where
    T: TestLayer,
    U: InstanceInfo + 'static,
    V: DeviceInfo + 'static,
{
    fn create_instance(
        &self,
        p_create_info: &'static vk::InstanceCreateInfo,
        p_allocator: Option<&'static vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Instance>> {
        self.global_hooks
            .lock()
            .unwrap()
            .create_instance(p_create_info, p_allocator)
    }
}

impl<T, U, V> Layer for Arc<TestLayerWrapper<T, U, V>>
where
    T: TestLayer,
    U: InstanceInfo + Default + 'static,
    V: DeviceInfo + Default + 'static,
{
    type DeviceInfo = V;
    type InstanceInfo = U;
    type DeviceInfoContainer = ArcDel<Self::DeviceInfo>;
    type InstanceInfoContainer = ArcDel<Self::InstanceInfo>;

    const LAYER_NAME: &'static str = <T as TestLayer>::LAYER_NAME;
    const LAYER_DESCRIPTION: &'static str = <T as TestLayer>::LAYER_DESCRIPTION;
    const IMPLEMENTATION_VERSION: u32 = <T as TestLayer>::IMPLEMENTATION_VERSION;
    const SPEC_VERSION: u32 = <T as TestLayer>::SPEC_VERSION;

    fn hooked_global_commands(&self) -> &[LayerVulkanCommand] {
        T::hooked_global_commands()
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
