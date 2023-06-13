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

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Error, ImplItem, ItemImpl};

pub(crate) fn expand_impl_test_vulkan_layer(
    mut item_impl: ItemImpl,
) -> Result<TokenStream2, Error> {
    let mut auto_impl_items = [
        quote! { const LAYER_NAME: &'static str = "VK_LAYER_GOOGLE_stub"; },
        quote! {const LAYER_DESCRIPTION: &'static str = ""; },
        quote! {const IMPLEMENTATION_VERSION: u32 = 1; },
        quote! {const SPEC_VERSION: u32 = vk::API_VERSION_1_1; },
        quote! {
            fn create_device_info(
                &self,
                _: vk::PhysicalDevice,
                _: &vk::DeviceCreateInfo,
                _: Option<&vk::AllocationCallbacks>,
                _: Arc<ash::Device>,
                _: vk::PFN_vkGetDeviceProcAddr,
            ) -> Self::DeviceInfo {
                Default::default()
            }
        },
        quote! {
            fn create_instance_info(
                &self,
                _: &vk::InstanceCreateInfo,
                _: Option<&vk::AllocationCallbacks>,
                _: Arc<ash::Instance>,
                _: vk::PFN_vkGetInstanceProcAddr,
            ) -> Self::InstanceInfo {
                Default::default()
            }
        },
    ]
    .into_iter()
    .map(ImplItem::Verbatim)
    .collect();
    item_impl.items.append(&mut auto_impl_items);
    Ok(item_impl.into_token_stream())
}
