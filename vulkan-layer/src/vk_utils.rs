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

use std::collections::BTreeSet;

use ash::vk;

use crate::global_simple_intercept::{ApiVersion, Extension, Feature};

/// A chain of
/// [`VkBaseOutStructure`](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseOutStructure.html).
///
/// This type provides an easy way to visit the `*mut c_void`
/// [`pNext` chain](https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-validusage-pNext).
/// One can convert an `Option<&mut vk::BaseOutStructure>` to [`VulkanBaseOutStructChain`].
///
/// # Examples
///
/// ```
/// use ash::vk;
/// use vulkan_layer::VulkanBaseOutStructChain;
///
/// let mut timeline_semaphore_feature = vk::PhysicalDeviceTimelineSemaphoreFeatures::builder();
/// let mut ycbcr_feature = vk::PhysicalDeviceSamplerYcbcrConversionFeatures::builder();
/// let features2 = vk::PhysicalDeviceFeatures2::builder()
///     .push_next(&mut timeline_semaphore_feature)
///     .push_next(&mut ycbcr_feature);
/// let out_chain: *mut vk::BaseOutStructure = unsafe { std::mem::transmute(features2.p_next) };
/// let mut out_chain: VulkanBaseOutStructChain = unsafe { out_chain.as_mut() }.into();
/// // `push_next` prepends the element to the pNext chain.
/// assert_eq!(
///     out_chain.next().unwrap().s_type,
///     vk::StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES
/// );
/// assert_eq!(
///     out_chain.next().unwrap().s_type,
///     vk::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES
/// );
/// assert!(out_chain.next().is_none());
/// ```
pub struct VulkanBaseOutStructChain<'a> {
    next: Option<&'a mut vk::BaseOutStructure>,
}

impl<'a> From<Option<&'a mut vk::BaseOutStructure>> for VulkanBaseOutStructChain<'a> {
    fn from(next: Option<&'a mut vk::BaseOutStructure>) -> Self {
        Self { next }
    }
}

impl<'a> Iterator for VulkanBaseOutStructChain<'a> {
    type Item = &'a mut vk::BaseOutStructure;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|element| {
            self.next = unsafe { element.p_next.as_mut() };
            element
        })
    }
}

/// A chain of
/// [`VkBaseInStructure`](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseInStructure.html).
///
/// This type provides an easy way to visit the `*const c_void`
/// [`pNext` chain](https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-validusage-pNext).
/// One can convert an `Option<&vk::BaseInStructure>` to [`VulkanBaseInStructChain`].
///
/// # Examples
/// ```
/// use ash::vk;
/// use vulkan_layer::VulkanBaseInStructChain;
///
/// let mut external_memory_image_info = vk::ExternalMemoryImageCreateInfo::builder();
/// let mut image_format_list_info = vk::ImageFormatListCreateInfo::builder();
/// let image_create_info = vk::ImageCreateInfo::builder()
///     .push_next(&mut external_memory_image_info)
///     .push_next(&mut image_format_list_info);
/// let in_chain: *const vk::BaseInStructure =
///     unsafe { std::mem::transmute(image_create_info.p_next) };
/// let mut in_chain: VulkanBaseInStructChain = unsafe { in_chain.as_ref() }.into();
/// // `push_next` prepends the element to the pNext chain.
/// assert_eq!(
///     in_chain.next().unwrap().s_type,
///     vk::StructureType::IMAGE_FORMAT_LIST_CREATE_INFO
/// );
/// assert_eq!(
///     in_chain.next().unwrap().s_type,
///     vk::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO
/// );
/// assert!(in_chain.next().is_none());
/// ```
pub struct VulkanBaseInStructChain<'a> {
    next: Option<&'a vk::BaseInStructure>,
}

impl<'a> From<Option<&'a vk::BaseInStructure>> for VulkanBaseInStructChain<'a> {
    fn from(next: Option<&'a vk::BaseInStructure>) -> Self {
        Self { next }
    }
}

impl<'a> Iterator for VulkanBaseInStructChain<'a> {
    type Item = &'a vk::BaseInStructure;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|element| {
            self.next = unsafe { element.p_next.as_ref() };
            element
        })
    }
}

