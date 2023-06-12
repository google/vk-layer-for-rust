#![allow(non_camel_case_types)]
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
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use ash::vk::{self, *};

type VkInstance = vk::Instance;
type VkPhysicalDevice = vk::PhysicalDevice;
type VkDevice = vk::Device;
type VkStructureType = vk::StructureType;
type VkResult = vk::Result;
type VkDeviceCreateInfo = vk::DeviceCreateInfo;
type VkAllocationCallbacks = vk::AllocationCallbacks;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkLoaderFeatureFlags(vk::Flags);
ash::vk_bitflags_wrapped!(VkLoaderFeatureFlags, vk::Flags);
impl VkLoaderFeatureFlags {
    pub const _PHYSICAL_DEVICE_SORTING: Self = Self(0x00000001);
}

include!("bindings/generated/vk_layer_bindings.rs");

unsafe impl vk::TaggedStructure for VkLayerInstanceCreateInfo {
    const STRUCTURE_TYPE: vk::StructureType = vk::StructureType::LOADER_INSTANCE_CREATE_INFO;
}

unsafe impl ExtendsInstanceCreateInfo for VkLayerInstanceCreateInfo {}

unsafe impl vk::TaggedStructure for VkLayerDeviceCreateInfo {
    const STRUCTURE_TYPE: vk::StructureType = vk::StructureType::LOADER_DEVICE_CREATE_INFO;
}

unsafe impl ExtendsDeviceCreateInfo for VkLayerDeviceCreateInfo {}
