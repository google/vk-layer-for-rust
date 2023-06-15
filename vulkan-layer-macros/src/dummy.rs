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
use quote::quote;
use syn::Type;

pub fn dummy_autoinfo_impl(name: &Type, layer_name: &Type) -> TokenStream2 {
    quote! {
        impl ::vulkan_layer::InstanceInfo for #name {
            type LayerInfo = #layer_name;
            type HooksType = Self;
            type HooksRefType<'a> = &'a Self;
            fn hooked_commands(_: &Self::LayerInfo) -> &[::vulkan_layer::LayerVulkanCommand] {
                unimplemented!()
            }

            fn hooks(&self) -> Self::HooksRefType<'_> {
                unimplemented!()
            }
        }
    }
}