/// Used to extend an iterator of [`Feature`].
pub trait IsCommandEnabled {
    /// Given a list of [`Feature`]s where the command is defined, the Vulkan API version and a set
    /// of enabled extensions, returns if the command is enabled.
    ///
    /// One command with the same name can appear in different features, e.g.
    /// [`vkCmdBindVertexBuffers2EXT`](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2EXT.html)
    /// is defined in both
    /// [`VK_EXT_extended_dynamic_state`](<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state.html#:~:text=New%20Commands-,vkCmdBindVertexBuffers2EXT,-vkCmdSetCullModeEXT>)
    /// and
    /// [`VK_EXT_shader_object`](<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html#:~:text=vkCmdBindShadersEXT-,vkCmdBindVertexBuffers2EXT,-vkCmdSetAlphaToCoverageEnableEXT>).
    /// `vkGetDeviceProcAddr` should return a valid function pointer `vkCmdBindVertexBuffers2EXT` if
    /// either extension is enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ash::vk;
    /// use std::collections::BTreeSet;
    /// use vulkan_layer::{
    ///     unstable_api::{ApiVersion, Feature, IsCommandEnabled},
    ///     Extension,
    /// };
    ///
    /// let features: Vec<Feature> = vec![
    ///     Extension::EXTExtendedDynamicState.into(),
    ///     Extension::EXTShaderObject.into(),
    /// ];
    /// assert!(features.is_command_enabled(
    ///     &ApiVersion::V1_1,
    ///     &BTreeSet::from([Extension::EXTExtendedDynamicState.into()])
    /// ));
    /// assert!(features.is_command_enabled(
    ///     &ApiVersion::V1_1,
    ///     &BTreeSet::from([Extension::EXTShaderObject.into()])
    /// ));
    /// assert!(!features.is_command_enabled(&ApiVersion::V1_1, &BTreeSet::new()));
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_command_enabled(
        self,
        api_version: &ApiVersion,
        enabled_extensions: &BTreeSet<Extension>,
    ) -> bool;
}

impl<'a, T: IntoIterator<Item = &'a Feature>> IsCommandEnabled for T {
    fn is_command_enabled(
        self,
        api_version: &ApiVersion,
        enabled_extensions: &BTreeSet<Extension>,
    ) -> bool {
        self.into_iter().any(|feature| match feature {
            Feature::Extension(extension) => enabled_extensions.contains(extension),
            Feature::Core(version) => version <= api_version,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiple_in_struct_nodes() {
        let mut external_memory_image_info = vk::ExternalMemoryImageCreateInfo::builder();
        let mut image_format_list_info = vk::ImageFormatListCreateInfo::builder();
        let image_create_info = vk::ImageCreateInfo::builder()
            .push_next(&mut external_memory_image_info)
            .push_next(&mut image_format_list_info);
        let in_chain: *const vk::BaseInStructure =
            unsafe { std::mem::transmute(image_create_info.p_next) };
        let mut in_chain: VulkanBaseInStructChain = unsafe { in_chain.as_ref() }.into();
        // `push_next` prepends the element to the pNext chain.
        assert_eq!(
            in_chain.next().unwrap().s_type,
            vk::StructureType::IMAGE_FORMAT_LIST_CREATE_INFO
        );
        assert_eq!(
            in_chain.next().unwrap().s_type,
            vk::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO
        );
        assert!(in_chain.next().is_none());
    }

    #[test]
    fn multiple_out_struct_nodes() {
        let mut timeline_semaphore_feature = vk::PhysicalDeviceTimelineSemaphoreFeatures::builder();
        let mut ycbcr_feature = vk::PhysicalDeviceSamplerYcbcrConversionFeatures::builder();
        let features2 = vk::PhysicalDeviceFeatures2::builder()
            .push_next(&mut timeline_semaphore_feature)
            .push_next(&mut ycbcr_feature);
        let out_chain: *mut vk::BaseOutStructure = unsafe { std::mem::transmute(features2.p_next) };
        let mut out_chain: VulkanBaseOutStructChain = unsafe { out_chain.as_mut() }.into();
        // `push_next` prepends the element to the pNext chain.
        assert_eq!(
            out_chain.next().unwrap().s_type,
            vk::StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES
        );
        assert_eq!(
            out_chain.next().unwrap().s_type,
            vk::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES
        );
        assert!(out_chain.next().is_none());
    }

    #[test]
    fn test_is_command_enabled() {
        let features: Vec<Feature> = vec![
            Extension::EXTExtendedDynamicState.into(),
            Extension::EXTShaderObject.into(),
        ];
        assert!(features.is_command_enabled(
            &ApiVersion::V1_1,
            &BTreeSet::from([Extension::EXTExtendedDynamicState])
        ));
        assert!(features.is_command_enabled(
            &ApiVersion::V1_1,
            &BTreeSet::from([Extension::EXTShaderObject])
        ));
        assert!(!features.is_command_enabled(&ApiVersion::V1_1, &BTreeSet::new()));
    }
}
