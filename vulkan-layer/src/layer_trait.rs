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

use crate::{bindings::vk_layer::VkLayerInstanceLink, global_simple_intercept::Extension, Global};
use ash::vk;
use std::{borrow::Borrow, ffi::CString, ops::Deref, sync::Arc};
use thiserror::Error;

pub mod generated;
pub use generated::*;

#[must_use]
#[derive(Clone)]
pub enum LayerResult<T> {
    Handled(T),
    Unhandled,
}

#[derive(Error, Debug)]
pub enum TryFromVulkanCommandError {
    #[error("unknown command `{0}`")]
    UnknownCommand(String),
}

pub trait DeviceInfo: Send + Sync {
    type HooksType: DeviceHooks;
    type HooksRefType<'a>: Deref<Target = Self::HooksType> + 'a
    where
        Self: 'a;
    fn hooked_commands() -> &'static [VulkanCommand];
    fn hooks(&self) -> Self::HooksRefType<'_>;
}

pub trait InstanceInfo: Send + Sync {
    type HooksType: InstanceHooks;
    type HooksRefType<'a>: Deref<Target = Self::HooksType> + 'a
    where
        Self: 'a;
    fn hooked_commands() -> &'static [VulkanCommand];
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
    fn hooked_commands() -> &'static [VulkanCommand];
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

    fn hooked_instance_commands(&self) -> &[VulkanCommand] {
        Self::InstanceInfo::hooked_commands()
    }

    fn hooked_device_commands(&self) -> &[VulkanCommand] {
        Self::DeviceInfo::hooked_commands()
    }

    fn hooked_global_commands(&self) -> &[VulkanCommand] {
        Self::GlobalHooksInfo::hooked_commands()
    }

    fn hooked_commands(&self) -> Box<dyn Iterator<Item = VulkanCommand> + '_> {
        Box::new(
            self.hooked_global_commands()
                .iter()
                .chain(self.hooked_device_commands())
                .chain(self.hooked_instance_commands())
                .cloned(),
        )
    }
}
