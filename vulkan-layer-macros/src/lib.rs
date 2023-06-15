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
pub fn auto_instanceinfo_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let layer_name = parse_macro_input!(attr as Type);
    let original_item: TokenStream2 = item.clone().into();
    let input = parse_macro_input!(item as ItemImpl);
    let to_append = details::autoinfo_impl(&input, &layer_name).unwrap_or_else(|e| {
        let dummy = dummy::dummy_autoinfo_impl(&input.self_ty, &layer_name);
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
