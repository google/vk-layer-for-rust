use std::{marker::PhantomData, sync::Arc};

use ash::vk;
use vulkan_layer_macros::impl_test_vulkan_layer;

use crate::{DeviceInfo, InstanceInfo, Layer};

pub use crate::bindings::{
    VkLayerDeviceCreateInfo, VkLayerFunction, VkLayerInstanceCreateInfo, VkLayerInstanceLink,
};

#[derive(Default)]
pub struct MockInstanceInfo;

impl InstanceInfo for MockInstanceInfo {}

#[derive(Default)]
pub struct MockDeviceInfo;

impl DeviceInfo for MockDeviceInfo {}

// This Tag type parameter allow us to create different layer type in different tests, so that
// Global<T> in different tests can have different global instances. And this allows us to run unit
// tests in the same process on different threads.
#[derive(Default)]
pub struct MockLayer<Tag = ()> {
    _marker: PhantomData<Tag>,
}

#[impl_test_vulkan_layer]
impl<T: 'static + Sync + Default> Layer for MockLayer<T> {
    type DeviceInfo = MockDeviceInfo;
    type InstanceInfo = MockInstanceInfo;
}
