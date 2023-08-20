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

/* automatically generated by rust-bindgen 0.65.1 */

impl VkNegotiateLayerStructType {
    pub const LAYER_NEGOTIATE_UNINTIALIZED: VkNegotiateLayerStructType =
        VkNegotiateLayerStructType(0);
}
impl VkNegotiateLayerStructType {
    pub const LAYER_NEGOTIATE_INTERFACE_STRUCT: VkNegotiateLayerStructType =
        VkNegotiateLayerStructType(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct VkNegotiateLayerStructType(pub ::std::os::raw::c_uint);
impl VkLayerFunction_ {
    pub const VK_LAYER_LINK_INFO: VkLayerFunction_ = VkLayerFunction_(0);
}
impl VkLayerFunction_ {
    pub const VK_LOADER_DATA_CALLBACK: VkLayerFunction_ = VkLayerFunction_(1);
}
impl VkLayerFunction_ {
    pub const VK_LOADER_LAYER_CREATE_DEVICE_CALLBACK: VkLayerFunction_ = VkLayerFunction_(2);
}
impl VkLayerFunction_ {
    pub const VK_LOADER_FEATURES: VkLayerFunction_ = VkLayerFunction_(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct VkLayerFunction_(pub ::std::os::raw::c_uint);
pub use self::VkLayerFunction_ as VkLayerFunction;