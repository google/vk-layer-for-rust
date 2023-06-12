use std::{marker::PhantomData, sync::Arc};

use ash::vk;

use crate::{DeviceInfo, InstanceInfo, Layer, LayerResult};

pub use crate::bindings::{
    VkLayerDeviceCreateInfo, VkLayerFunction, VkLayerInstanceCreateInfo, VkLayerInstanceLink,
};

pub struct MockInstanceInfo;

impl InstanceInfo for MockInstanceInfo {
    fn destroy_surface_khr(
        &self,
        _surface: vk::SurfaceKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
}

pub struct MockDeviceInfo;

impl DeviceInfo for MockDeviceInfo {}

// This Tag type parameter allow us to create different layer type in different tests, so that
// Global<T> in different tests can have different global instances. And this allows us to run unit
// tests in the same process on different threads.
#[derive(Default)]
pub struct MockLayer<Tag = ()> {
    _marker: PhantomData<Tag>,
}

impl<T: 'static + Sync + Default> Layer for MockLayer<T> {
    const LAYER_NAME: &'static str = "VK_LAYER_GOOGLE_stub";
    const LAYER_DESCRIPTION: &'static str = "";
    const IMPLEMENTATION_VERSION: u32 = 1;
    const SPEC_VERSION: u32 = vk::API_VERSION_1_1;

    type DeviceInfo = MockDeviceInfo;
    type InstanceInfo = MockInstanceInfo;

    fn create_device_info(
        &self,
        _: vk::PhysicalDevice,
        _: &vk::DeviceCreateInfo,
        _: Option<&vk::AllocationCallbacks>,
        _: Arc<ash::Device>,
        _: vk::PFN_vkGetDeviceProcAddr,
    ) -> Self::DeviceInfo {
        MockDeviceInfo
    }

    fn create_instance_info(
        &self,
        _: &vk::InstanceCreateInfo,
        _: Option<&vk::AllocationCallbacks>,
        _: Arc<ash::Instance>,
        _: vk::PFN_vkGetInstanceProcAddr,
    ) -> Self::InstanceInfo {
        MockInstanceInfo
    }
}
