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

cfg_if::cfg_if! {
    if #[cfg(feature = "_test")] {
        use proc_macro::TokenStream;
        use proc_macro2::TokenStream as TokenStream2;
        use syn::{parse_macro_input, ItemImpl};
        use quote::quote;

        mod test_utils;
        use test_utils::expand_impl_test_vulkan_layer;

        /// Provide the default values for some Layer trait items.
        #[proc_macro_attribute]
        pub fn impl_test_vulkan_layer(_attr: TokenStream, item: TokenStream) -> TokenStream {
            let original_item: TokenStream2 = item.clone().into();
            let item_impl = parse_macro_input!(item as ItemImpl);
            expand_impl_test_vulkan_layer(item_impl).unwrap_or_else(|e| {
                let compile_error = e.to_compile_error();
                quote!{
                    #original_item
                    #compile_error
                }
            }).into()
        }
    }
}
