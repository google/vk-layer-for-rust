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

pub(crate) struct VulkanBaseOutStructChain<'a> {
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
