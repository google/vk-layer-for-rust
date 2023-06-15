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
use std::iter::zip;
use syn::{spanned::Spanned, Error, Ident, ImplItem, ItemImpl, Type};

fn snake_case_to_upper_camel_case(input: &str) -> String {
    let first_char = match input.chars().next() {
        Some(first) => first,
        None => return input.to_owned(),
    };
    let first_char = first_char.to_uppercase();
    let mut res: String = first_char.to_string();
    for (prev, (cur, next)) in zip(
        input.chars(),
        zip(input.chars().skip(1), input.chars().skip(2)),
    ) {
        if cur == '_' && next != '_' {
            continue;
        }
        if prev == '_' {
            res.push_str(&cur.to_uppercase().to_string());
            continue;
        }
        res.push(cur);
    }
    res.push(input.chars().last().unwrap());
    res
}

pub fn autoinfo_impl(item: &ItemImpl, layer_type: &Type) -> Result<TokenStream2, Error> {
    let type_name = item.self_ty.as_ref();
    let hooked_commands = item.items.iter().filter_map(|item| {
        let function = if let ImplItem::Fn(function) = item {
            function
        } else {
            return None;
        };
        let func_name = function.sig.ident.to_string();
        let enum_variant_name = snake_case_to_upper_camel_case(&func_name);
        let enum_variant_name = Ident::new(&enum_variant_name, function.span());
        Some(quote!(::vulkan_layer::LayerVulkanCommand::#enum_variant_name))
    });
    Ok(quote! {
        impl ::vulkan_layer::InstanceInfo for #type_name {
            type LayerInfo = #layer_type;
            type HooksType = Self;
            type HooksRefType<'a> = &'a Self;
            fn hooked_commands(_: &Self::LayerInfo) -> &[::vulkan_layer::LayerVulkanCommand] {
                &[#(#hooked_commands),*]
            }

            fn hooks(&self) -> &Self {
                self
            }
        }
    })
}
