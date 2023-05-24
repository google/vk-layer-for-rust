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

use ash::vk;

struct VulkanBaseOutStructChain<'a> {
    next: Option<&'a mut vk::BaseOutStructure>,
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

pub(crate) fn find_struct<T: vk::TaggedStructure>(
    chain: Option<&mut vk::BaseOutStructure>,
) -> impl Iterator<Item = &mut T> {
    let iter = VulkanBaseOutStructChain { next: chain };
    iter.filter_map(|base_out_structure| {
        if base_out_structure.s_type == T::STRUCTURE_TYPE {
            Some(unsafe { std::mem::transmute(base_out_structure) })
        } else {
            None
        }
    })
}
