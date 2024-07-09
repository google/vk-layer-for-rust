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

use std::ffi::{c_void, CStr};

use ash::vk;

use smallvec::SmallVec;
use thiserror::Error;

pub mod generated;
pub use generated::*;

#[derive(Error, Debug)]
pub enum TryFromExtensionError {
    #[error("unknown extension `{0}`")]
    UnknownExtension(String),
}

/// A union type of extensions and core API version.
///
/// In `vk.xml`, Vulkan commands and types are grouped under different API version and extensions.
/// The tag of those group XML elements is `feature` or `extension`. One command will have only one
/// single correspondent feature. This type is mostly used to tell if a command should be returned
/// by `vkGet*ProcAddr` given the supported/enabled Vulkan API version and extensions.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Feature {
    /// Vulkan core API interface.
    Core(ApiVersion),
    /// Vulkan extension interface.
    Extension(Extension),
}

impl From<ApiVersion> for Feature {
    fn from(value: ApiVersion) -> Self {
        Self::Core(value)
    }
}

impl From<Extension> for Feature {
    fn from(value: Extension) -> Self {
        Self::Extension(value)
    }
}

/// Vulkan API version number.
///
/// Can be used to store the result decoded from
/// [`VK_MAKE_API_VERSION`](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_MAKE_API_VERSION.html).
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ApiVersion {
    /// The major version number. At most 7 bits.
    pub major: u8,
    /// The minor version number. At most 10 bits
    pub minor: u16,
}

impl ApiVersion {
    /// Vulkan version 1.0. The initial release of the Vulkan API.
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_0.html>
    pub const V1_0: Self = Self { major: 1, minor: 0 };
    /// Vulkan version 1.1.
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html>
    pub const V1_1: Self = Self { major: 1, minor: 1 };
}

impl From<u32> for ApiVersion {
    fn from(value: u32) -> Self {
        Self {
            major: vk::api_version_major(value)
                .try_into()
                .expect("The major version must be no more than 7 bits."),
            minor: vk::api_version_minor(value)
                .try_into()
                .expect("The minor version must be no more than 10 bits."),
        }
    }
}

impl From<ApiVersion> for u32 {
    fn from(value: ApiVersion) -> Self {
        vk::make_api_version(0, value.major.into(), value.minor.into(), 0)
    }
}

pub(crate) struct VulkanCommand {
    pub name: &'static str,
    pub features: SmallVec<[Feature; 2]>,
    pub hooked: bool,
    pub proc: vk::PFN_vkVoidFunction,
}

fn get_instance_proc_addr_loader(
    get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    instance: &ash::Instance,
) -> impl Fn(&CStr) -> *const c_void + '_ {
    move |name| {
        // Safe because the VkInstance is valid, and a valid C string pointer is passed to the
        // `p_name` parameter.
        let fp = unsafe { get_instance_proc_addr(instance.handle(), name.as_ptr()) };
        // It's safe to cast a function pointer to void*.
        unsafe { std::mem::transmute(fp) }
    }
}

fn get_device_proc_addr_loader(
    get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    device: &ash::Device,
) -> impl Fn(&CStr) -> *const c_void + '_ {
    move |name| {
        // Safe because the VkDevice is valid, and a valid C string pointer is passed to the
        // `p_name` parameter.
        let fp = unsafe { get_device_proc_addr(device.handle(), name.as_ptr()) };
        // It's safe to cast a function pointer to void*.
        unsafe { std::mem::transmute(fp) }
    }
}
