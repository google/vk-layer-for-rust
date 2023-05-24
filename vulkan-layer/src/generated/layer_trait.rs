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

// This file is generated from the Vulkan XML API registry.
#![allow(clippy::too_many_arguments)]
use std::ffi::{c_int, c_void};
use std::sync::Arc;

use ash::{prelude::VkResult, vk};

use super::LayerResult;

// Unhandled commands:
// * vkMapMemory2KHR: The ash Rust binding doesn't have proper bindings yet.
// * vkUnmapMemory2KHR: The ash Rust binding doesn't have proper bindings yet.
// * vkCmdSetDiscardRectangleEnableEXT: The ash Rust binding doesn't have proper bindings yet.
// * vkCmdSetDiscardRectangleModeEXT: The ash Rust binding doesn't have proper bindings yet.
// * vkCmdSetExclusiveScissorEnableNV: The ash Rust binding doesn't have proper bindings yet.
// * vkGetDeviceFaultInfoEXT: The length info and the data pointer are nested in structs.
// * vkCmdDrawClusterHUAWEI: The ash Rust binding doesn't have proper bindings yet.
// * vkCmdDrawClusterIndirectHUAWEI: The ash Rust binding doesn't have proper bindings yet.
// * vkCreateShadersEXT: The ash Rust binding doesn't have proper bindings yet.
// * vkDestroyShaderEXT: The ash Rust binding doesn't have proper bindings yet.
// * vkGetShaderBinaryDataEXT: The ash Rust binding doesn't have proper bindings yet.
// * vkCmdBindShadersEXT: The ash Rust binding doesn't have proper bindings yet.
// * vkCmdSetAttachmentFeedbackLoopEnableEXT: The ash Rust binding doesn't have proper bindings yet.
// * vkCmdBuildAccelerationStructuresKHR: Dynamic multi-dimensional array bindings are not supported
//   yet.
// * vkCmdBuildAccelerationStructuresIndirectKHR: Dynamic multi-dimensional array bindings are not
//   supported yet.
// * vkBuildAccelerationStructuresKHR: Dynamic multi-dimensional array bindings are not supported
//   yet.
pub trait DeviceInfo: Send + Sync {
    fn get_device_queue(
        &self,
        _queue_family_index: u32,
        _queue_index: u32,
    ) -> LayerResult<vk::Queue> {
        LayerResult::Unhandled
    }
    fn queue_submit(
        &self,
        _queue: vk::Queue,
        _p_submits: &[vk::SubmitInfo],
        _fence: vk::Fence,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn queue_wait_idle(&self, _queue: vk::Queue) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn device_wait_idle(&self) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn allocate_memory(
        &self,
        _p_allocate_info: &vk::MemoryAllocateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::DeviceMemory>> {
        LayerResult::Unhandled
    }
    fn free_memory(
        &self,
        _memory: vk::DeviceMemory,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn map_memory(
        &self,
        _memory: vk::DeviceMemory,
        _offset: vk::DeviceSize,
        _size: vk::DeviceSize,
        _flags: vk::MemoryMapFlags,
    ) -> LayerResult<VkResult<Option<*mut c_void>>> {
        LayerResult::Unhandled
    }
    fn unmap_memory(&self, _memory: vk::DeviceMemory) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn flush_mapped_memory_ranges(
        &self,
        _p_memory_ranges: &[vk::MappedMemoryRange],
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn invalidate_mapped_memory_ranges(
        &self,
        _p_memory_ranges: &[vk::MappedMemoryRange],
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_device_memory_commitment(
        &self,
        _memory: vk::DeviceMemory,
    ) -> LayerResult<vk::DeviceSize> {
        LayerResult::Unhandled
    }
    fn bind_buffer_memory(
        &self,
        _buffer: vk::Buffer,
        _memory: vk::DeviceMemory,
        _memory_offset: vk::DeviceSize,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn bind_image_memory(
        &self,
        _image: vk::Image,
        _memory: vk::DeviceMemory,
        _memory_offset: vk::DeviceSize,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_buffer_memory_requirements(
        &self,
        _buffer: vk::Buffer,
    ) -> LayerResult<vk::MemoryRequirements> {
        LayerResult::Unhandled
    }
    fn get_image_memory_requirements(
        &self,
        _image: vk::Image,
    ) -> LayerResult<vk::MemoryRequirements> {
        LayerResult::Unhandled
    }
    fn get_image_sparse_memory_requirements(
        &self,
        _image: vk::Image,
    ) -> LayerResult<Vec<vk::SparseImageMemoryRequirements>> {
        LayerResult::Unhandled
    }
    fn queue_bind_sparse(
        &self,
        _queue: vk::Queue,
        _p_bind_info: &[vk::BindSparseInfo],
        _fence: vk::Fence,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn create_fence(
        &self,
        _p_create_info: &vk::FenceCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Fence>> {
        LayerResult::Unhandled
    }
    fn destroy_fence(
        &self,
        _fence: vk::Fence,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn reset_fences(&self, _p_fences: &[vk::Fence]) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_fence_status(&self, _fence: vk::Fence) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn wait_for_fences(
        &self,
        _p_fences: &[vk::Fence],
        _wait_all: bool,
        _timeout: u64,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn create_semaphore(
        &self,
        _p_create_info: &vk::SemaphoreCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Semaphore>> {
        LayerResult::Unhandled
    }
    fn destroy_semaphore(
        &self,
        _semaphore: vk::Semaphore,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_event(
        &self,
        _p_create_info: &vk::EventCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Event>> {
        LayerResult::Unhandled
    }
    fn destroy_event(
        &self,
        _event: vk::Event,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_event_status(&self, _event: vk::Event) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn set_event(&self, _event: vk::Event) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn reset_event(&self, _event: vk::Event) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn create_query_pool(
        &self,
        _p_create_info: &vk::QueryPoolCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::QueryPool>> {
        LayerResult::Unhandled
    }
    fn destroy_query_pool(
        &self,
        _query_pool: vk::QueryPool,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_query_pool_results(
        &self,
        _query_pool: vk::QueryPool,
        _first_query: u32,
        _query_count: u32,
        _p_data: &mut [u8],
        _stride: vk::DeviceSize,
        _flags: vk::QueryResultFlags,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn create_buffer(
        &self,
        _p_create_info: &vk::BufferCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Buffer>> {
        LayerResult::Unhandled
    }
    fn destroy_buffer(
        &self,
        _buffer: vk::Buffer,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_buffer_view(
        &self,
        _p_create_info: &vk::BufferViewCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::BufferView>> {
        LayerResult::Unhandled
    }
    fn destroy_buffer_view(
        &self,
        _buffer_view: vk::BufferView,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_image(
        &self,
        _p_create_info: &vk::ImageCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Image>> {
        LayerResult::Unhandled
    }
    fn destroy_image(
        &self,
        _image: vk::Image,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_image_subresource_layout(
        &self,
        _image: vk::Image,
        _p_subresource: &vk::ImageSubresource,
    ) -> LayerResult<vk::SubresourceLayout> {
        LayerResult::Unhandled
    }
    fn create_image_view(
        &self,
        _p_create_info: &vk::ImageViewCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::ImageView>> {
        LayerResult::Unhandled
    }
    fn destroy_image_view(
        &self,
        _image_view: vk::ImageView,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_shader_module(
        &self,
        _p_create_info: &vk::ShaderModuleCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::ShaderModule>> {
        LayerResult::Unhandled
    }
    fn destroy_shader_module(
        &self,
        _shader_module: vk::ShaderModule,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_pipeline_cache(
        &self,
        _p_create_info: &vk::PipelineCacheCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::PipelineCache>> {
        LayerResult::Unhandled
    }
    fn destroy_pipeline_cache(
        &self,
        _pipeline_cache: vk::PipelineCache,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_pipeline_cache_data(
        &self,
        _pipeline_cache: vk::PipelineCache,
    ) -> LayerResult<VkResult<Vec<u8>>> {
        LayerResult::Unhandled
    }
    fn merge_pipeline_caches(
        &self,
        _dst_cache: vk::PipelineCache,
        _p_src_caches: &[vk::PipelineCache],
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn create_graphics_pipelines(
        &self,
        _pipeline_cache: vk::PipelineCache,
        _p_create_infos: &[vk::GraphicsPipelineCreateInfo],
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<Vec<vk::Pipeline>>> {
        LayerResult::Unhandled
    }
    fn create_compute_pipelines(
        &self,
        _pipeline_cache: vk::PipelineCache,
        _p_create_infos: &[vk::ComputePipelineCreateInfo],
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<Vec<vk::Pipeline>>> {
        LayerResult::Unhandled
    }
    fn destroy_pipeline(
        &self,
        _pipeline: vk::Pipeline,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_pipeline_layout(
        &self,
        _p_create_info: &vk::PipelineLayoutCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::PipelineLayout>> {
        LayerResult::Unhandled
    }
    fn destroy_pipeline_layout(
        &self,
        _pipeline_layout: vk::PipelineLayout,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_sampler(
        &self,
        _p_create_info: &vk::SamplerCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Sampler>> {
        LayerResult::Unhandled
    }
    fn destroy_sampler(
        &self,
        _sampler: vk::Sampler,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_descriptor_set_layout(
        &self,
        _p_create_info: &vk::DescriptorSetLayoutCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::DescriptorSetLayout>> {
        LayerResult::Unhandled
    }
    fn destroy_descriptor_set_layout(
        &self,
        _descriptor_set_layout: vk::DescriptorSetLayout,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_descriptor_pool(
        &self,
        _p_create_info: &vk::DescriptorPoolCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::DescriptorPool>> {
        LayerResult::Unhandled
    }
    fn destroy_descriptor_pool(
        &self,
        _descriptor_pool: vk::DescriptorPool,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn reset_descriptor_pool(
        &self,
        _descriptor_pool: vk::DescriptorPool,
        _flags: vk::DescriptorPoolResetFlags,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn allocate_descriptor_sets(
        &self,
        _p_allocate_info: &vk::DescriptorSetAllocateInfo,
    ) -> LayerResult<VkResult<Vec<vk::DescriptorSet>>> {
        LayerResult::Unhandled
    }
    fn free_descriptor_sets(
        &self,
        _descriptor_pool: vk::DescriptorPool,
        _p_descriptor_sets: &[vk::DescriptorSet],
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn update_descriptor_sets(
        &self,
        _p_descriptor_writes: &[vk::WriteDescriptorSet],
        _p_descriptor_copies: &[vk::CopyDescriptorSet],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_framebuffer(
        &self,
        _p_create_info: &vk::FramebufferCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Framebuffer>> {
        LayerResult::Unhandled
    }
    fn destroy_framebuffer(
        &self,
        _framebuffer: vk::Framebuffer,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_render_pass(
        &self,
        _p_create_info: &vk::RenderPassCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::RenderPass>> {
        LayerResult::Unhandled
    }
    fn destroy_render_pass(
        &self,
        _render_pass: vk::RenderPass,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_render_area_granularity(
        &self,
        _render_pass: vk::RenderPass,
    ) -> LayerResult<vk::Extent2D> {
        LayerResult::Unhandled
    }
    fn create_command_pool(
        &self,
        _p_create_info: &vk::CommandPoolCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::CommandPool>> {
        LayerResult::Unhandled
    }
    fn destroy_command_pool(
        &self,
        _command_pool: vk::CommandPool,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn reset_command_pool(
        &self,
        _command_pool: vk::CommandPool,
        _flags: vk::CommandPoolResetFlags,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn allocate_command_buffers(
        &self,
        _p_allocate_info: &vk::CommandBufferAllocateInfo,
    ) -> LayerResult<VkResult<Vec<vk::CommandBuffer>>> {
        LayerResult::Unhandled
    }
    fn free_command_buffers(
        &self,
        _command_pool: vk::CommandPool,
        _p_command_buffers: &[vk::CommandBuffer],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn begin_command_buffer(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_begin_info: &vk::CommandBufferBeginInfo,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn end_command_buffer(&self, _command_buffer: vk::CommandBuffer) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn reset_command_buffer(
        &self,
        _command_buffer: vk::CommandBuffer,
        _flags: vk::CommandBufferResetFlags,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn cmd_bind_pipeline(
        &self,
        _command_buffer: vk::CommandBuffer,
        _pipeline_bind_point: vk::PipelineBindPoint,
        _pipeline: vk::Pipeline,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_viewport(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_viewport: u32,
        _p_viewports: &[vk::Viewport],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_scissor(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_scissor: u32,
        _p_scissors: &[vk::Rect2D],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_line_width(
        &self,
        _command_buffer: vk::CommandBuffer,
        _line_width: f32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_depth_bias(
        &self,
        _command_buffer: vk::CommandBuffer,
        _depth_bias_constant_factor: f32,
        _depth_bias_clamp: f32,
        _depth_bias_slope_factor: f32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_blend_constants(
        &self,
        _command_buffer: vk::CommandBuffer,
        _blend_constants: &[f32; 4],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_depth_bounds(
        &self,
        _command_buffer: vk::CommandBuffer,
        _min_depth_bounds: f32,
        _max_depth_bounds: f32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_stencil_compare_mask(
        &self,
        _command_buffer: vk::CommandBuffer,
        _face_mask: vk::StencilFaceFlags,
        _compare_mask: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_stencil_write_mask(
        &self,
        _command_buffer: vk::CommandBuffer,
        _face_mask: vk::StencilFaceFlags,
        _write_mask: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_stencil_reference(
        &self,
        _command_buffer: vk::CommandBuffer,
        _face_mask: vk::StencilFaceFlags,
        _reference: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_bind_descriptor_sets(
        &self,
        _command_buffer: vk::CommandBuffer,
        _pipeline_bind_point: vk::PipelineBindPoint,
        _layout: vk::PipelineLayout,
        _first_set: u32,
        _p_descriptor_sets: &[vk::DescriptorSet],
        _p_dynamic_offsets: &[u32],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_bind_index_buffer(
        &self,
        _command_buffer: vk::CommandBuffer,
        _buffer: vk::Buffer,
        _offset: vk::DeviceSize,
        _index_type: vk::IndexType,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_bind_vertex_buffers(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_binding: u32,
        _p_buffers: &[vk::Buffer],
        _p_offsets: &[vk::DeviceSize],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw(
        &self,
        _command_buffer: vk::CommandBuffer,
        _vertex_count: u32,
        _instance_count: u32,
        _first_vertex: u32,
        _first_instance: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_indexed(
        &self,
        _command_buffer: vk::CommandBuffer,
        _index_count: u32,
        _instance_count: u32,
        _first_index: u32,
        _vertex_offset: i32,
        _first_instance: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_indirect(
        &self,
        _command_buffer: vk::CommandBuffer,
        _buffer: vk::Buffer,
        _offset: vk::DeviceSize,
        _draw_count: u32,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_indexed_indirect(
        &self,
        _command_buffer: vk::CommandBuffer,
        _buffer: vk::Buffer,
        _offset: vk::DeviceSize,
        _draw_count: u32,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_dispatch(
        &self,
        _command_buffer: vk::CommandBuffer,
        _group_countx: u32,
        _group_county: u32,
        _group_countz: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_dispatch_indirect(
        &self,
        _command_buffer: vk::CommandBuffer,
        _buffer: vk::Buffer,
        _offset: vk::DeviceSize,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_buffer(
        &self,
        _command_buffer: vk::CommandBuffer,
        _src_buffer: vk::Buffer,
        _dst_buffer: vk::Buffer,
        _p_regions: &[vk::BufferCopy],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_image(
        &self,
        _command_buffer: vk::CommandBuffer,
        _src_image: vk::Image,
        _src_image_layout: vk::ImageLayout,
        _dst_image: vk::Image,
        _dst_image_layout: vk::ImageLayout,
        _p_regions: &[vk::ImageCopy],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_blit_image(
        &self,
        _command_buffer: vk::CommandBuffer,
        _src_image: vk::Image,
        _src_image_layout: vk::ImageLayout,
        _dst_image: vk::Image,
        _dst_image_layout: vk::ImageLayout,
        _p_regions: &[vk::ImageBlit],
        _filter: vk::Filter,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_buffer_to_image(
        &self,
        _command_buffer: vk::CommandBuffer,
        _src_buffer: vk::Buffer,
        _dst_image: vk::Image,
        _dst_image_layout: vk::ImageLayout,
        _p_regions: &[vk::BufferImageCopy],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_image_to_buffer(
        &self,
        _command_buffer: vk::CommandBuffer,
        _src_image: vk::Image,
        _src_image_layout: vk::ImageLayout,
        _dst_buffer: vk::Buffer,
        _p_regions: &[vk::BufferImageCopy],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_update_buffer(
        &self,
        _command_buffer: vk::CommandBuffer,
        _dst_buffer: vk::Buffer,
        _dst_offset: vk::DeviceSize,
        _p_data: &[u8],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_fill_buffer(
        &self,
        _command_buffer: vk::CommandBuffer,
        _dst_buffer: vk::Buffer,
        _dst_offset: vk::DeviceSize,
        _size: vk::DeviceSize,
        _data: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_clear_color_image(
        &self,
        _command_buffer: vk::CommandBuffer,
        _image: vk::Image,
        _image_layout: vk::ImageLayout,
        _p_color: &vk::ClearColorValue,
        _p_ranges: &[vk::ImageSubresourceRange],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_clear_depth_stencil_image(
        &self,
        _command_buffer: vk::CommandBuffer,
        _image: vk::Image,
        _image_layout: vk::ImageLayout,
        _p_depth_stencil: &vk::ClearDepthStencilValue,
        _p_ranges: &[vk::ImageSubresourceRange],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_clear_attachments(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_attachments: &[vk::ClearAttachment],
        _p_rects: &[vk::ClearRect],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_resolve_image(
        &self,
        _command_buffer: vk::CommandBuffer,
        _src_image: vk::Image,
        _src_image_layout: vk::ImageLayout,
        _dst_image: vk::Image,
        _dst_image_layout: vk::ImageLayout,
        _p_regions: &[vk::ImageResolve],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_event(
        &self,
        _command_buffer: vk::CommandBuffer,
        _event: vk::Event,
        _stage_mask: vk::PipelineStageFlags,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_reset_event(
        &self,
        _command_buffer: vk::CommandBuffer,
        _event: vk::Event,
        _stage_mask: vk::PipelineStageFlags,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_wait_events(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_events: &[vk::Event],
        _src_stage_mask: vk::PipelineStageFlags,
        _dst_stage_mask: vk::PipelineStageFlags,
        _p_memory_barriers: &[vk::MemoryBarrier],
        _p_buffer_memory_barriers: &[vk::BufferMemoryBarrier],
        _p_image_memory_barriers: &[vk::ImageMemoryBarrier],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_pipeline_barrier(
        &self,
        _command_buffer: vk::CommandBuffer,
        _src_stage_mask: vk::PipelineStageFlags,
        _dst_stage_mask: vk::PipelineStageFlags,
        _dependency_flags: vk::DependencyFlags,
        _p_memory_barriers: &[vk::MemoryBarrier],
        _p_buffer_memory_barriers: &[vk::BufferMemoryBarrier],
        _p_image_memory_barriers: &[vk::ImageMemoryBarrier],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_begin_query(
        &self,
        _command_buffer: vk::CommandBuffer,
        _query_pool: vk::QueryPool,
        _query: u32,
        _flags: vk::QueryControlFlags,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_end_query(
        &self,
        _command_buffer: vk::CommandBuffer,
        _query_pool: vk::QueryPool,
        _query: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_reset_query_pool(
        &self,
        _command_buffer: vk::CommandBuffer,
        _query_pool: vk::QueryPool,
        _first_query: u32,
        _query_count: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_write_timestamp(
        &self,
        _command_buffer: vk::CommandBuffer,
        _pipeline_stage: vk::PipelineStageFlags,
        _query_pool: vk::QueryPool,
        _query: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_query_pool_results(
        &self,
        _command_buffer: vk::CommandBuffer,
        _query_pool: vk::QueryPool,
        _first_query: u32,
        _query_count: u32,
        _dst_buffer: vk::Buffer,
        _dst_offset: vk::DeviceSize,
        _stride: vk::DeviceSize,
        _flags: vk::QueryResultFlags,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_push_constants(
        &self,
        _command_buffer: vk::CommandBuffer,
        _layout: vk::PipelineLayout,
        _stage_flags: vk::ShaderStageFlags,
        _offset: u32,
        _p_values: &[u8],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_begin_render_pass(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_render_pass_begin: &vk::RenderPassBeginInfo,
        _contents: vk::SubpassContents,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_next_subpass(
        &self,
        _command_buffer: vk::CommandBuffer,
        _contents: vk::SubpassContents,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_end_render_pass(&self, _command_buffer: vk::CommandBuffer) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_execute_commands(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_command_buffers: &[vk::CommandBuffer],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn bind_buffer_memory2(
        &self,
        _p_bind_infos: &[vk::BindBufferMemoryInfo],
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn bind_image_memory2(
        &self,
        _p_bind_infos: &[vk::BindImageMemoryInfo],
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_device_group_peer_memory_features(
        &self,
        _heap_index: u32,
        _local_device_index: u32,
        _remote_device_index: u32,
    ) -> LayerResult<vk::PeerMemoryFeatureFlags> {
        LayerResult::Unhandled
    }
    fn cmd_set_device_mask(
        &self,
        _command_buffer: vk::CommandBuffer,
        _device_mask: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_dispatch_base(
        &self,
        _command_buffer: vk::CommandBuffer,
        _base_groupx: u32,
        _base_groupy: u32,
        _base_groupz: u32,
        _group_countx: u32,
        _group_county: u32,
        _group_countz: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_image_memory_requirements2(
        &self,
        _p_info: &vk::ImageMemoryRequirementsInfo2,
    ) -> LayerResult<vk::MemoryRequirements2> {
        LayerResult::Unhandled
    }
    fn get_buffer_memory_requirements2(
        &self,
        _p_info: &vk::BufferMemoryRequirementsInfo2,
    ) -> LayerResult<vk::MemoryRequirements2> {
        LayerResult::Unhandled
    }
    fn get_image_sparse_memory_requirements2(
        &self,
        _p_info: &vk::ImageSparseMemoryRequirementsInfo2,
    ) -> LayerResult<Vec<vk::SparseImageMemoryRequirements2>> {
        LayerResult::Unhandled
    }
    fn trim_command_pool(
        &self,
        _command_pool: vk::CommandPool,
        _flags: vk::CommandPoolTrimFlags,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_device_queue2(&self, _p_queue_info: &vk::DeviceQueueInfo2) -> LayerResult<vk::Queue> {
        LayerResult::Unhandled
    }
    fn create_sampler_ycbcr_conversion(
        &self,
        _p_create_info: &vk::SamplerYcbcrConversionCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SamplerYcbcrConversion>> {
        LayerResult::Unhandled
    }
    fn destroy_sampler_ycbcr_conversion(
        &self,
        _ycbcr_conversion: vk::SamplerYcbcrConversion,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_descriptor_update_template(
        &self,
        _p_create_info: &vk::DescriptorUpdateTemplateCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::DescriptorUpdateTemplate>> {
        LayerResult::Unhandled
    }
    fn destroy_descriptor_update_template(
        &self,
        _descriptor_update_template: vk::DescriptorUpdateTemplate,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn update_descriptor_set_with_template(
        &self,
        _descriptor_set: vk::DescriptorSet,
        _descriptor_update_template: vk::DescriptorUpdateTemplate,
        _p_data: *const c_void,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_descriptor_set_layout_support(
        &self,
        _p_create_info: &vk::DescriptorSetLayoutCreateInfo,
    ) -> LayerResult<vk::DescriptorSetLayoutSupport> {
        LayerResult::Unhandled
    }
    fn cmd_draw_indirect_count(
        &self,
        _command_buffer: vk::CommandBuffer,
        _buffer: vk::Buffer,
        _offset: vk::DeviceSize,
        _count_buffer: vk::Buffer,
        _count_buffer_offset: vk::DeviceSize,
        _max_draw_count: u32,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_indexed_indirect_count(
        &self,
        _command_buffer: vk::CommandBuffer,
        _buffer: vk::Buffer,
        _offset: vk::DeviceSize,
        _count_buffer: vk::Buffer,
        _count_buffer_offset: vk::DeviceSize,
        _max_draw_count: u32,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_render_pass2(
        &self,
        _p_create_info: &vk::RenderPassCreateInfo2,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::RenderPass>> {
        LayerResult::Unhandled
    }
    fn cmd_begin_render_pass2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_render_pass_begin: &vk::RenderPassBeginInfo,
        _p_subpass_begin_info: &vk::SubpassBeginInfo,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_next_subpass2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_subpass_begin_info: &vk::SubpassBeginInfo,
        _p_subpass_end_info: &vk::SubpassEndInfo,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_end_render_pass2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_subpass_end_info: &vk::SubpassEndInfo,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn reset_query_pool(
        &self,
        _query_pool: vk::QueryPool,
        _first_query: u32,
        _query_count: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_semaphore_counter_value(&self, _semaphore: vk::Semaphore) -> LayerResult<VkResult<u64>> {
        LayerResult::Unhandled
    }
    fn wait_semaphores(
        &self,
        _p_wait_info: &vk::SemaphoreWaitInfo,
        _timeout: u64,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn signal_semaphore(
        &self,
        _p_signal_info: &vk::SemaphoreSignalInfo,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_buffer_device_address(
        &self,
        _p_info: &vk::BufferDeviceAddressInfo,
    ) -> LayerResult<vk::DeviceAddress> {
        LayerResult::Unhandled
    }
    fn get_buffer_opaque_capture_address(
        &self,
        _p_info: &vk::BufferDeviceAddressInfo,
    ) -> LayerResult<u64> {
        LayerResult::Unhandled
    }
    fn get_device_memory_opaque_capture_address(
        &self,
        _p_info: &vk::DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> LayerResult<u64> {
        LayerResult::Unhandled
    }
    fn create_private_data_slot(
        &self,
        _p_create_info: &vk::PrivateDataSlotCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::PrivateDataSlot>> {
        LayerResult::Unhandled
    }
    fn destroy_private_data_slot(
        &self,
        _private_data_slot: vk::PrivateDataSlot,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn set_private_data(
        &self,
        _object_type: vk::ObjectType,
        _object_handle: u64,
        _private_data_slot: vk::PrivateDataSlot,
        _data: u64,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_private_data(
        &self,
        _object_type: vk::ObjectType,
        _object_handle: u64,
        _private_data_slot: vk::PrivateDataSlot,
    ) -> LayerResult<u64> {
        LayerResult::Unhandled
    }
    fn cmd_set_event2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _event: vk::Event,
        _p_dependency_info: &vk::DependencyInfo,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_reset_event2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _event: vk::Event,
        _stage_mask: vk::PipelineStageFlags2,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_wait_events2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_events: &[vk::Event],
        _p_dependency_infos: &[vk::DependencyInfo],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_pipeline_barrier2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_dependency_info: &vk::DependencyInfo,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_write_timestamp2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _stage: vk::PipelineStageFlags2,
        _query_pool: vk::QueryPool,
        _query: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn queue_submit2(
        &self,
        _queue: vk::Queue,
        _p_submits: &[vk::SubmitInfo2],
        _fence: vk::Fence,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn cmd_copy_buffer2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_copy_buffer_info: &vk::CopyBufferInfo2,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_image2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_copy_image_info: &vk::CopyImageInfo2,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_buffer_to_image2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_copy_buffer_to_image_info: &vk::CopyBufferToImageInfo2,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_image_to_buffer2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_copy_image_to_buffer_info: &vk::CopyImageToBufferInfo2,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_blit_image2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_blit_image_info: &vk::BlitImageInfo2,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_resolve_image2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_resolve_image_info: &vk::ResolveImageInfo2,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_begin_rendering(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_rendering_info: &vk::RenderingInfo,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_end_rendering(&self, _command_buffer: vk::CommandBuffer) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_cull_mode(
        &self,
        _command_buffer: vk::CommandBuffer,
        _cull_mode: vk::CullModeFlags,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_front_face(
        &self,
        _command_buffer: vk::CommandBuffer,
        _front_face: vk::FrontFace,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_primitive_topology(
        &self,
        _command_buffer: vk::CommandBuffer,
        _primitive_topology: vk::PrimitiveTopology,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_viewport_with_count(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_viewports: &[vk::Viewport],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_scissor_with_count(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_scissors: &[vk::Rect2D],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_bind_vertex_buffers2(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_binding: u32,
        _p_buffers: &[vk::Buffer],
        _p_offsets: &[vk::DeviceSize],
        _p_sizes: Option<&[vk::DeviceSize]>,
        _p_strides: Option<&[vk::DeviceSize]>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_depth_test_enable(
        &self,
        _command_buffer: vk::CommandBuffer,
        _depth_test_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_depth_write_enable(
        &self,
        _command_buffer: vk::CommandBuffer,
        _depth_write_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_depth_compare_op(
        &self,
        _command_buffer: vk::CommandBuffer,
        _depth_compare_op: vk::CompareOp,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_depth_bounds_test_enable(
        &self,
        _command_buffer: vk::CommandBuffer,
        _depth_bounds_test_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_stencil_test_enable(
        &self,
        _command_buffer: vk::CommandBuffer,
        _stencil_test_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_stencil_op(
        &self,
        _command_buffer: vk::CommandBuffer,
        _face_mask: vk::StencilFaceFlags,
        _fail_op: vk::StencilOp,
        _pass_op: vk::StencilOp,
        _depth_fail_op: vk::StencilOp,
        _compare_op: vk::CompareOp,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_rasterizer_discard_enable(
        &self,
        _command_buffer: vk::CommandBuffer,
        _rasterizer_discard_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_depth_bias_enable(
        &self,
        _command_buffer: vk::CommandBuffer,
        _depth_bias_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_primitive_restart_enable(
        &self,
        _command_buffer: vk::CommandBuffer,
        _primitive_restart_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_device_buffer_memory_requirements(
        &self,
        _p_info: &vk::DeviceBufferMemoryRequirements,
    ) -> LayerResult<vk::MemoryRequirements2> {
        LayerResult::Unhandled
    }
    fn get_device_image_memory_requirements(
        &self,
        _p_info: &vk::DeviceImageMemoryRequirements,
    ) -> LayerResult<vk::MemoryRequirements2> {
        LayerResult::Unhandled
    }
    fn get_device_image_sparse_memory_requirements(
        &self,
        _p_info: &vk::DeviceImageMemoryRequirements,
    ) -> LayerResult<Vec<vk::SparseImageMemoryRequirements2>> {
        LayerResult::Unhandled
    }
    fn create_swapchain_khr(
        &self,
        _p_create_info: &vk::SwapchainCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SwapchainKHR>> {
        LayerResult::Unhandled
    }
    fn destroy_swapchain_khr(
        &self,
        _swapchain: vk::SwapchainKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_swapchain_images_khr(
        &self,
        _swapchain: vk::SwapchainKHR,
    ) -> LayerResult<VkResult<Vec<vk::Image>>> {
        LayerResult::Unhandled
    }
    fn acquire_next_image_khr(
        &self,
        _swapchain: vk::SwapchainKHR,
        _timeout: u64,
        _semaphore: vk::Semaphore,
        _fence: vk::Fence,
    ) -> LayerResult<VkResult<u32>> {
        LayerResult::Unhandled
    }
    fn queue_present_khr(
        &self,
        _queue: vk::Queue,
        _p_present_info: &vk::PresentInfoKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_device_group_present_capabilities_khr(
        &self,
    ) -> LayerResult<VkResult<vk::DeviceGroupPresentCapabilitiesKHR>> {
        LayerResult::Unhandled
    }
    fn get_device_group_surface_present_modes_khr(
        &self,
        _surface: vk::SurfaceKHR,
    ) -> LayerResult<VkResult<vk::DeviceGroupPresentModeFlagsKHR>> {
        LayerResult::Unhandled
    }
    fn acquire_next_image2_khr(
        &self,
        _p_acquire_info: &vk::AcquireNextImageInfoKHR,
    ) -> LayerResult<VkResult<u32>> {
        LayerResult::Unhandled
    }
    fn create_shared_swapchains_khr(
        &self,
        _p_create_infos: &[vk::SwapchainCreateInfoKHR],
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<Vec<vk::SwapchainKHR>>> {
        LayerResult::Unhandled
    }
    fn create_video_session_khr(
        &self,
        _p_create_info: &vk::VideoSessionCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::VideoSessionKHR>> {
        LayerResult::Unhandled
    }
    fn destroy_video_session_khr(
        &self,
        _video_session: vk::VideoSessionKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_video_session_memory_requirements_khr(
        &self,
        _video_session: vk::VideoSessionKHR,
    ) -> LayerResult<VkResult<Vec<vk::VideoSessionMemoryRequirementsKHR>>> {
        LayerResult::Unhandled
    }
    fn bind_video_session_memory_khr(
        &self,
        _video_session: vk::VideoSessionKHR,
        _p_bind_session_memory_infos: &[vk::BindVideoSessionMemoryInfoKHR],
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn create_video_session_parameters_khr(
        &self,
        _p_create_info: &vk::VideoSessionParametersCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::VideoSessionParametersKHR>> {
        LayerResult::Unhandled
    }
    fn update_video_session_parameters_khr(
        &self,
        _video_session_parameters: vk::VideoSessionParametersKHR,
        _p_update_info: &vk::VideoSessionParametersUpdateInfoKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn destroy_video_session_parameters_khr(
        &self,
        _video_session_parameters: vk::VideoSessionParametersKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_begin_video_coding_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_begin_info: &vk::VideoBeginCodingInfoKHR,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_end_video_coding_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_end_coding_info: &vk::VideoEndCodingInfoKHR,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_control_video_coding_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_coding_control_info: &vk::VideoCodingControlInfoKHR,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_decode_video_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_decode_info: &vk::VideoDecodeInfoKHR,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_memory_win32_handle_khr(
        &self,
        _p_get_win32_handle_info: &vk::MemoryGetWin32HandleInfoKHR,
    ) -> LayerResult<VkResult<vk::HANDLE>> {
        LayerResult::Unhandled
    }
    fn get_memory_win32_handle_properties_khr(
        &self,
        _handle_type: vk::ExternalMemoryHandleTypeFlags,
        _handle: vk::HANDLE,
    ) -> LayerResult<VkResult<vk::MemoryWin32HandlePropertiesKHR>> {
        LayerResult::Unhandled
    }
    fn get_memory_fd_khr(
        &self,
        _p_get_fd_info: &vk::MemoryGetFdInfoKHR,
    ) -> LayerResult<VkResult<c_int>> {
        LayerResult::Unhandled
    }
    fn get_memory_fd_properties_khr(
        &self,
        _handle_type: vk::ExternalMemoryHandleTypeFlags,
        _fd: c_int,
    ) -> LayerResult<VkResult<vk::MemoryFdPropertiesKHR>> {
        LayerResult::Unhandled
    }
    fn import_semaphore_win32_handle_khr(
        &self,
        _p_import_semaphore_win32_handle_info: &vk::ImportSemaphoreWin32HandleInfoKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_semaphore_win32_handle_khr(
        &self,
        _p_get_win32_handle_info: &vk::SemaphoreGetWin32HandleInfoKHR,
    ) -> LayerResult<VkResult<vk::HANDLE>> {
        LayerResult::Unhandled
    }
    fn import_semaphore_fd_khr(
        &self,
        _p_import_semaphore_fd_info: &vk::ImportSemaphoreFdInfoKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_semaphore_fd_khr(
        &self,
        _p_get_fd_info: &vk::SemaphoreGetFdInfoKHR,
    ) -> LayerResult<VkResult<c_int>> {
        LayerResult::Unhandled
    }
    fn cmd_push_descriptor_set_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _pipeline_bind_point: vk::PipelineBindPoint,
        _layout: vk::PipelineLayout,
        _set: u32,
        _p_descriptor_writes: &[vk::WriteDescriptorSet],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_push_descriptor_set_with_template_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _descriptor_update_template: vk::DescriptorUpdateTemplate,
        _layout: vk::PipelineLayout,
        _set: u32,
        _p_data: *const c_void,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_swapchain_status_khr(&self, _swapchain: vk::SwapchainKHR) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn import_fence_win32_handle_khr(
        &self,
        _p_import_fence_win32_handle_info: &vk::ImportFenceWin32HandleInfoKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_fence_win32_handle_khr(
        &self,
        _p_get_win32_handle_info: &vk::FenceGetWin32HandleInfoKHR,
    ) -> LayerResult<VkResult<vk::HANDLE>> {
        LayerResult::Unhandled
    }
    fn import_fence_fd_khr(
        &self,
        _p_import_fence_fd_info: &vk::ImportFenceFdInfoKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_fence_fd_khr(
        &self,
        _p_get_fd_info: &vk::FenceGetFdInfoKHR,
    ) -> LayerResult<VkResult<c_int>> {
        LayerResult::Unhandled
    }
    fn acquire_profiling_lock_khr(
        &self,
        _p_info: &vk::AcquireProfilingLockInfoKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn release_profiling_lock_khr(&self) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_fragment_shading_rate_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_fragment_size: &vk::Extent2D,
        _combiner_ops: &[vk::FragmentShadingRateCombinerOpKHR; 2],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn wait_for_present_khr(
        &self,
        _swapchain: vk::SwapchainKHR,
        _present_id: u64,
        _timeout: u64,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn create_deferred_operation_khr(
        &self,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::DeferredOperationKHR>> {
        LayerResult::Unhandled
    }
    fn destroy_deferred_operation_khr(
        &self,
        _operation: vk::DeferredOperationKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_deferred_operation_max_concurrency_khr(
        &self,
        _operation: vk::DeferredOperationKHR,
    ) -> LayerResult<u32> {
        LayerResult::Unhandled
    }
    fn get_deferred_operation_result_khr(
        &self,
        _operation: vk::DeferredOperationKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn deferred_operation_join_khr(
        &self,
        _operation: vk::DeferredOperationKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_pipeline_executable_properties_khr(
        &self,
        _p_pipeline_info: &vk::PipelineInfoKHR,
    ) -> LayerResult<VkResult<Vec<vk::PipelineExecutablePropertiesKHR>>> {
        LayerResult::Unhandled
    }
    fn get_pipeline_executable_statistics_khr(
        &self,
        _p_executable_info: &vk::PipelineExecutableInfoKHR,
    ) -> LayerResult<VkResult<Vec<vk::PipelineExecutableStatisticKHR>>> {
        LayerResult::Unhandled
    }
    fn get_pipeline_executable_internal_representations_khr(
        &self,
        _p_executable_info: &vk::PipelineExecutableInfoKHR,
    ) -> LayerResult<VkResult<Vec<vk::PipelineExecutableInternalRepresentationKHR>>> {
        LayerResult::Unhandled
    }
    fn cmd_encode_video_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_encode_info: &vk::VideoEncodeInfoKHR,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_write_buffer_marker2_amd(
        &self,
        _command_buffer: vk::CommandBuffer,
        _stage: vk::PipelineStageFlags2,
        _dst_buffer: vk::Buffer,
        _dst_offset: vk::DeviceSize,
        _marker: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_queue_checkpoint_data2_nv(
        &self,
        _queue: vk::Queue,
    ) -> LayerResult<Vec<vk::CheckpointData2NV>> {
        LayerResult::Unhandled
    }
    fn cmd_trace_rays_indirect2_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _indirect_device_address: vk::DeviceAddress,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn debug_marker_set_object_tag_ext(
        &self,
        _p_tag_info: &vk::DebugMarkerObjectTagInfoEXT,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn debug_marker_set_object_name_ext(
        &self,
        _p_name_info: &vk::DebugMarkerObjectNameInfoEXT,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn cmd_debug_marker_begin_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_marker_info: &vk::DebugMarkerMarkerInfoEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_debug_marker_end_ext(&self, _command_buffer: vk::CommandBuffer) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_debug_marker_insert_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_marker_info: &vk::DebugMarkerMarkerInfoEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_bind_transform_feedback_buffers_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_binding: u32,
        _p_buffers: &[vk::Buffer],
        _p_offsets: &[vk::DeviceSize],
        _p_sizes: Option<&[vk::DeviceSize]>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_begin_transform_feedback_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_counter_buffer: u32,
        _p_counter_buffers: &[vk::Buffer],
        _p_counter_buffer_offsets: Option<&[vk::DeviceSize]>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_end_transform_feedback_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_counter_buffer: u32,
        _p_counter_buffers: &[vk::Buffer],
        _p_counter_buffer_offsets: Option<&[vk::DeviceSize]>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_begin_query_indexed_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _query_pool: vk::QueryPool,
        _query: u32,
        _flags: vk::QueryControlFlags,
        _index: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_end_query_indexed_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _query_pool: vk::QueryPool,
        _query: u32,
        _index: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_indirect_byte_count_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _instance_count: u32,
        _first_instance: u32,
        _counter_buffer: vk::Buffer,
        _counter_buffer_offset: vk::DeviceSize,
        _counter_offset: u32,
        _vertex_stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_cu_module_nvx(
        &self,
        _p_create_info: &vk::CuModuleCreateInfoNVX,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::CuModuleNVX>> {
        LayerResult::Unhandled
    }
    fn create_cu_function_nvx(
        &self,
        _p_create_info: &vk::CuFunctionCreateInfoNVX,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::CuFunctionNVX>> {
        LayerResult::Unhandled
    }
    fn destroy_cu_module_nvx(
        &self,
        _module: vk::CuModuleNVX,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn destroy_cu_function_nvx(
        &self,
        _function: vk::CuFunctionNVX,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_cu_launch_kernel_nvx(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_launch_info: &vk::CuLaunchInfoNVX,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_image_view_handle_nvx(&self, _p_info: &vk::ImageViewHandleInfoNVX) -> LayerResult<u32> {
        LayerResult::Unhandled
    }
    fn get_image_view_address_nvx(
        &self,
        _image_view: vk::ImageView,
    ) -> LayerResult<VkResult<vk::ImageViewAddressPropertiesNVX>> {
        LayerResult::Unhandled
    }
    fn get_shader_info_amd(
        &self,
        _pipeline: vk::Pipeline,
        _shader_stage: vk::ShaderStageFlags,
        _info_type: vk::ShaderInfoTypeAMD,
    ) -> LayerResult<VkResult<Vec<u8>>> {
        LayerResult::Unhandled
    }
    fn get_memory_win32_handle_nv(
        &self,
        _memory: vk::DeviceMemory,
        _handle_type: vk::ExternalMemoryHandleTypeFlagsNV,
    ) -> LayerResult<VkResult<vk::HANDLE>> {
        LayerResult::Unhandled
    }
    fn cmd_begin_conditional_rendering_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_conditional_rendering_begin: &vk::ConditionalRenderingBeginInfoEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_end_conditional_rendering_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_viewport_w_scaling_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_viewport: u32,
        _p_viewport_w_scalings: &[vk::ViewportWScalingNV],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn display_power_control_ext(
        &self,
        _display: vk::DisplayKHR,
        _p_display_power_info: &vk::DisplayPowerInfoEXT,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn register_device_event_ext(
        &self,
        _p_device_event_info: &vk::DeviceEventInfoEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Fence>> {
        LayerResult::Unhandled
    }
    fn register_display_event_ext(
        &self,
        _display: vk::DisplayKHR,
        _p_display_event_info: &vk::DisplayEventInfoEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Fence>> {
        LayerResult::Unhandled
    }
    fn get_swapchain_counter_ext(
        &self,
        _swapchain: vk::SwapchainKHR,
        _counter: vk::SurfaceCounterFlagsEXT,
    ) -> LayerResult<VkResult<u64>> {
        LayerResult::Unhandled
    }
    fn get_refresh_cycle_duration_google(
        &self,
        _swapchain: vk::SwapchainKHR,
    ) -> LayerResult<VkResult<vk::RefreshCycleDurationGOOGLE>> {
        LayerResult::Unhandled
    }
    fn get_past_presentation_timing_google(
        &self,
        _swapchain: vk::SwapchainKHR,
    ) -> LayerResult<VkResult<Vec<vk::PastPresentationTimingGOOGLE>>> {
        LayerResult::Unhandled
    }
    fn cmd_set_discard_rectangle_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_discard_rectangle: u32,
        _p_discard_rectangles: &[vk::Rect2D],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn set_hdr_metadata_ext(
        &self,
        _p_swapchains: &[vk::SwapchainKHR],
        _p_metadata: &[vk::HdrMetadataEXT],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn set_debug_utils_object_name_ext(
        &self,
        _p_name_info: &vk::DebugUtilsObjectNameInfoEXT,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn set_debug_utils_object_tag_ext(
        &self,
        _p_tag_info: &vk::DebugUtilsObjectTagInfoEXT,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn queue_begin_debug_utils_label_ext(
        &self,
        _queue: vk::Queue,
        _p_label_info: &vk::DebugUtilsLabelEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn queue_end_debug_utils_label_ext(&self, _queue: vk::Queue) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn queue_insert_debug_utils_label_ext(
        &self,
        _queue: vk::Queue,
        _p_label_info: &vk::DebugUtilsLabelEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_begin_debug_utils_label_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_label_info: &vk::DebugUtilsLabelEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_end_debug_utils_label_ext(&self, _command_buffer: vk::CommandBuffer) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_insert_debug_utils_label_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_label_info: &vk::DebugUtilsLabelEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_android_hardware_buffer_properties_android(
        &self,
        _buffer: *const vk::AHardwareBuffer,
    ) -> LayerResult<VkResult<vk::AndroidHardwareBufferPropertiesANDROID>> {
        LayerResult::Unhandled
    }
    fn get_memory_android_hardware_buffer_android(
        &self,
        _p_info: &vk::MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> LayerResult<VkResult<*mut vk::AHardwareBuffer>> {
        LayerResult::Unhandled
    }
    fn cmd_set_sample_locations_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_sample_locations_info: &vk::SampleLocationsInfoEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_image_drm_format_modifier_properties_ext(
        &self,
        _image: vk::Image,
    ) -> LayerResult<VkResult<vk::ImageDrmFormatModifierPropertiesEXT>> {
        LayerResult::Unhandled
    }
    fn create_validation_cache_ext(
        &self,
        _p_create_info: &vk::ValidationCacheCreateInfoEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::ValidationCacheEXT>> {
        LayerResult::Unhandled
    }
    fn destroy_validation_cache_ext(
        &self,
        _validation_cache: vk::ValidationCacheEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn merge_validation_caches_ext(
        &self,
        _dst_cache: vk::ValidationCacheEXT,
        _p_src_caches: &[vk::ValidationCacheEXT],
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_validation_cache_data_ext(
        &self,
        _validation_cache: vk::ValidationCacheEXT,
    ) -> LayerResult<VkResult<Vec<u8>>> {
        LayerResult::Unhandled
    }
    fn cmd_bind_shading_rate_image_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _image_view: vk::ImageView,
        _image_layout: vk::ImageLayout,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_viewport: u32,
        _p_shading_rate_palettes: &[vk::ShadingRatePaletteNV],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_coarse_sample_order_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _sample_order_type: vk::CoarseSampleOrderTypeNV,
        _p_custom_sample_orders: &[vk::CoarseSampleOrderCustomNV],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_acceleration_structure_nv(
        &self,
        _p_create_info: &vk::AccelerationStructureCreateInfoNV,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::AccelerationStructureNV>> {
        LayerResult::Unhandled
    }
    fn destroy_acceleration_structure_nv(
        &self,
        _acceleration_structure: vk::AccelerationStructureNV,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_acceleration_structure_memory_requirements_nv(
        &self,
        _p_info: &vk::AccelerationStructureMemoryRequirementsInfoNV,
    ) -> LayerResult<vk::MemoryRequirements2KHR> {
        LayerResult::Unhandled
    }
    fn bind_acceleration_structure_memory_nv(
        &self,
        _p_bind_infos: &[vk::BindAccelerationStructureMemoryInfoNV],
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn cmd_build_acceleration_structure_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_info: &vk::AccelerationStructureInfoNV,
        _instance_data: vk::Buffer,
        _instance_offset: vk::DeviceSize,
        _update: bool,
        _dst: vk::AccelerationStructureNV,
        _src: vk::AccelerationStructureNV,
        _scratch: vk::Buffer,
        _scratch_offset: vk::DeviceSize,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_acceleration_structure_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _dst: vk::AccelerationStructureNV,
        _src: vk::AccelerationStructureNV,
        _mode: vk::CopyAccelerationStructureModeKHR,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_trace_rays_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _raygen_shader_binding_table_buffer: vk::Buffer,
        _raygen_shader_binding_offset: vk::DeviceSize,
        _miss_shader_binding_table_buffer: vk::Buffer,
        _miss_shader_binding_offset: vk::DeviceSize,
        _miss_shader_binding_stride: vk::DeviceSize,
        _hit_shader_binding_table_buffer: vk::Buffer,
        _hit_shader_binding_offset: vk::DeviceSize,
        _hit_shader_binding_stride: vk::DeviceSize,
        _callable_shader_binding_table_buffer: vk::Buffer,
        _callable_shader_binding_offset: vk::DeviceSize,
        _callable_shader_binding_stride: vk::DeviceSize,
        _width: u32,
        _height: u32,
        _depth: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_ray_tracing_pipelines_nv(
        &self,
        _pipeline_cache: vk::PipelineCache,
        _p_create_infos: &[vk::RayTracingPipelineCreateInfoNV],
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<Vec<vk::Pipeline>>> {
        LayerResult::Unhandled
    }
    fn get_ray_tracing_shader_group_handles_khr(
        &self,
        _pipeline: vk::Pipeline,
        _first_group: u32,
        _group_count: u32,
    ) -> LayerResult<VkResult<Vec<u8>>> {
        LayerResult::Unhandled
    }
    fn get_acceleration_structure_handle_nv(
        &self,
        _acceleration_structure: vk::AccelerationStructureNV,
    ) -> LayerResult<VkResult<Vec<u8>>> {
        LayerResult::Unhandled
    }
    fn cmd_write_acceleration_structures_properties_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_acceleration_structures: &[vk::AccelerationStructureNV],
        _query_type: vk::QueryType,
        _query_pool: vk::QueryPool,
        _first_query: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn compile_deferred_nv(
        &self,
        _pipeline: vk::Pipeline,
        _shader: u32,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_memory_host_pointer_properties_ext(
        &self,
        _handle_type: vk::ExternalMemoryHandleTypeFlags,
        _p_host_pointer: *const c_void,
    ) -> LayerResult<VkResult<vk::MemoryHostPointerPropertiesEXT>> {
        LayerResult::Unhandled
    }
    fn cmd_write_buffer_marker_amd(
        &self,
        _command_buffer: vk::CommandBuffer,
        _pipeline_stage: vk::PipelineStageFlags,
        _dst_buffer: vk::Buffer,
        _dst_offset: vk::DeviceSize,
        _marker: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_calibrated_timestamps_ext(
        &self,
        _p_timestamp_infos: &[vk::CalibratedTimestampInfoEXT],
        _p_timestamps: &mut [u64],
    ) -> LayerResult<VkResult<u64>> {
        LayerResult::Unhandled
    }
    fn cmd_draw_mesh_tasks_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _task_count: u32,
        _first_task: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _buffer: vk::Buffer,
        _offset: vk::DeviceSize,
        _draw_count: u32,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_mesh_tasks_indirect_count_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _buffer: vk::Buffer,
        _offset: vk::DeviceSize,
        _count_buffer: vk::Buffer,
        _count_buffer_offset: vk::DeviceSize,
        _max_draw_count: u32,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_exclusive_scissor_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_exclusive_scissor: u32,
        _p_exclusive_scissors: &[vk::Rect2D],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_checkpoint_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_checkpoint_marker: *const c_void,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_queue_checkpoint_data_nv(
        &self,
        _queue: vk::Queue,
    ) -> LayerResult<Vec<vk::CheckpointDataNV>> {
        LayerResult::Unhandled
    }
    fn initialize_performance_api_intel(
        &self,
        _p_initialize_info: &vk::InitializePerformanceApiInfoINTEL,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn uninitialize_performance_api_intel(&self) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_performance_marker_intel(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_marker_info: &vk::PerformanceMarkerInfoINTEL,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn cmd_set_performance_stream_marker_intel(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_marker_info: &vk::PerformanceStreamMarkerInfoINTEL,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn cmd_set_performance_override_intel(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_override_info: &vk::PerformanceOverrideInfoINTEL,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn acquire_performance_configuration_intel(
        &self,
        _p_acquire_info: &vk::PerformanceConfigurationAcquireInfoINTEL,
    ) -> LayerResult<VkResult<vk::PerformanceConfigurationINTEL>> {
        LayerResult::Unhandled
    }
    fn release_performance_configuration_intel(
        &self,
        _configuration: vk::PerformanceConfigurationINTEL,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn queue_set_performance_configuration_intel(
        &self,
        _queue: vk::Queue,
        _configuration: vk::PerformanceConfigurationINTEL,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_performance_parameter_intel(
        &self,
        _parameter: vk::PerformanceParameterTypeINTEL,
    ) -> LayerResult<VkResult<vk::PerformanceValueINTEL>> {
        LayerResult::Unhandled
    }
    fn set_local_dimming_amd(
        &self,
        _swap_chain: vk::SwapchainKHR,
        _local_dimming_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn acquire_full_screen_exclusive_mode_ext(
        &self,
        _swapchain: vk::SwapchainKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn release_full_screen_exclusive_mode_ext(
        &self,
        _swapchain: vk::SwapchainKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_device_group_surface_present_modes2_ext(
        &self,
        _p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> LayerResult<VkResult<vk::DeviceGroupPresentModeFlagsKHR>> {
        LayerResult::Unhandled
    }
    fn cmd_set_line_stipple_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _line_stipple_factor: u32,
        _line_stipple_pattern: u16,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn release_swapchain_images_ext(
        &self,
        _p_release_info: &vk::ReleaseSwapchainImagesInfoEXT,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_generated_commands_memory_requirements_nv(
        &self,
        _p_info: &vk::GeneratedCommandsMemoryRequirementsInfoNV,
    ) -> LayerResult<vk::MemoryRequirements2> {
        LayerResult::Unhandled
    }
    fn cmd_preprocess_generated_commands_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_generated_commands_info: &vk::GeneratedCommandsInfoNV,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_execute_generated_commands_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _is_preprocessed: bool,
        _p_generated_commands_info: &vk::GeneratedCommandsInfoNV,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_bind_pipeline_shader_group_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _pipeline_bind_point: vk::PipelineBindPoint,
        _pipeline: vk::Pipeline,
        _group_index: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_indirect_commands_layout_nv(
        &self,
        _p_create_info: &vk::IndirectCommandsLayoutCreateInfoNV,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::IndirectCommandsLayoutNV>> {
        LayerResult::Unhandled
    }
    fn destroy_indirect_commands_layout_nv(
        &self,
        _indirect_commands_layout: vk::IndirectCommandsLayoutNV,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn export_metal_objects_ext(&self) -> LayerResult<vk::ExportMetalObjectsInfoEXT> {
        LayerResult::Unhandled
    }
    fn get_descriptor_set_layout_size_ext(
        &self,
        _layout: vk::DescriptorSetLayout,
    ) -> LayerResult<vk::DeviceSize> {
        LayerResult::Unhandled
    }
    fn get_descriptor_set_layout_binding_offset_ext(
        &self,
        _layout: vk::DescriptorSetLayout,
        _binding: u32,
    ) -> LayerResult<vk::DeviceSize> {
        LayerResult::Unhandled
    }
    fn get_descriptor_ext(
        &self,
        _p_descriptor_info: &vk::DescriptorGetInfoEXT,
    ) -> LayerResult<Vec<u8>> {
        LayerResult::Unhandled
    }
    fn cmd_bind_descriptor_buffers_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_binding_infos: &[vk::DescriptorBufferBindingInfoEXT],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_descriptor_buffer_offsets_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _pipeline_bind_point: vk::PipelineBindPoint,
        _layout: vk::PipelineLayout,
        _first_set: u32,
        _p_buffer_indices: &[u32],
        _p_offsets: &[vk::DeviceSize],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _pipeline_bind_point: vk::PipelineBindPoint,
        _layout: vk::PipelineLayout,
        _set: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_buffer_opaque_capture_descriptor_data_ext(
        &self,
        _p_info: &vk::BufferCaptureDescriptorDataInfoEXT,
        _p_data: *mut c_void,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_image_opaque_capture_descriptor_data_ext(
        &self,
        _p_info: &vk::ImageCaptureDescriptorDataInfoEXT,
        _p_data: *mut c_void,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_image_view_opaque_capture_descriptor_data_ext(
        &self,
        _p_info: &vk::ImageViewCaptureDescriptorDataInfoEXT,
        _p_data: *mut c_void,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_sampler_opaque_capture_descriptor_data_ext(
        &self,
        _p_info: &vk::SamplerCaptureDescriptorDataInfoEXT,
        _p_data: *mut c_void,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
        &self,
        _p_info: &vk::AccelerationStructureCaptureDescriptorDataInfoEXT,
        _p_data: *mut c_void,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn cmd_set_fragment_shading_rate_enum_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _shading_rate: vk::FragmentShadingRateNV,
        _combiner_ops: &[vk::FragmentShadingRateCombinerOpKHR; 2],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_image_subresource_layout2_ext(
        &self,
        _image: vk::Image,
        _p_subresource: &vk::ImageSubresource2EXT,
    ) -> LayerResult<vk::SubresourceLayout2EXT> {
        LayerResult::Unhandled
    }
    fn cmd_set_vertex_input_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_vertex_binding_descriptions: &[vk::VertexInputBindingDescription2EXT],
        _p_vertex_attribute_descriptions: &[vk::VertexInputAttributeDescription2EXT],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_memory_zircon_handle_fuchsia(
        &self,
        _p_get_zircon_handle_info: &vk::MemoryGetZirconHandleInfoFUCHSIA,
    ) -> LayerResult<VkResult<vk::zx_handle_t>> {
        LayerResult::Unhandled
    }
    fn get_memory_zircon_handle_properties_fuchsia(
        &self,
        _handle_type: vk::ExternalMemoryHandleTypeFlags,
        _zircon_handle: vk::zx_handle_t,
    ) -> LayerResult<VkResult<vk::MemoryZirconHandlePropertiesFUCHSIA>> {
        LayerResult::Unhandled
    }
    fn import_semaphore_zircon_handle_fuchsia(
        &self,
        _p_import_semaphore_zircon_handle_info: &vk::ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_semaphore_zircon_handle_fuchsia(
        &self,
        _p_get_zircon_handle_info: &vk::SemaphoreGetZirconHandleInfoFUCHSIA,
    ) -> LayerResult<VkResult<vk::zx_handle_t>> {
        LayerResult::Unhandled
    }
    fn create_buffer_collection_fuchsia(
        &self,
        _p_create_info: &vk::BufferCollectionCreateInfoFUCHSIA,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::BufferCollectionFUCHSIA>> {
        LayerResult::Unhandled
    }
    fn set_buffer_collection_image_constraints_fuchsia(
        &self,
        _collection: vk::BufferCollectionFUCHSIA,
        _p_image_constraints_info: &vk::ImageConstraintsInfoFUCHSIA,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn set_buffer_collection_buffer_constraints_fuchsia(
        &self,
        _collection: vk::BufferCollectionFUCHSIA,
        _p_buffer_constraints_info: &vk::BufferConstraintsInfoFUCHSIA,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn destroy_buffer_collection_fuchsia(
        &self,
        _collection: vk::BufferCollectionFUCHSIA,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_buffer_collection_properties_fuchsia(
        &self,
        _collection: vk::BufferCollectionFUCHSIA,
    ) -> LayerResult<VkResult<vk::BufferCollectionPropertiesFUCHSIA>> {
        LayerResult::Unhandled
    }
    fn get_device_subpass_shading_max_workgroup_size_huawei(
        &self,
        _renderpass: vk::RenderPass,
    ) -> LayerResult<VkResult<vk::Extent2D>> {
        LayerResult::Unhandled
    }
    fn cmd_subpass_shading_huawei(&self, _command_buffer: vk::CommandBuffer) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_bind_invocation_mask_huawei(
        &self,
        _command_buffer: vk::CommandBuffer,
        _image_view: vk::ImageView,
        _image_layout: vk::ImageLayout,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_memory_remote_address_nv(
        &self,
        _p_memory_get_remote_address_info: &vk::MemoryGetRemoteAddressInfoNV,
    ) -> LayerResult<VkResult<vk::RemoteAddressNV>> {
        LayerResult::Unhandled
    }
    fn get_pipeline_properties_ext(
        &self,
        _p_pipeline_info: &vk::PipelineInfoEXT,
    ) -> LayerResult<VkResult<vk::BaseOutStructure>> {
        LayerResult::Unhandled
    }
    fn cmd_set_patch_control_points_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _patch_control_points: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_logic_op_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _logic_op: vk::LogicOp,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_color_write_enable_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_color_write_enables: impl Iterator<Item = bool>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_multi_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_vertex_info: &[vk::MultiDrawInfoEXT],
        _instance_count: u32,
        _first_instance: u32,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_multi_indexed_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_index_info: &[vk::MultiDrawIndexedInfoEXT],
        _instance_count: u32,
        _first_instance: u32,
        _stride: u32,
        _p_vertex_offset: Option<&i32>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_micromap_ext(
        &self,
        _p_create_info: &vk::MicromapCreateInfoEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::MicromapEXT>> {
        LayerResult::Unhandled
    }
    fn destroy_micromap_ext(
        &self,
        _micromap: vk::MicromapEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_build_micromaps_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_infos: &[vk::MicromapBuildInfoEXT],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn build_micromaps_ext(
        &self,
        _deferred_operation: vk::DeferredOperationKHR,
        _p_infos: &[vk::MicromapBuildInfoEXT],
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn copy_micromap_ext(
        &self,
        _deferred_operation: vk::DeferredOperationKHR,
        _p_info: &vk::CopyMicromapInfoEXT,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn copy_micromap_to_memory_ext(
        &self,
        _deferred_operation: vk::DeferredOperationKHR,
        _p_info: &vk::CopyMicromapToMemoryInfoEXT,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn copy_memory_to_micromap_ext(
        &self,
        _deferred_operation: vk::DeferredOperationKHR,
        _p_info: &vk::CopyMemoryToMicromapInfoEXT,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn write_micromaps_properties_ext(
        &self,
        _p_micromaps: &[vk::MicromapEXT],
        _query_type: vk::QueryType,
        _p_data: &mut [u8],
        _stride: usize,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn cmd_copy_micromap_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_info: &vk::CopyMicromapInfoEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_micromap_to_memory_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_info: &vk::CopyMicromapToMemoryInfoEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_memory_to_micromap_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_info: &vk::CopyMemoryToMicromapInfoEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_write_micromaps_properties_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_micromaps: &[vk::MicromapEXT],
        _query_type: vk::QueryType,
        _query_pool: vk::QueryPool,
        _first_query: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_device_micromap_compatibility_ext(
        &self,
        _p_version_info: &vk::MicromapVersionInfoEXT,
    ) -> LayerResult<vk::AccelerationStructureCompatibilityKHR> {
        LayerResult::Unhandled
    }
    fn get_micromap_build_sizes_ext(
        &self,
        _build_type: vk::AccelerationStructureBuildTypeKHR,
        _p_build_info: &vk::MicromapBuildInfoEXT,
    ) -> LayerResult<vk::MicromapBuildSizesInfoEXT> {
        LayerResult::Unhandled
    }
    fn set_device_memory_priority_ext(
        &self,
        _memory: vk::DeviceMemory,
        _priority: f32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_descriptor_set_layout_host_mapping_info_valve(
        &self,
        _p_binding_reference: &vk::DescriptorSetBindingReferenceVALVE,
    ) -> LayerResult<vk::DescriptorSetLayoutHostMappingInfoVALVE> {
        LayerResult::Unhandled
    }
    fn get_descriptor_set_host_mapping_valve(
        &self,
        _descriptor_set: vk::DescriptorSet,
    ) -> LayerResult<*mut c_void> {
        LayerResult::Unhandled
    }
    fn cmd_copy_memory_indirect_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _copy_buffer_address: vk::DeviceAddress,
        _copy_count: u32,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_memory_to_image_indirect_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _copy_buffer_address: vk::DeviceAddress,
        _stride: u32,
        _dst_image: vk::Image,
        _dst_image_layout: vk::ImageLayout,
        _p_image_subresources: &[vk::ImageSubresourceLayers],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_decompress_memory_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_decompress_memory_regions: &[vk::DecompressMemoryRegionNV],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_decompress_memory_indirect_count_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _indirect_commands_address: vk::DeviceAddress,
        _indirect_commands_count_address: vk::DeviceAddress,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_tessellation_domain_origin_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _domain_origin: vk::TessellationDomainOrigin,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_depth_clamp_enable_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _depth_clamp_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_polygon_mode_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _polygon_mode: vk::PolygonMode,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_rasterization_samples_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _rasterization_samples: vk::SampleCountFlags,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_sample_mask_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _samples: vk::SampleCountFlags,
        _p_sample_mask: &[vk::SampleMask],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_alpha_to_coverage_enable_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _alpha_to_coverage_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_alpha_to_one_enable_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _alpha_to_one_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_logic_op_enable_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _logic_op_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_color_blend_enable_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_attachment: u32,
        _p_color_blend_enables: impl Iterator<Item = bool>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_color_blend_equation_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_attachment: u32,
        _p_color_blend_equations: &[vk::ColorBlendEquationEXT],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_color_write_mask_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_attachment: u32,
        _p_color_write_masks: &[vk::ColorComponentFlags],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_rasterization_stream_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _rasterization_stream: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_conservative_rasterization_mode_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _conservative_rasterization_mode: vk::ConservativeRasterizationModeEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_extra_primitive_overestimation_size_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _extra_primitive_overestimation_size: f32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_depth_clip_enable_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _depth_clip_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_sample_locations_enable_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _sample_locations_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_color_blend_advanced_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_attachment: u32,
        _p_color_blend_advanced: &[vk::ColorBlendAdvancedEXT],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_provoking_vertex_mode_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _provoking_vertex_mode: vk::ProvokingVertexModeEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_line_rasterization_mode_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _line_rasterization_mode: vk::LineRasterizationModeEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_line_stipple_enable_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _stippled_line_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_depth_clip_negative_one_to_one_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _negative_one_to_one: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_viewport_w_scaling_enable_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _viewport_w_scaling_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_viewport_swizzle_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _first_viewport: u32,
        _p_viewport_swizzles: &[vk::ViewportSwizzleNV],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_coverage_to_color_enable_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _coverage_to_color_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_coverage_to_color_location_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _coverage_to_color_location: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_coverage_modulation_mode_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _coverage_modulation_mode: vk::CoverageModulationModeNV,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_coverage_modulation_table_enable_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _coverage_modulation_table_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_coverage_modulation_table_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_coverage_modulation_table: &[f32],
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_shading_rate_image_enable_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _shading_rate_image_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_representative_fragment_test_enable_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _representative_fragment_test_enable: bool,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_set_coverage_reduction_mode_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _coverage_reduction_mode: vk::CoverageReductionModeNV,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_shader_module_identifier_ext(
        &self,
        _shader_module: vk::ShaderModule,
    ) -> LayerResult<vk::ShaderModuleIdentifierEXT> {
        LayerResult::Unhandled
    }
    fn get_shader_module_create_info_identifier_ext(
        &self,
        _p_create_info: &vk::ShaderModuleCreateInfo,
    ) -> LayerResult<vk::ShaderModuleIdentifierEXT> {
        LayerResult::Unhandled
    }
    fn create_optical_flow_session_nv(
        &self,
        _p_create_info: &vk::OpticalFlowSessionCreateInfoNV,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::OpticalFlowSessionNV>> {
        LayerResult::Unhandled
    }
    fn destroy_optical_flow_session_nv(
        &self,
        _session: vk::OpticalFlowSessionNV,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn bind_optical_flow_session_image_nv(
        &self,
        _session: vk::OpticalFlowSessionNV,
        _binding_point: vk::OpticalFlowSessionBindingPointNV,
        _view: vk::ImageView,
        _layout: vk::ImageLayout,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn cmd_optical_flow_execute_nv(
        &self,
        _command_buffer: vk::CommandBuffer,
        _session: vk::OpticalFlowSessionNV,
        _p_execute_info: &vk::OpticalFlowExecuteInfoNV,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_framebuffer_tile_properties_qcom(
        &self,
        _framebuffer: vk::Framebuffer,
    ) -> LayerResult<VkResult<Vec<vk::TilePropertiesQCOM>>> {
        LayerResult::Unhandled
    }
    fn get_dynamic_rendering_tile_properties_qcom(
        &self,
        _p_rendering_info: &vk::RenderingInfo,
    ) -> LayerResult<VkResult<vk::TilePropertiesQCOM>> {
        LayerResult::Unhandled
    }
    fn create_acceleration_structure_khr(
        &self,
        _p_create_info: &vk::AccelerationStructureCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::AccelerationStructureKHR>> {
        LayerResult::Unhandled
    }
    fn destroy_acceleration_structure_khr(
        &self,
        _acceleration_structure: vk::AccelerationStructureKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn copy_acceleration_structure_khr(
        &self,
        _deferred_operation: vk::DeferredOperationKHR,
        _p_info: &vk::CopyAccelerationStructureInfoKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn copy_acceleration_structure_to_memory_khr(
        &self,
        _deferred_operation: vk::DeferredOperationKHR,
        _p_info: &vk::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn copy_memory_to_acceleration_structure_khr(
        &self,
        _deferred_operation: vk::DeferredOperationKHR,
        _p_info: &vk::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn write_acceleration_structures_properties_khr(
        &self,
        _p_acceleration_structures: &[vk::AccelerationStructureKHR],
        _query_type: vk::QueryType,
        _p_data: &mut [u8],
        _stride: usize,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn cmd_copy_acceleration_structure_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_info: &vk::CopyAccelerationStructureInfoKHR,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_acceleration_structure_to_memory_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_info: &vk::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_copy_memory_to_acceleration_structure_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_info: &vk::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_acceleration_structure_device_address_khr(
        &self,
        _p_info: &vk::AccelerationStructureDeviceAddressInfoKHR,
    ) -> LayerResult<vk::DeviceAddress> {
        LayerResult::Unhandled
    }
    fn cmd_write_acceleration_structures_properties_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_acceleration_structures: &[vk::AccelerationStructureKHR],
        _query_type: vk::QueryType,
        _query_pool: vk::QueryPool,
        _first_query: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_device_acceleration_structure_compatibility_khr(
        &self,
        _p_version_info: &vk::AccelerationStructureVersionInfoKHR,
    ) -> LayerResult<vk::AccelerationStructureCompatibilityKHR> {
        LayerResult::Unhandled
    }
    fn get_acceleration_structure_build_sizes_khr(
        &self,
        _build_type: vk::AccelerationStructureBuildTypeKHR,
        _p_build_info: &vk::AccelerationStructureBuildGeometryInfoKHR,
        _p_max_primitive_counts: Option<&[u32]>,
    ) -> LayerResult<vk::AccelerationStructureBuildSizesInfoKHR> {
        LayerResult::Unhandled
    }
    fn cmd_trace_rays_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_raygen_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        _p_miss_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        _p_hit_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        _p_callable_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        _width: u32,
        _height: u32,
        _depth: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_ray_tracing_pipelines_khr(
        &self,
        _deferred_operation: vk::DeferredOperationKHR,
        _pipeline_cache: vk::PipelineCache,
        _p_create_infos: &[vk::RayTracingPipelineCreateInfoKHR],
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<Vec<vk::Pipeline>>> {
        LayerResult::Unhandled
    }
    fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        &self,
        _pipeline: vk::Pipeline,
        _first_group: u32,
        _group_count: u32,
    ) -> LayerResult<VkResult<Vec<u8>>> {
        LayerResult::Unhandled
    }
    fn cmd_trace_rays_indirect_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _p_raygen_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        _p_miss_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        _p_hit_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        _p_callable_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        _indirect_device_address: vk::DeviceAddress,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_ray_tracing_shader_group_stack_size_khr(
        &self,
        _pipeline: vk::Pipeline,
        _group: u32,
        _group_shader: vk::ShaderGroupShaderKHR,
    ) -> LayerResult<vk::DeviceSize> {
        LayerResult::Unhandled
    }
    fn cmd_set_ray_tracing_pipeline_stack_size_khr(
        &self,
        _command_buffer: vk::CommandBuffer,
        _pipeline_stack_size: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_mesh_tasks_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _group_countx: u32,
        _group_county: u32,
        _group_countz: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_mesh_tasks_indirect_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _buffer: vk::Buffer,
        _offset: vk::DeviceSize,
        _draw_count: u32,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn cmd_draw_mesh_tasks_indirect_count_ext(
        &self,
        _command_buffer: vk::CommandBuffer,
        _buffer: vk::Buffer,
        _offset: vk::DeviceSize,
        _count_buffer: vk::Buffer,
        _count_buffer_offset: vk::DeviceSize,
        _max_draw_count: u32,
        _stride: u32,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
}

pub trait InstanceInfo: Send + Sync {
    fn get_physical_device_features(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<vk::PhysicalDeviceFeatures> {
        LayerResult::Unhandled
    }
    fn get_physical_device_format_properties(
        &self,
        _physical_device: vk::PhysicalDevice,
        _format: vk::Format,
    ) -> LayerResult<vk::FormatProperties> {
        LayerResult::Unhandled
    }
    fn get_physical_device_image_format_properties(
        &self,
        _physical_device: vk::PhysicalDevice,
        _format: vk::Format,
        _type: vk::ImageType,
        _tiling: vk::ImageTiling,
        _usage: vk::ImageUsageFlags,
        _flags: vk::ImageCreateFlags,
    ) -> LayerResult<VkResult<vk::ImageFormatProperties>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_properties(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<vk::PhysicalDeviceProperties> {
        LayerResult::Unhandled
    }
    fn get_physical_device_queue_family_properties(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<Vec<vk::QueueFamilyProperties>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_memory_properties(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<vk::PhysicalDeviceMemoryProperties> {
        LayerResult::Unhandled
    }
    fn create_device(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_create_info: &vk::DeviceCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Device>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_sparse_image_format_properties(
        &self,
        _physical_device: vk::PhysicalDevice,
        _format: vk::Format,
        _type: vk::ImageType,
        _samples: vk::SampleCountFlags,
        _usage: vk::ImageUsageFlags,
        _tiling: vk::ImageTiling,
    ) -> LayerResult<Vec<vk::SparseImageFormatProperties>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_features2(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<vk::PhysicalDeviceFeatures2> {
        LayerResult::Unhandled
    }
    fn get_physical_device_properties2(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<vk::PhysicalDeviceProperties2> {
        LayerResult::Unhandled
    }
    fn get_physical_device_format_properties2(
        &self,
        _physical_device: vk::PhysicalDevice,
        _format: vk::Format,
    ) -> LayerResult<vk::FormatProperties2> {
        LayerResult::Unhandled
    }
    fn get_physical_device_image_format_properties2(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_image_format_info: &vk::PhysicalDeviceImageFormatInfo2,
    ) -> LayerResult<VkResult<vk::ImageFormatProperties2>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_queue_family_properties2(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<Vec<vk::QueueFamilyProperties2>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_memory_properties2(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<vk::PhysicalDeviceMemoryProperties2> {
        LayerResult::Unhandled
    }
    fn get_physical_device_sparse_image_format_properties2(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_format_info: &vk::PhysicalDeviceSparseImageFormatInfo2,
    ) -> LayerResult<Vec<vk::SparseImageFormatProperties2>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_external_buffer_properties(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_external_buffer_info: &vk::PhysicalDeviceExternalBufferInfo,
    ) -> LayerResult<vk::ExternalBufferProperties> {
        LayerResult::Unhandled
    }
    fn get_physical_device_external_fence_properties(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_external_fence_info: &vk::PhysicalDeviceExternalFenceInfo,
    ) -> LayerResult<vk::ExternalFenceProperties> {
        LayerResult::Unhandled
    }
    fn get_physical_device_external_semaphore_properties(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfo,
    ) -> LayerResult<vk::ExternalSemaphoreProperties> {
        LayerResult::Unhandled
    }
    fn get_physical_device_tool_properties(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<VkResult<Vec<vk::PhysicalDeviceToolProperties>>> {
        LayerResult::Unhandled
    }
    fn destroy_surface_khr(
        &self,
        _surface: vk::SurfaceKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_physical_device_surface_support_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _queue_family_index: u32,
        _surface: vk::SurfaceKHR,
    ) -> LayerResult<VkResult<bool>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_surface_capabilities_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _surface: vk::SurfaceKHR,
    ) -> LayerResult<VkResult<vk::SurfaceCapabilitiesKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_surface_formats_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _surface: vk::SurfaceKHR,
    ) -> LayerResult<VkResult<Vec<vk::SurfaceFormatKHR>>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_surface_present_modes_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _surface: vk::SurfaceKHR,
    ) -> LayerResult<VkResult<Vec<vk::PresentModeKHR>>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_present_rectangles_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _surface: vk::SurfaceKHR,
    ) -> LayerResult<VkResult<Vec<vk::Rect2D>>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_display_properties_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<VkResult<Vec<vk::DisplayPropertiesKHR>>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_display_plane_properties_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<VkResult<Vec<vk::DisplayPlanePropertiesKHR>>> {
        LayerResult::Unhandled
    }
    fn get_display_plane_supported_displays_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _plane_index: u32,
    ) -> LayerResult<VkResult<Vec<vk::DisplayKHR>>> {
        LayerResult::Unhandled
    }
    fn get_display_mode_properties_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _display: vk::DisplayKHR,
    ) -> LayerResult<VkResult<Vec<vk::DisplayModePropertiesKHR>>> {
        LayerResult::Unhandled
    }
    fn create_display_mode_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _display: vk::DisplayKHR,
        _p_create_info: &vk::DisplayModeCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::DisplayModeKHR>> {
        LayerResult::Unhandled
    }
    fn get_display_plane_capabilities_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _mode: vk::DisplayModeKHR,
        _plane_index: u32,
    ) -> LayerResult<VkResult<vk::DisplayPlaneCapabilitiesKHR>> {
        LayerResult::Unhandled
    }
    fn create_display_plane_surface_khr(
        &self,
        _p_create_info: &vk::DisplaySurfaceCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn create_xlib_surface_khr(
        &self,
        _p_create_info: &vk::XlibSurfaceCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_xlib_presentation_support_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _queue_family_index: u32,
        _dpy: &mut vk::Display,
        _visual_id: vk::VisualID,
    ) -> LayerResult<bool> {
        LayerResult::Unhandled
    }
    fn create_xcb_surface_khr(
        &self,
        _p_create_info: &vk::XcbSurfaceCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_xcb_presentation_support_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _queue_family_index: u32,
        _connection: *mut vk::xcb_connection_t,
        _visual_id: vk::xcb_visualid_t,
    ) -> LayerResult<bool> {
        LayerResult::Unhandled
    }
    fn create_wayland_surface_khr(
        &self,
        _p_create_info: &vk::WaylandSurfaceCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_wayland_presentation_support_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _queue_family_index: u32,
        _display: *mut vk::wl_display,
    ) -> LayerResult<bool> {
        LayerResult::Unhandled
    }
    fn create_android_surface_khr(
        &self,
        _p_create_info: &vk::AndroidSurfaceCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn create_win32_surface_khr(
        &self,
        _p_create_info: &vk::Win32SurfaceCreateInfoKHR,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_win32_presentation_support_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _queue_family_index: u32,
    ) -> LayerResult<bool> {
        LayerResult::Unhandled
    }
    fn get_physical_device_video_capabilities_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_video_profile: &vk::VideoProfileInfoKHR,
    ) -> LayerResult<VkResult<vk::VideoCapabilitiesKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_video_format_properties_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_video_format_info: &vk::PhysicalDeviceVideoFormatInfoKHR,
    ) -> LayerResult<VkResult<Vec<vk::VideoFormatPropertiesKHR>>> {
        LayerResult::Unhandled
    }
    fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _queue_family_index: u32,
        _p_counters: Option<&mut [vk::PerformanceCounterKHR]>,
    ) -> LayerResult<VkResult<Vec<vk::PerformanceCounterDescriptionKHR>>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_queue_family_performance_query_passes_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_performance_query_create_info: &vk::QueryPoolPerformanceCreateInfoKHR,
    ) -> LayerResult<u32> {
        LayerResult::Unhandled
    }
    fn get_physical_device_surface_capabilities2_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> LayerResult<VkResult<vk::SurfaceCapabilities2KHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_surface_formats2_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> LayerResult<VkResult<Vec<vk::SurfaceFormat2KHR>>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_display_properties2_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<VkResult<Vec<vk::DisplayProperties2KHR>>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_display_plane_properties2_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<VkResult<Vec<vk::DisplayPlaneProperties2KHR>>> {
        LayerResult::Unhandled
    }
    fn get_display_mode_properties2_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _display: vk::DisplayKHR,
    ) -> LayerResult<VkResult<Vec<vk::DisplayModeProperties2KHR>>> {
        LayerResult::Unhandled
    }
    fn get_display_plane_capabilities2_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_display_plane_info: &vk::DisplayPlaneInfo2KHR,
    ) -> LayerResult<VkResult<vk::DisplayPlaneCapabilities2KHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_fragment_shading_rates_khr(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<VkResult<Vec<vk::PhysicalDeviceFragmentShadingRateKHR>>> {
        LayerResult::Unhandled
    }
    fn create_debug_report_callback_ext(
        &self,
        _p_create_info: &vk::DebugReportCallbackCreateInfoEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::DebugReportCallbackEXT>> {
        LayerResult::Unhandled
    }
    fn destroy_debug_report_callback_ext(
        &self,
        _callback: vk::DebugReportCallbackEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn debug_report_message_ext(
        &self,
        _flags: vk::DebugReportFlagsEXT,
        _object_type: vk::DebugReportObjectTypeEXT,
        _object: u64,
        _location: usize,
        _message_code: i32,
        _p_layer_prefix: &str,
        _p_message: &str,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn create_stream_descriptor_surface_ggp(
        &self,
        _p_create_info: &vk::StreamDescriptorSurfaceCreateInfoGGP,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_external_image_format_properties_nv(
        &self,
        _physical_device: vk::PhysicalDevice,
        _format: vk::Format,
        _type: vk::ImageType,
        _tiling: vk::ImageTiling,
        _usage: vk::ImageUsageFlags,
        _flags: vk::ImageCreateFlags,
        _external_handle_type: vk::ExternalMemoryHandleTypeFlagsNV,
    ) -> LayerResult<VkResult<vk::ExternalImageFormatPropertiesNV>> {
        LayerResult::Unhandled
    }
    fn create_vi_surface_nn(
        &self,
        _p_create_info: &vk::ViSurfaceCreateInfoNN,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn release_display_ext(
        &self,
        _physical_device: vk::PhysicalDevice,
        _display: vk::DisplayKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn acquire_xlib_display_ext(
        &self,
        _physical_device: vk::PhysicalDevice,
        _dpy: &mut vk::Display,
        _display: vk::DisplayKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_rand_r_output_display_ext(
        &self,
        _physical_device: vk::PhysicalDevice,
        _dpy: &mut vk::Display,
        _rr_output: vk::RROutput,
    ) -> LayerResult<VkResult<vk::DisplayKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_surface_capabilities2_ext(
        &self,
        _physical_device: vk::PhysicalDevice,
        _surface: vk::SurfaceKHR,
    ) -> LayerResult<VkResult<vk::SurfaceCapabilities2EXT>> {
        LayerResult::Unhandled
    }
    fn create_ios_surface_mvk(
        &self,
        _p_create_info: &vk::IOSSurfaceCreateInfoMVK,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn create_mac_os_surface_mvk(
        &self,
        _p_create_info: &vk::MacOSSurfaceCreateInfoMVK,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn create_debug_utils_messenger_ext(
        &self,
        _p_create_info: &vk::DebugUtilsMessengerCreateInfoEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::DebugUtilsMessengerEXT>> {
        LayerResult::Unhandled
    }
    fn destroy_debug_utils_messenger_ext(
        &self,
        _messenger: vk::DebugUtilsMessengerEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn submit_debug_utils_message_ext(
        &self,
        _message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        _message_types: vk::DebugUtilsMessageTypeFlagsEXT,
        _p_callback_data: &vk::DebugUtilsMessengerCallbackDataEXT,
    ) -> LayerResult<()> {
        LayerResult::Unhandled
    }
    fn get_physical_device_multisample_properties_ext(
        &self,
        _physical_device: vk::PhysicalDevice,
        _samples: vk::SampleCountFlags,
    ) -> LayerResult<vk::MultisamplePropertiesEXT> {
        LayerResult::Unhandled
    }
    fn get_physical_device_calibrateable_time_domains_ext(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<VkResult<Vec<vk::TimeDomainEXT>>> {
        LayerResult::Unhandled
    }
    fn create_image_pipe_surface_fuchsia(
        &self,
        _p_create_info: &vk::ImagePipeSurfaceCreateInfoFUCHSIA,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn create_metal_surface_ext(
        &self,
        _p_create_info: &vk::MetalSurfaceCreateInfoEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_cooperative_matrix_properties_nv(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<VkResult<Vec<vk::CooperativeMatrixPropertiesNV>>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        &self,
        _physical_device: vk::PhysicalDevice,
    ) -> LayerResult<VkResult<Vec<vk::FramebufferMixedSamplesCombinationNV>>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_surface_present_modes2_ext(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> LayerResult<VkResult<Vec<vk::PresentModeKHR>>> {
        LayerResult::Unhandled
    }
    fn create_headless_surface_ext(
        &self,
        _p_create_info: &vk::HeadlessSurfaceCreateInfoEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn acquire_drm_display_ext(
        &self,
        _physical_device: vk::PhysicalDevice,
        _drm_fd: i32,
        _display: vk::DisplayKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_drm_display_ext(
        &self,
        _physical_device: vk::PhysicalDevice,
        _drm_fd: i32,
        _connector_id: u32,
    ) -> LayerResult<VkResult<vk::DisplayKHR>> {
        LayerResult::Unhandled
    }
    fn acquire_winrt_display_nv(
        &self,
        _physical_device: vk::PhysicalDevice,
        _display: vk::DisplayKHR,
    ) -> LayerResult<VkResult<()>> {
        LayerResult::Unhandled
    }
    fn get_winrt_display_nv(
        &self,
        _physical_device: vk::PhysicalDevice,
        _device_relative_id: u32,
    ) -> LayerResult<VkResult<vk::DisplayKHR>> {
        LayerResult::Unhandled
    }
    fn create_direct_fb_surface_ext(
        &self,
        _p_create_info: &vk::DirectFBSurfaceCreateInfoEXT,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_direct_fb_presentation_support_ext(
        &self,
        _physical_device: vk::PhysicalDevice,
        _queue_family_index: u32,
        _dfb: &mut vk::IDirectFB,
    ) -> LayerResult<bool> {
        LayerResult::Unhandled
    }
    fn create_screen_surface_qnx(
        &self,
        _p_create_info: &vk::ScreenSurfaceCreateInfoQNX,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::SurfaceKHR>> {
        LayerResult::Unhandled
    }
    fn get_physical_device_screen_presentation_support_qnx(
        &self,
        _physical_device: vk::PhysicalDevice,
        _queue_family_index: u32,
        _window: *mut vk::_screen_window,
    ) -> LayerResult<bool> {
        LayerResult::Unhandled
    }
    fn get_physical_device_optical_flow_image_formats_nv(
        &self,
        _physical_device: vk::PhysicalDevice,
        _p_optical_flow_image_format_info: &vk::OpticalFlowImageFormatInfoNV,
    ) -> LayerResult<VkResult<Vec<vk::OpticalFlowImageFormatPropertiesNV>>> {
        LayerResult::Unhandled
    }
}

pub trait Layer: 'static + Sync + Default {
    const LAYER_NAME: &'static str;
    const SPEC_VERSION: u32;
    const IMPLEMENTATION_VERSION: u32;
    const LAYER_DESCRIPTION: &'static str;

    type InstanceInfo: InstanceInfo;
    type DeviceInfo: DeviceInfo;

    fn create_instance_info(
        &self,
        create_info: &vk::InstanceCreateInfo,
        allocator: Option<&vk::AllocationCallbacks>,
        instance: Arc<ash::Instance>,
    ) -> Self::InstanceInfo;

    fn create_device_info(
        &self,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
        allocator: Option<&vk::AllocationCallbacks>,
        device: Arc<ash::Device>,
    ) -> Self::DeviceInfo;

    fn create_instance(
        &self,
        _p_create_info: &vk::InstanceCreateInfo,
        _p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LayerResult<VkResult<vk::Instance>> {
        LayerResult::Unhandled
    }
}
