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

use std::{
    borrow::Borrow,
    ffi::{c_void, CStr, CString},
    ops::Deref,
    sync::Arc,
};

use ash::vk;

use smallvec::SmallVec;
use thiserror::Error;

pub mod generated;
pub use generated::*;

use crate::{
    DeviceHooks, Global, InstanceHooks, LayerResult, LayerVulkanCommand, VkLayerInstanceLink,
};

#[derive(Error, Debug)]
pub enum TryFromExtensionError {
    #[error("unknown extension `{0}`")]
    UnknownExtension(String),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Feature {
    Core(ApiVersion),
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ApiVersion {
    // At most 7 bits
    pub major: u8,
    // At most 10 bits
    pub minor: u16,
}

impl ApiVersion {
    pub const V1_0: Self = Self { major: 1, minor: 0 };
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
        // It's safe to cast a funtion pointer to void*.
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
        // It's safe to cast a funtion pointer to void*.
        unsafe { std::mem::transmute(fp) }
    }
}

pub trait DeviceInfo: Send + Sync {
    type HooksType: DeviceHooks;
    type HooksRefType<'a>: Deref<Target = Self::HooksType> + 'a
    where
        Self: 'a;
    fn hooked_commands() -> &'static [LayerVulkanCommand];
    fn hooks(&self) -> Self::HooksRefType<'_>;
}

pub trait InstanceInfo: Send + Sync {
    type HooksType: InstanceHooks;
    type HooksRefType<'a>: Deref<Target = Self::HooksType> + 'a
    where
        Self: 'a;
    fn hooked_commands() -> &'static [LayerVulkanCommand];
    fn hooks(&self) -> Self::HooksRefType<'_>;
}

pub trait GlobalHooks: Send + Sync {
    fn create_instance(
        &self,
        _p_create_info: &vk::InstanceCreateInfo,
        _layer_instance_link: &VkLayerInstanceLink,
        _p_allocator: Option<&vk::AllocationCallbacks>,
        _p_instance: *mut vk::Instance,
    ) -> LayerResult<ash::prelude::VkResult<()>> {
        LayerResult::Unhandled
    }
}

pub trait GlobalHooksInfo: Send + Sync {
    type HooksType: GlobalHooks;
    type HooksRefType<'a>: Deref<Target = Self::HooksType> + 'a
    where
        Self: 'a;
    fn hooked_commands() -> &'static [LayerVulkanCommand];
    fn hooks(&self) -> Self::HooksRefType<'_>;
}

#[derive(Clone)]
pub struct ExtensionProperties {
    pub name: Extension,
    pub spec_version: u32,
}

impl From<ExtensionProperties> for vk::ExtensionProperties {
    fn from(ExtensionProperties { name, spec_version }: ExtensionProperties) -> Self {
        let name: &str = name.into();
        let name = CString::new(name).unwrap();
        let byte_len = name.as_bytes().len();
        let name = unsafe { std::slice::from_raw_parts(name.as_ptr(), byte_len) };
        let mut res = Self::builder()
            .extension_name([0; vk::MAX_EXTENSION_NAME_SIZE])
            .spec_version(spec_version)
            .build();
        res.extension_name[0..byte_len].copy_from_slice(name);
        res
    }
}

#[non_exhaustive]
#[derive(Default, Clone)]
pub struct LayerManifest {
    pub name: &'static str,
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: &'static str,
    pub device_extensions: &'static [ExtensionProperties],
}

pub trait Layer: Sync + Default + 'static {
    type GlobalHooksInfo: GlobalHooksInfo;
    type InstanceInfo: InstanceInfo;
    type DeviceInfo: DeviceInfo;
    type InstanceInfoContainer: Borrow<Self::InstanceInfo> + Sync + Send;
    type DeviceInfoContainer: Borrow<Self::DeviceInfo> + Sync + Send;

    fn manifest() -> LayerManifest;

    fn global_instance() -> &'static Global<Self>;

    fn get_global_hooks_info(&self) -> &Self::GlobalHooksInfo;

    fn get_global_hooks(&self) -> <Self::GlobalHooksInfo as GlobalHooksInfo>::HooksRefType<'_> {
        self.get_global_hooks_info().hooks()
    }

    fn create_instance_info(
        &self,
        create_info: &vk::InstanceCreateInfo,
        allocator: Option<&vk::AllocationCallbacks>,
        instance: Arc<ash::Instance>,
        next_get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    ) -> Self::InstanceInfoContainer;

    fn create_device_info(
        &self,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
        allocator: Option<&vk::AllocationCallbacks>,
        device: Arc<ash::Device>,
        next_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    ) -> Self::DeviceInfoContainer;

    fn hooked_instance_commands(&self) -> &[LayerVulkanCommand] {
        Self::InstanceInfo::hooked_commands()
    }

    fn hooked_device_commands(&self) -> &[LayerVulkanCommand] {
        Self::DeviceInfo::hooked_commands()
    }

    fn hooked_global_commands(&self) -> &[LayerVulkanCommand] {
        Self::GlobalHooksInfo::hooked_commands()
    }

    fn hooked_commands(&self) -> Box<dyn Iterator<Item = LayerVulkanCommand> + '_> {
        Box::new(
            self.hooked_global_commands()
                .iter()
                .chain(self.hooked_device_commands())
                .chain(self.hooked_instance_commands())
                .cloned(),
        )
    }
}
