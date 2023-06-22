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

use std::ffi::c_char;

use ash::vk;

mod layer_impl;
mod bindings;
mod cros_gralloc_helpers;

type Global = vulkan_layer::Global<layer_impl::NexusLayer>;

// Introspection queryies required by Android:
// vkEnumerateInstanceLayerProperties, vkEnumerateInstanceExtensionProperties,
// vkEnumerateDeviceLayerProperties, vkEnumerateDeviceExtensionProperties, vkGetInstanceProcAddr,
// vkGetDeviceProcAddr

/// # Safety
/// See valid usage of `vkEnumerateInstanceLayerProperties` at
/// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html
#[deny(unsafe_op_in_unsafe_fn)]
#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceLayerProperties(
    property_count: *mut u32,
    properties: *mut vk::LayerProperties,
) -> vk::Result {
    // Safe, because the caller is supposed to follow the exact same safety requirement.
    unsafe { Global::enumerate_instance_layer_properties(property_count, properties) }
}

/// # Safety
/// See valid usage of `vkEnumerateInstanceExtensionProperties` at
/// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html.
#[deny(unsafe_op_in_unsafe_fn)]
#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceExtensionProperties(
    layer_name: *const c_char,
    property_count: *mut u32,
    p_properties: *mut vk::ExtensionProperties,
) -> vk::Result {
    // Safe, because the caller is supposed to follow the exact same safety requirement.
    unsafe {
        Global::enumerate_instance_extension_properties(layer_name, property_count, p_properties)
    }
}

/// # Safety
/// See valid usage of `vkEnumerateDeviceLayerProperties` at
/// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html.
#[deny(unsafe_op_in_unsafe_fn)]
#[no_mangle]
pub unsafe extern "C" fn vkEnumerateDeviceLayerProperties(
    physical_device: vk::PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut vk::LayerProperties,
) -> vk::Result {
    // Safe, because the caller is supposed to follow the exact same safety requirement.
    unsafe {
        Global::enumerate_device_layer_properties(physical_device, p_property_count, p_properties)
    }
}

/// # Safety
/// See valid usage of `vkEnumerateDeviceExtensionProperties` at
/// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html.
#[deny(unsafe_op_in_unsafe_fn)]
#[no_mangle]
pub unsafe extern "C" fn vkEnumerateDeviceExtensionProperties(
    physical_device: vk::PhysicalDevice,
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut vk::ExtensionProperties,
) -> vk::Result {
    // Safe, because the caller is supposed to follow the exact same safety requirement.
    unsafe {
        Global::enumerate_device_extension_properties(
            physical_device,
            p_layer_name,
            p_property_count,
            p_properties,
        )
    }
}

/// # Safety
/// See valid usage of `vkGetInstanceProcAddr` at
/// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html.
#[deny(unsafe_op_in_unsafe_fn)]
#[no_mangle]
pub unsafe extern "C" fn vkGetInstanceProcAddr(
    instance: vk::Instance,
    p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    // Safe, because the caller is supposed to follow the exact same safety requirement.
    unsafe { Global::get_instance_proc_addr(instance, p_name) }
}

/// # Safety
/// See valid usage of `vkGetInstanceProcAddr` at
/// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html.
#[deny(unsafe_op_in_unsafe_fn)]
#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceProcAddr(
    device: vk::Device,
    p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    // Safe, because the caller is supposed to follow the exact same safety requirement.
    unsafe { Global::get_device_proc_addr(device, p_name) }
}
