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

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, Type};

mod details;
mod dummy;

#[proc_macro_attribute]
pub fn auto_globalhooksinfo_impl(_: TokenStream, item: TokenStream) -> TokenStream {
    let original_item: TokenStream2 = item.clone().into();
    let input = parse_macro_input!(item as ItemImpl);
    let target_trait = quote!(::vulkan_layer::GlobalHooksInfo);
    let to_append = details::autoinfo_impl(&input, &target_trait).unwrap_or_else(|e| {
        let dummy = dummy::dummy_autoinfo_impl(&input.self_ty, &target_trait);
        let compile_error = e.to_compile_error();
        quote! {
            #dummy
            #compile_error
        }
    });
    quote! {
        #original_item
        #to_append
    }
    .into()
}

#[proc_macro_attribute]
pub fn auto_instanceinfo_impl(_: TokenStream, item: TokenStream) -> TokenStream {
    let original_item: TokenStream2 = item.clone().into();
    let input = parse_macro_input!(item as ItemImpl);
    let target_trait = quote!(::vulkan_layer::InstanceInfo);
    let to_append = details::autoinfo_impl(&input, &target_trait).unwrap_or_else(|e| {
        let dummy = dummy::dummy_autoinfo_impl(&input.self_ty, &target_trait);
        let compile_error = e.to_compile_error();
        quote! {
            #dummy
            #compile_error
        }
    });
    quote! {
        #original_item
        #to_append
    }
    .into()
}

#[proc_macro_attribute]
pub fn auto_deviceinfo_impl(_: TokenStream, item: TokenStream) -> TokenStream {
    let original_item: TokenStream2 = item.clone().into();
    let input = parse_macro_input!(item as ItemImpl);
    let target_trait = quote!(::vulkan_layer::DeviceInfo);
    let to_append = details::autoinfo_impl(&input, &target_trait).unwrap_or_else(|e| {
        let dummy = dummy::dummy_autoinfo_impl(&input.self_ty, &target_trait);
        let compile_error = e.to_compile_error();
        quote! {
            #dummy
            #compile_error
        }
    });
    quote! {
        #original_item
        #to_append
    }
    .into()
}

/// Declare the required introspection queries for Android given an instantiated
/// `vulkan_layer::Global` type.
///
/// All functions are defined without name mangling, so that they are exported as C symbols in the generated dynamic library.  This is recommended by [the Vulkan loader doc](https://github.com/KhronosGroup/Vulkan-Loader/blob/280997da523951c4016f4ca6af66d58a31e36ab3/docs/LoaderLayerInterface.md#layer-manifest-file-usage:~:text=These%20introspection%20functions%20are%20not%20used%20by%20the%20Khronos%20loader%20but%20should%20be%20present%20in%20layers%20to%20maintain%20consistency.%20The%20specific%20%22introspection%22%20functions%20are%20called%20out%20in%20the%20Layer%20Manifest%20File%20Format%20table):
///
/// > These introspection functions are not used by the Khronos loader but should be present in
/// > layers to maintain consistency. The specific "introspection" functions are called out in the
/// > Layer Manifest File Format table.
///
/// According to the [the Vulkan loader doc](https://github.com/KhronosGroup/Vulkan-Loader/blob/280997da523951c4016f4ca6af66d58a31e36ab3/docs/LoaderLayerInterface.md#layer-manifest-file-format), introspection queries include:
/// * `vkEnumerateInstanceLayerProperties`
/// * `vkEnumerateInstanceExtensionProperties`
/// * `vkEnumerateDeviceLayerProperties`
/// * `vkEnumerateDeviceExtensionProperties`
/// * `vkGetInstanceProcAddr`
/// * `vkGetDeviceProcAddr`
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// # use vulkan_layer::{InstanceHooks, DeviceHooks, auto_instanceinfo_impl, auto_deviceinfo_impl, declare_introspection_queries, auto_globalhooksinfo_impl, GlobalHooks, Layer, LayerManifest, Global};
/// # use once_cell::sync::Lazy;
/// # use ash::{vk, self};
/// #
/// struct InstanceInfo;
///
/// #[auto_instanceinfo_impl]
/// impl InstanceHooks for InstanceInfo {}
///
/// struct DeviceInfo;
///
/// #[auto_deviceinfo_impl]
/// impl DeviceHooks for DeviceInfo {}
///
/// #[derive(Default)]
/// struct MyLayer;
///
/// #[auto_globalhooksinfo_impl]
/// impl GlobalHooks for MyLayer {}
///
/// impl Layer for MyLayer {
///     // ...
/// #     type GlobalHooksInfo = Self;
/// #     type InstanceInfo = InstanceInfo;
/// #     type DeviceInfo = DeviceInfo;
/// #     type InstanceInfoContainer = InstanceInfo;
/// #     type DeviceInfoContainer = DeviceInfo;
/// #     
/// #     fn global_instance() -> &'static Global<Self> {
/// #         static GLOBAL: Lazy<Global<MyLayer>> = Lazy::new(Default::default);
/// #         &*GLOBAL
/// #     }
/// #
/// #     fn manifest() -> LayerManifest {
/// #         Default::default()
/// #     }
/// #
/// #     fn get_global_hooks_info(&self) -> &Self::GlobalHooksInfo {
/// #         self
/// #     }
/// #
/// #     fn create_instance_info(
/// #         &self,
/// #         _: &vk::InstanceCreateInfo,
/// #         _: Option<&vk::AllocationCallbacks>,
/// #         _: Arc<ash::Instance>,
/// #         _next_get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
/// #     ) -> Self::InstanceInfoContainer {
/// #         InstanceInfo
/// #     }
/// #
/// #     fn create_device_info(
/// #         &self,
/// #         _: vk::PhysicalDevice,
/// #         _: &vk::DeviceCreateInfo,
/// #         _: Option<&vk::AllocationCallbacks>,
/// #         _: Arc<ash::Device>,
/// #         _next_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
/// #     ) -> Self::DeviceInfoContainer {
/// #         DeviceInfo
/// #     }
/// }
///
/// type MyGlobal = Global::<MyLayer>;
/// declare_introspection_queries!(MyGlobal);
/// # let _: vk::PFN_vkEnumerateInstanceLayerProperties = vkEnumerateInstanceLayerProperties;
/// # let _: vk::PFN_vkEnumerateInstanceExtensionProperties = vkEnumerateInstanceExtensionProperties;
/// # let _: vk::PFN_vkEnumerateDeviceLayerProperties = vkEnumerateDeviceLayerProperties;
/// # let _: vk::PFN_vkEnumerateDeviceExtensionProperties = vkEnumerateDeviceExtensionProperties;
/// # let _: vk::PFN_vkGetInstanceProcAddr = vkGetInstanceProcAddr;
/// # let _: vk::PFN_vkGetDeviceProcAddr = vkGetDeviceProcAddr;
/// ```
#[proc_macro]
pub fn declare_introspection_queries(item: TokenStream) -> TokenStream {
    let global_type = parse_macro_input!(item as Type);
    details::declare_introspection_queries_impl(&global_type)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
