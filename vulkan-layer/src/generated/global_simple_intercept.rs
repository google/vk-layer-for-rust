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
#![allow(unused_unsafe)]
use ash::vk;
use std::{
    ffi::{c_char, c_int, c_void, CStr},
    ptr::NonNull,
    sync::Arc,
};

use super::{get_device_proc_addr_loader, get_instance_proc_addr_loader, VulkanCommand};
use crate::{fill_vk_out_array, DeviceInfo, Global, InstanceInfo, Layer, LayerResult};

pub(crate) struct DeviceDispatchTable {
    pub core: Arc<ash::Device>,
    khr_swapchain: Arc<ash::vk::KhrSwapchainFn>,
    khr_video_queue: Arc<ash::vk::KhrVideoQueueFn>,
    khr_performance_query: Arc<ash::vk::KhrPerformanceQueryFn>,
    khr_fragment_shading_rate: Arc<ash::vk::KhrFragmentShadingRateFn>,
    ext_debug_utils: Arc<ash::vk::ExtDebugUtilsFn>,
    ext_sample_locations: Arc<ash::vk::ExtSampleLocationsFn>,
    ext_calibrated_timestamps: Arc<ash::vk::ExtCalibratedTimestampsFn>,
    ext_full_screen_exclusive: Arc<ash::vk::ExtFullScreenExclusiveFn>,
    nv_optical_flow: Arc<ash::vk::NvOpticalFlowFn>,
    khr_display_swapchain: Arc<ash::vk::KhrDisplaySwapchainFn>,
    khr_video_decode_queue: Arc<ash::vk::KhrVideoDecodeQueueFn>,
    khr_external_memory_win32: Arc<ash::vk::KhrExternalMemoryWin32Fn>,
    khr_external_memory_fd: Arc<ash::vk::KhrExternalMemoryFdFn>,
    khr_external_semaphore_win32: Arc<ash::vk::KhrExternalSemaphoreWin32Fn>,
    khr_external_semaphore_fd: Arc<ash::vk::KhrExternalSemaphoreFdFn>,
    khr_push_descriptor: Arc<ash::vk::KhrPushDescriptorFn>,
    khr_shared_presentable_image: Arc<ash::vk::KhrSharedPresentableImageFn>,
    khr_external_fence_win32: Arc<ash::vk::KhrExternalFenceWin32Fn>,
    khr_external_fence_fd: Arc<ash::vk::KhrExternalFenceFdFn>,
    khr_present_wait: Arc<ash::vk::KhrPresentWaitFn>,
    khr_deferred_host_operations: Arc<ash::vk::KhrDeferredHostOperationsFn>,
    khr_pipeline_executable_properties: Arc<ash::vk::KhrPipelineExecutablePropertiesFn>,
    khr_video_encode_queue: Arc<ash::vk::KhrVideoEncodeQueueFn>,
    khr_synchronization2: Arc<ash::vk::KhrSynchronization2Fn>,
    khr_ray_tracing_maintenance1: Arc<ash::vk::KhrRayTracingMaintenance1Fn>,
    ext_debug_marker: Arc<ash::vk::ExtDebugMarkerFn>,
    ext_transform_feedback: Arc<ash::vk::ExtTransformFeedbackFn>,
    nvx_binary_import: Arc<ash::vk::NvxBinaryImportFn>,
    nvx_image_view_handle: Arc<ash::vk::NvxImageViewHandleFn>,
    amd_shader_info: Arc<ash::vk::AmdShaderInfoFn>,
    nv_external_memory_win32: Arc<ash::vk::NvExternalMemoryWin32Fn>,
    ext_conditional_rendering: Arc<ash::vk::ExtConditionalRenderingFn>,
    nv_clip_space_w_scaling: Arc<ash::vk::NvClipSpaceWScalingFn>,
    ext_display_control: Arc<ash::vk::ExtDisplayControlFn>,
    google_display_timing: Arc<ash::vk::GoogleDisplayTimingFn>,
    ext_discard_rectangles: Arc<ash::vk::ExtDiscardRectanglesFn>,
    ext_hdr_metadata: Arc<ash::vk::ExtHdrMetadataFn>,
    android_external_memory_android_hardware_buffer:
        Arc<ash::vk::AndroidExternalMemoryAndroidHardwareBufferFn>,
    ext_image_drm_format_modifier: Arc<ash::vk::ExtImageDrmFormatModifierFn>,
    ext_validation_cache: Arc<ash::vk::ExtValidationCacheFn>,
    nv_shading_rate_image: Arc<ash::vk::NvShadingRateImageFn>,
    nv_ray_tracing: Arc<ash::vk::NvRayTracingFn>,
    khr_ray_tracing_pipeline: Arc<ash::vk::KhrRayTracingPipelineFn>,
    ext_external_memory_host: Arc<ash::vk::ExtExternalMemoryHostFn>,
    amd_buffer_marker: Arc<ash::vk::AmdBufferMarkerFn>,
    nv_mesh_shader: Arc<ash::vk::NvMeshShaderFn>,
    nv_scissor_exclusive: Arc<ash::vk::NvScissorExclusiveFn>,
    nv_device_diagnostic_checkpoints: Arc<ash::vk::NvDeviceDiagnosticCheckpointsFn>,
    intel_performance_query: Arc<ash::vk::IntelPerformanceQueryFn>,
    amd_display_native_hdr: Arc<ash::vk::AmdDisplayNativeHdrFn>,
    ext_line_rasterization: Arc<ash::vk::ExtLineRasterizationFn>,
    ext_swapchain_maintenance1: Arc<ash::vk::ExtSwapchainMaintenance1Fn>,
    nv_device_generated_commands: Arc<ash::vk::NvDeviceGeneratedCommandsFn>,
    ext_metal_objects: Arc<ash::vk::ExtMetalObjectsFn>,
    ext_descriptor_buffer: Arc<ash::vk::ExtDescriptorBufferFn>,
    nv_fragment_shading_rate_enums: Arc<ash::vk::NvFragmentShadingRateEnumsFn>,
    ext_image_compression_control: Arc<ash::vk::ExtImageCompressionControlFn>,
    ext_vertex_input_dynamic_state: Arc<ash::vk::ExtVertexInputDynamicStateFn>,
    fuchsia_external_memory: Arc<ash::vk::FuchsiaExternalMemoryFn>,
    fuchsia_external_semaphore: Arc<ash::vk::FuchsiaExternalSemaphoreFn>,
    fuchsia_buffer_collection: Arc<ash::vk::FuchsiaBufferCollectionFn>,
    huawei_subpass_shading: Arc<ash::vk::HuaweiSubpassShadingFn>,
    huawei_invocation_mask: Arc<ash::vk::HuaweiInvocationMaskFn>,
    nv_external_memory_rdma: Arc<ash::vk::NvExternalMemoryRdmaFn>,
    ext_pipeline_properties: Arc<ash::vk::ExtPipelinePropertiesFn>,
    ext_extended_dynamic_state2: Arc<ash::vk::ExtExtendedDynamicState2Fn>,
    ext_color_write_enable: Arc<ash::vk::ExtColorWriteEnableFn>,
    ext_multi_draw: Arc<ash::vk::ExtMultiDrawFn>,
    ext_opacity_micromap: Arc<ash::vk::ExtOpacityMicromapFn>,
    ext_pageable_device_local_memory: Arc<ash::vk::ExtPageableDeviceLocalMemoryFn>,
    valve_descriptor_set_host_mapping: Arc<ash::vk::ValveDescriptorSetHostMappingFn>,
    nv_copy_memory_indirect: Arc<ash::vk::NvCopyMemoryIndirectFn>,
    nv_memory_decompression: Arc<ash::vk::NvMemoryDecompressionFn>,
    ext_extended_dynamic_state3: Arc<ash::vk::ExtExtendedDynamicState3Fn>,
    ext_shader_module_identifier: Arc<ash::vk::ExtShaderModuleIdentifierFn>,
    qcom_tile_properties: Arc<ash::vk::QcomTilePropertiesFn>,
    khr_acceleration_structure: Arc<ash::vk::KhrAccelerationStructureFn>,
    ext_mesh_shader: Arc<ash::vk::ExtMeshShaderFn>,
}
impl DeviceDispatchTable {
    pub(crate) fn load(
        get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
        device: Arc<ash::Device>,
    ) -> Self {
        let proc_addr_loader = get_device_proc_addr_loader(get_device_proc_addr, &device);
        Self {
            core: Arc::clone(&device),
            khr_swapchain: Arc::new(ash::vk::KhrSwapchainFn::load(&proc_addr_loader)),
            khr_video_queue: Arc::new(ash::vk::KhrVideoQueueFn::load(&proc_addr_loader)),
            khr_performance_query: Arc::new(ash::vk::KhrPerformanceQueryFn::load(
                &proc_addr_loader,
            )),
            khr_fragment_shading_rate: Arc::new(ash::vk::KhrFragmentShadingRateFn::load(
                &proc_addr_loader,
            )),
            ext_debug_utils: Arc::new(ash::vk::ExtDebugUtilsFn::load(&proc_addr_loader)),
            ext_sample_locations: Arc::new(ash::vk::ExtSampleLocationsFn::load(&proc_addr_loader)),
            ext_calibrated_timestamps: Arc::new(ash::vk::ExtCalibratedTimestampsFn::load(
                &proc_addr_loader,
            )),
            ext_full_screen_exclusive: Arc::new(ash::vk::ExtFullScreenExclusiveFn::load(
                &proc_addr_loader,
            )),
            nv_optical_flow: Arc::new(ash::vk::NvOpticalFlowFn::load(&proc_addr_loader)),
            khr_display_swapchain: Arc::new(ash::vk::KhrDisplaySwapchainFn::load(
                &proc_addr_loader,
            )),
            khr_video_decode_queue: Arc::new(ash::vk::KhrVideoDecodeQueueFn::load(
                &proc_addr_loader,
            )),
            khr_external_memory_win32: Arc::new(ash::vk::KhrExternalMemoryWin32Fn::load(
                &proc_addr_loader,
            )),
            khr_external_memory_fd: Arc::new(ash::vk::KhrExternalMemoryFdFn::load(
                &proc_addr_loader,
            )),
            khr_external_semaphore_win32: Arc::new(ash::vk::KhrExternalSemaphoreWin32Fn::load(
                &proc_addr_loader,
            )),
            khr_external_semaphore_fd: Arc::new(ash::vk::KhrExternalSemaphoreFdFn::load(
                &proc_addr_loader,
            )),
            khr_push_descriptor: Arc::new(ash::vk::KhrPushDescriptorFn::load(&proc_addr_loader)),
            khr_shared_presentable_image: Arc::new(ash::vk::KhrSharedPresentableImageFn::load(
                &proc_addr_loader,
            )),
            khr_external_fence_win32: Arc::new(ash::vk::KhrExternalFenceWin32Fn::load(
                &proc_addr_loader,
            )),
            khr_external_fence_fd: Arc::new(ash::vk::KhrExternalFenceFdFn::load(&proc_addr_loader)),
            khr_present_wait: Arc::new(ash::vk::KhrPresentWaitFn::load(&proc_addr_loader)),
            khr_deferred_host_operations: Arc::new(ash::vk::KhrDeferredHostOperationsFn::load(
                &proc_addr_loader,
            )),
            khr_pipeline_executable_properties: Arc::new(
                ash::vk::KhrPipelineExecutablePropertiesFn::load(&proc_addr_loader),
            ),
            khr_video_encode_queue: Arc::new(ash::vk::KhrVideoEncodeQueueFn::load(
                &proc_addr_loader,
            )),
            khr_synchronization2: Arc::new(ash::vk::KhrSynchronization2Fn::load(&proc_addr_loader)),
            khr_ray_tracing_maintenance1: Arc::new(ash::vk::KhrRayTracingMaintenance1Fn::load(
                &proc_addr_loader,
            )),
            ext_debug_marker: Arc::new(ash::vk::ExtDebugMarkerFn::load(&proc_addr_loader)),
            ext_transform_feedback: Arc::new(ash::vk::ExtTransformFeedbackFn::load(
                &proc_addr_loader,
            )),
            nvx_binary_import: Arc::new(ash::vk::NvxBinaryImportFn::load(&proc_addr_loader)),
            nvx_image_view_handle: Arc::new(ash::vk::NvxImageViewHandleFn::load(&proc_addr_loader)),
            amd_shader_info: Arc::new(ash::vk::AmdShaderInfoFn::load(&proc_addr_loader)),
            nv_external_memory_win32: Arc::new(ash::vk::NvExternalMemoryWin32Fn::load(
                &proc_addr_loader,
            )),
            ext_conditional_rendering: Arc::new(ash::vk::ExtConditionalRenderingFn::load(
                &proc_addr_loader,
            )),
            nv_clip_space_w_scaling: Arc::new(ash::vk::NvClipSpaceWScalingFn::load(
                &proc_addr_loader,
            )),
            ext_display_control: Arc::new(ash::vk::ExtDisplayControlFn::load(&proc_addr_loader)),
            google_display_timing: Arc::new(ash::vk::GoogleDisplayTimingFn::load(
                &proc_addr_loader,
            )),
            ext_discard_rectangles: Arc::new(ash::vk::ExtDiscardRectanglesFn::load(
                &proc_addr_loader,
            )),
            ext_hdr_metadata: Arc::new(ash::vk::ExtHdrMetadataFn::load(&proc_addr_loader)),
            android_external_memory_android_hardware_buffer: Arc::new(
                ash::vk::AndroidExternalMemoryAndroidHardwareBufferFn::load(&proc_addr_loader),
            ),
            ext_image_drm_format_modifier: Arc::new(ash::vk::ExtImageDrmFormatModifierFn::load(
                &proc_addr_loader,
            )),
            ext_validation_cache: Arc::new(ash::vk::ExtValidationCacheFn::load(&proc_addr_loader)),
            nv_shading_rate_image: Arc::new(ash::vk::NvShadingRateImageFn::load(&proc_addr_loader)),
            nv_ray_tracing: Arc::new(ash::vk::NvRayTracingFn::load(&proc_addr_loader)),
            khr_ray_tracing_pipeline: Arc::new(ash::vk::KhrRayTracingPipelineFn::load(
                &proc_addr_loader,
            )),
            ext_external_memory_host: Arc::new(ash::vk::ExtExternalMemoryHostFn::load(
                &proc_addr_loader,
            )),
            amd_buffer_marker: Arc::new(ash::vk::AmdBufferMarkerFn::load(&proc_addr_loader)),
            nv_mesh_shader: Arc::new(ash::vk::NvMeshShaderFn::load(&proc_addr_loader)),
            nv_scissor_exclusive: Arc::new(ash::vk::NvScissorExclusiveFn::load(&proc_addr_loader)),
            nv_device_diagnostic_checkpoints: Arc::new(
                ash::vk::NvDeviceDiagnosticCheckpointsFn::load(&proc_addr_loader),
            ),
            intel_performance_query: Arc::new(ash::vk::IntelPerformanceQueryFn::load(
                &proc_addr_loader,
            )),
            amd_display_native_hdr: Arc::new(ash::vk::AmdDisplayNativeHdrFn::load(
                &proc_addr_loader,
            )),
            ext_line_rasterization: Arc::new(ash::vk::ExtLineRasterizationFn::load(
                &proc_addr_loader,
            )),
            ext_swapchain_maintenance1: Arc::new(ash::vk::ExtSwapchainMaintenance1Fn::load(
                &proc_addr_loader,
            )),
            nv_device_generated_commands: Arc::new(ash::vk::NvDeviceGeneratedCommandsFn::load(
                &proc_addr_loader,
            )),
            ext_metal_objects: Arc::new(ash::vk::ExtMetalObjectsFn::load(&proc_addr_loader)),
            ext_descriptor_buffer: Arc::new(ash::vk::ExtDescriptorBufferFn::load(
                &proc_addr_loader,
            )),
            nv_fragment_shading_rate_enums: Arc::new(ash::vk::NvFragmentShadingRateEnumsFn::load(
                &proc_addr_loader,
            )),
            ext_image_compression_control: Arc::new(ash::vk::ExtImageCompressionControlFn::load(
                &proc_addr_loader,
            )),
            ext_vertex_input_dynamic_state: Arc::new(ash::vk::ExtVertexInputDynamicStateFn::load(
                &proc_addr_loader,
            )),
            fuchsia_external_memory: Arc::new(ash::vk::FuchsiaExternalMemoryFn::load(
                &proc_addr_loader,
            )),
            fuchsia_external_semaphore: Arc::new(ash::vk::FuchsiaExternalSemaphoreFn::load(
                &proc_addr_loader,
            )),
            fuchsia_buffer_collection: Arc::new(ash::vk::FuchsiaBufferCollectionFn::load(
                &proc_addr_loader,
            )),
            huawei_subpass_shading: Arc::new(ash::vk::HuaweiSubpassShadingFn::load(
                &proc_addr_loader,
            )),
            huawei_invocation_mask: Arc::new(ash::vk::HuaweiInvocationMaskFn::load(
                &proc_addr_loader,
            )),
            nv_external_memory_rdma: Arc::new(ash::vk::NvExternalMemoryRdmaFn::load(
                &proc_addr_loader,
            )),
            ext_pipeline_properties: Arc::new(ash::vk::ExtPipelinePropertiesFn::load(
                &proc_addr_loader,
            )),
            ext_extended_dynamic_state2: Arc::new(ash::vk::ExtExtendedDynamicState2Fn::load(
                &proc_addr_loader,
            )),
            ext_color_write_enable: Arc::new(ash::vk::ExtColorWriteEnableFn::load(
                &proc_addr_loader,
            )),
            ext_multi_draw: Arc::new(ash::vk::ExtMultiDrawFn::load(&proc_addr_loader)),
            ext_opacity_micromap: Arc::new(ash::vk::ExtOpacityMicromapFn::load(&proc_addr_loader)),
            ext_pageable_device_local_memory: Arc::new(
                ash::vk::ExtPageableDeviceLocalMemoryFn::load(&proc_addr_loader),
            ),
            valve_descriptor_set_host_mapping: Arc::new(
                ash::vk::ValveDescriptorSetHostMappingFn::load(&proc_addr_loader),
            ),
            nv_copy_memory_indirect: Arc::new(ash::vk::NvCopyMemoryIndirectFn::load(
                &proc_addr_loader,
            )),
            nv_memory_decompression: Arc::new(ash::vk::NvMemoryDecompressionFn::load(
                &proc_addr_loader,
            )),
            ext_extended_dynamic_state3: Arc::new(ash::vk::ExtExtendedDynamicState3Fn::load(
                &proc_addr_loader,
            )),
            ext_shader_module_identifier: Arc::new(ash::vk::ExtShaderModuleIdentifierFn::load(
                &proc_addr_loader,
            )),
            qcom_tile_properties: Arc::new(ash::vk::QcomTilePropertiesFn::load(&proc_addr_loader)),
            khr_acceleration_structure: Arc::new(ash::vk::KhrAccelerationStructureFn::load(
                &proc_addr_loader,
            )),
            ext_mesh_shader: Arc::new(ash::vk::ExtMeshShaderFn::load(&proc_addr_loader)),
        }
    }
}
pub(crate) struct InstanceDispatchTable {
    pub core: Arc<ash::Instance>,
    khr_surface: Arc<ash::vk::KhrSurfaceFn>,
    khr_swapchain: Arc<ash::vk::KhrSwapchainFn>,
    khr_display: Arc<ash::vk::KhrDisplayFn>,
    khr_xlib_surface: Arc<ash::vk::KhrXlibSurfaceFn>,
    khr_xcb_surface: Arc<ash::vk::KhrXcbSurfaceFn>,
    khr_wayland_surface: Arc<ash::vk::KhrWaylandSurfaceFn>,
    khr_android_surface: Arc<ash::vk::KhrAndroidSurfaceFn>,
    khr_win32_surface: Arc<ash::vk::KhrWin32SurfaceFn>,
    khr_video_queue: Arc<ash::vk::KhrVideoQueueFn>,
    khr_performance_query: Arc<ash::vk::KhrPerformanceQueryFn>,
    khr_get_surface_capabilities2: Arc<ash::vk::KhrGetSurfaceCapabilities2Fn>,
    khr_get_display_properties2: Arc<ash::vk::KhrGetDisplayProperties2Fn>,
    khr_fragment_shading_rate: Arc<ash::vk::KhrFragmentShadingRateFn>,
    #[allow(deprecated)]
    ext_debug_report: Arc<ash::vk::ExtDebugReportFn>,
    ggp_stream_descriptor_surface: Arc<ash::vk::GgpStreamDescriptorSurfaceFn>,
    nv_external_memory_capabilities: Arc<ash::vk::NvExternalMemoryCapabilitiesFn>,
    nn_vi_surface: Arc<ash::vk::NnViSurfaceFn>,
    ext_direct_mode_display: Arc<ash::vk::ExtDirectModeDisplayFn>,
    ext_acquire_xlib_display: Arc<ash::vk::ExtAcquireXlibDisplayFn>,
    ext_display_surface_counter: Arc<ash::vk::ExtDisplaySurfaceCounterFn>,
    mvk_ios_surface: Arc<ash::vk::MvkIosSurfaceFn>,
    mvk_macos_surface: Arc<ash::vk::MvkMacosSurfaceFn>,
    ext_debug_utils: Arc<ash::vk::ExtDebugUtilsFn>,
    ext_sample_locations: Arc<ash::vk::ExtSampleLocationsFn>,
    ext_calibrated_timestamps: Arc<ash::vk::ExtCalibratedTimestampsFn>,
    fuchsia_imagepipe_surface: Arc<ash::vk::FuchsiaImagepipeSurfaceFn>,
    ext_metal_surface: Arc<ash::vk::ExtMetalSurfaceFn>,
    nv_cooperative_matrix: Arc<ash::vk::NvCooperativeMatrixFn>,
    nv_coverage_reduction_mode: Arc<ash::vk::NvCoverageReductionModeFn>,
    ext_full_screen_exclusive: Arc<ash::vk::ExtFullScreenExclusiveFn>,
    ext_headless_surface: Arc<ash::vk::ExtHeadlessSurfaceFn>,
    ext_acquire_drm_display: Arc<ash::vk::ExtAcquireDrmDisplayFn>,
    nv_acquire_winrt_display: Arc<ash::vk::NvAcquireWinrtDisplayFn>,
    ext_directfb_surface: Arc<ash::vk::ExtDirectfbSurfaceFn>,
    qnx_screen_surface: Arc<ash::vk::QnxScreenSurfaceFn>,
    nv_optical_flow: Arc<ash::vk::NvOpticalFlowFn>,
}
impl InstanceDispatchTable {
    pub(crate) fn load(
        get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
        instance: Arc<ash::Instance>,
    ) -> Self {
        let proc_addr_loader = get_instance_proc_addr_loader(get_instance_proc_addr, &instance);
        Self {
            core: Arc::clone(&instance),
            khr_surface: Arc::new(ash::vk::KhrSurfaceFn::load(&proc_addr_loader)),
            khr_swapchain: Arc::new(ash::vk::KhrSwapchainFn::load(&proc_addr_loader)),
            khr_display: Arc::new(ash::vk::KhrDisplayFn::load(&proc_addr_loader)),
            khr_xlib_surface: Arc::new(ash::vk::KhrXlibSurfaceFn::load(&proc_addr_loader)),
            khr_xcb_surface: Arc::new(ash::vk::KhrXcbSurfaceFn::load(&proc_addr_loader)),
            khr_wayland_surface: Arc::new(ash::vk::KhrWaylandSurfaceFn::load(&proc_addr_loader)),
            khr_android_surface: Arc::new(ash::vk::KhrAndroidSurfaceFn::load(&proc_addr_loader)),
            khr_win32_surface: Arc::new(ash::vk::KhrWin32SurfaceFn::load(&proc_addr_loader)),
            khr_video_queue: Arc::new(ash::vk::KhrVideoQueueFn::load(&proc_addr_loader)),
            khr_performance_query: Arc::new(ash::vk::KhrPerformanceQueryFn::load(
                &proc_addr_loader,
            )),
            khr_get_surface_capabilities2: Arc::new(ash::vk::KhrGetSurfaceCapabilities2Fn::load(
                &proc_addr_loader,
            )),
            khr_get_display_properties2: Arc::new(ash::vk::KhrGetDisplayProperties2Fn::load(
                &proc_addr_loader,
            )),
            khr_fragment_shading_rate: Arc::new(ash::vk::KhrFragmentShadingRateFn::load(
                &proc_addr_loader,
            )),
            ext_debug_report: Arc::new(ash::vk::ExtDebugReportFn::load(&proc_addr_loader)),
            ggp_stream_descriptor_surface: Arc::new(ash::vk::GgpStreamDescriptorSurfaceFn::load(
                &proc_addr_loader,
            )),
            nv_external_memory_capabilities: Arc::new(
                ash::vk::NvExternalMemoryCapabilitiesFn::load(&proc_addr_loader),
            ),
            nn_vi_surface: Arc::new(ash::vk::NnViSurfaceFn::load(&proc_addr_loader)),
            ext_direct_mode_display: Arc::new(ash::vk::ExtDirectModeDisplayFn::load(
                &proc_addr_loader,
            )),
            ext_acquire_xlib_display: Arc::new(ash::vk::ExtAcquireXlibDisplayFn::load(
                &proc_addr_loader,
            )),
            ext_display_surface_counter: Arc::new(ash::vk::ExtDisplaySurfaceCounterFn::load(
                &proc_addr_loader,
            )),
            mvk_ios_surface: Arc::new(ash::vk::MvkIosSurfaceFn::load(&proc_addr_loader)),
            mvk_macos_surface: Arc::new(ash::vk::MvkMacosSurfaceFn::load(&proc_addr_loader)),
            ext_debug_utils: Arc::new(ash::vk::ExtDebugUtilsFn::load(&proc_addr_loader)),
            ext_sample_locations: Arc::new(ash::vk::ExtSampleLocationsFn::load(&proc_addr_loader)),
            ext_calibrated_timestamps: Arc::new(ash::vk::ExtCalibratedTimestampsFn::load(
                &proc_addr_loader,
            )),
            fuchsia_imagepipe_surface: Arc::new(ash::vk::FuchsiaImagepipeSurfaceFn::load(
                &proc_addr_loader,
            )),
            ext_metal_surface: Arc::new(ash::vk::ExtMetalSurfaceFn::load(&proc_addr_loader)),
            nv_cooperative_matrix: Arc::new(ash::vk::NvCooperativeMatrixFn::load(
                &proc_addr_loader,
            )),
            nv_coverage_reduction_mode: Arc::new(ash::vk::NvCoverageReductionModeFn::load(
                &proc_addr_loader,
            )),
            ext_full_screen_exclusive: Arc::new(ash::vk::ExtFullScreenExclusiveFn::load(
                &proc_addr_loader,
            )),
            ext_headless_surface: Arc::new(ash::vk::ExtHeadlessSurfaceFn::load(&proc_addr_loader)),
            ext_acquire_drm_display: Arc::new(ash::vk::ExtAcquireDrmDisplayFn::load(
                &proc_addr_loader,
            )),
            nv_acquire_winrt_display: Arc::new(ash::vk::NvAcquireWinrtDisplayFn::load(
                &proc_addr_loader,
            )),
            ext_directfb_surface: Arc::new(ash::vk::ExtDirectfbSurfaceFn::load(&proc_addr_loader)),
            qnx_screen_surface: Arc::new(ash::vk::QnxScreenSurfaceFn::load(&proc_addr_loader)),
            nv_optical_flow: Arc::new(ash::vk::NvOpticalFlowFn::load(&proc_addr_loader)),
        }
    }
}

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

impl<T: Layer> Global<T> {
    pub(crate) const DEVICE_COMMANDS: [VulkanCommand; 513] = [
        VulkanCommand {
            name: "vkAcquireFullScreenExclusiveModeEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::acquire_full_screen_exclusive_mode_ext
                        as vk::PFN_vkAcquireFullScreenExclusiveModeEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkAcquireNextImage2KHR",
            proc: unsafe {
                std::mem::transmute(Self::acquire_next_image2_khr as vk::PFN_vkAcquireNextImage2KHR)
            },
        },
        VulkanCommand {
            name: "vkAcquireNextImageKHR",
            proc: unsafe {
                std::mem::transmute(Self::acquire_next_image_khr as vk::PFN_vkAcquireNextImageKHR)
            },
        },
        VulkanCommand {
            name: "vkAcquirePerformanceConfigurationINTEL",
            proc: unsafe {
                std::mem::transmute(
                    Self::acquire_performance_configuration_intel
                        as vk::PFN_vkAcquirePerformanceConfigurationINTEL,
                )
            },
        },
        VulkanCommand {
            name: "vkAcquireProfilingLockKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::acquire_profiling_lock_khr as vk::PFN_vkAcquireProfilingLockKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkAllocateCommandBuffers",
            proc: unsafe {
                std::mem::transmute(
                    Self::allocate_command_buffers as vk::PFN_vkAllocateCommandBuffers,
                )
            },
        },
        VulkanCommand {
            name: "vkAllocateDescriptorSets",
            proc: unsafe {
                std::mem::transmute(
                    Self::allocate_descriptor_sets as vk::PFN_vkAllocateDescriptorSets,
                )
            },
        },
        VulkanCommand {
            name: "vkAllocateMemory",
            proc: unsafe { std::mem::transmute(Self::allocate_memory as vk::PFN_vkAllocateMemory) },
        },
        VulkanCommand {
            name: "vkBeginCommandBuffer",
            proc: unsafe {
                std::mem::transmute(Self::begin_command_buffer as vk::PFN_vkBeginCommandBuffer)
            },
        },
        VulkanCommand {
            name: "vkBindAccelerationStructureMemoryNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::bind_acceleration_structure_memory_nv
                        as vk::PFN_vkBindAccelerationStructureMemoryNV,
                )
            },
        },
        VulkanCommand {
            name: "vkBindBufferMemory",
            proc: unsafe {
                std::mem::transmute(Self::bind_buffer_memory as vk::PFN_vkBindBufferMemory)
            },
        },
        VulkanCommand {
            name: "vkBindBufferMemory2",
            proc: unsafe {
                std::mem::transmute(Self::bind_buffer_memory2 as vk::PFN_vkBindBufferMemory2)
            },
        },
        VulkanCommand {
            name: "vkBindBufferMemory2KHR",
            proc: unsafe {
                std::mem::transmute(Self::bind_buffer_memory2 as vk::PFN_vkBindBufferMemory2)
            },
        },
        VulkanCommand {
            name: "vkBindImageMemory",
            proc: unsafe {
                std::mem::transmute(Self::bind_image_memory as vk::PFN_vkBindImageMemory)
            },
        },
        VulkanCommand {
            name: "vkBindImageMemory2",
            proc: unsafe {
                std::mem::transmute(Self::bind_image_memory2 as vk::PFN_vkBindImageMemory2)
            },
        },
        VulkanCommand {
            name: "vkBindImageMemory2KHR",
            proc: unsafe {
                std::mem::transmute(Self::bind_image_memory2 as vk::PFN_vkBindImageMemory2)
            },
        },
        VulkanCommand {
            name: "vkBindOpticalFlowSessionImageNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::bind_optical_flow_session_image_nv
                        as vk::PFN_vkBindOpticalFlowSessionImageNV,
                )
            },
        },
        VulkanCommand {
            name: "vkBindVideoSessionMemoryKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::bind_video_session_memory_khr as vk::PFN_vkBindVideoSessionMemoryKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkBuildMicromapsEXT",
            proc: unsafe {
                std::mem::transmute(Self::build_micromaps_ext as vk::PFN_vkBuildMicromapsEXT)
            },
        },
        VulkanCommand {
            name: "vkCmdBeginConditionalRenderingEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_begin_conditional_rendering_ext
                        as vk::PFN_vkCmdBeginConditionalRenderingEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBeginDebugUtilsLabelEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_begin_debug_utils_label_ext as vk::PFN_vkCmdBeginDebugUtilsLabelEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBeginQuery",
            proc: unsafe { std::mem::transmute(Self::cmd_begin_query as vk::PFN_vkCmdBeginQuery) },
        },
        VulkanCommand {
            name: "vkCmdBeginQueryIndexedEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_begin_query_indexed_ext as vk::PFN_vkCmdBeginQueryIndexedEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBeginRenderPass",
            proc: unsafe {
                std::mem::transmute(Self::cmd_begin_render_pass as vk::PFN_vkCmdBeginRenderPass)
            },
        },
        VulkanCommand {
            name: "vkCmdBeginRenderPass2",
            proc: unsafe {
                std::mem::transmute(Self::cmd_begin_render_pass2 as vk::PFN_vkCmdBeginRenderPass2)
            },
        },
        VulkanCommand {
            name: "vkCmdBeginRenderPass2KHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_begin_render_pass2 as vk::PFN_vkCmdBeginRenderPass2)
            },
        },
        VulkanCommand {
            name: "vkCmdBeginRendering",
            proc: unsafe {
                std::mem::transmute(Self::cmd_begin_rendering as vk::PFN_vkCmdBeginRendering)
            },
        },
        VulkanCommand {
            name: "vkCmdBeginRenderingKHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_begin_rendering as vk::PFN_vkCmdBeginRendering)
            },
        },
        VulkanCommand {
            name: "vkCmdBeginTransformFeedbackEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_begin_transform_feedback_ext
                        as vk::PFN_vkCmdBeginTransformFeedbackEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBeginVideoCodingKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_begin_video_coding_khr as vk::PFN_vkCmdBeginVideoCodingKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBindDescriptorBufferEmbeddedSamplersEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_bind_descriptor_buffer_embedded_samplers_ext
                        as vk::PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBindDescriptorBuffersEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_bind_descriptor_buffers_ext as vk::PFN_vkCmdBindDescriptorBuffersEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBindDescriptorSets",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_bind_descriptor_sets as vk::PFN_vkCmdBindDescriptorSets,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBindIndexBuffer",
            proc: unsafe {
                std::mem::transmute(Self::cmd_bind_index_buffer as vk::PFN_vkCmdBindIndexBuffer)
            },
        },
        VulkanCommand {
            name: "vkCmdBindInvocationMaskHUAWEI",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_bind_invocation_mask_huawei as vk::PFN_vkCmdBindInvocationMaskHUAWEI,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBindPipeline",
            proc: unsafe {
                std::mem::transmute(Self::cmd_bind_pipeline as vk::PFN_vkCmdBindPipeline)
            },
        },
        VulkanCommand {
            name: "vkCmdBindPipelineShaderGroupNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_bind_pipeline_shader_group_nv
                        as vk::PFN_vkCmdBindPipelineShaderGroupNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBindShadingRateImageNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_bind_shading_rate_image_nv as vk::PFN_vkCmdBindShadingRateImageNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBindTransformFeedbackBuffersEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_bind_transform_feedback_buffers_ext
                        as vk::PFN_vkCmdBindTransformFeedbackBuffersEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBindVertexBuffers",
            proc: unsafe {
                std::mem::transmute(Self::cmd_bind_vertex_buffers as vk::PFN_vkCmdBindVertexBuffers)
            },
        },
        VulkanCommand {
            name: "vkCmdBindVertexBuffers2",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_bind_vertex_buffers2 as vk::PFN_vkCmdBindVertexBuffers2,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBindVertexBuffers2EXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_bind_vertex_buffers2 as vk::PFN_vkCmdBindVertexBuffers2,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBlitImage",
            proc: unsafe { std::mem::transmute(Self::cmd_blit_image as vk::PFN_vkCmdBlitImage) },
        },
        VulkanCommand {
            name: "vkCmdBlitImage2",
            proc: unsafe { std::mem::transmute(Self::cmd_blit_image2 as vk::PFN_vkCmdBlitImage2) },
        },
        VulkanCommand {
            name: "vkCmdBlitImage2KHR",
            proc: unsafe { std::mem::transmute(Self::cmd_blit_image2 as vk::PFN_vkCmdBlitImage2) },
        },
        VulkanCommand {
            name: "vkCmdBuildAccelerationStructureNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_build_acceleration_structure_nv
                        as vk::PFN_vkCmdBuildAccelerationStructureNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdBuildMicromapsEXT",
            proc: unsafe {
                std::mem::transmute(Self::cmd_build_micromaps_ext as vk::PFN_vkCmdBuildMicromapsEXT)
            },
        },
        VulkanCommand {
            name: "vkCmdClearAttachments",
            proc: unsafe {
                std::mem::transmute(Self::cmd_clear_attachments as vk::PFN_vkCmdClearAttachments)
            },
        },
        VulkanCommand {
            name: "vkCmdClearColorImage",
            proc: unsafe {
                std::mem::transmute(Self::cmd_clear_color_image as vk::PFN_vkCmdClearColorImage)
            },
        },
        VulkanCommand {
            name: "vkCmdClearDepthStencilImage",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_clear_depth_stencil_image as vk::PFN_vkCmdClearDepthStencilImage,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdControlVideoCodingKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_control_video_coding_khr as vk::PFN_vkCmdControlVideoCodingKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyAccelerationStructureKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_acceleration_structure_khr
                        as vk::PFN_vkCmdCopyAccelerationStructureKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyAccelerationStructureNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_acceleration_structure_nv
                        as vk::PFN_vkCmdCopyAccelerationStructureNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyAccelerationStructureToMemoryKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_acceleration_structure_to_memory_khr
                        as vk::PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyBuffer",
            proc: unsafe { std::mem::transmute(Self::cmd_copy_buffer as vk::PFN_vkCmdCopyBuffer) },
        },
        VulkanCommand {
            name: "vkCmdCopyBuffer2",
            proc: unsafe {
                std::mem::transmute(Self::cmd_copy_buffer2 as vk::PFN_vkCmdCopyBuffer2)
            },
        },
        VulkanCommand {
            name: "vkCmdCopyBuffer2KHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_copy_buffer2 as vk::PFN_vkCmdCopyBuffer2)
            },
        },
        VulkanCommand {
            name: "vkCmdCopyBufferToImage",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_buffer_to_image as vk::PFN_vkCmdCopyBufferToImage,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyBufferToImage2",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_buffer_to_image2 as vk::PFN_vkCmdCopyBufferToImage2,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyBufferToImage2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_buffer_to_image2 as vk::PFN_vkCmdCopyBufferToImage2,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyImage",
            proc: unsafe { std::mem::transmute(Self::cmd_copy_image as vk::PFN_vkCmdCopyImage) },
        },
        VulkanCommand {
            name: "vkCmdCopyImage2",
            proc: unsafe { std::mem::transmute(Self::cmd_copy_image2 as vk::PFN_vkCmdCopyImage2) },
        },
        VulkanCommand {
            name: "vkCmdCopyImage2KHR",
            proc: unsafe { std::mem::transmute(Self::cmd_copy_image2 as vk::PFN_vkCmdCopyImage2) },
        },
        VulkanCommand {
            name: "vkCmdCopyImageToBuffer",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_image_to_buffer as vk::PFN_vkCmdCopyImageToBuffer,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyImageToBuffer2",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_image_to_buffer2 as vk::PFN_vkCmdCopyImageToBuffer2,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyImageToBuffer2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_image_to_buffer2 as vk::PFN_vkCmdCopyImageToBuffer2,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyMemoryIndirectNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_memory_indirect_nv as vk::PFN_vkCmdCopyMemoryIndirectNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyMemoryToAccelerationStructureKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_memory_to_acceleration_structure_khr
                        as vk::PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyMemoryToImageIndirectNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_memory_to_image_indirect_nv
                        as vk::PFN_vkCmdCopyMemoryToImageIndirectNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyMemoryToMicromapEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_memory_to_micromap_ext as vk::PFN_vkCmdCopyMemoryToMicromapEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyMicromapEXT",
            proc: unsafe {
                std::mem::transmute(Self::cmd_copy_micromap_ext as vk::PFN_vkCmdCopyMicromapEXT)
            },
        },
        VulkanCommand {
            name: "vkCmdCopyMicromapToMemoryEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_micromap_to_memory_ext as vk::PFN_vkCmdCopyMicromapToMemoryEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCopyQueryPoolResults",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_copy_query_pool_results as vk::PFN_vkCmdCopyQueryPoolResults,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdCuLaunchKernelNVX",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_cu_launch_kernel_nvx as vk::PFN_vkCmdCuLaunchKernelNVX,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDebugMarkerBeginEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_debug_marker_begin_ext as vk::PFN_vkCmdDebugMarkerBeginEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDebugMarkerEndEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_debug_marker_end_ext as vk::PFN_vkCmdDebugMarkerEndEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDebugMarkerInsertEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_debug_marker_insert_ext as vk::PFN_vkCmdDebugMarkerInsertEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDecodeVideoKHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_decode_video_khr as vk::PFN_vkCmdDecodeVideoKHR)
            },
        },
        VulkanCommand {
            name: "vkCmdDecompressMemoryIndirectCountNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_decompress_memory_indirect_count_nv
                        as vk::PFN_vkCmdDecompressMemoryIndirectCountNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDecompressMemoryNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_decompress_memory_nv as vk::PFN_vkCmdDecompressMemoryNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDispatch",
            proc: unsafe { std::mem::transmute(Self::cmd_dispatch as vk::PFN_vkCmdDispatch) },
        },
        VulkanCommand {
            name: "vkCmdDispatchBase",
            proc: unsafe {
                std::mem::transmute(Self::cmd_dispatch_base as vk::PFN_vkCmdDispatchBase)
            },
        },
        VulkanCommand {
            name: "vkCmdDispatchBaseKHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_dispatch_base as vk::PFN_vkCmdDispatchBase)
            },
        },
        VulkanCommand {
            name: "vkCmdDispatchIndirect",
            proc: unsafe {
                std::mem::transmute(Self::cmd_dispatch_indirect as vk::PFN_vkCmdDispatchIndirect)
            },
        },
        VulkanCommand {
            name: "vkCmdDraw",
            proc: unsafe { std::mem::transmute(Self::cmd_draw as vk::PFN_vkCmdDraw) },
        },
        VulkanCommand {
            name: "vkCmdDrawIndexed",
            proc: unsafe {
                std::mem::transmute(Self::cmd_draw_indexed as vk::PFN_vkCmdDrawIndexed)
            },
        },
        VulkanCommand {
            name: "vkCmdDrawIndexedIndirect",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_draw_indexed_indirect as vk::PFN_vkCmdDrawIndexedIndirect,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDrawIndexedIndirectCount",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_draw_indexed_indirect_count as vk::PFN_vkCmdDrawIndexedIndirectCount,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDrawIndexedIndirectCountAMD",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_draw_indexed_indirect_count as vk::PFN_vkCmdDrawIndexedIndirectCount,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDrawIndexedIndirectCountKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_draw_indexed_indirect_count as vk::PFN_vkCmdDrawIndexedIndirectCount,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDrawIndirect",
            proc: unsafe {
                std::mem::transmute(Self::cmd_draw_indirect as vk::PFN_vkCmdDrawIndirect)
            },
        },
        VulkanCommand {
            name: "vkCmdDrawIndirectByteCountEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_draw_indirect_byte_count_ext as vk::PFN_vkCmdDrawIndirectByteCountEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDrawIndirectCount",
            proc: unsafe {
                std::mem::transmute(Self::cmd_draw_indirect_count as vk::PFN_vkCmdDrawIndirectCount)
            },
        },
        VulkanCommand {
            name: "vkCmdDrawIndirectCountAMD",
            proc: unsafe {
                std::mem::transmute(Self::cmd_draw_indirect_count as vk::PFN_vkCmdDrawIndirectCount)
            },
        },
        VulkanCommand {
            name: "vkCmdDrawIndirectCountKHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_draw_indirect_count as vk::PFN_vkCmdDrawIndirectCount)
            },
        },
        VulkanCommand {
            name: "vkCmdDrawMeshTasksEXT",
            proc: unsafe {
                std::mem::transmute(Self::cmd_draw_mesh_tasks_ext as vk::PFN_vkCmdDrawMeshTasksEXT)
            },
        },
        VulkanCommand {
            name: "vkCmdDrawMeshTasksIndirectCountEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_draw_mesh_tasks_indirect_count_ext
                        as vk::PFN_vkCmdDrawMeshTasksIndirectCountEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDrawMeshTasksIndirectCountNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_draw_mesh_tasks_indirect_count_nv
                        as vk::PFN_vkCmdDrawMeshTasksIndirectCountNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDrawMeshTasksIndirectEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_draw_mesh_tasks_indirect_ext as vk::PFN_vkCmdDrawMeshTasksIndirectEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDrawMeshTasksIndirectNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_draw_mesh_tasks_indirect_nv as vk::PFN_vkCmdDrawMeshTasksIndirectNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdDrawMeshTasksNV",
            proc: unsafe {
                std::mem::transmute(Self::cmd_draw_mesh_tasks_nv as vk::PFN_vkCmdDrawMeshTasksNV)
            },
        },
        VulkanCommand {
            name: "vkCmdDrawMultiEXT",
            proc: unsafe {
                std::mem::transmute(Self::cmd_draw_multi_ext as vk::PFN_vkCmdDrawMultiEXT)
            },
        },
        VulkanCommand {
            name: "vkCmdDrawMultiIndexedEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_draw_multi_indexed_ext as vk::PFN_vkCmdDrawMultiIndexedEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdEncodeVideoKHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_encode_video_khr as vk::PFN_vkCmdEncodeVideoKHR)
            },
        },
        VulkanCommand {
            name: "vkCmdEndConditionalRenderingEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_end_conditional_rendering_ext
                        as vk::PFN_vkCmdEndConditionalRenderingEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdEndDebugUtilsLabelEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_end_debug_utils_label_ext as vk::PFN_vkCmdEndDebugUtilsLabelEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdEndQuery",
            proc: unsafe { std::mem::transmute(Self::cmd_end_query as vk::PFN_vkCmdEndQuery) },
        },
        VulkanCommand {
            name: "vkCmdEndQueryIndexedEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_end_query_indexed_ext as vk::PFN_vkCmdEndQueryIndexedEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdEndRenderPass",
            proc: unsafe {
                std::mem::transmute(Self::cmd_end_render_pass as vk::PFN_vkCmdEndRenderPass)
            },
        },
        VulkanCommand {
            name: "vkCmdEndRenderPass2",
            proc: unsafe {
                std::mem::transmute(Self::cmd_end_render_pass2 as vk::PFN_vkCmdEndRenderPass2)
            },
        },
        VulkanCommand {
            name: "vkCmdEndRenderPass2KHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_end_render_pass2 as vk::PFN_vkCmdEndRenderPass2)
            },
        },
        VulkanCommand {
            name: "vkCmdEndRendering",
            proc: unsafe {
                std::mem::transmute(Self::cmd_end_rendering as vk::PFN_vkCmdEndRendering)
            },
        },
        VulkanCommand {
            name: "vkCmdEndRenderingKHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_end_rendering as vk::PFN_vkCmdEndRendering)
            },
        },
        VulkanCommand {
            name: "vkCmdEndTransformFeedbackEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_end_transform_feedback_ext as vk::PFN_vkCmdEndTransformFeedbackEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdEndVideoCodingKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_end_video_coding_khr as vk::PFN_vkCmdEndVideoCodingKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdExecuteCommands",
            proc: unsafe {
                std::mem::transmute(Self::cmd_execute_commands as vk::PFN_vkCmdExecuteCommands)
            },
        },
        VulkanCommand {
            name: "vkCmdExecuteGeneratedCommandsNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_execute_generated_commands_nv
                        as vk::PFN_vkCmdExecuteGeneratedCommandsNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdFillBuffer",
            proc: unsafe { std::mem::transmute(Self::cmd_fill_buffer as vk::PFN_vkCmdFillBuffer) },
        },
        VulkanCommand {
            name: "vkCmdInsertDebugUtilsLabelEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_insert_debug_utils_label_ext as vk::PFN_vkCmdInsertDebugUtilsLabelEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdNextSubpass",
            proc: unsafe {
                std::mem::transmute(Self::cmd_next_subpass as vk::PFN_vkCmdNextSubpass)
            },
        },
        VulkanCommand {
            name: "vkCmdNextSubpass2",
            proc: unsafe {
                std::mem::transmute(Self::cmd_next_subpass2 as vk::PFN_vkCmdNextSubpass2)
            },
        },
        VulkanCommand {
            name: "vkCmdNextSubpass2KHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_next_subpass2 as vk::PFN_vkCmdNextSubpass2)
            },
        },
        VulkanCommand {
            name: "vkCmdOpticalFlowExecuteNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_optical_flow_execute_nv as vk::PFN_vkCmdOpticalFlowExecuteNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdPipelineBarrier",
            proc: unsafe {
                std::mem::transmute(Self::cmd_pipeline_barrier as vk::PFN_vkCmdPipelineBarrier)
            },
        },
        VulkanCommand {
            name: "vkCmdPipelineBarrier2",
            proc: unsafe {
                std::mem::transmute(Self::cmd_pipeline_barrier2 as vk::PFN_vkCmdPipelineBarrier2)
            },
        },
        VulkanCommand {
            name: "vkCmdPipelineBarrier2KHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_pipeline_barrier2 as vk::PFN_vkCmdPipelineBarrier2)
            },
        },
        VulkanCommand {
            name: "vkCmdPreprocessGeneratedCommandsNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_preprocess_generated_commands_nv
                        as vk::PFN_vkCmdPreprocessGeneratedCommandsNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdPushConstants",
            proc: unsafe {
                std::mem::transmute(Self::cmd_push_constants as vk::PFN_vkCmdPushConstants)
            },
        },
        VulkanCommand {
            name: "vkCmdPushDescriptorSetKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_push_descriptor_set_khr as vk::PFN_vkCmdPushDescriptorSetKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdPushDescriptorSetWithTemplateKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_push_descriptor_set_with_template_khr
                        as vk::PFN_vkCmdPushDescriptorSetWithTemplateKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdResetEvent",
            proc: unsafe { std::mem::transmute(Self::cmd_reset_event as vk::PFN_vkCmdResetEvent) },
        },
        VulkanCommand {
            name: "vkCmdResetEvent2",
            proc: unsafe {
                std::mem::transmute(Self::cmd_reset_event2 as vk::PFN_vkCmdResetEvent2)
            },
        },
        VulkanCommand {
            name: "vkCmdResetEvent2KHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_reset_event2 as vk::PFN_vkCmdResetEvent2)
            },
        },
        VulkanCommand {
            name: "vkCmdResetQueryPool",
            proc: unsafe {
                std::mem::transmute(Self::cmd_reset_query_pool as vk::PFN_vkCmdResetQueryPool)
            },
        },
        VulkanCommand {
            name: "vkCmdResolveImage",
            proc: unsafe {
                std::mem::transmute(Self::cmd_resolve_image as vk::PFN_vkCmdResolveImage)
            },
        },
        VulkanCommand {
            name: "vkCmdResolveImage2",
            proc: unsafe {
                std::mem::transmute(Self::cmd_resolve_image2 as vk::PFN_vkCmdResolveImage2)
            },
        },
        VulkanCommand {
            name: "vkCmdResolveImage2KHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_resolve_image2 as vk::PFN_vkCmdResolveImage2)
            },
        },
        VulkanCommand {
            name: "vkCmdSetAlphaToCoverageEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_alpha_to_coverage_enable_ext
                        as vk::PFN_vkCmdSetAlphaToCoverageEnableEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetAlphaToOneEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_alpha_to_one_enable_ext as vk::PFN_vkCmdSetAlphaToOneEnableEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetBlendConstants",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_blend_constants as vk::PFN_vkCmdSetBlendConstants)
            },
        },
        VulkanCommand {
            name: "vkCmdSetCheckpointNV",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_checkpoint_nv as vk::PFN_vkCmdSetCheckpointNV)
            },
        },
        VulkanCommand {
            name: "vkCmdSetCoarseSampleOrderNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_coarse_sample_order_nv as vk::PFN_vkCmdSetCoarseSampleOrderNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetColorBlendAdvancedEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_color_blend_advanced_ext as vk::PFN_vkCmdSetColorBlendAdvancedEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetColorBlendEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_color_blend_enable_ext as vk::PFN_vkCmdSetColorBlendEnableEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetColorBlendEquationEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_color_blend_equation_ext as vk::PFN_vkCmdSetColorBlendEquationEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetColorWriteEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_color_write_enable_ext as vk::PFN_vkCmdSetColorWriteEnableEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetColorWriteMaskEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_color_write_mask_ext as vk::PFN_vkCmdSetColorWriteMaskEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetConservativeRasterizationModeEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_conservative_rasterization_mode_ext
                        as vk::PFN_vkCmdSetConservativeRasterizationModeEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetCoverageModulationModeNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_coverage_modulation_mode_nv
                        as vk::PFN_vkCmdSetCoverageModulationModeNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetCoverageModulationTableEnableNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_coverage_modulation_table_enable_nv
                        as vk::PFN_vkCmdSetCoverageModulationTableEnableNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetCoverageModulationTableNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_coverage_modulation_table_nv
                        as vk::PFN_vkCmdSetCoverageModulationTableNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetCoverageReductionModeNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_coverage_reduction_mode_nv
                        as vk::PFN_vkCmdSetCoverageReductionModeNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetCoverageToColorEnableNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_coverage_to_color_enable_nv
                        as vk::PFN_vkCmdSetCoverageToColorEnableNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetCoverageToColorLocationNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_coverage_to_color_location_nv
                        as vk::PFN_vkCmdSetCoverageToColorLocationNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetCullMode",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_cull_mode as vk::PFN_vkCmdSetCullMode)
            },
        },
        VulkanCommand {
            name: "vkCmdSetCullModeEXT",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_cull_mode as vk::PFN_vkCmdSetCullMode)
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthBias",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_depth_bias as vk::PFN_vkCmdSetDepthBias)
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthBiasEnable",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_bias_enable as vk::PFN_vkCmdSetDepthBiasEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthBiasEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_bias_enable as vk::PFN_vkCmdSetDepthBiasEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthBounds",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_depth_bounds as vk::PFN_vkCmdSetDepthBounds)
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthBoundsTestEnable",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_bounds_test_enable as vk::PFN_vkCmdSetDepthBoundsTestEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthBoundsTestEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_bounds_test_enable as vk::PFN_vkCmdSetDepthBoundsTestEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthClampEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_clamp_enable_ext as vk::PFN_vkCmdSetDepthClampEnableEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthClipEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_clip_enable_ext as vk::PFN_vkCmdSetDepthClipEnableEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthClipNegativeOneToOneEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_clip_negative_one_to_one_ext
                        as vk::PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthCompareOp",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_compare_op as vk::PFN_vkCmdSetDepthCompareOp,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthCompareOpEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_compare_op as vk::PFN_vkCmdSetDepthCompareOp,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthTestEnable",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_test_enable as vk::PFN_vkCmdSetDepthTestEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthTestEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_test_enable as vk::PFN_vkCmdSetDepthTestEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthWriteEnable",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_write_enable as vk::PFN_vkCmdSetDepthWriteEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDepthWriteEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_depth_write_enable as vk::PFN_vkCmdSetDepthWriteEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDescriptorBufferOffsetsEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_descriptor_buffer_offsets_ext
                        as vk::PFN_vkCmdSetDescriptorBufferOffsetsEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetDeviceMask",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_device_mask as vk::PFN_vkCmdSetDeviceMask)
            },
        },
        VulkanCommand {
            name: "vkCmdSetDeviceMaskKHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_device_mask as vk::PFN_vkCmdSetDeviceMask)
            },
        },
        VulkanCommand {
            name: "vkCmdSetDiscardRectangleEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_discard_rectangle_ext as vk::PFN_vkCmdSetDiscardRectangleEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetEvent",
            proc: unsafe { std::mem::transmute(Self::cmd_set_event as vk::PFN_vkCmdSetEvent) },
        },
        VulkanCommand {
            name: "vkCmdSetEvent2",
            proc: unsafe { std::mem::transmute(Self::cmd_set_event2 as vk::PFN_vkCmdSetEvent2) },
        },
        VulkanCommand {
            name: "vkCmdSetEvent2KHR",
            proc: unsafe { std::mem::transmute(Self::cmd_set_event2 as vk::PFN_vkCmdSetEvent2) },
        },
        VulkanCommand {
            name: "vkCmdSetExclusiveScissorNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_exclusive_scissor_nv as vk::PFN_vkCmdSetExclusiveScissorNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetExtraPrimitiveOverestimationSizeEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_extra_primitive_overestimation_size_ext
                        as vk::PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetFragmentShadingRateEnumNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_fragment_shading_rate_enum_nv
                        as vk::PFN_vkCmdSetFragmentShadingRateEnumNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetFragmentShadingRateKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_fragment_shading_rate_khr
                        as vk::PFN_vkCmdSetFragmentShadingRateKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetFrontFace",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_front_face as vk::PFN_vkCmdSetFrontFace)
            },
        },
        VulkanCommand {
            name: "vkCmdSetFrontFaceEXT",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_front_face as vk::PFN_vkCmdSetFrontFace)
            },
        },
        VulkanCommand {
            name: "vkCmdSetLineRasterizationModeEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_line_rasterization_mode_ext
                        as vk::PFN_vkCmdSetLineRasterizationModeEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetLineStippleEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_line_stipple_ext as vk::PFN_vkCmdSetLineStippleEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetLineStippleEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_line_stipple_enable_ext as vk::PFN_vkCmdSetLineStippleEnableEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetLineWidth",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_line_width as vk::PFN_vkCmdSetLineWidth)
            },
        },
        VulkanCommand {
            name: "vkCmdSetLogicOpEXT",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_logic_op_ext as vk::PFN_vkCmdSetLogicOpEXT)
            },
        },
        VulkanCommand {
            name: "vkCmdSetLogicOpEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_logic_op_enable_ext as vk::PFN_vkCmdSetLogicOpEnableEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetPatchControlPointsEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_patch_control_points_ext as vk::PFN_vkCmdSetPatchControlPointsEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetPerformanceMarkerINTEL",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_performance_marker_intel
                        as vk::PFN_vkCmdSetPerformanceMarkerINTEL,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetPerformanceOverrideINTEL",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_performance_override_intel
                        as vk::PFN_vkCmdSetPerformanceOverrideINTEL,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetPerformanceStreamMarkerINTEL",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_performance_stream_marker_intel
                        as vk::PFN_vkCmdSetPerformanceStreamMarkerINTEL,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetPolygonModeEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_polygon_mode_ext as vk::PFN_vkCmdSetPolygonModeEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetPrimitiveRestartEnable",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_primitive_restart_enable
                        as vk::PFN_vkCmdSetPrimitiveRestartEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetPrimitiveRestartEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_primitive_restart_enable
                        as vk::PFN_vkCmdSetPrimitiveRestartEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetPrimitiveTopology",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_primitive_topology as vk::PFN_vkCmdSetPrimitiveTopology,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetPrimitiveTopologyEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_primitive_topology as vk::PFN_vkCmdSetPrimitiveTopology,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetProvokingVertexModeEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_provoking_vertex_mode_ext
                        as vk::PFN_vkCmdSetProvokingVertexModeEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetRasterizationSamplesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_rasterization_samples_ext
                        as vk::PFN_vkCmdSetRasterizationSamplesEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetRasterizationStreamEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_rasterization_stream_ext
                        as vk::PFN_vkCmdSetRasterizationStreamEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetRasterizerDiscardEnable",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_rasterizer_discard_enable
                        as vk::PFN_vkCmdSetRasterizerDiscardEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetRasterizerDiscardEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_rasterizer_discard_enable
                        as vk::PFN_vkCmdSetRasterizerDiscardEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetRayTracingPipelineStackSizeKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_ray_tracing_pipeline_stack_size_khr
                        as vk::PFN_vkCmdSetRayTracingPipelineStackSizeKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetRepresentativeFragmentTestEnableNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_representative_fragment_test_enable_nv
                        as vk::PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetSampleLocationsEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_sample_locations_ext as vk::PFN_vkCmdSetSampleLocationsEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetSampleLocationsEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_sample_locations_enable_ext
                        as vk::PFN_vkCmdSetSampleLocationsEnableEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetSampleMaskEXT",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_sample_mask_ext as vk::PFN_vkCmdSetSampleMaskEXT)
            },
        },
        VulkanCommand {
            name: "vkCmdSetScissor",
            proc: unsafe { std::mem::transmute(Self::cmd_set_scissor as vk::PFN_vkCmdSetScissor) },
        },
        VulkanCommand {
            name: "vkCmdSetScissorWithCount",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_scissor_with_count as vk::PFN_vkCmdSetScissorWithCount,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetScissorWithCountEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_scissor_with_count as vk::PFN_vkCmdSetScissorWithCount,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetShadingRateImageEnableNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_shading_rate_image_enable_nv
                        as vk::PFN_vkCmdSetShadingRateImageEnableNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetStencilCompareMask",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_stencil_compare_mask as vk::PFN_vkCmdSetStencilCompareMask,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetStencilOp",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_stencil_op as vk::PFN_vkCmdSetStencilOp)
            },
        },
        VulkanCommand {
            name: "vkCmdSetStencilOpEXT",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_stencil_op as vk::PFN_vkCmdSetStencilOp)
            },
        },
        VulkanCommand {
            name: "vkCmdSetStencilReference",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_stencil_reference as vk::PFN_vkCmdSetStencilReference,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetStencilTestEnable",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_stencil_test_enable as vk::PFN_vkCmdSetStencilTestEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetStencilTestEnableEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_stencil_test_enable as vk::PFN_vkCmdSetStencilTestEnable,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetStencilWriteMask",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_stencil_write_mask as vk::PFN_vkCmdSetStencilWriteMask,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetTessellationDomainOriginEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_tessellation_domain_origin_ext
                        as vk::PFN_vkCmdSetTessellationDomainOriginEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetVertexInputEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_vertex_input_ext as vk::PFN_vkCmdSetVertexInputEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetViewport",
            proc: unsafe {
                std::mem::transmute(Self::cmd_set_viewport as vk::PFN_vkCmdSetViewport)
            },
        },
        VulkanCommand {
            name: "vkCmdSetViewportShadingRatePaletteNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_viewport_shading_rate_palette_nv
                        as vk::PFN_vkCmdSetViewportShadingRatePaletteNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetViewportSwizzleNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_viewport_swizzle_nv as vk::PFN_vkCmdSetViewportSwizzleNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetViewportWScalingEnableNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_viewport_w_scaling_enable_nv
                        as vk::PFN_vkCmdSetViewportWScalingEnableNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetViewportWScalingNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_viewport_w_scaling_nv as vk::PFN_vkCmdSetViewportWScalingNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetViewportWithCount",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_viewport_with_count as vk::PFN_vkCmdSetViewportWithCount,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSetViewportWithCountEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_set_viewport_with_count as vk::PFN_vkCmdSetViewportWithCount,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdSubpassShadingHUAWEI",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_subpass_shading_huawei as vk::PFN_vkCmdSubpassShadingHUAWEI,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdTraceRaysIndirect2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_trace_rays_indirect2_khr as vk::PFN_vkCmdTraceRaysIndirect2KHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdTraceRaysIndirectKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_trace_rays_indirect_khr as vk::PFN_vkCmdTraceRaysIndirectKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdTraceRaysKHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_trace_rays_khr as vk::PFN_vkCmdTraceRaysKHR)
            },
        },
        VulkanCommand {
            name: "vkCmdTraceRaysNV",
            proc: unsafe {
                std::mem::transmute(Self::cmd_trace_rays_nv as vk::PFN_vkCmdTraceRaysNV)
            },
        },
        VulkanCommand {
            name: "vkCmdUpdateBuffer",
            proc: unsafe {
                std::mem::transmute(Self::cmd_update_buffer as vk::PFN_vkCmdUpdateBuffer)
            },
        },
        VulkanCommand {
            name: "vkCmdWaitEvents",
            proc: unsafe { std::mem::transmute(Self::cmd_wait_events as vk::PFN_vkCmdWaitEvents) },
        },
        VulkanCommand {
            name: "vkCmdWaitEvents2",
            proc: unsafe {
                std::mem::transmute(Self::cmd_wait_events2 as vk::PFN_vkCmdWaitEvents2)
            },
        },
        VulkanCommand {
            name: "vkCmdWaitEvents2KHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_wait_events2 as vk::PFN_vkCmdWaitEvents2)
            },
        },
        VulkanCommand {
            name: "vkCmdWriteAccelerationStructuresPropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_write_acceleration_structures_properties_khr
                        as vk::PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdWriteAccelerationStructuresPropertiesNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_write_acceleration_structures_properties_nv
                        as vk::PFN_vkCmdWriteAccelerationStructuresPropertiesNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdWriteBufferMarker2AMD",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_write_buffer_marker2_amd as vk::PFN_vkCmdWriteBufferMarker2AMD,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdWriteBufferMarkerAMD",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_write_buffer_marker_amd as vk::PFN_vkCmdWriteBufferMarkerAMD,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdWriteMicromapsPropertiesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::cmd_write_micromaps_properties_ext
                        as vk::PFN_vkCmdWriteMicromapsPropertiesEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCmdWriteTimestamp",
            proc: unsafe {
                std::mem::transmute(Self::cmd_write_timestamp as vk::PFN_vkCmdWriteTimestamp)
            },
        },
        VulkanCommand {
            name: "vkCmdWriteTimestamp2",
            proc: unsafe {
                std::mem::transmute(Self::cmd_write_timestamp2 as vk::PFN_vkCmdWriteTimestamp2)
            },
        },
        VulkanCommand {
            name: "vkCmdWriteTimestamp2KHR",
            proc: unsafe {
                std::mem::transmute(Self::cmd_write_timestamp2 as vk::PFN_vkCmdWriteTimestamp2)
            },
        },
        VulkanCommand {
            name: "vkCompileDeferredNV",
            proc: unsafe {
                std::mem::transmute(Self::compile_deferred_nv as vk::PFN_vkCompileDeferredNV)
            },
        },
        VulkanCommand {
            name: "vkCopyAccelerationStructureKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::copy_acceleration_structure_khr as vk::PFN_vkCopyAccelerationStructureKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCopyAccelerationStructureToMemoryKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::copy_acceleration_structure_to_memory_khr
                        as vk::PFN_vkCopyAccelerationStructureToMemoryKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCopyMemoryToAccelerationStructureKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::copy_memory_to_acceleration_structure_khr
                        as vk::PFN_vkCopyMemoryToAccelerationStructureKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCopyMemoryToMicromapEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::copy_memory_to_micromap_ext as vk::PFN_vkCopyMemoryToMicromapEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCopyMicromapEXT",
            proc: unsafe {
                std::mem::transmute(Self::copy_micromap_ext as vk::PFN_vkCopyMicromapEXT)
            },
        },
        VulkanCommand {
            name: "vkCopyMicromapToMemoryEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::copy_micromap_to_memory_ext as vk::PFN_vkCopyMicromapToMemoryEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateAccelerationStructureKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_acceleration_structure_khr
                        as vk::PFN_vkCreateAccelerationStructureKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateAccelerationStructureNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_acceleration_structure_nv
                        as vk::PFN_vkCreateAccelerationStructureNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateBuffer",
            proc: unsafe { std::mem::transmute(Self::create_buffer as vk::PFN_vkCreateBuffer) },
        },
        VulkanCommand {
            name: "vkCreateBufferCollectionFUCHSIA",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_buffer_collection_fuchsia
                        as vk::PFN_vkCreateBufferCollectionFUCHSIA,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateBufferView",
            proc: unsafe {
                std::mem::transmute(Self::create_buffer_view as vk::PFN_vkCreateBufferView)
            },
        },
        VulkanCommand {
            name: "vkCreateCommandPool",
            proc: unsafe {
                std::mem::transmute(Self::create_command_pool as vk::PFN_vkCreateCommandPool)
            },
        },
        VulkanCommand {
            name: "vkCreateComputePipelines",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_compute_pipelines as vk::PFN_vkCreateComputePipelines,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateCuFunctionNVX",
            proc: unsafe {
                std::mem::transmute(Self::create_cu_function_nvx as vk::PFN_vkCreateCuFunctionNVX)
            },
        },
        VulkanCommand {
            name: "vkCreateCuModuleNVX",
            proc: unsafe {
                std::mem::transmute(Self::create_cu_module_nvx as vk::PFN_vkCreateCuModuleNVX)
            },
        },
        VulkanCommand {
            name: "vkCreateDeferredOperationKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_deferred_operation_khr as vk::PFN_vkCreateDeferredOperationKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateDescriptorPool",
            proc: unsafe {
                std::mem::transmute(Self::create_descriptor_pool as vk::PFN_vkCreateDescriptorPool)
            },
        },
        VulkanCommand {
            name: "vkCreateDescriptorSetLayout",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_descriptor_set_layout as vk::PFN_vkCreateDescriptorSetLayout,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateDescriptorUpdateTemplate",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_descriptor_update_template
                        as vk::PFN_vkCreateDescriptorUpdateTemplate,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateDescriptorUpdateTemplateKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_descriptor_update_template
                        as vk::PFN_vkCreateDescriptorUpdateTemplate,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateEvent",
            proc: unsafe { std::mem::transmute(Self::create_event as vk::PFN_vkCreateEvent) },
        },
        VulkanCommand {
            name: "vkCreateFence",
            proc: unsafe { std::mem::transmute(Self::create_fence as vk::PFN_vkCreateFence) },
        },
        VulkanCommand {
            name: "vkCreateFramebuffer",
            proc: unsafe {
                std::mem::transmute(Self::create_framebuffer as vk::PFN_vkCreateFramebuffer)
            },
        },
        VulkanCommand {
            name: "vkCreateGraphicsPipelines",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_graphics_pipelines as vk::PFN_vkCreateGraphicsPipelines,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateImage",
            proc: unsafe { std::mem::transmute(Self::create_image as vk::PFN_vkCreateImage) },
        },
        VulkanCommand {
            name: "vkCreateImageView",
            proc: unsafe {
                std::mem::transmute(Self::create_image_view as vk::PFN_vkCreateImageView)
            },
        },
        VulkanCommand {
            name: "vkCreateIndirectCommandsLayoutNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_indirect_commands_layout_nv
                        as vk::PFN_vkCreateIndirectCommandsLayoutNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateMicromapEXT",
            proc: unsafe {
                std::mem::transmute(Self::create_micromap_ext as vk::PFN_vkCreateMicromapEXT)
            },
        },
        VulkanCommand {
            name: "vkCreateOpticalFlowSessionNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_optical_flow_session_nv as vk::PFN_vkCreateOpticalFlowSessionNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCreatePipelineCache",
            proc: unsafe {
                std::mem::transmute(Self::create_pipeline_cache as vk::PFN_vkCreatePipelineCache)
            },
        },
        VulkanCommand {
            name: "vkCreatePipelineLayout",
            proc: unsafe {
                std::mem::transmute(Self::create_pipeline_layout as vk::PFN_vkCreatePipelineLayout)
            },
        },
        VulkanCommand {
            name: "vkCreatePrivateDataSlot",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_private_data_slot as vk::PFN_vkCreatePrivateDataSlot,
                )
            },
        },
        VulkanCommand {
            name: "vkCreatePrivateDataSlotEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_private_data_slot as vk::PFN_vkCreatePrivateDataSlot,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateQueryPool",
            proc: unsafe {
                std::mem::transmute(Self::create_query_pool as vk::PFN_vkCreateQueryPool)
            },
        },
        VulkanCommand {
            name: "vkCreateRayTracingPipelinesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_ray_tracing_pipelines_khr
                        as vk::PFN_vkCreateRayTracingPipelinesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateRayTracingPipelinesNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_ray_tracing_pipelines_nv as vk::PFN_vkCreateRayTracingPipelinesNV,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateRenderPass",
            proc: unsafe {
                std::mem::transmute(Self::create_render_pass as vk::PFN_vkCreateRenderPass)
            },
        },
        VulkanCommand {
            name: "vkCreateRenderPass2",
            proc: unsafe {
                std::mem::transmute(Self::create_render_pass2 as vk::PFN_vkCreateRenderPass2)
            },
        },
        VulkanCommand {
            name: "vkCreateRenderPass2KHR",
            proc: unsafe {
                std::mem::transmute(Self::create_render_pass2 as vk::PFN_vkCreateRenderPass2)
            },
        },
        VulkanCommand {
            name: "vkCreateSampler",
            proc: unsafe { std::mem::transmute(Self::create_sampler as vk::PFN_vkCreateSampler) },
        },
        VulkanCommand {
            name: "vkCreateSamplerYcbcrConversion",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_sampler_ycbcr_conversion as vk::PFN_vkCreateSamplerYcbcrConversion,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateSamplerYcbcrConversionKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_sampler_ycbcr_conversion as vk::PFN_vkCreateSamplerYcbcrConversion,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateSemaphore",
            proc: unsafe {
                std::mem::transmute(Self::create_semaphore as vk::PFN_vkCreateSemaphore)
            },
        },
        VulkanCommand {
            name: "vkCreateShaderModule",
            proc: unsafe {
                std::mem::transmute(Self::create_shader_module as vk::PFN_vkCreateShaderModule)
            },
        },
        VulkanCommand {
            name: "vkCreateSharedSwapchainsKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_shared_swapchains_khr as vk::PFN_vkCreateSharedSwapchainsKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateSwapchainKHR",
            proc: unsafe {
                std::mem::transmute(Self::create_swapchain_khr as vk::PFN_vkCreateSwapchainKHR)
            },
        },
        VulkanCommand {
            name: "vkCreateValidationCacheEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_validation_cache_ext as vk::PFN_vkCreateValidationCacheEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateVideoSessionKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_video_session_khr as vk::PFN_vkCreateVideoSessionKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateVideoSessionParametersKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_video_session_parameters_khr
                        as vk::PFN_vkCreateVideoSessionParametersKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkDebugMarkerSetObjectNameEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::debug_marker_set_object_name_ext as vk::PFN_vkDebugMarkerSetObjectNameEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkDebugMarkerSetObjectTagEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::debug_marker_set_object_tag_ext as vk::PFN_vkDebugMarkerSetObjectTagEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkDeferredOperationJoinKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::deferred_operation_join_khr as vk::PFN_vkDeferredOperationJoinKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyAccelerationStructureKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_acceleration_structure_khr
                        as vk::PFN_vkDestroyAccelerationStructureKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyAccelerationStructureNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_acceleration_structure_nv
                        as vk::PFN_vkDestroyAccelerationStructureNV,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyBuffer",
            proc: unsafe { std::mem::transmute(Self::destroy_buffer as vk::PFN_vkDestroyBuffer) },
        },
        VulkanCommand {
            name: "vkDestroyBufferCollectionFUCHSIA",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_buffer_collection_fuchsia
                        as vk::PFN_vkDestroyBufferCollectionFUCHSIA,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyBufferView",
            proc: unsafe {
                std::mem::transmute(Self::destroy_buffer_view as vk::PFN_vkDestroyBufferView)
            },
        },
        VulkanCommand {
            name: "vkDestroyCommandPool",
            proc: unsafe {
                std::mem::transmute(Self::destroy_command_pool as vk::PFN_vkDestroyCommandPool)
            },
        },
        VulkanCommand {
            name: "vkDestroyCuFunctionNVX",
            proc: unsafe {
                std::mem::transmute(Self::destroy_cu_function_nvx as vk::PFN_vkDestroyCuFunctionNVX)
            },
        },
        VulkanCommand {
            name: "vkDestroyCuModuleNVX",
            proc: unsafe {
                std::mem::transmute(Self::destroy_cu_module_nvx as vk::PFN_vkDestroyCuModuleNVX)
            },
        },
        VulkanCommand {
            name: "vkDestroyDeferredOperationKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_deferred_operation_khr as vk::PFN_vkDestroyDeferredOperationKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyDescriptorPool",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_descriptor_pool as vk::PFN_vkDestroyDescriptorPool,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyDescriptorSetLayout",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_descriptor_set_layout as vk::PFN_vkDestroyDescriptorSetLayout,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyDescriptorUpdateTemplate",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_descriptor_update_template
                        as vk::PFN_vkDestroyDescriptorUpdateTemplate,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyDescriptorUpdateTemplateKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_descriptor_update_template
                        as vk::PFN_vkDestroyDescriptorUpdateTemplate,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyDevice",
            proc: unsafe { std::mem::transmute(Self::destroy_device as vk::PFN_vkDestroyDevice) },
        },
        VulkanCommand {
            name: "vkDestroyEvent",
            proc: unsafe { std::mem::transmute(Self::destroy_event as vk::PFN_vkDestroyEvent) },
        },
        VulkanCommand {
            name: "vkDestroyFence",
            proc: unsafe { std::mem::transmute(Self::destroy_fence as vk::PFN_vkDestroyFence) },
        },
        VulkanCommand {
            name: "vkDestroyFramebuffer",
            proc: unsafe {
                std::mem::transmute(Self::destroy_framebuffer as vk::PFN_vkDestroyFramebuffer)
            },
        },
        VulkanCommand {
            name: "vkDestroyImage",
            proc: unsafe { std::mem::transmute(Self::destroy_image as vk::PFN_vkDestroyImage) },
        },
        VulkanCommand {
            name: "vkDestroyImageView",
            proc: unsafe {
                std::mem::transmute(Self::destroy_image_view as vk::PFN_vkDestroyImageView)
            },
        },
        VulkanCommand {
            name: "vkDestroyIndirectCommandsLayoutNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_indirect_commands_layout_nv
                        as vk::PFN_vkDestroyIndirectCommandsLayoutNV,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyMicromapEXT",
            proc: unsafe {
                std::mem::transmute(Self::destroy_micromap_ext as vk::PFN_vkDestroyMicromapEXT)
            },
        },
        VulkanCommand {
            name: "vkDestroyOpticalFlowSessionNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_optical_flow_session_nv as vk::PFN_vkDestroyOpticalFlowSessionNV,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyPipeline",
            proc: unsafe {
                std::mem::transmute(Self::destroy_pipeline as vk::PFN_vkDestroyPipeline)
            },
        },
        VulkanCommand {
            name: "vkDestroyPipelineCache",
            proc: unsafe {
                std::mem::transmute(Self::destroy_pipeline_cache as vk::PFN_vkDestroyPipelineCache)
            },
        },
        VulkanCommand {
            name: "vkDestroyPipelineLayout",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_pipeline_layout as vk::PFN_vkDestroyPipelineLayout,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyPrivateDataSlot",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_private_data_slot as vk::PFN_vkDestroyPrivateDataSlot,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyPrivateDataSlotEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_private_data_slot as vk::PFN_vkDestroyPrivateDataSlot,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyQueryPool",
            proc: unsafe {
                std::mem::transmute(Self::destroy_query_pool as vk::PFN_vkDestroyQueryPool)
            },
        },
        VulkanCommand {
            name: "vkDestroyRenderPass",
            proc: unsafe {
                std::mem::transmute(Self::destroy_render_pass as vk::PFN_vkDestroyRenderPass)
            },
        },
        VulkanCommand {
            name: "vkDestroySampler",
            proc: unsafe { std::mem::transmute(Self::destroy_sampler as vk::PFN_vkDestroySampler) },
        },
        VulkanCommand {
            name: "vkDestroySamplerYcbcrConversion",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_sampler_ycbcr_conversion
                        as vk::PFN_vkDestroySamplerYcbcrConversion,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroySamplerYcbcrConversionKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_sampler_ycbcr_conversion
                        as vk::PFN_vkDestroySamplerYcbcrConversion,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroySemaphore",
            proc: unsafe {
                std::mem::transmute(Self::destroy_semaphore as vk::PFN_vkDestroySemaphore)
            },
        },
        VulkanCommand {
            name: "vkDestroyShaderModule",
            proc: unsafe {
                std::mem::transmute(Self::destroy_shader_module as vk::PFN_vkDestroyShaderModule)
            },
        },
        VulkanCommand {
            name: "vkDestroySwapchainKHR",
            proc: unsafe {
                std::mem::transmute(Self::destroy_swapchain_khr as vk::PFN_vkDestroySwapchainKHR)
            },
        },
        VulkanCommand {
            name: "vkDestroyValidationCacheEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_validation_cache_ext as vk::PFN_vkDestroyValidationCacheEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyVideoSessionKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_video_session_khr as vk::PFN_vkDestroyVideoSessionKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyVideoSessionParametersKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_video_session_parameters_khr
                        as vk::PFN_vkDestroyVideoSessionParametersKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkDeviceWaitIdle",
            proc: unsafe {
                std::mem::transmute(Self::device_wait_idle as vk::PFN_vkDeviceWaitIdle)
            },
        },
        VulkanCommand {
            name: "vkDisplayPowerControlEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::display_power_control_ext as vk::PFN_vkDisplayPowerControlEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkEndCommandBuffer",
            proc: unsafe {
                std::mem::transmute(Self::end_command_buffer as vk::PFN_vkEndCommandBuffer)
            },
        },
        VulkanCommand {
            name: "vkExportMetalObjectsEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::export_metal_objects_ext as vk::PFN_vkExportMetalObjectsEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkFlushMappedMemoryRanges",
            proc: unsafe {
                std::mem::transmute(
                    Self::flush_mapped_memory_ranges as vk::PFN_vkFlushMappedMemoryRanges,
                )
            },
        },
        VulkanCommand {
            name: "vkFreeCommandBuffers",
            proc: unsafe {
                std::mem::transmute(Self::free_command_buffers as vk::PFN_vkFreeCommandBuffers)
            },
        },
        VulkanCommand {
            name: "vkFreeDescriptorSets",
            proc: unsafe {
                std::mem::transmute(Self::free_descriptor_sets as vk::PFN_vkFreeDescriptorSets)
            },
        },
        VulkanCommand {
            name: "vkFreeMemory",
            proc: unsafe { std::mem::transmute(Self::free_memory as vk::PFN_vkFreeMemory) },
        },
        VulkanCommand {
            name: "vkGetAccelerationStructureBuildSizesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_acceleration_structure_build_sizes_khr
                        as vk::PFN_vkGetAccelerationStructureBuildSizesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetAccelerationStructureDeviceAddressKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_acceleration_structure_device_address_khr
                        as vk::PFN_vkGetAccelerationStructureDeviceAddressKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetAccelerationStructureHandleNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_acceleration_structure_handle_nv
                        as vk::PFN_vkGetAccelerationStructureHandleNV,
                )
            },
        },
        VulkanCommand {
            name: "vkGetAccelerationStructureMemoryRequirementsNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_acceleration_structure_memory_requirements_nv
                        as vk::PFN_vkGetAccelerationStructureMemoryRequirementsNV,
                )
            },
        },
        VulkanCommand {
            name: "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_acceleration_structure_opaque_capture_descriptor_data_ext
                        as vk::PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetAndroidHardwareBufferPropertiesANDROID",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_android_hardware_buffer_properties_android
                        as vk::PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
                )
            },
        },
        VulkanCommand {
            name: "vkGetBufferCollectionPropertiesFUCHSIA",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_buffer_collection_properties_fuchsia
                        as vk::PFN_vkGetBufferCollectionPropertiesFUCHSIA,
                )
            },
        },
        VulkanCommand {
            name: "vkGetBufferDeviceAddress",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_buffer_device_address as vk::PFN_vkGetBufferDeviceAddress,
                )
            },
        },
        VulkanCommand {
            name: "vkGetBufferDeviceAddressEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_buffer_device_address as vk::PFN_vkGetBufferDeviceAddress,
                )
            },
        },
        VulkanCommand {
            name: "vkGetBufferDeviceAddressKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_buffer_device_address as vk::PFN_vkGetBufferDeviceAddress,
                )
            },
        },
        VulkanCommand {
            name: "vkGetBufferMemoryRequirements",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_buffer_memory_requirements as vk::PFN_vkGetBufferMemoryRequirements,
                )
            },
        },
        VulkanCommand {
            name: "vkGetBufferMemoryRequirements2",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_buffer_memory_requirements2 as vk::PFN_vkGetBufferMemoryRequirements2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetBufferMemoryRequirements2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_buffer_memory_requirements2 as vk::PFN_vkGetBufferMemoryRequirements2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetBufferOpaqueCaptureAddress",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_buffer_opaque_capture_address
                        as vk::PFN_vkGetBufferOpaqueCaptureAddress,
                )
            },
        },
        VulkanCommand {
            name: "vkGetBufferOpaqueCaptureAddressKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_buffer_opaque_capture_address
                        as vk::PFN_vkGetBufferOpaqueCaptureAddress,
                )
            },
        },
        VulkanCommand {
            name: "vkGetBufferOpaqueCaptureDescriptorDataEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_buffer_opaque_capture_descriptor_data_ext
                        as vk::PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetCalibratedTimestampsEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_calibrated_timestamps_ext as vk::PFN_vkGetCalibratedTimestampsEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeferredOperationMaxConcurrencyKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_deferred_operation_max_concurrency_khr
                        as vk::PFN_vkGetDeferredOperationMaxConcurrencyKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeferredOperationResultKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_deferred_operation_result_khr
                        as vk::PFN_vkGetDeferredOperationResultKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDescriptorEXT",
            proc: unsafe {
                std::mem::transmute(Self::get_descriptor_ext as vk::PFN_vkGetDescriptorEXT)
            },
        },
        VulkanCommand {
            name: "vkGetDescriptorSetHostMappingVALVE",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_descriptor_set_host_mapping_valve
                        as vk::PFN_vkGetDescriptorSetHostMappingVALVE,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDescriptorSetLayoutBindingOffsetEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_descriptor_set_layout_binding_offset_ext
                        as vk::PFN_vkGetDescriptorSetLayoutBindingOffsetEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDescriptorSetLayoutHostMappingInfoVALVE",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_descriptor_set_layout_host_mapping_info_valve
                        as vk::PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDescriptorSetLayoutSizeEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_descriptor_set_layout_size_ext
                        as vk::PFN_vkGetDescriptorSetLayoutSizeEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDescriptorSetLayoutSupport",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_descriptor_set_layout_support
                        as vk::PFN_vkGetDescriptorSetLayoutSupport,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDescriptorSetLayoutSupportKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_descriptor_set_layout_support
                        as vk::PFN_vkGetDescriptorSetLayoutSupport,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceAccelerationStructureCompatibilityKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_acceleration_structure_compatibility_khr
                        as vk::PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceBufferMemoryRequirements",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_buffer_memory_requirements
                        as vk::PFN_vkGetDeviceBufferMemoryRequirements,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceBufferMemoryRequirementsKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_buffer_memory_requirements
                        as vk::PFN_vkGetDeviceBufferMemoryRequirements,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceGroupPeerMemoryFeatures",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_group_peer_memory_features
                        as vk::PFN_vkGetDeviceGroupPeerMemoryFeatures,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceGroupPeerMemoryFeaturesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_group_peer_memory_features
                        as vk::PFN_vkGetDeviceGroupPeerMemoryFeatures,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceGroupPresentCapabilitiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_group_present_capabilities_khr
                        as vk::PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceGroupSurfacePresentModes2EXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_group_surface_present_modes2_ext
                        as vk::PFN_vkGetDeviceGroupSurfacePresentModes2EXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceGroupSurfacePresentModesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_group_surface_present_modes_khr
                        as vk::PFN_vkGetDeviceGroupSurfacePresentModesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceImageMemoryRequirements",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_image_memory_requirements
                        as vk::PFN_vkGetDeviceImageMemoryRequirements,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceImageMemoryRequirementsKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_image_memory_requirements
                        as vk::PFN_vkGetDeviceImageMemoryRequirements,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceImageSparseMemoryRequirements",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_image_sparse_memory_requirements
                        as vk::PFN_vkGetDeviceImageSparseMemoryRequirements,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceImageSparseMemoryRequirementsKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_image_sparse_memory_requirements
                        as vk::PFN_vkGetDeviceImageSparseMemoryRequirements,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceMemoryCommitment",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_memory_commitment as vk::PFN_vkGetDeviceMemoryCommitment,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceMemoryOpaqueCaptureAddress",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_memory_opaque_capture_address
                        as vk::PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceMemoryOpaqueCaptureAddressKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_memory_opaque_capture_address
                        as vk::PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceMicromapCompatibilityEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_micromap_compatibility_ext
                        as vk::PFN_vkGetDeviceMicromapCompatibilityEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDeviceProcAddr",
            proc: unsafe {
                std::mem::transmute(Self::get_device_proc_addr as vk::PFN_vkGetDeviceProcAddr)
            },
        },
        VulkanCommand {
            name: "vkGetDeviceQueue",
            proc: unsafe {
                std::mem::transmute(Self::get_device_queue as vk::PFN_vkGetDeviceQueue)
            },
        },
        VulkanCommand {
            name: "vkGetDeviceQueue2",
            proc: unsafe {
                std::mem::transmute(Self::get_device_queue2 as vk::PFN_vkGetDeviceQueue2)
            },
        },
        VulkanCommand {
            name: "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_device_subpass_shading_max_workgroup_size_huawei
                        as vk::PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDynamicRenderingTilePropertiesQCOM",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_dynamic_rendering_tile_properties_qcom
                        as vk::PFN_vkGetDynamicRenderingTilePropertiesQCOM,
                )
            },
        },
        VulkanCommand {
            name: "vkGetEventStatus",
            proc: unsafe {
                std::mem::transmute(Self::get_event_status as vk::PFN_vkGetEventStatus)
            },
        },
        VulkanCommand {
            name: "vkGetFenceFdKHR",
            proc: unsafe { std::mem::transmute(Self::get_fence_fd_khr as vk::PFN_vkGetFenceFdKHR) },
        },
        VulkanCommand {
            name: "vkGetFenceStatus",
            proc: unsafe {
                std::mem::transmute(Self::get_fence_status as vk::PFN_vkGetFenceStatus)
            },
        },
        VulkanCommand {
            name: "vkGetFenceWin32HandleKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_fence_win32_handle_khr as vk::PFN_vkGetFenceWin32HandleKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetFramebufferTilePropertiesQCOM",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_framebuffer_tile_properties_qcom
                        as vk::PFN_vkGetFramebufferTilePropertiesQCOM,
                )
            },
        },
        VulkanCommand {
            name: "vkGetGeneratedCommandsMemoryRequirementsNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_generated_commands_memory_requirements_nv
                        as vk::PFN_vkGetGeneratedCommandsMemoryRequirementsNV,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageDrmFormatModifierPropertiesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_drm_format_modifier_properties_ext
                        as vk::PFN_vkGetImageDrmFormatModifierPropertiesEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageMemoryRequirements",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_memory_requirements as vk::PFN_vkGetImageMemoryRequirements,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageMemoryRequirements2",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_memory_requirements2 as vk::PFN_vkGetImageMemoryRequirements2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageMemoryRequirements2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_memory_requirements2 as vk::PFN_vkGetImageMemoryRequirements2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageOpaqueCaptureDescriptorDataEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_opaque_capture_descriptor_data_ext
                        as vk::PFN_vkGetImageOpaqueCaptureDescriptorDataEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageSparseMemoryRequirements",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_sparse_memory_requirements
                        as vk::PFN_vkGetImageSparseMemoryRequirements,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageSparseMemoryRequirements2",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_sparse_memory_requirements2
                        as vk::PFN_vkGetImageSparseMemoryRequirements2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageSparseMemoryRequirements2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_sparse_memory_requirements2
                        as vk::PFN_vkGetImageSparseMemoryRequirements2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageSubresourceLayout",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_subresource_layout as vk::PFN_vkGetImageSubresourceLayout,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageSubresourceLayout2EXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_subresource_layout2_ext
                        as vk::PFN_vkGetImageSubresourceLayout2EXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageViewAddressNVX",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_view_address_nvx as vk::PFN_vkGetImageViewAddressNVX,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageViewHandleNVX",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_view_handle_nvx as vk::PFN_vkGetImageViewHandleNVX,
                )
            },
        },
        VulkanCommand {
            name: "vkGetImageViewOpaqueCaptureDescriptorDataEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_image_view_opaque_capture_descriptor_data_ext
                        as vk::PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetMemoryAndroidHardwareBufferANDROID",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_memory_android_hardware_buffer_android
                        as vk::PFN_vkGetMemoryAndroidHardwareBufferANDROID,
                )
            },
        },
        VulkanCommand {
            name: "vkGetMemoryFdKHR",
            proc: unsafe {
                std::mem::transmute(Self::get_memory_fd_khr as vk::PFN_vkGetMemoryFdKHR)
            },
        },
        VulkanCommand {
            name: "vkGetMemoryFdPropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_memory_fd_properties_khr as vk::PFN_vkGetMemoryFdPropertiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetMemoryHostPointerPropertiesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_memory_host_pointer_properties_ext
                        as vk::PFN_vkGetMemoryHostPointerPropertiesEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetMemoryRemoteAddressNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_memory_remote_address_nv as vk::PFN_vkGetMemoryRemoteAddressNV,
                )
            },
        },
        VulkanCommand {
            name: "vkGetMemoryWin32HandleKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_memory_win32_handle_khr as vk::PFN_vkGetMemoryWin32HandleKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetMemoryWin32HandleNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_memory_win32_handle_nv as vk::PFN_vkGetMemoryWin32HandleNV,
                )
            },
        },
        VulkanCommand {
            name: "vkGetMemoryWin32HandlePropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_memory_win32_handle_properties_khr
                        as vk::PFN_vkGetMemoryWin32HandlePropertiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetMemoryZirconHandleFUCHSIA",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_memory_zircon_handle_fuchsia
                        as vk::PFN_vkGetMemoryZirconHandleFUCHSIA,
                )
            },
        },
        VulkanCommand {
            name: "vkGetMemoryZirconHandlePropertiesFUCHSIA",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_memory_zircon_handle_properties_fuchsia
                        as vk::PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA,
                )
            },
        },
        VulkanCommand {
            name: "vkGetMicromapBuildSizesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_micromap_build_sizes_ext as vk::PFN_vkGetMicromapBuildSizesEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPastPresentationTimingGOOGLE",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_past_presentation_timing_google
                        as vk::PFN_vkGetPastPresentationTimingGOOGLE,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPerformanceParameterINTEL",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_performance_parameter_intel as vk::PFN_vkGetPerformanceParameterINTEL,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPipelineCacheData",
            proc: unsafe {
                std::mem::transmute(Self::get_pipeline_cache_data as vk::PFN_vkGetPipelineCacheData)
            },
        },
        VulkanCommand {
            name: "vkGetPipelineExecutableInternalRepresentationsKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_pipeline_executable_internal_representations_khr
                        as vk::PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPipelineExecutablePropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_pipeline_executable_properties_khr
                        as vk::PFN_vkGetPipelineExecutablePropertiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPipelineExecutableStatisticsKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_pipeline_executable_statistics_khr
                        as vk::PFN_vkGetPipelineExecutableStatisticsKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPipelinePropertiesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_pipeline_properties_ext as vk::PFN_vkGetPipelinePropertiesEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPrivateData",
            proc: unsafe {
                std::mem::transmute(Self::get_private_data as vk::PFN_vkGetPrivateData)
            },
        },
        VulkanCommand {
            name: "vkGetPrivateDataEXT",
            proc: unsafe {
                std::mem::transmute(Self::get_private_data as vk::PFN_vkGetPrivateData)
            },
        },
        VulkanCommand {
            name: "vkGetQueryPoolResults",
            proc: unsafe {
                std::mem::transmute(Self::get_query_pool_results as vk::PFN_vkGetQueryPoolResults)
            },
        },
        VulkanCommand {
            name: "vkGetQueueCheckpointData2NV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_queue_checkpoint_data2_nv as vk::PFN_vkGetQueueCheckpointData2NV,
                )
            },
        },
        VulkanCommand {
            name: "vkGetQueueCheckpointDataNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_queue_checkpoint_data_nv as vk::PFN_vkGetQueueCheckpointDataNV,
                )
            },
        },
        VulkanCommand {
            name: "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_ray_tracing_capture_replay_shader_group_handles_khr
                        as vk::PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetRayTracingShaderGroupHandlesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_ray_tracing_shader_group_handles_khr
                        as vk::PFN_vkGetRayTracingShaderGroupHandlesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetRayTracingShaderGroupHandlesNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_ray_tracing_shader_group_handles_khr
                        as vk::PFN_vkGetRayTracingShaderGroupHandlesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetRayTracingShaderGroupStackSizeKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_ray_tracing_shader_group_stack_size_khr
                        as vk::PFN_vkGetRayTracingShaderGroupStackSizeKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetRefreshCycleDurationGOOGLE",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_refresh_cycle_duration_google
                        as vk::PFN_vkGetRefreshCycleDurationGOOGLE,
                )
            },
        },
        VulkanCommand {
            name: "vkGetRenderAreaGranularity",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_render_area_granularity as vk::PFN_vkGetRenderAreaGranularity,
                )
            },
        },
        VulkanCommand {
            name: "vkGetSamplerOpaqueCaptureDescriptorDataEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_sampler_opaque_capture_descriptor_data_ext
                        as vk::PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetSemaphoreCounterValue",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_semaphore_counter_value as vk::PFN_vkGetSemaphoreCounterValue,
                )
            },
        },
        VulkanCommand {
            name: "vkGetSemaphoreCounterValueKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_semaphore_counter_value as vk::PFN_vkGetSemaphoreCounterValue,
                )
            },
        },
        VulkanCommand {
            name: "vkGetSemaphoreFdKHR",
            proc: unsafe {
                std::mem::transmute(Self::get_semaphore_fd_khr as vk::PFN_vkGetSemaphoreFdKHR)
            },
        },
        VulkanCommand {
            name: "vkGetSemaphoreWin32HandleKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_semaphore_win32_handle_khr as vk::PFN_vkGetSemaphoreWin32HandleKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetSemaphoreZirconHandleFUCHSIA",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_semaphore_zircon_handle_fuchsia
                        as vk::PFN_vkGetSemaphoreZirconHandleFUCHSIA,
                )
            },
        },
        VulkanCommand {
            name: "vkGetShaderInfoAMD",
            proc: unsafe {
                std::mem::transmute(Self::get_shader_info_amd as vk::PFN_vkGetShaderInfoAMD)
            },
        },
        VulkanCommand {
            name: "vkGetShaderModuleCreateInfoIdentifierEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_shader_module_create_info_identifier_ext
                        as vk::PFN_vkGetShaderModuleCreateInfoIdentifierEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetShaderModuleIdentifierEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_shader_module_identifier_ext
                        as vk::PFN_vkGetShaderModuleIdentifierEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetSwapchainCounterEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_swapchain_counter_ext as vk::PFN_vkGetSwapchainCounterEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetSwapchainImagesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_swapchain_images_khr as vk::PFN_vkGetSwapchainImagesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetSwapchainStatusKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_swapchain_status_khr as vk::PFN_vkGetSwapchainStatusKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetValidationCacheDataEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_validation_cache_data_ext as vk::PFN_vkGetValidationCacheDataEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetVideoSessionMemoryRequirementsKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_video_session_memory_requirements_khr
                        as vk::PFN_vkGetVideoSessionMemoryRequirementsKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkImportFenceFdKHR",
            proc: unsafe {
                std::mem::transmute(Self::import_fence_fd_khr as vk::PFN_vkImportFenceFdKHR)
            },
        },
        VulkanCommand {
            name: "vkImportFenceWin32HandleKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::import_fence_win32_handle_khr as vk::PFN_vkImportFenceWin32HandleKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkImportSemaphoreFdKHR",
            proc: unsafe {
                std::mem::transmute(Self::import_semaphore_fd_khr as vk::PFN_vkImportSemaphoreFdKHR)
            },
        },
        VulkanCommand {
            name: "vkImportSemaphoreWin32HandleKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::import_semaphore_win32_handle_khr
                        as vk::PFN_vkImportSemaphoreWin32HandleKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkImportSemaphoreZirconHandleFUCHSIA",
            proc: unsafe {
                std::mem::transmute(
                    Self::import_semaphore_zircon_handle_fuchsia
                        as vk::PFN_vkImportSemaphoreZirconHandleFUCHSIA,
                )
            },
        },
        VulkanCommand {
            name: "vkInitializePerformanceApiINTEL",
            proc: unsafe {
                std::mem::transmute(
                    Self::initialize_performance_api_intel
                        as vk::PFN_vkInitializePerformanceApiINTEL,
                )
            },
        },
        VulkanCommand {
            name: "vkInvalidateMappedMemoryRanges",
            proc: unsafe {
                std::mem::transmute(
                    Self::invalidate_mapped_memory_ranges as vk::PFN_vkInvalidateMappedMemoryRanges,
                )
            },
        },
        VulkanCommand {
            name: "vkMapMemory",
            proc: unsafe { std::mem::transmute(Self::map_memory as vk::PFN_vkMapMemory) },
        },
        VulkanCommand {
            name: "vkMergePipelineCaches",
            proc: unsafe {
                std::mem::transmute(Self::merge_pipeline_caches as vk::PFN_vkMergePipelineCaches)
            },
        },
        VulkanCommand {
            name: "vkMergeValidationCachesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::merge_validation_caches_ext as vk::PFN_vkMergeValidationCachesEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkQueueBeginDebugUtilsLabelEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::queue_begin_debug_utils_label_ext
                        as vk::PFN_vkQueueBeginDebugUtilsLabelEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkQueueBindSparse",
            proc: unsafe {
                std::mem::transmute(Self::queue_bind_sparse as vk::PFN_vkQueueBindSparse)
            },
        },
        VulkanCommand {
            name: "vkQueueEndDebugUtilsLabelEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::queue_end_debug_utils_label_ext as vk::PFN_vkQueueEndDebugUtilsLabelEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkQueueInsertDebugUtilsLabelEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::queue_insert_debug_utils_label_ext
                        as vk::PFN_vkQueueInsertDebugUtilsLabelEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkQueuePresentKHR",
            proc: unsafe {
                std::mem::transmute(Self::queue_present_khr as vk::PFN_vkQueuePresentKHR)
            },
        },
        VulkanCommand {
            name: "vkQueueSetPerformanceConfigurationINTEL",
            proc: unsafe {
                std::mem::transmute(
                    Self::queue_set_performance_configuration_intel
                        as vk::PFN_vkQueueSetPerformanceConfigurationINTEL,
                )
            },
        },
        VulkanCommand {
            name: "vkQueueSubmit",
            proc: unsafe { std::mem::transmute(Self::queue_submit as vk::PFN_vkQueueSubmit) },
        },
        VulkanCommand {
            name: "vkQueueSubmit2",
            proc: unsafe { std::mem::transmute(Self::queue_submit2 as vk::PFN_vkQueueSubmit2) },
        },
        VulkanCommand {
            name: "vkQueueSubmit2KHR",
            proc: unsafe { std::mem::transmute(Self::queue_submit2 as vk::PFN_vkQueueSubmit2) },
        },
        VulkanCommand {
            name: "vkQueueWaitIdle",
            proc: unsafe { std::mem::transmute(Self::queue_wait_idle as vk::PFN_vkQueueWaitIdle) },
        },
        VulkanCommand {
            name: "vkRegisterDeviceEventEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::register_device_event_ext as vk::PFN_vkRegisterDeviceEventEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkRegisterDisplayEventEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::register_display_event_ext as vk::PFN_vkRegisterDisplayEventEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkReleaseFullScreenExclusiveModeEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::release_full_screen_exclusive_mode_ext
                        as vk::PFN_vkReleaseFullScreenExclusiveModeEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkReleasePerformanceConfigurationINTEL",
            proc: unsafe {
                std::mem::transmute(
                    Self::release_performance_configuration_intel
                        as vk::PFN_vkReleasePerformanceConfigurationINTEL,
                )
            },
        },
        VulkanCommand {
            name: "vkReleaseProfilingLockKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::release_profiling_lock_khr as vk::PFN_vkReleaseProfilingLockKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkReleaseSwapchainImagesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::release_swapchain_images_ext as vk::PFN_vkReleaseSwapchainImagesEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkResetCommandBuffer",
            proc: unsafe {
                std::mem::transmute(Self::reset_command_buffer as vk::PFN_vkResetCommandBuffer)
            },
        },
        VulkanCommand {
            name: "vkResetCommandPool",
            proc: unsafe {
                std::mem::transmute(Self::reset_command_pool as vk::PFN_vkResetCommandPool)
            },
        },
        VulkanCommand {
            name: "vkResetDescriptorPool",
            proc: unsafe {
                std::mem::transmute(Self::reset_descriptor_pool as vk::PFN_vkResetDescriptorPool)
            },
        },
        VulkanCommand {
            name: "vkResetEvent",
            proc: unsafe { std::mem::transmute(Self::reset_event as vk::PFN_vkResetEvent) },
        },
        VulkanCommand {
            name: "vkResetFences",
            proc: unsafe { std::mem::transmute(Self::reset_fences as vk::PFN_vkResetFences) },
        },
        VulkanCommand {
            name: "vkResetQueryPool",
            proc: unsafe {
                std::mem::transmute(Self::reset_query_pool as vk::PFN_vkResetQueryPool)
            },
        },
        VulkanCommand {
            name: "vkResetQueryPoolEXT",
            proc: unsafe {
                std::mem::transmute(Self::reset_query_pool as vk::PFN_vkResetQueryPool)
            },
        },
        VulkanCommand {
            name: "vkSetBufferCollectionBufferConstraintsFUCHSIA",
            proc: unsafe {
                std::mem::transmute(
                    Self::set_buffer_collection_buffer_constraints_fuchsia
                        as vk::PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,
                )
            },
        },
        VulkanCommand {
            name: "vkSetBufferCollectionImageConstraintsFUCHSIA",
            proc: unsafe {
                std::mem::transmute(
                    Self::set_buffer_collection_image_constraints_fuchsia
                        as vk::PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,
                )
            },
        },
        VulkanCommand {
            name: "vkSetDebugUtilsObjectNameEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::set_debug_utils_object_name_ext as vk::PFN_vkSetDebugUtilsObjectNameEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkSetDebugUtilsObjectTagEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::set_debug_utils_object_tag_ext as vk::PFN_vkSetDebugUtilsObjectTagEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkSetDeviceMemoryPriorityEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::set_device_memory_priority_ext as vk::PFN_vkSetDeviceMemoryPriorityEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkSetEvent",
            proc: unsafe { std::mem::transmute(Self::set_event as vk::PFN_vkSetEvent) },
        },
        VulkanCommand {
            name: "vkSetHdrMetadataEXT",
            proc: unsafe {
                std::mem::transmute(Self::set_hdr_metadata_ext as vk::PFN_vkSetHdrMetadataEXT)
            },
        },
        VulkanCommand {
            name: "vkSetLocalDimmingAMD",
            proc: unsafe {
                std::mem::transmute(Self::set_local_dimming_amd as vk::PFN_vkSetLocalDimmingAMD)
            },
        },
        VulkanCommand {
            name: "vkSetPrivateData",
            proc: unsafe {
                std::mem::transmute(Self::set_private_data as vk::PFN_vkSetPrivateData)
            },
        },
        VulkanCommand {
            name: "vkSetPrivateDataEXT",
            proc: unsafe {
                std::mem::transmute(Self::set_private_data as vk::PFN_vkSetPrivateData)
            },
        },
        VulkanCommand {
            name: "vkSignalSemaphore",
            proc: unsafe {
                std::mem::transmute(Self::signal_semaphore as vk::PFN_vkSignalSemaphore)
            },
        },
        VulkanCommand {
            name: "vkSignalSemaphoreKHR",
            proc: unsafe {
                std::mem::transmute(Self::signal_semaphore as vk::PFN_vkSignalSemaphore)
            },
        },
        VulkanCommand {
            name: "vkTrimCommandPool",
            proc: unsafe {
                std::mem::transmute(Self::trim_command_pool as vk::PFN_vkTrimCommandPool)
            },
        },
        VulkanCommand {
            name: "vkTrimCommandPoolKHR",
            proc: unsafe {
                std::mem::transmute(Self::trim_command_pool as vk::PFN_vkTrimCommandPool)
            },
        },
        VulkanCommand {
            name: "vkUninitializePerformanceApiINTEL",
            proc: unsafe {
                std::mem::transmute(
                    Self::uninitialize_performance_api_intel
                        as vk::PFN_vkUninitializePerformanceApiINTEL,
                )
            },
        },
        VulkanCommand {
            name: "vkUnmapMemory",
            proc: unsafe { std::mem::transmute(Self::unmap_memory as vk::PFN_vkUnmapMemory) },
        },
        VulkanCommand {
            name: "vkUpdateDescriptorSetWithTemplate",
            proc: unsafe {
                std::mem::transmute(
                    Self::update_descriptor_set_with_template
                        as vk::PFN_vkUpdateDescriptorSetWithTemplate,
                )
            },
        },
        VulkanCommand {
            name: "vkUpdateDescriptorSetWithTemplateKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::update_descriptor_set_with_template
                        as vk::PFN_vkUpdateDescriptorSetWithTemplate,
                )
            },
        },
        VulkanCommand {
            name: "vkUpdateDescriptorSets",
            proc: unsafe {
                std::mem::transmute(Self::update_descriptor_sets as vk::PFN_vkUpdateDescriptorSets)
            },
        },
        VulkanCommand {
            name: "vkUpdateVideoSessionParametersKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::update_video_session_parameters_khr
                        as vk::PFN_vkUpdateVideoSessionParametersKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkWaitForFences",
            proc: unsafe { std::mem::transmute(Self::wait_for_fences as vk::PFN_vkWaitForFences) },
        },
        VulkanCommand {
            name: "vkWaitForPresentKHR",
            proc: unsafe {
                std::mem::transmute(Self::wait_for_present_khr as vk::PFN_vkWaitForPresentKHR)
            },
        },
        VulkanCommand {
            name: "vkWaitSemaphores",
            proc: unsafe { std::mem::transmute(Self::wait_semaphores as vk::PFN_vkWaitSemaphores) },
        },
        VulkanCommand {
            name: "vkWaitSemaphoresKHR",
            proc: unsafe { std::mem::transmute(Self::wait_semaphores as vk::PFN_vkWaitSemaphores) },
        },
        VulkanCommand {
            name: "vkWriteAccelerationStructuresPropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::write_acceleration_structures_properties_khr
                        as vk::PFN_vkWriteAccelerationStructuresPropertiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkWriteMicromapsPropertiesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::write_micromaps_properties_ext as vk::PFN_vkWriteMicromapsPropertiesEXT,
                )
            },
        },
    ];
    pub(crate) const INSTANCE_COMMANDS: [VulkanCommand; 102] = [
        VulkanCommand {
            name: "vkAcquireDrmDisplayEXT",
            proc: unsafe {
                std::mem::transmute(Self::acquire_drm_display_ext as vk::PFN_vkAcquireDrmDisplayEXT)
            },
        },
        VulkanCommand {
            name: "vkAcquireWinrtDisplayNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::acquire_winrt_display_nv as vk::PFN_vkAcquireWinrtDisplayNV,
                )
            },
        },
        VulkanCommand {
            name: "vkAcquireXlibDisplayEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::acquire_xlib_display_ext as vk::PFN_vkAcquireXlibDisplayEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateAndroidSurfaceKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_android_surface_khr as vk::PFN_vkCreateAndroidSurfaceKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateDebugReportCallbackEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_debug_report_callback_ext
                        as vk::PFN_vkCreateDebugReportCallbackEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateDebugUtilsMessengerEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_debug_utils_messenger_ext
                        as vk::PFN_vkCreateDebugUtilsMessengerEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateDevice",
            proc: unsafe { std::mem::transmute(Self::create_device as vk::PFN_vkCreateDevice) },
        },
        VulkanCommand {
            name: "vkCreateDirectFBSurfaceEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_direct_fb_surface_ext as vk::PFN_vkCreateDirectFBSurfaceEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateDisplayModeKHR",
            proc: unsafe {
                std::mem::transmute(Self::create_display_mode_khr as vk::PFN_vkCreateDisplayModeKHR)
            },
        },
        VulkanCommand {
            name: "vkCreateDisplayPlaneSurfaceKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_display_plane_surface_khr
                        as vk::PFN_vkCreateDisplayPlaneSurfaceKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateHeadlessSurfaceEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_headless_surface_ext as vk::PFN_vkCreateHeadlessSurfaceEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateIOSSurfaceMVK",
            proc: unsafe {
                std::mem::transmute(Self::create_ios_surface_mvk as vk::PFN_vkCreateIOSSurfaceMVK)
            },
        },
        VulkanCommand {
            name: "vkCreateImagePipeSurfaceFUCHSIA",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_image_pipe_surface_fuchsia
                        as vk::PFN_vkCreateImagePipeSurfaceFUCHSIA,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateMacOSSurfaceMVK",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_mac_os_surface_mvk as vk::PFN_vkCreateMacOSSurfaceMVK,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateMetalSurfaceEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_metal_surface_ext as vk::PFN_vkCreateMetalSurfaceEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateScreenSurfaceQNX",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_screen_surface_qnx as vk::PFN_vkCreateScreenSurfaceQNX,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateStreamDescriptorSurfaceGGP",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_stream_descriptor_surface_ggp
                        as vk::PFN_vkCreateStreamDescriptorSurfaceGGP,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateViSurfaceNN",
            proc: unsafe {
                std::mem::transmute(Self::create_vi_surface_nn as vk::PFN_vkCreateViSurfaceNN)
            },
        },
        VulkanCommand {
            name: "vkCreateWaylandSurfaceKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_wayland_surface_khr as vk::PFN_vkCreateWaylandSurfaceKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateWin32SurfaceKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::create_win32_surface_khr as vk::PFN_vkCreateWin32SurfaceKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkCreateXcbSurfaceKHR",
            proc: unsafe {
                std::mem::transmute(Self::create_xcb_surface_khr as vk::PFN_vkCreateXcbSurfaceKHR)
            },
        },
        VulkanCommand {
            name: "vkCreateXlibSurfaceKHR",
            proc: unsafe {
                std::mem::transmute(Self::create_xlib_surface_khr as vk::PFN_vkCreateXlibSurfaceKHR)
            },
        },
        VulkanCommand {
            name: "vkDebugReportMessageEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::debug_report_message_ext as vk::PFN_vkDebugReportMessageEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyDebugReportCallbackEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_debug_report_callback_ext
                        as vk::PFN_vkDestroyDebugReportCallbackEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyDebugUtilsMessengerEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::destroy_debug_utils_messenger_ext
                        as vk::PFN_vkDestroyDebugUtilsMessengerEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkDestroyInstance",
            proc: unsafe {
                std::mem::transmute(Self::destroy_instance as vk::PFN_vkDestroyInstance)
            },
        },
        VulkanCommand {
            name: "vkDestroySurfaceKHR",
            proc: unsafe {
                std::mem::transmute(Self::destroy_surface_khr as vk::PFN_vkDestroySurfaceKHR)
            },
        },
        VulkanCommand {
            name: "vkEnumerateDeviceExtensionProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::enumerate_device_extension_properties
                        as vk::PFN_vkEnumerateDeviceExtensionProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkEnumerateDeviceLayerProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::enumerate_device_layer_properties
                        as vk::PFN_vkEnumerateDeviceLayerProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkEnumeratePhysicalDeviceGroups",
            proc: unsafe {
                std::mem::transmute(
                    Self::enumerate_physical_device_groups
                        as vk::PFN_vkEnumeratePhysicalDeviceGroups,
                )
            },
        },
        VulkanCommand {
            name: "vkEnumeratePhysicalDeviceGroupsKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::enumerate_physical_device_groups
                        as vk::PFN_vkEnumeratePhysicalDeviceGroups,
                )
            },
        },
        VulkanCommand {
            name: "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::enumerate_physical_device_queue_family_performance_query_counters_khr
                        as vk::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkEnumeratePhysicalDevices",
            proc: unsafe {
                std::mem::transmute(
                    Self::enumerate_physical_devices as vk::PFN_vkEnumeratePhysicalDevices,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDisplayModeProperties2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_display_mode_properties2_khr
                        as vk::PFN_vkGetDisplayModeProperties2KHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDisplayModePropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_display_mode_properties_khr as vk::PFN_vkGetDisplayModePropertiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDisplayPlaneCapabilities2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_display_plane_capabilities2_khr
                        as vk::PFN_vkGetDisplayPlaneCapabilities2KHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDisplayPlaneCapabilitiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_display_plane_capabilities_khr
                        as vk::PFN_vkGetDisplayPlaneCapabilitiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDisplayPlaneSupportedDisplaysKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_display_plane_supported_displays_khr
                        as vk::PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetDrmDisplayEXT",
            proc: unsafe {
                std::mem::transmute(Self::get_drm_display_ext as vk::PFN_vkGetDrmDisplayEXT)
            },
        },
        VulkanCommand {
            name: "vkGetInstanceProcAddr",
            proc: unsafe {
                std::mem::transmute(Self::get_instance_proc_addr as vk::PFN_vkGetInstanceProcAddr)
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_calibrateable_time_domains_ext
                        as vk::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_cooperative_matrix_properties_nv
                        as vk::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceDirectFBPresentationSupportEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_direct_fb_presentation_support_ext
                        as vk::PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceDisplayPlaneProperties2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_display_plane_properties2_khr
                        as vk::PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceDisplayPlanePropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_display_plane_properties_khr
                        as vk::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceDisplayProperties2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_display_properties2_khr
                        as vk::PFN_vkGetPhysicalDeviceDisplayProperties2KHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceDisplayPropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_display_properties_khr
                        as vk::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceExternalBufferProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_external_buffer_properties
                        as vk::PFN_vkGetPhysicalDeviceExternalBufferProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceExternalBufferPropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_external_buffer_properties
                        as vk::PFN_vkGetPhysicalDeviceExternalBufferProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceExternalFenceProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_external_fence_properties
                        as vk::PFN_vkGetPhysicalDeviceExternalFenceProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceExternalFencePropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_external_fence_properties
                        as vk::PFN_vkGetPhysicalDeviceExternalFenceProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceExternalImageFormatPropertiesNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_external_image_format_properties_nv
                        as vk::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceExternalSemaphoreProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_external_semaphore_properties
                        as vk::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_external_semaphore_properties
                        as vk::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceFeatures",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_features as vk::PFN_vkGetPhysicalDeviceFeatures,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceFeatures2",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_features2 as vk::PFN_vkGetPhysicalDeviceFeatures2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceFeatures2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_features2 as vk::PFN_vkGetPhysicalDeviceFeatures2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceFormatProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_format_properties
                        as vk::PFN_vkGetPhysicalDeviceFormatProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceFormatProperties2",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_format_properties2
                        as vk::PFN_vkGetPhysicalDeviceFormatProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceFormatProperties2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_format_properties2
                        as vk::PFN_vkGetPhysicalDeviceFormatProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceFragmentShadingRatesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_fragment_shading_rates_khr
                        as vk::PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceImageFormatProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_image_format_properties
                        as vk::PFN_vkGetPhysicalDeviceImageFormatProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceImageFormatProperties2",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_image_format_properties2
                        as vk::PFN_vkGetPhysicalDeviceImageFormatProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceImageFormatProperties2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_image_format_properties2
                        as vk::PFN_vkGetPhysicalDeviceImageFormatProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceMemoryProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_memory_properties
                        as vk::PFN_vkGetPhysicalDeviceMemoryProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceMemoryProperties2",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_memory_properties2
                        as vk::PFN_vkGetPhysicalDeviceMemoryProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceMemoryProperties2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_memory_properties2
                        as vk::PFN_vkGetPhysicalDeviceMemoryProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceMultisamplePropertiesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_multisample_properties_ext
                        as vk::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceOpticalFlowImageFormatsNV",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_optical_flow_image_formats_nv
                        as vk::PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDevicePresentRectanglesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_present_rectangles_khr
                        as vk::PFN_vkGetPhysicalDevicePresentRectanglesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_properties as vk::PFN_vkGetPhysicalDeviceProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceProperties2",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_properties2 as vk::PFN_vkGetPhysicalDeviceProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceProperties2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_properties2 as vk::PFN_vkGetPhysicalDeviceProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_queue_family_performance_query_passes_khr
                        as vk::PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceQueueFamilyProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_queue_family_properties
                        as vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceQueueFamilyProperties2",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_queue_family_properties2
                        as vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceQueueFamilyProperties2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_queue_family_properties2
                        as vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceScreenPresentationSupportQNX",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_screen_presentation_support_qnx
                        as vk::PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSparseImageFormatProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_sparse_image_format_properties
                        as vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSparseImageFormatProperties2",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_sparse_image_format_properties2
                        as vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSparseImageFormatProperties2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_sparse_image_format_properties2
                        as vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV",
            proc: unsafe {
                std::mem::transmute(Self::get_physical_device_supported_framebuffer_mixed_samples_combinations_nv as vk::PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV)
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSurfaceCapabilities2EXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_surface_capabilities2_ext
                        as vk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSurfaceCapabilities2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_surface_capabilities2_khr
                        as vk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_surface_capabilities_khr
                        as vk::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSurfaceFormats2KHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_surface_formats2_khr
                        as vk::PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSurfaceFormatsKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_surface_formats_khr
                        as vk::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSurfacePresentModes2EXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_surface_present_modes2_ext
                        as vk::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSurfacePresentModesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_surface_present_modes_khr
                        as vk::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceSurfaceSupportKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_surface_support_khr
                        as vk::PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceToolProperties",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_tool_properties
                        as vk::PFN_vkGetPhysicalDeviceToolProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceToolPropertiesEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_tool_properties
                        as vk::PFN_vkGetPhysicalDeviceToolProperties,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceVideoCapabilitiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_video_capabilities_khr
                        as vk::PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceVideoFormatPropertiesKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_video_format_properties_khr
                        as vk::PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceWaylandPresentationSupportKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_wayland_presentation_support_khr
                        as vk::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceWin32PresentationSupportKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_win32_presentation_support_khr
                        as vk::PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceXcbPresentationSupportKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_xcb_presentation_support_khr
                        as vk::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetPhysicalDeviceXlibPresentationSupportKHR",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_physical_device_xlib_presentation_support_khr
                        as vk::PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
                )
            },
        },
        VulkanCommand {
            name: "vkGetRandROutputDisplayEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::get_rand_r_output_display_ext as vk::PFN_vkGetRandROutputDisplayEXT,
                )
            },
        },
        VulkanCommand {
            name: "vkGetWinrtDisplayNV",
            proc: unsafe {
                std::mem::transmute(Self::get_winrt_display_nv as vk::PFN_vkGetWinrtDisplayNV)
            },
        },
        VulkanCommand {
            name: "vkReleaseDisplayEXT",
            proc: unsafe {
                std::mem::transmute(Self::release_display_ext as vk::PFN_vkReleaseDisplayEXT)
            },
        },
        VulkanCommand {
            name: "vkSubmitDebugUtilsMessageEXT",
            proc: unsafe {
                std::mem::transmute(
                    Self::submit_debug_utils_message_ext as vk::PFN_vkSubmitDebugUtilsMessageEXT,
                )
            },
        },
    ];
    extern "system" fn get_physical_device_features(
        physical_device: vk::PhysicalDevice,
        p_features: *mut vk::PhysicalDeviceFeatures,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceFeatures
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_0();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_features(physical_device);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_features.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_features)(physical_device, p_features)
            },
        }
    }
    extern "system" fn get_physical_device_format_properties(
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        p_format_properties: *mut vk::FormatProperties,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceFormatProperties
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_0();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_format_properties(physical_device, format);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_format_properties.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_format_properties)(
                    physical_device,
                    format,
                    p_format_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_image_format_properties(
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        _type: vk::ImageType,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        flags: vk::ImageCreateFlags,
        p_image_format_properties: *mut vk::ImageFormatProperties,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceImageFormatProperties
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_0();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_image_format_properties(
                physical_device,
                format,
                _type,
                tiling,
                usage,
                flags,
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_image_format_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_image_format_properties)(
                    physical_device,
                    format,
                    _type,
                    tiling,
                    usage,
                    flags,
                    p_image_format_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_properties(
        physical_device: vk::PhysicalDevice,
        p_properties: *mut vk::PhysicalDeviceProperties,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceProperties
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_0();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_properties(physical_device);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_properties.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_properties)(physical_device, p_properties)
            },
        }
    }
    extern "system" fn get_physical_device_queue_family_properties(
        physical_device: vk::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut vk::QueueFamilyProperties,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceQueueFamilyProperties
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_0();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_queue_family_properties(physical_device);
        match layer_result {
            LayerResult::Handled(res) => {
                // We can't return INCOMPLETE from vkGetPhysicalDeviceQueueFamilyProperties
                #[allow(unused_must_use)]
                unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_queue_family_property_count).unwrap(),
                        p_queue_family_properties,
                    );
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_queue_family_properties)(
                    physical_device,
                    p_queue_family_property_count,
                    p_queue_family_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_memory_properties(
        physical_device: vk::PhysicalDevice,
        p_memory_properties: *mut vk::PhysicalDeviceMemoryProperties,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceMemoryProperties
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_0();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_memory_properties(physical_device);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_memory_properties.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_memory_properties)(
                    physical_device,
                    p_memory_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_sparse_image_format_properties(
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        _type: vk::ImageType,
        samples: vk::SampleCountFlags,
        usage: vk::ImageUsageFlags,
        tiling: vk::ImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut vk::SparseImageFormatProperties,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceSparseImageFormatProperties
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_0();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_sparse_image_format_properties(
                physical_device,
                format,
                _type,
                samples,
                usage,
                tiling,
            );
        match layer_result {
            LayerResult::Handled(res) => {
                // We can't return INCOMPLETE from vkGetPhysicalDeviceSparseImageFormatProperties
                #[allow(unused_must_use)]
                unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_property_count).unwrap(), p_properties);
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_sparse_image_format_properties)(
                    physical_device,
                    format,
                    _type,
                    samples,
                    usage,
                    tiling,
                    p_property_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_features2(
        physical_device: vk::PhysicalDevice,
        p_features: *mut vk::PhysicalDeviceFeatures2,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceFeatures2
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_1();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_features2(physical_device);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_features.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_features2)(physical_device, p_features)
            },
        }
    }
    extern "system" fn get_physical_device_properties2(
        physical_device: vk::PhysicalDevice,
        p_properties: *mut vk::PhysicalDeviceProperties2,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceProperties2
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_1();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_properties2(physical_device);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_properties.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_properties2)(physical_device, p_properties)
            },
        }
    }
    extern "system" fn get_physical_device_format_properties2(
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        p_format_properties: *mut vk::FormatProperties2,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceFormatProperties2
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_1();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_format_properties2(physical_device, format);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_format_properties.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_format_properties2)(
                    physical_device,
                    format,
                    p_format_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_image_format_properties2(
        physical_device: vk::PhysicalDevice,
        p_image_format_info: *const vk::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut vk::ImageFormatProperties2,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceImageFormatProperties2
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_1();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_image_format_properties2(
                physical_device,
                unsafe { p_image_format_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_image_format_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_image_format_properties2)(
                    physical_device,
                    p_image_format_info,
                    p_image_format_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_queue_family_properties2(
        physical_device: vk::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut vk::QueueFamilyProperties2,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceQueueFamilyProperties2
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_1();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_queue_family_properties2(physical_device);
        match layer_result {
            LayerResult::Handled(res) => {
                // We can't return INCOMPLETE from vkGetPhysicalDeviceQueueFamilyProperties2
                #[allow(unused_must_use)]
                unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_queue_family_property_count).unwrap(),
                        p_queue_family_properties,
                    );
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_queue_family_properties2)(
                    physical_device,
                    p_queue_family_property_count,
                    p_queue_family_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_memory_properties2(
        physical_device: vk::PhysicalDevice,
        p_memory_properties: *mut vk::PhysicalDeviceMemoryProperties2,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceMemoryProperties2
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_1();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_memory_properties2(physical_device);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_memory_properties.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_memory_properties2)(
                    physical_device,
                    p_memory_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_sparse_image_format_properties2(
        physical_device: vk::PhysicalDevice,
        p_format_info: *const vk::PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut vk::SparseImageFormatProperties2,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceSparseImageFormatProperties2
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_1();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_sparse_image_format_properties2(
                physical_device,
                unsafe { p_format_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                // We can't return INCOMPLETE from vkGetPhysicalDeviceSparseImageFormatProperties2
                #[allow(unused_must_use)]
                unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_property_count).unwrap(), p_properties);
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_sparse_image_format_properties2)(
                    physical_device,
                    p_format_info,
                    p_property_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_external_buffer_properties(
        physical_device: vk::PhysicalDevice,
        p_external_buffer_info: *const vk::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *mut vk::ExternalBufferProperties,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceExternalBufferProperties
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_1();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_external_buffer_properties(
                physical_device,
                unsafe { p_external_buffer_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_external_buffer_properties.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_external_buffer_properties)(
                    physical_device,
                    p_external_buffer_info,
                    p_external_buffer_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_external_fence_properties(
        physical_device: vk::PhysicalDevice,
        p_external_fence_info: *const vk::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *mut vk::ExternalFenceProperties,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceExternalFenceProperties
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_1();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_external_fence_properties(
                physical_device,
                unsafe { p_external_fence_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_external_fence_properties.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_external_fence_properties)(
                    physical_device,
                    p_external_fence_info,
                    p_external_fence_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_external_semaphore_properties(
        physical_device: vk::PhysicalDevice,
        p_external_semaphore_info: *const vk::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *mut vk::ExternalSemaphoreProperties,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceExternalSemaphoreProperties
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_1();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_external_semaphore_properties(
                physical_device,
                unsafe { p_external_semaphore_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_external_semaphore_properties.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_external_semaphore_properties)(
                    physical_device,
                    p_external_semaphore_info,
                    p_external_semaphore_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_tool_properties(
        physical_device: vk::PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut vk::PhysicalDeviceToolProperties,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceToolProperties
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.core.fp_v1_3();
        let layer_result = instance_info
            .customized_info
            .get_physical_device_tool_properties(physical_device);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_tool_count).unwrap(), p_tool_properties)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_tool_properties)(
                    physical_device,
                    p_tool_count,
                    p_tool_properties,
                )
            },
        }
    }
    extern "system" fn destroy_surface_khr(
        instance: vk::Instance,
        surface: vk::SurfaceKHR,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroySurfaceKHR
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_surface;
        let layer_result = instance_info
            .customized_info
            .destroy_surface_khr(surface, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_surface_khr)(instance, surface, p_allocator)
            },
        }
    }
    extern "system" fn get_physical_device_surface_support_khr(
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        surface: vk::SurfaceKHR,
        p_supported: *mut vk::Bool32,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceSurfaceSupportKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_surface;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_surface_support_khr(physical_device, queue_family_index, surface);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_supported.as_mut() }.unwrap() =
                        if res { vk::TRUE } else { vk::FALSE };
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_surface_support_khr)(
                    physical_device,
                    queue_family_index,
                    surface,
                    p_supported,
                )
            },
        }
    }
    extern "system" fn get_physical_device_surface_capabilities_khr(
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        p_surface_capabilities: *mut vk::SurfaceCapabilitiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceSurfaceCapabilitiesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_surface;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_surface_capabilities_khr(physical_device, surface);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface_capabilities.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_surface_capabilities_khr)(
                    physical_device,
                    surface,
                    p_surface_capabilities,
                )
            },
        }
    }
    extern "system" fn get_physical_device_surface_formats_khr(
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut vk::SurfaceFormatKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceSurfaceFormatsKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_surface;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_surface_formats_khr(physical_device, surface);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_surface_format_count).unwrap(),
                        p_surface_formats,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_surface_formats_khr)(
                    physical_device,
                    surface,
                    p_surface_format_count,
                    p_surface_formats,
                )
            },
        }
    }
    extern "system" fn get_physical_device_surface_present_modes_khr(
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut vk::PresentModeKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceSurfacePresentModesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_surface;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_surface_present_modes_khr(physical_device, surface);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_present_mode_count).unwrap(),
                        p_present_modes,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_surface_present_modes_khr)(
                    physical_device,
                    surface,
                    p_present_mode_count,
                    p_present_modes,
                )
            },
        }
    }
    extern "system" fn get_physical_device_present_rectangles_khr(
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut vk::Rect2D,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDevicePresentRectanglesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_swapchain;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_present_rectangles_khr(physical_device, surface);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_rect_count).unwrap(), p_rects)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_present_rectangles_khr)(
                    physical_device,
                    surface,
                    p_rect_count,
                    p_rects,
                )
            },
        }
    }
    extern "system" fn get_physical_device_display_properties_khr(
        physical_device: vk::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut vk::DisplayPropertiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceDisplayPropertiesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_display;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_display_properties_khr(physical_device);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_property_count).unwrap(), p_properties)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_display_properties_khr)(
                    physical_device,
                    p_property_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_display_plane_properties_khr(
        physical_device: vk::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut vk::DisplayPlanePropertiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceDisplayPlanePropertiesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_display;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_display_plane_properties_khr(physical_device);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_property_count).unwrap(), p_properties)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_display_plane_properties_khr)(
                    physical_device,
                    p_property_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_display_plane_supported_displays_khr(
        physical_device: vk::PhysicalDevice,
        plane_index: u32,
        p_display_count: *mut u32,
        p_displays: *mut vk::DisplayKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDisplayPlaneSupportedDisplaysKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_display;
        let layer_result = instance_info
            .customized_info
            .get_display_plane_supported_displays_khr(physical_device, plane_index);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_display_count).unwrap(), p_displays)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_display_plane_supported_displays_khr)(
                    physical_device,
                    plane_index,
                    p_display_count,
                    p_displays,
                )
            },
        }
    }
    extern "system" fn get_display_mode_properties_khr(
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut vk::DisplayModePropertiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDisplayModePropertiesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_display;
        let layer_result = instance_info
            .customized_info
            .get_display_mode_properties_khr(physical_device, display);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_property_count).unwrap(), p_properties)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_display_mode_properties_khr)(
                    physical_device,
                    display,
                    p_property_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn create_display_mode_khr(
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        p_create_info: *const vk::DisplayModeCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_mode: *mut vk::DisplayModeKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateDisplayModeKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_display;
        let layer_result = instance_info.customized_info.create_display_mode_khr(
            physical_device,
            display,
            unsafe { p_create_info.as_ref() }.unwrap(),
            unsafe { p_allocator.as_ref() },
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_mode.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_display_mode_khr)(
                    physical_device,
                    display,
                    p_create_info,
                    p_allocator,
                    p_mode,
                )
            },
        }
    }
    extern "system" fn get_display_plane_capabilities_khr(
        physical_device: vk::PhysicalDevice,
        mode: vk::DisplayModeKHR,
        plane_index: u32,
        p_capabilities: *mut vk::DisplayPlaneCapabilitiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDisplayPlaneCapabilitiesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_display;
        let layer_result = instance_info
            .customized_info
            .get_display_plane_capabilities_khr(physical_device, mode, plane_index);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_capabilities.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_display_plane_capabilities_khr)(
                    physical_device,
                    mode,
                    plane_index,
                    p_capabilities,
                )
            },
        }
    }
    extern "system" fn create_display_plane_surface_khr(
        instance: vk::Instance,
        p_create_info: *const vk::DisplaySurfaceCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateDisplayPlaneSurfaceKHR
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_display;
        let layer_result = instance_info
            .customized_info
            .create_display_plane_surface_khr(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_display_plane_surface_khr)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn create_xlib_surface_khr(
        instance: vk::Instance,
        p_create_info: *const vk::XlibSurfaceCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateXlibSurfaceKHR
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_xlib_surface;
        let layer_result = instance_info
            .customized_info
            .create_xlib_surface_khr(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_xlib_surface_khr)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn get_physical_device_xlib_presentation_support_khr(
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut vk::Display,
        visual_id: vk::VisualID,
    ) -> vk::Bool32 {
        let global = Self::instance();
        // vkGetPhysicalDeviceXlibPresentationSupportKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_xlib_surface;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_xlib_presentation_support_khr(
                physical_device,
                queue_family_index,
                unsafe { dpy.as_mut() }.unwrap(),
                visual_id,
            );
        match layer_result {
            LayerResult::Handled(res) => {
                if res {
                    vk::TRUE
                } else {
                    vk::FALSE
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_xlib_presentation_support_khr)(
                    physical_device,
                    queue_family_index,
                    dpy,
                    visual_id,
                )
            },
        }
    }
    extern "system" fn create_xcb_surface_khr(
        instance: vk::Instance,
        p_create_info: *const vk::XcbSurfaceCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateXcbSurfaceKHR
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_xcb_surface;
        let layer_result = instance_info
            .customized_info
            .create_xcb_surface_khr(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_xcb_surface_khr)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn get_physical_device_xcb_presentation_support_khr(
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        connection: *mut vk::xcb_connection_t,
        visual_id: vk::xcb_visualid_t,
    ) -> vk::Bool32 {
        let global = Self::instance();
        // vkGetPhysicalDeviceXcbPresentationSupportKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_xcb_surface;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_xcb_presentation_support_khr(
                physical_device,
                queue_family_index,
                unsafe { connection.as_mut() }.unwrap(),
                visual_id,
            );
        match layer_result {
            LayerResult::Handled(res) => {
                if res {
                    vk::TRUE
                } else {
                    vk::FALSE
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_xcb_presentation_support_khr)(
                    physical_device,
                    queue_family_index,
                    connection,
                    visual_id,
                )
            },
        }
    }
    extern "system" fn create_wayland_surface_khr(
        instance: vk::Instance,
        p_create_info: *const vk::WaylandSurfaceCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateWaylandSurfaceKHR
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_wayland_surface;
        let layer_result = instance_info
            .customized_info
            .create_wayland_surface_khr(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_wayland_surface_khr)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn get_physical_device_wayland_presentation_support_khr(
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        display: *mut vk::wl_display,
    ) -> vk::Bool32 {
        let global = Self::instance();
        // vkGetPhysicalDeviceWaylandPresentationSupportKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_wayland_surface;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_wayland_presentation_support_khr(
                physical_device,
                queue_family_index,
                unsafe { display.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                if res {
                    vk::TRUE
                } else {
                    vk::FALSE
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_wayland_presentation_support_khr)(
                    physical_device,
                    queue_family_index,
                    display,
                )
            },
        }
    }
    extern "system" fn create_android_surface_khr(
        instance: vk::Instance,
        p_create_info: *const vk::AndroidSurfaceCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateAndroidSurfaceKHR
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_android_surface;
        let layer_result = instance_info
            .customized_info
            .create_android_surface_khr(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_android_surface_khr)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn create_win32_surface_khr(
        instance: vk::Instance,
        p_create_info: *const vk::Win32SurfaceCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateWin32SurfaceKHR
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_win32_surface;
        let layer_result = instance_info
            .customized_info
            .create_win32_surface_khr(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_win32_surface_khr)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn get_physical_device_win32_presentation_support_khr(
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> vk::Bool32 {
        let global = Self::instance();
        // vkGetPhysicalDeviceWin32PresentationSupportKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_win32_surface;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_win32_presentation_support_khr(
                physical_device,
                queue_family_index,
            );
        match layer_result {
            LayerResult::Handled(res) => {
                if res {
                    vk::TRUE
                } else {
                    vk::FALSE
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_win32_presentation_support_khr)(
                    physical_device,
                    queue_family_index,
                )
            },
        }
    }
    extern "system" fn get_physical_device_video_capabilities_khr(
        physical_device: vk::PhysicalDevice,
        p_video_profile: *const vk::VideoProfileInfoKHR,
        p_capabilities: *mut vk::VideoCapabilitiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceVideoCapabilitiesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_video_queue;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_video_capabilities_khr(
                physical_device,
                unsafe { p_video_profile.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_capabilities.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_video_capabilities_khr)(
                    physical_device,
                    p_video_profile,
                    p_capabilities,
                )
            },
        }
    }
    extern "system" fn get_physical_device_video_format_properties_khr(
        physical_device: vk::PhysicalDevice,
        p_video_format_info: *const vk::PhysicalDeviceVideoFormatInfoKHR,
        p_video_format_property_count: *mut u32,
        p_video_format_properties: *mut vk::VideoFormatPropertiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceVideoFormatPropertiesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_video_queue;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_video_format_properties_khr(
                physical_device,
                unsafe { p_video_format_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_video_format_property_count).unwrap(),
                        p_video_format_properties,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_video_format_properties_khr)(
                    physical_device,
                    p_video_format_info,
                    p_video_format_property_count,
                    p_video_format_properties,
                )
            },
        }
    }
    extern "system" fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut vk::PerformanceCounterKHR,
        p_counter_descriptions: *mut vk::PerformanceCounterDescriptionKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_performance_query;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .enumerate_physical_device_queue_family_performance_query_counters_khr(
                physical_device,
                queue_family_index,
                if p_counters.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_counters,
                            *unsafe { p_counter_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_counter_count).unwrap(),
                        p_counter_descriptions,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table
                    .enumerate_physical_device_queue_family_performance_query_counters_khr)(
                    physical_device,
                    queue_family_index,
                    p_counter_count,
                    p_counters,
                    p_counter_descriptions,
                )
            },
        }
    }
    extern "system" fn get_physical_device_queue_family_performance_query_passes_khr(
        physical_device: vk::PhysicalDevice,
        p_performance_query_create_info: *const vk::QueryPoolPerformanceCreateInfoKHR,
        p_num_passes: *mut u32,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_performance_query;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_queue_family_performance_query_passes_khr(
                physical_device,
                unsafe { p_performance_query_create_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_num_passes.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_queue_family_performance_query_passes_khr)(
                    physical_device,
                    p_performance_query_create_info,
                    p_num_passes,
                )
            },
        }
    }
    extern "system" fn get_physical_device_surface_capabilities2_khr(
        physical_device: vk::PhysicalDevice,
        p_surface_info: *const vk::PhysicalDeviceSurfaceInfo2KHR,
        p_surface_capabilities: *mut vk::SurfaceCapabilities2KHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceSurfaceCapabilities2KHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_get_surface_capabilities2;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_surface_capabilities2_khr(
                physical_device,
                unsafe { p_surface_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface_capabilities.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_surface_capabilities2_khr)(
                    physical_device,
                    p_surface_info,
                    p_surface_capabilities,
                )
            },
        }
    }
    extern "system" fn get_physical_device_surface_formats2_khr(
        physical_device: vk::PhysicalDevice,
        p_surface_info: *const vk::PhysicalDeviceSurfaceInfo2KHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut vk::SurfaceFormat2KHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceSurfaceFormats2KHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_get_surface_capabilities2;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_surface_formats2_khr(
                physical_device,
                unsafe { p_surface_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_surface_format_count).unwrap(),
                        p_surface_formats,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_surface_formats2_khr)(
                    physical_device,
                    p_surface_info,
                    p_surface_format_count,
                    p_surface_formats,
                )
            },
        }
    }
    extern "system" fn get_physical_device_display_properties2_khr(
        physical_device: vk::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut vk::DisplayProperties2KHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceDisplayProperties2KHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_get_display_properties2;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_display_properties2_khr(physical_device);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_property_count).unwrap(), p_properties)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_display_properties2_khr)(
                    physical_device,
                    p_property_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_display_plane_properties2_khr(
        physical_device: vk::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut vk::DisplayPlaneProperties2KHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceDisplayPlaneProperties2KHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_get_display_properties2;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_display_plane_properties2_khr(physical_device);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_property_count).unwrap(), p_properties)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_display_plane_properties2_khr)(
                    physical_device,
                    p_property_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_display_mode_properties2_khr(
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut vk::DisplayModeProperties2KHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDisplayModeProperties2KHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_get_display_properties2;
        let layer_result = instance_info
            .customized_info
            .get_display_mode_properties2_khr(physical_device, display);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_property_count).unwrap(), p_properties)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_display_mode_properties2_khr)(
                    physical_device,
                    display,
                    p_property_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_display_plane_capabilities2_khr(
        physical_device: vk::PhysicalDevice,
        p_display_plane_info: *const vk::DisplayPlaneInfo2KHR,
        p_capabilities: *mut vk::DisplayPlaneCapabilities2KHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDisplayPlaneCapabilities2KHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_get_display_properties2;
        let layer_result = instance_info
            .customized_info
            .get_display_plane_capabilities2_khr(
                physical_device,
                unsafe { p_display_plane_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_capabilities.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_display_plane_capabilities2_khr)(
                    physical_device,
                    p_display_plane_info,
                    p_capabilities,
                )
            },
        }
    }
    extern "system" fn get_physical_device_fragment_shading_rates_khr(
        physical_device: vk::PhysicalDevice,
        p_fragment_shading_rate_count: *mut u32,
        p_fragment_shading_rates: *mut vk::PhysicalDeviceFragmentShadingRateKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceFragmentShadingRatesKHR
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.khr_fragment_shading_rate;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_fragment_shading_rates_khr(physical_device);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_fragment_shading_rate_count).unwrap(),
                        p_fragment_shading_rates,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_fragment_shading_rates_khr)(
                    physical_device,
                    p_fragment_shading_rate_count,
                    p_fragment_shading_rates,
                )
            },
        }
    }
    extern "system" fn create_debug_report_callback_ext(
        instance: vk::Instance,
        p_create_info: *const vk::DebugReportCallbackCreateInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
        p_callback: *mut vk::DebugReportCallbackEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateDebugReportCallbackEXT
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_debug_report;
        let layer_result = instance_info
            .customized_info
            .create_debug_report_callback_ext(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_callback.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_debug_report_callback_ext)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_callback,
                )
            },
        }
    }
    extern "system" fn destroy_debug_report_callback_ext(
        instance: vk::Instance,
        callback: vk::DebugReportCallbackEXT,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyDebugReportCallbackEXT
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_debug_report;
        let layer_result = instance_info
            .customized_info
            .destroy_debug_report_callback_ext(callback, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_debug_report_callback_ext)(instance, callback, p_allocator)
            },
        }
    }
    extern "system" fn debug_report_message_ext(
        instance: vk::Instance,
        flags: vk::DebugReportFlagsEXT,
        object_type: vk::DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
    ) {
        let global = Self::instance();
        // vkDebugReportMessageEXT
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_debug_report;
        let layer_result = instance_info.customized_info.debug_report_message_ext(
            flags,
            object_type,
            object,
            location,
            message_code,
            unsafe { CStr::from_ptr(p_layer_prefix) }.to_str().unwrap(),
            unsafe { CStr::from_ptr(p_message) }.to_str().unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.debug_report_message_ext)(
                    instance,
                    flags,
                    object_type,
                    object,
                    location,
                    message_code,
                    p_layer_prefix,
                    p_message,
                )
            },
        }
    }
    extern "system" fn create_stream_descriptor_surface_ggp(
        instance: vk::Instance,
        p_create_info: *const vk::StreamDescriptorSurfaceCreateInfoGGP,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateStreamDescriptorSurfaceGGP
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ggp_stream_descriptor_surface;
        let layer_result = instance_info
            .customized_info
            .create_stream_descriptor_surface_ggp(
                unsafe { p_create_info.as_ref() }.unwrap(),
                unsafe { p_allocator.as_ref() },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_stream_descriptor_surface_ggp)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn get_physical_device_external_image_format_properties_nv(
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        _type: vk::ImageType,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        flags: vk::ImageCreateFlags,
        external_handle_type: vk::ExternalMemoryHandleTypeFlagsNV,
        p_external_image_format_properties: *mut vk::ExternalImageFormatPropertiesNV,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceExternalImageFormatPropertiesNV
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.nv_external_memory_capabilities;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_external_image_format_properties_nv(
                physical_device,
                format,
                _type,
                tiling,
                usage,
                flags,
                external_handle_type,
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_external_image_format_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_external_image_format_properties_nv)(
                    physical_device,
                    format,
                    _type,
                    tiling,
                    usage,
                    flags,
                    external_handle_type,
                    p_external_image_format_properties,
                )
            },
        }
    }
    extern "system" fn create_vi_surface_nn(
        instance: vk::Instance,
        p_create_info: *const vk::ViSurfaceCreateInfoNN,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateViSurfaceNN
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.nn_vi_surface;
        let layer_result = instance_info
            .customized_info
            .create_vi_surface_nn(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_vi_surface_nn)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn release_display_ext(
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkReleaseDisplayEXT
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_direct_mode_display;
        let layer_result = instance_info
            .customized_info
            .release_display_ext(physical_device, display);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.release_display_ext)(physical_device, display)
            },
        }
    }
    extern "system" fn acquire_xlib_display_ext(
        physical_device: vk::PhysicalDevice,
        dpy: *mut vk::Display,
        display: vk::DisplayKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAcquireXlibDisplayEXT
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_acquire_xlib_display;
        let layer_result = instance_info.customized_info.acquire_xlib_display_ext(
            physical_device,
            unsafe { dpy.as_mut() }.unwrap(),
            display,
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.acquire_xlib_display_ext)(physical_device, dpy, display)
            },
        }
    }
    extern "system" fn get_rand_r_output_display_ext(
        physical_device: vk::PhysicalDevice,
        dpy: *mut vk::Display,
        rr_output: vk::RROutput,
        p_display: *mut vk::DisplayKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetRandROutputDisplayEXT
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_acquire_xlib_display;
        let layer_result = instance_info.customized_info.get_rand_r_output_display_ext(
            physical_device,
            unsafe { dpy.as_mut() }.unwrap(),
            rr_output,
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_display.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_rand_r_output_display_ext)(
                    physical_device,
                    dpy,
                    rr_output,
                    p_display,
                )
            },
        }
    }
    extern "system" fn get_physical_device_surface_capabilities2_ext(
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        p_surface_capabilities: *mut vk::SurfaceCapabilities2EXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceSurfaceCapabilities2EXT
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_display_surface_counter;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_surface_capabilities2_ext(physical_device, surface);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface_capabilities.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_surface_capabilities2_ext)(
                    physical_device,
                    surface,
                    p_surface_capabilities,
                )
            },
        }
    }
    extern "system" fn create_ios_surface_mvk(
        instance: vk::Instance,
        p_create_info: *const vk::IOSSurfaceCreateInfoMVK,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateIOSSurfaceMVK
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.mvk_ios_surface;
        let layer_result = instance_info
            .customized_info
            .create_ios_surface_mvk(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_ios_surface_mvk)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn create_mac_os_surface_mvk(
        instance: vk::Instance,
        p_create_info: *const vk::MacOSSurfaceCreateInfoMVK,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateMacOSSurfaceMVK
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.mvk_macos_surface;
        let layer_result = instance_info
            .customized_info
            .create_mac_os_surface_mvk(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_mac_os_surface_mvk)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn create_debug_utils_messenger_ext(
        instance: vk::Instance,
        p_create_info: *const vk::DebugUtilsMessengerCreateInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
        p_messenger: *mut vk::DebugUtilsMessengerEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateDebugUtilsMessengerEXT
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_debug_utils;
        let layer_result = instance_info
            .customized_info
            .create_debug_utils_messenger_ext(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_messenger.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_debug_utils_messenger_ext)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_messenger,
                )
            },
        }
    }
    extern "system" fn destroy_debug_utils_messenger_ext(
        instance: vk::Instance,
        messenger: vk::DebugUtilsMessengerEXT,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyDebugUtilsMessengerEXT
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_debug_utils;
        let layer_result = instance_info
            .customized_info
            .destroy_debug_utils_messenger_ext(messenger, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_debug_utils_messenger_ext)(instance, messenger, p_allocator)
            },
        }
    }
    extern "system" fn submit_debug_utils_message_ext(
        instance: vk::Instance,
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_types: vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    ) {
        let global = Self::instance();
        // vkSubmitDebugUtilsMessageEXT
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_debug_utils;
        let layer_result = instance_info
            .customized_info
            .submit_debug_utils_message_ext(
                message_severity,
                message_types,
                unsafe { p_callback_data.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.submit_debug_utils_message_ext)(
                    instance,
                    message_severity,
                    message_types,
                    p_callback_data,
                )
            },
        }
    }
    extern "system" fn get_physical_device_multisample_properties_ext(
        physical_device: vk::PhysicalDevice,
        samples: vk::SampleCountFlags,
        p_multisample_properties: *mut vk::MultisamplePropertiesEXT,
    ) {
        let global = Self::instance();
        // vkGetPhysicalDeviceMultisamplePropertiesEXT
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_sample_locations;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_multisample_properties_ext(physical_device, samples);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_multisample_properties.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_multisample_properties_ext)(
                    physical_device,
                    samples,
                    p_multisample_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_calibrateable_time_domains_ext(
        physical_device: vk::PhysicalDevice,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut vk::TimeDomainEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceCalibrateableTimeDomainsEXT
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_calibrated_timestamps;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_calibrateable_time_domains_ext(physical_device);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_time_domain_count).unwrap(),
                        p_time_domains,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_calibrateable_time_domains_ext)(
                    physical_device,
                    p_time_domain_count,
                    p_time_domains,
                )
            },
        }
    }
    extern "system" fn create_image_pipe_surface_fuchsia(
        instance: vk::Instance,
        p_create_info: *const vk::ImagePipeSurfaceCreateInfoFUCHSIA,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateImagePipeSurfaceFUCHSIA
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.fuchsia_imagepipe_surface;
        let layer_result = instance_info
            .customized_info
            .create_image_pipe_surface_fuchsia(
                unsafe { p_create_info.as_ref() }.unwrap(),
                unsafe { p_allocator.as_ref() },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_image_pipe_surface_fuchsia)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn create_metal_surface_ext(
        instance: vk::Instance,
        p_create_info: *const vk::MetalSurfaceCreateInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateMetalSurfaceEXT
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_metal_surface;
        let layer_result = instance_info
            .customized_info
            .create_metal_surface_ext(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_metal_surface_ext)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn get_physical_device_cooperative_matrix_properties_nv(
        physical_device: vk::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut vk::CooperativeMatrixPropertiesNV,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceCooperativeMatrixPropertiesNV
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.nv_cooperative_matrix;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_cooperative_matrix_properties_nv(physical_device);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_property_count).unwrap(), p_properties)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_cooperative_matrix_properties_nv)(
                    physical_device,
                    p_property_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        physical_device: vk::PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut vk::FramebufferMixedSamplesCombinationNV,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.nv_coverage_reduction_mode;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
                physical_device,
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_combination_count).unwrap(),
                        p_combinations,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table
                    .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv)(
                    physical_device,
                    p_combination_count,
                    p_combinations,
                )
            },
        }
    }
    extern "system" fn get_physical_device_surface_present_modes2_ext(
        physical_device: vk::PhysicalDevice,
        p_surface_info: *const vk::PhysicalDeviceSurfaceInfo2KHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut vk::PresentModeKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceSurfacePresentModes2EXT
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_full_screen_exclusive;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_surface_present_modes2_ext(
                physical_device,
                unsafe { p_surface_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_present_mode_count).unwrap(),
                        p_present_modes,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_surface_present_modes2_ext)(
                    physical_device,
                    p_surface_info,
                    p_present_mode_count,
                    p_present_modes,
                )
            },
        }
    }
    extern "system" fn create_headless_surface_ext(
        instance: vk::Instance,
        p_create_info: *const vk::HeadlessSurfaceCreateInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateHeadlessSurfaceEXT
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_headless_surface;
        let layer_result = instance_info
            .customized_info
            .create_headless_surface_ext(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_headless_surface_ext)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn acquire_drm_display_ext(
        physical_device: vk::PhysicalDevice,
        drm_fd: i32,
        display: vk::DisplayKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAcquireDrmDisplayEXT
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_acquire_drm_display;
        let layer_result =
            instance_info
                .customized_info
                .acquire_drm_display_ext(physical_device, drm_fd, display);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.acquire_drm_display_ext)(physical_device, drm_fd, display)
            },
        }
    }
    extern "system" fn get_drm_display_ext(
        physical_device: vk::PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
        display: *mut vk::DisplayKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDrmDisplayEXT
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_acquire_drm_display;
        let layer_result = instance_info.customized_info.get_drm_display_ext(
            physical_device,
            drm_fd,
            connector_id,
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { display.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_drm_display_ext)(physical_device, drm_fd, connector_id, display)
            },
        }
    }
    extern "system" fn acquire_winrt_display_nv(
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAcquireWinrtDisplayNV
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.nv_acquire_winrt_display;
        let layer_result = instance_info
            .customized_info
            .acquire_winrt_display_nv(physical_device, display);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.acquire_winrt_display_nv)(physical_device, display)
            },
        }
    }
    extern "system" fn get_winrt_display_nv(
        physical_device: vk::PhysicalDevice,
        device_relative_id: u32,
        p_display: *mut vk::DisplayKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetWinrtDisplayNV
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.nv_acquire_winrt_display;
        let layer_result = instance_info
            .customized_info
            .get_winrt_display_nv(physical_device, device_relative_id);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_display.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_winrt_display_nv)(
                    physical_device,
                    device_relative_id,
                    p_display,
                )
            },
        }
    }
    extern "system" fn create_direct_fb_surface_ext(
        instance: vk::Instance,
        p_create_info: *const vk::DirectFBSurfaceCreateInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateDirectFBSurfaceEXT
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_directfb_surface;
        let layer_result = instance_info
            .customized_info
            .create_direct_fb_surface_ext(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_direct_fb_surface_ext)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn get_physical_device_direct_fb_presentation_support_ext(
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut vk::IDirectFB,
    ) -> vk::Bool32 {
        let global = Self::instance();
        // vkGetPhysicalDeviceDirectFBPresentationSupportEXT
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.ext_directfb_surface;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_direct_fb_presentation_support_ext(
                physical_device,
                queue_family_index,
                unsafe { dfb.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                if res {
                    vk::TRUE
                } else {
                    vk::FALSE
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_direct_fb_presentation_support_ext)(
                    physical_device,
                    queue_family_index,
                    dfb,
                )
            },
        }
    }
    extern "system" fn create_screen_surface_qnx(
        instance: vk::Instance,
        p_create_info: *const vk::ScreenSurfaceCreateInfoQNX,
        p_allocator: *const vk::AllocationCallbacks,
        p_surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateScreenSurfaceQNX
        let instance_info = global.get_instance_info(instance).unwrap();
        let dispatch_table = &instance_info.dispatch_table.qnx_screen_surface;
        let layer_result = instance_info
            .customized_info
            .create_screen_surface_qnx(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_surface.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_screen_surface_qnx)(
                    instance,
                    p_create_info,
                    p_allocator,
                    p_surface,
                )
            },
        }
    }
    extern "system" fn get_physical_device_screen_presentation_support_qnx(
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        window: *mut vk::_screen_window,
    ) -> vk::Bool32 {
        let global = Self::instance();
        // vkGetPhysicalDeviceScreenPresentationSupportQNX
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.qnx_screen_surface;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_screen_presentation_support_qnx(
                physical_device,
                queue_family_index,
                unsafe { window.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                if res {
                    vk::TRUE
                } else {
                    vk::FALSE
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_screen_presentation_support_qnx)(
                    physical_device,
                    queue_family_index,
                    window,
                )
            },
        }
    }
    extern "system" fn get_physical_device_optical_flow_image_formats_nv(
        physical_device: vk::PhysicalDevice,
        p_optical_flow_image_format_info: *const vk::OpticalFlowImageFormatInfoNV,
        p_format_count: *mut u32,
        p_image_format_properties: *mut vk::OpticalFlowImageFormatPropertiesNV,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPhysicalDeviceOpticalFlowImageFormatsNV
        let instance_info = global.get_instance_info(physical_device).unwrap();
        let dispatch_table = &instance_info.dispatch_table.nv_optical_flow;
        let layer_result = instance_info
            .customized_info
            .get_physical_device_optical_flow_image_formats_nv(
                physical_device,
                unsafe { p_optical_flow_image_format_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_format_count).unwrap(),
                        p_image_format_properties,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_physical_device_optical_flow_image_formats_nv)(
                    physical_device,
                    p_optical_flow_image_format_info,
                    p_format_count,
                    p_image_format_properties,
                )
            },
        }
    }
    extern "system" fn get_device_queue(
        device: vk::Device,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut vk::Queue,
    ) {
        let global = Self::instance();
        // vkGetDeviceQueue
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .get_device_queue(queue_family_index, queue_index);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_queue.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_queue)(device, queue_family_index, queue_index, p_queue)
            },
        }
    }
    extern "system" fn queue_submit(
        queue: vk::Queue,
        submit_count: u32,
        p_submits: *const vk::SubmitInfo,
        fence: vk::Fence,
    ) -> vk::Result {
        let global = Self::instance();
        // vkQueueSubmit
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.queue_submit(
            queue,
            unsafe { std::slice::from_raw_parts(p_submits, submit_count as usize) },
            fence,
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.queue_submit)(queue, submit_count, p_submits, fence)
            },
        }
    }
    extern "system" fn queue_wait_idle(queue: vk::Queue) -> vk::Result {
        let global = Self::instance();
        // vkQueueWaitIdle
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.queue_wait_idle(queue);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe { (dispatch_table.queue_wait_idle)(queue) },
        }
    }
    extern "system" fn device_wait_idle(device: vk::Device) -> vk::Result {
        let global = Self::instance();
        // vkDeviceWaitIdle
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.device_wait_idle();
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe { (dispatch_table.device_wait_idle)(device) },
        }
    }
    extern "system" fn allocate_memory(
        device: vk::Device,
        p_allocate_info: *const vk::MemoryAllocateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_memory: *mut vk::DeviceMemory,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAllocateMemory
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .allocate_memory(unsafe { p_allocate_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_memory.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.allocate_memory)(device, p_allocate_info, p_allocator, p_memory)
            },
        }
    }
    extern "system" fn free_memory(
        device: vk::Device,
        memory: vk::DeviceMemory,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkFreeMemory
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .free_memory(memory, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.free_memory)(device, memory, p_allocator)
            },
        }
    }
    extern "system" fn map_memory(
        device: vk::Device,
        memory: vk::DeviceMemory,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        flags: vk::MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkMapMemory
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .map_memory(memory, offset, size, flags);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { pp_data.as_mut() }.unwrap() = res.unwrap_or(std::ptr::null_mut());
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.map_memory)(device, memory, offset, size, flags, pp_data)
            },
        }
    }
    extern "system" fn unmap_memory(device: vk::Device, memory: vk::DeviceMemory) {
        let global = Self::instance();
        // vkUnmapMemory
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.unmap_memory(memory);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe { (dispatch_table.unmap_memory)(device, memory) },
        }
    }
    extern "system" fn flush_mapped_memory_ranges(
        device: vk::Device,
        memory_range_count: u32,
        p_memory_ranges: *const vk::MappedMemoryRange,
    ) -> vk::Result {
        let global = Self::instance();
        // vkFlushMappedMemoryRanges
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .flush_mapped_memory_ranges(unsafe {
                std::slice::from_raw_parts(p_memory_ranges, memory_range_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.flush_mapped_memory_ranges)(
                    device,
                    memory_range_count,
                    p_memory_ranges,
                )
            },
        }
    }
    extern "system" fn invalidate_mapped_memory_ranges(
        device: vk::Device,
        memory_range_count: u32,
        p_memory_ranges: *const vk::MappedMemoryRange,
    ) -> vk::Result {
        let global = Self::instance();
        // vkInvalidateMappedMemoryRanges
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .invalidate_mapped_memory_ranges(unsafe {
                std::slice::from_raw_parts(p_memory_ranges, memory_range_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.invalidate_mapped_memory_ranges)(
                    device,
                    memory_range_count,
                    p_memory_ranges,
                )
            },
        }
    }
    extern "system" fn get_device_memory_commitment(
        device: vk::Device,
        memory: vk::DeviceMemory,
        p_committed_memory_in_bytes: *mut vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkGetDeviceMemoryCommitment
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .get_device_memory_commitment(memory);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_committed_memory_in_bytes.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_memory_commitment)(
                    device,
                    memory,
                    p_committed_memory_in_bytes,
                )
            },
        }
    }
    extern "system" fn bind_buffer_memory(
        device: vk::Device,
        buffer: vk::Buffer,
        memory: vk::DeviceMemory,
        memory_offset: vk::DeviceSize,
    ) -> vk::Result {
        let global = Self::instance();
        // vkBindBufferMemory
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result =
            device_info
                .customized_info
                .bind_buffer_memory(buffer, memory, memory_offset);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.bind_buffer_memory)(device, buffer, memory, memory_offset)
            },
        }
    }
    extern "system" fn bind_image_memory(
        device: vk::Device,
        image: vk::Image,
        memory: vk::DeviceMemory,
        memory_offset: vk::DeviceSize,
    ) -> vk::Result {
        let global = Self::instance();
        // vkBindImageMemory
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result =
            device_info
                .customized_info
                .bind_image_memory(image, memory, memory_offset);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.bind_image_memory)(device, image, memory, memory_offset)
            },
        }
    }
    extern "system" fn get_buffer_memory_requirements(
        device: vk::Device,
        buffer: vk::Buffer,
        p_memory_requirements: *mut vk::MemoryRequirements,
    ) {
        let global = Self::instance();
        // vkGetBufferMemoryRequirements
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .get_buffer_memory_requirements(buffer);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_memory_requirements.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_buffer_memory_requirements)(
                    device,
                    buffer,
                    p_memory_requirements,
                )
            },
        }
    }
    extern "system" fn get_image_memory_requirements(
        device: vk::Device,
        image: vk::Image,
        p_memory_requirements: *mut vk::MemoryRequirements,
    ) {
        let global = Self::instance();
        // vkGetImageMemoryRequirements
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .get_image_memory_requirements(image);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_memory_requirements.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_memory_requirements)(device, image, p_memory_requirements)
            },
        }
    }
    extern "system" fn get_image_sparse_memory_requirements(
        device: vk::Device,
        image: vk::Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut vk::SparseImageMemoryRequirements,
    ) {
        let global = Self::instance();
        // vkGetImageSparseMemoryRequirements
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .get_image_sparse_memory_requirements(image);
        match layer_result {
            LayerResult::Handled(res) => {
                // We can't return INCOMPLETE from vkGetImageSparseMemoryRequirements
                #[allow(unused_must_use)]
                unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_sparse_memory_requirement_count).unwrap(),
                        p_sparse_memory_requirements,
                    );
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_sparse_memory_requirements)(
                    device,
                    image,
                    p_sparse_memory_requirement_count,
                    p_sparse_memory_requirements,
                )
            },
        }
    }
    extern "system" fn queue_bind_sparse(
        queue: vk::Queue,
        bind_info_count: u32,
        p_bind_info: *const vk::BindSparseInfo,
        fence: vk::Fence,
    ) -> vk::Result {
        let global = Self::instance();
        // vkQueueBindSparse
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.queue_bind_sparse(
            queue,
            unsafe { std::slice::from_raw_parts(p_bind_info, bind_info_count as usize) },
            fence,
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.queue_bind_sparse)(queue, bind_info_count, p_bind_info, fence)
            },
        }
    }
    extern "system" fn create_fence(
        device: vk::Device,
        p_create_info: *const vk::FenceCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_fence: *mut vk::Fence,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateFence
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_fence(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_fence.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_fence)(device, p_create_info, p_allocator, p_fence)
            },
        }
    }
    extern "system" fn destroy_fence(
        device: vk::Device,
        fence: vk::Fence,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyFence
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_fence(fence, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_fence)(device, fence, p_allocator)
            },
        }
    }
    extern "system" fn reset_fences(
        device: vk::Device,
        fence_count: u32,
        p_fences: *const vk::Fence,
    ) -> vk::Result {
        let global = Self::instance();
        // vkResetFences
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .reset_fences(unsafe { std::slice::from_raw_parts(p_fences, fence_count as usize) });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.reset_fences)(device, fence_count, p_fences)
            },
        }
    }
    extern "system" fn get_fence_status(device: vk::Device, fence: vk::Fence) -> vk::Result {
        let global = Self::instance();
        // vkGetFenceStatus
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.get_fence_status(fence);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe { (dispatch_table.get_fence_status)(device, fence) },
        }
    }
    extern "system" fn wait_for_fences(
        device: vk::Device,
        fence_count: u32,
        p_fences: *const vk::Fence,
        wait_all: vk::Bool32,
        timeout: u64,
    ) -> vk::Result {
        let global = Self::instance();
        // vkWaitForFences
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.wait_for_fences(
            unsafe { std::slice::from_raw_parts(p_fences, fence_count as usize) },
            wait_all == vk::TRUE,
            timeout,
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.wait_for_fences)(device, fence_count, p_fences, wait_all, timeout)
            },
        }
    }
    extern "system" fn create_semaphore(
        device: vk::Device,
        p_create_info: *const vk::SemaphoreCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_semaphore: *mut vk::Semaphore,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateSemaphore
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_semaphore(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_semaphore.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_semaphore)(device, p_create_info, p_allocator, p_semaphore)
            },
        }
    }
    extern "system" fn destroy_semaphore(
        device: vk::Device,
        semaphore: vk::Semaphore,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroySemaphore
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_semaphore(semaphore, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_semaphore)(device, semaphore, p_allocator)
            },
        }
    }
    extern "system" fn create_event(
        device: vk::Device,
        p_create_info: *const vk::EventCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_event: *mut vk::Event,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateEvent
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_event(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_event.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_event)(device, p_create_info, p_allocator, p_event)
            },
        }
    }
    extern "system" fn destroy_event(
        device: vk::Device,
        event: vk::Event,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyEvent
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_event(event, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_event)(device, event, p_allocator)
            },
        }
    }
    extern "system" fn get_event_status(device: vk::Device, event: vk::Event) -> vk::Result {
        let global = Self::instance();
        // vkGetEventStatus
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.get_event_status(event);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe { (dispatch_table.get_event_status)(device, event) },
        }
    }
    extern "system" fn set_event(device: vk::Device, event: vk::Event) -> vk::Result {
        let global = Self::instance();
        // vkSetEvent
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.set_event(event);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe { (dispatch_table.set_event)(device, event) },
        }
    }
    extern "system" fn reset_event(device: vk::Device, event: vk::Event) -> vk::Result {
        let global = Self::instance();
        // vkResetEvent
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.reset_event(event);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe { (dispatch_table.reset_event)(device, event) },
        }
    }
    extern "system" fn create_query_pool(
        device: vk::Device,
        p_create_info: *const vk::QueryPoolCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_query_pool: *mut vk::QueryPool,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateQueryPool
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_query_pool(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_query_pool.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_query_pool)(device, p_create_info, p_allocator, p_query_pool)
            },
        }
    }
    extern "system" fn destroy_query_pool(
        device: vk::Device,
        query_pool: vk::QueryPool,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyQueryPool
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_query_pool(query_pool, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_query_pool)(device, query_pool, p_allocator)
            },
        }
    }
    extern "system" fn get_query_pool_results(
        device: vk::Device,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
        stride: vk::DeviceSize,
        flags: vk::QueryResultFlags,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetQueryPoolResults
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.get_query_pool_results(
            query_pool,
            first_query,
            query_count,
            unsafe { std::slice::from_raw_parts_mut(p_data as *mut u8, data_size as usize) },
            stride,
            flags,
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_query_pool_results)(
                    device,
                    query_pool,
                    first_query,
                    query_count,
                    data_size,
                    p_data,
                    stride,
                    flags,
                )
            },
        }
    }
    extern "system" fn create_buffer(
        device: vk::Device,
        p_create_info: *const vk::BufferCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_buffer: *mut vk::Buffer,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateBuffer
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_buffer(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_buffer.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_buffer)(device, p_create_info, p_allocator, p_buffer)
            },
        }
    }
    extern "system" fn destroy_buffer(
        device: vk::Device,
        buffer: vk::Buffer,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyBuffer
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_buffer(buffer, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_buffer)(device, buffer, p_allocator)
            },
        }
    }
    extern "system" fn create_buffer_view(
        device: vk::Device,
        p_create_info: *const vk::BufferViewCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_view: *mut vk::BufferView,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateBufferView
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_buffer_view(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_view.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_buffer_view)(device, p_create_info, p_allocator, p_view)
            },
        }
    }
    extern "system" fn destroy_buffer_view(
        device: vk::Device,
        buffer_view: vk::BufferView,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyBufferView
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_buffer_view(buffer_view, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_buffer_view)(device, buffer_view, p_allocator)
            },
        }
    }
    extern "system" fn create_image(
        device: vk::Device,
        p_create_info: *const vk::ImageCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_image: *mut vk::Image,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateImage
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_image(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_image.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_image)(device, p_create_info, p_allocator, p_image)
            },
        }
    }
    extern "system" fn destroy_image(
        device: vk::Device,
        image: vk::Image,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyImage
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_image(image, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_image)(device, image, p_allocator)
            },
        }
    }
    extern "system" fn get_image_subresource_layout(
        device: vk::Device,
        image: vk::Image,
        p_subresource: *const vk::ImageSubresource,
        p_layout: *mut vk::SubresourceLayout,
    ) {
        let global = Self::instance();
        // vkGetImageSubresourceLayout
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .get_image_subresource_layout(image, unsafe { p_subresource.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_layout.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_subresource_layout)(
                    device,
                    image,
                    p_subresource,
                    p_layout,
                )
            },
        }
    }
    extern "system" fn create_image_view(
        device: vk::Device,
        p_create_info: *const vk::ImageViewCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_view: *mut vk::ImageView,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateImageView
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_image_view(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_view.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_image_view)(device, p_create_info, p_allocator, p_view)
            },
        }
    }
    extern "system" fn destroy_image_view(
        device: vk::Device,
        image_view: vk::ImageView,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyImageView
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_image_view(image_view, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_image_view)(device, image_view, p_allocator)
            },
        }
    }
    extern "system" fn create_shader_module(
        device: vk::Device,
        p_create_info: *const vk::ShaderModuleCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_shader_module: *mut vk::ShaderModule,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateShaderModule
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_shader_module(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_shader_module.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_shader_module)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_shader_module,
                )
            },
        }
    }
    extern "system" fn destroy_shader_module(
        device: vk::Device,
        shader_module: vk::ShaderModule,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyShaderModule
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_shader_module(shader_module, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_shader_module)(device, shader_module, p_allocator)
            },
        }
    }
    extern "system" fn create_pipeline_cache(
        device: vk::Device,
        p_create_info: *const vk::PipelineCacheCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_pipeline_cache: *mut vk::PipelineCache,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreatePipelineCache
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_pipeline_cache(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_pipeline_cache.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_pipeline_cache)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_pipeline_cache,
                )
            },
        }
    }
    extern "system" fn destroy_pipeline_cache(
        device: vk::Device,
        pipeline_cache: vk::PipelineCache,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyPipelineCache
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_pipeline_cache(pipeline_cache, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_pipeline_cache)(device, pipeline_cache, p_allocator)
            },
        }
    }
    extern "system" fn get_pipeline_cache_data(
        device: vk::Device,
        pipeline_cache: vk::PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPipelineCacheData
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .get_pipeline_cache_data(pipeline_cache);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    let p_data = p_data as *mut u8;
                    unsafe { fill_vk_out_array(&res, NonNull::new(p_data_size).unwrap(), p_data) }
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_pipeline_cache_data)(
                    device,
                    pipeline_cache,
                    p_data_size,
                    p_data,
                )
            },
        }
    }
    extern "system" fn merge_pipeline_caches(
        device: vk::Device,
        dst_cache: vk::PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const vk::PipelineCache,
    ) -> vk::Result {
        let global = Self::instance();
        // vkMergePipelineCaches
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .merge_pipeline_caches(dst_cache, unsafe {
                std::slice::from_raw_parts(p_src_caches, src_cache_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.merge_pipeline_caches)(
                    device,
                    dst_cache,
                    src_cache_count,
                    p_src_caches,
                )
            },
        }
    }
    extern "system" fn create_graphics_pipelines(
        device: vk::Device,
        pipeline_cache: vk::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const vk::GraphicsPipelineCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_pipelines: *mut vk::Pipeline,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateGraphicsPipelines
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.create_graphics_pipelines(
            pipeline_cache,
            unsafe { std::slice::from_raw_parts(p_create_infos, create_info_count as usize) },
            unsafe { p_allocator.as_ref() },
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    unsafe {
                        std::slice::from_raw_parts_mut(
                            p_pipelines,
                            create_info_count.try_into().unwrap(),
                        )
                    }
                    .copy_from_slice(&res);
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_graphics_pipelines)(
                    device,
                    pipeline_cache,
                    create_info_count,
                    p_create_infos,
                    p_allocator,
                    p_pipelines,
                )
            },
        }
    }
    extern "system" fn create_compute_pipelines(
        device: vk::Device,
        pipeline_cache: vk::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const vk::ComputePipelineCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_pipelines: *mut vk::Pipeline,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateComputePipelines
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.create_compute_pipelines(
            pipeline_cache,
            unsafe { std::slice::from_raw_parts(p_create_infos, create_info_count as usize) },
            unsafe { p_allocator.as_ref() },
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    unsafe {
                        std::slice::from_raw_parts_mut(
                            p_pipelines,
                            create_info_count.try_into().unwrap(),
                        )
                    }
                    .copy_from_slice(&res);
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_compute_pipelines)(
                    device,
                    pipeline_cache,
                    create_info_count,
                    p_create_infos,
                    p_allocator,
                    p_pipelines,
                )
            },
        }
    }
    extern "system" fn destroy_pipeline(
        device: vk::Device,
        pipeline: vk::Pipeline,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyPipeline
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_pipeline(pipeline, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_pipeline)(device, pipeline, p_allocator)
            },
        }
    }
    extern "system" fn create_pipeline_layout(
        device: vk::Device,
        p_create_info: *const vk::PipelineLayoutCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_pipeline_layout: *mut vk::PipelineLayout,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreatePipelineLayout
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_pipeline_layout(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_pipeline_layout.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_pipeline_layout)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_pipeline_layout,
                )
            },
        }
    }
    extern "system" fn destroy_pipeline_layout(
        device: vk::Device,
        pipeline_layout: vk::PipelineLayout,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyPipelineLayout
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_pipeline_layout(pipeline_layout, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_pipeline_layout)(device, pipeline_layout, p_allocator)
            },
        }
    }
    extern "system" fn create_sampler(
        device: vk::Device,
        p_create_info: *const vk::SamplerCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_sampler: *mut vk::Sampler,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateSampler
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_sampler(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_sampler.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_sampler)(device, p_create_info, p_allocator, p_sampler)
            },
        }
    }
    extern "system" fn destroy_sampler(
        device: vk::Device,
        sampler: vk::Sampler,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroySampler
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_sampler(sampler, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_sampler)(device, sampler, p_allocator)
            },
        }
    }
    extern "system" fn create_descriptor_set_layout(
        device: vk::Device,
        p_create_info: *const vk::DescriptorSetLayoutCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_set_layout: *mut vk::DescriptorSetLayout,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateDescriptorSetLayout
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_descriptor_set_layout(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_set_layout.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_descriptor_set_layout)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_set_layout,
                )
            },
        }
    }
    extern "system" fn destroy_descriptor_set_layout(
        device: vk::Device,
        descriptor_set_layout: vk::DescriptorSetLayout,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyDescriptorSetLayout
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_descriptor_set_layout(descriptor_set_layout, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_descriptor_set_layout)(
                    device,
                    descriptor_set_layout,
                    p_allocator,
                )
            },
        }
    }
    extern "system" fn create_descriptor_pool(
        device: vk::Device,
        p_create_info: *const vk::DescriptorPoolCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_descriptor_pool: *mut vk::DescriptorPool,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateDescriptorPool
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_descriptor_pool(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_descriptor_pool.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_descriptor_pool)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_descriptor_pool,
                )
            },
        }
    }
    extern "system" fn destroy_descriptor_pool(
        device: vk::Device,
        descriptor_pool: vk::DescriptorPool,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyDescriptorPool
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_descriptor_pool(descriptor_pool, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_descriptor_pool)(device, descriptor_pool, p_allocator)
            },
        }
    }
    extern "system" fn reset_descriptor_pool(
        device: vk::Device,
        descriptor_pool: vk::DescriptorPool,
        flags: vk::DescriptorPoolResetFlags,
    ) -> vk::Result {
        let global = Self::instance();
        // vkResetDescriptorPool
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .reset_descriptor_pool(descriptor_pool, flags);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.reset_descriptor_pool)(device, descriptor_pool, flags)
            },
        }
    }
    extern "system" fn allocate_descriptor_sets(
        device: vk::Device,
        p_allocate_info: *const vk::DescriptorSetAllocateInfo,
        p_descriptor_sets: *mut vk::DescriptorSet,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAllocateDescriptorSets
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .allocate_descriptor_sets(unsafe { p_allocate_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    unsafe {
                        std::slice::from_raw_parts_mut(
                            p_descriptor_sets,
                            unsafe { p_allocate_info.as_ref().unwrap() }
                                .descriptor_set_count
                                .try_into()
                                .unwrap(),
                        )
                    }
                    .copy_from_slice(&res);
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.allocate_descriptor_sets)(
                    device,
                    p_allocate_info,
                    p_descriptor_sets,
                )
            },
        }
    }
    extern "system" fn free_descriptor_sets(
        device: vk::Device,
        descriptor_pool: vk::DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const vk::DescriptorSet,
    ) -> vk::Result {
        let global = Self::instance();
        // vkFreeDescriptorSets
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .free_descriptor_sets(descriptor_pool, unsafe {
                std::slice::from_raw_parts(p_descriptor_sets, descriptor_set_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.free_descriptor_sets)(
                    device,
                    descriptor_pool,
                    descriptor_set_count,
                    p_descriptor_sets,
                )
            },
        }
    }
    extern "system" fn update_descriptor_sets(
        device: vk::Device,
        descriptor_write_count: u32,
        p_descriptor_writes: *const vk::WriteDescriptorSet,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const vk::CopyDescriptorSet,
    ) {
        let global = Self::instance();
        // vkUpdateDescriptorSets
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.update_descriptor_sets(
            unsafe {
                std::slice::from_raw_parts(p_descriptor_writes, descriptor_write_count as usize)
            },
            unsafe {
                std::slice::from_raw_parts(p_descriptor_copies, descriptor_copy_count as usize)
            },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.update_descriptor_sets)(
                    device,
                    descriptor_write_count,
                    p_descriptor_writes,
                    descriptor_copy_count,
                    p_descriptor_copies,
                )
            },
        }
    }
    extern "system" fn create_framebuffer(
        device: vk::Device,
        p_create_info: *const vk::FramebufferCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_framebuffer: *mut vk::Framebuffer,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateFramebuffer
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_framebuffer(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_framebuffer.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_framebuffer)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_framebuffer,
                )
            },
        }
    }
    extern "system" fn destroy_framebuffer(
        device: vk::Device,
        framebuffer: vk::Framebuffer,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyFramebuffer
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_framebuffer(framebuffer, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_framebuffer)(device, framebuffer, p_allocator)
            },
        }
    }
    extern "system" fn create_render_pass(
        device: vk::Device,
        p_create_info: *const vk::RenderPassCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_render_pass: *mut vk::RenderPass,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateRenderPass
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_render_pass(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_render_pass.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_render_pass)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_render_pass,
                )
            },
        }
    }
    extern "system" fn destroy_render_pass(
        device: vk::Device,
        render_pass: vk::RenderPass,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyRenderPass
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_render_pass(render_pass, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_render_pass)(device, render_pass, p_allocator)
            },
        }
    }
    extern "system" fn get_render_area_granularity(
        device: vk::Device,
        render_pass: vk::RenderPass,
        p_granularity: *mut vk::Extent2D,
    ) {
        let global = Self::instance();
        // vkGetRenderAreaGranularity
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .get_render_area_granularity(render_pass);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_granularity.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_render_area_granularity)(device, render_pass, p_granularity)
            },
        }
    }
    extern "system" fn create_command_pool(
        device: vk::Device,
        p_create_info: *const vk::CommandPoolCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_command_pool: *mut vk::CommandPool,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateCommandPool
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .create_command_pool(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_command_pool.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_command_pool)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_command_pool,
                )
            },
        }
    }
    extern "system" fn destroy_command_pool(
        device: vk::Device,
        command_pool: vk::CommandPool,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyCommandPool
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .destroy_command_pool(command_pool, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_command_pool)(device, command_pool, p_allocator)
            },
        }
    }
    extern "system" fn reset_command_pool(
        device: vk::Device,
        command_pool: vk::CommandPool,
        flags: vk::CommandPoolResetFlags,
    ) -> vk::Result {
        let global = Self::instance();
        // vkResetCommandPool
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .reset_command_pool(command_pool, flags);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.reset_command_pool)(device, command_pool, flags)
            },
        }
    }
    extern "system" fn allocate_command_buffers(
        device: vk::Device,
        p_allocate_info: *const vk::CommandBufferAllocateInfo,
        p_command_buffers: *mut vk::CommandBuffer,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAllocateCommandBuffers
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .allocate_command_buffers(unsafe { p_allocate_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    unsafe {
                        std::slice::from_raw_parts_mut(
                            p_command_buffers,
                            unsafe { p_allocate_info.as_ref().unwrap() }
                                .command_buffer_count
                                .try_into()
                                .unwrap(),
                        )
                    }
                    .copy_from_slice(&res);
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.allocate_command_buffers)(
                    device,
                    p_allocate_info,
                    p_command_buffers,
                )
            },
        }
    }
    extern "system" fn free_command_buffers(
        device: vk::Device,
        command_pool: vk::CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const vk::CommandBuffer,
    ) {
        let global = Self::instance();
        // vkFreeCommandBuffers
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .free_command_buffers(command_pool, unsafe {
                std::slice::from_raw_parts(p_command_buffers, command_buffer_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.free_command_buffers)(
                    device,
                    command_pool,
                    command_buffer_count,
                    p_command_buffers,
                )
            },
        }
    }
    extern "system" fn begin_command_buffer(
        command_buffer: vk::CommandBuffer,
        p_begin_info: *const vk::CommandBufferBeginInfo,
    ) -> vk::Result {
        let global = Self::instance();
        // vkBeginCommandBuffer
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .begin_command_buffer(command_buffer, unsafe { p_begin_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.begin_command_buffer)(command_buffer, p_begin_info)
            },
        }
    }
    extern "system" fn end_command_buffer(command_buffer: vk::CommandBuffer) -> vk::Result {
        let global = Self::instance();
        // vkEndCommandBuffer
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .end_command_buffer(command_buffer);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.end_command_buffer)(command_buffer)
            },
        }
    }
    extern "system" fn reset_command_buffer(
        command_buffer: vk::CommandBuffer,
        flags: vk::CommandBufferResetFlags,
    ) -> vk::Result {
        let global = Self::instance();
        // vkResetCommandBuffer
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .reset_command_buffer(command_buffer, flags);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.reset_command_buffer)(command_buffer, flags)
            },
        }
    }
    extern "system" fn cmd_bind_pipeline(
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) {
        let global = Self::instance();
        // vkCmdBindPipeline
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_bind_pipeline(
            command_buffer,
            pipeline_bind_point,
            pipeline,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_pipeline)(command_buffer, pipeline_bind_point, pipeline)
            },
        }
    }
    extern "system" fn cmd_set_viewport(
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const vk::Viewport,
    ) {
        let global = Self::instance();
        // vkCmdSetViewport
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result =
            device_info
                .customized_info
                .cmd_set_viewport(command_buffer, first_viewport, unsafe {
                    std::slice::from_raw_parts(p_viewports, viewport_count as usize)
                });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_viewport)(
                    command_buffer,
                    first_viewport,
                    viewport_count,
                    p_viewports,
                )
            },
        }
    }
    extern "system" fn cmd_set_scissor(
        command_buffer: vk::CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const vk::Rect2D,
    ) {
        let global = Self::instance();
        // vkCmdSetScissor
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result =
            device_info
                .customized_info
                .cmd_set_scissor(command_buffer, first_scissor, unsafe {
                    std::slice::from_raw_parts(p_scissors, scissor_count as usize)
                });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_scissor)(
                    command_buffer,
                    first_scissor,
                    scissor_count,
                    p_scissors,
                )
            },
        }
    }
    extern "system" fn cmd_set_line_width(command_buffer: vk::CommandBuffer, line_width: f32) {
        let global = Self::instance();
        // vkCmdSetLineWidth
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .cmd_set_line_width(command_buffer, line_width);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_line_width)(command_buffer, line_width)
            },
        }
    }
    extern "system" fn cmd_set_depth_bias(
        command_buffer: vk::CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        let global = Self::instance();
        // vkCmdSetDepthBias
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_set_depth_bias(
            command_buffer,
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_depth_bias)(
                    command_buffer,
                    depth_bias_constant_factor,
                    depth_bias_clamp,
                    depth_bias_slope_factor,
                )
            },
        }
    }
    extern "system" fn cmd_set_blend_constants(
        command_buffer: vk::CommandBuffer,
        blend_constants: *const [f32; 4],
    ) {
        let global = Self::instance();
        // vkCmdSetBlendConstants
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let blend_constants = unsafe { blend_constants.as_ref() }.unwrap();
        let layer_result = device_info
            .customized_info
            .cmd_set_blend_constants(command_buffer, blend_constants);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_blend_constants)(command_buffer, blend_constants)
            },
        }
    }
    extern "system" fn cmd_set_depth_bounds(
        command_buffer: vk::CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        let global = Self::instance();
        // vkCmdSetDepthBounds
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_set_depth_bounds(
            command_buffer,
            min_depth_bounds,
            max_depth_bounds,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_depth_bounds)(
                    command_buffer,
                    min_depth_bounds,
                    max_depth_bounds,
                )
            },
        }
    }
    extern "system" fn cmd_set_stencil_compare_mask(
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        compare_mask: u32,
    ) {
        let global = Self::instance();
        // vkCmdSetStencilCompareMask
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_set_stencil_compare_mask(
            command_buffer,
            face_mask,
            compare_mask,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_stencil_compare_mask)(
                    command_buffer,
                    face_mask,
                    compare_mask,
                )
            },
        }
    }
    extern "system" fn cmd_set_stencil_write_mask(
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        write_mask: u32,
    ) {
        let global = Self::instance();
        // vkCmdSetStencilWriteMask
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_set_stencil_write_mask(
            command_buffer,
            face_mask,
            write_mask,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_stencil_write_mask)(command_buffer, face_mask, write_mask)
            },
        }
    }
    extern "system" fn cmd_set_stencil_reference(
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        reference: u32,
    ) {
        let global = Self::instance();
        // vkCmdSetStencilReference
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_set_stencil_reference(
            command_buffer,
            face_mask,
            reference,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_stencil_reference)(command_buffer, face_mask, reference)
            },
        }
    }
    extern "system" fn cmd_bind_descriptor_sets(
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const vk::DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    ) {
        let global = Self::instance();
        // vkCmdBindDescriptorSets
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_bind_descriptor_sets(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            unsafe { std::slice::from_raw_parts(p_descriptor_sets, descriptor_set_count as usize) },
            unsafe { std::slice::from_raw_parts(p_dynamic_offsets, dynamic_offset_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_descriptor_sets)(
                    command_buffer,
                    pipeline_bind_point,
                    layout,
                    first_set,
                    descriptor_set_count,
                    p_descriptor_sets,
                    dynamic_offset_count,
                    p_dynamic_offsets,
                )
            },
        }
    }
    extern "system" fn cmd_bind_index_buffer(
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        index_type: vk::IndexType,
    ) {
        let global = Self::instance();
        // vkCmdBindIndexBuffer
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_bind_index_buffer(
            command_buffer,
            buffer,
            offset,
            index_type,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_index_buffer)(command_buffer, buffer, offset, index_type)
            },
        }
    }
    extern "system" fn cmd_bind_vertex_buffers(
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const vk::Buffer,
        p_offsets: *const vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkCmdBindVertexBuffers
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_bind_vertex_buffers(
            command_buffer,
            first_binding,
            unsafe { std::slice::from_raw_parts(p_buffers, binding_count as usize) },
            unsafe { std::slice::from_raw_parts(p_offsets, binding_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_vertex_buffers)(
                    command_buffer,
                    first_binding,
                    binding_count,
                    p_buffers,
                    p_offsets,
                )
            },
        }
    }
    extern "system" fn cmd_draw(
        command_buffer: vk::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        let global = Self::instance();
        // vkCmdDraw
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_draw(
            command_buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw)(
                    command_buffer,
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                )
            },
        }
    }
    extern "system" fn cmd_draw_indexed(
        command_buffer: vk::CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawIndexed
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_draw_indexed(
            command_buffer,
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_indexed)(
                    command_buffer,
                    index_count,
                    instance_count,
                    first_index,
                    vertex_offset,
                    first_instance,
                )
            },
        }
    }
    extern "system" fn cmd_draw_indirect(
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawIndirect
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_draw_indirect(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_indirect)(
                    command_buffer,
                    buffer,
                    offset,
                    draw_count,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_draw_indexed_indirect(
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawIndexedIndirect
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_draw_indexed_indirect(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_indexed_indirect)(
                    command_buffer,
                    buffer,
                    offset,
                    draw_count,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_dispatch(
        command_buffer: vk::CommandBuffer,
        group_countx: u32,
        group_county: u32,
        group_countz: u32,
    ) {
        let global = Self::instance();
        // vkCmdDispatch
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_dispatch(
            command_buffer,
            group_countx,
            group_county,
            group_countz,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_dispatch)(
                    command_buffer,
                    group_countx,
                    group_county,
                    group_countz,
                )
            },
        }
    }
    extern "system" fn cmd_dispatch_indirect(
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkCmdDispatchIndirect
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result =
            device_info
                .customized_info
                .cmd_dispatch_indirect(command_buffer, buffer, offset);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_dispatch_indirect)(command_buffer, buffer, offset)
            },
        }
    }
    extern "system" fn cmd_copy_buffer(
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_buffer: vk::Buffer,
        region_count: u32,
        p_regions: *const vk::BufferCopy,
    ) {
        let global = Self::instance();
        // vkCmdCopyBuffer
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_copy_buffer(
            command_buffer,
            src_buffer,
            dst_buffer,
            unsafe { std::slice::from_raw_parts(p_regions, region_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_buffer)(
                    command_buffer,
                    src_buffer,
                    dst_buffer,
                    region_count,
                    p_regions,
                )
            },
        }
    }
    extern "system" fn cmd_copy_image(
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        region_count: u32,
        p_regions: *const vk::ImageCopy,
    ) {
        let global = Self::instance();
        // vkCmdCopyImage
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_copy_image(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            unsafe { std::slice::from_raw_parts(p_regions, region_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_image)(
                    command_buffer,
                    src_image,
                    src_image_layout,
                    dst_image,
                    dst_image_layout,
                    region_count,
                    p_regions,
                )
            },
        }
    }
    extern "system" fn cmd_blit_image(
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        region_count: u32,
        p_regions: *const vk::ImageBlit,
        filter: vk::Filter,
    ) {
        let global = Self::instance();
        // vkCmdBlitImage
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_blit_image(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            unsafe { std::slice::from_raw_parts(p_regions, region_count as usize) },
            filter,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_blit_image)(
                    command_buffer,
                    src_image,
                    src_image_layout,
                    dst_image,
                    dst_image_layout,
                    region_count,
                    p_regions,
                    filter,
                )
            },
        }
    }
    extern "system" fn cmd_copy_buffer_to_image(
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        region_count: u32,
        p_regions: *const vk::BufferImageCopy,
    ) {
        let global = Self::instance();
        // vkCmdCopyBufferToImage
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_copy_buffer_to_image(
            command_buffer,
            src_buffer,
            dst_image,
            dst_image_layout,
            unsafe { std::slice::from_raw_parts(p_regions, region_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_buffer_to_image)(
                    command_buffer,
                    src_buffer,
                    dst_image,
                    dst_image_layout,
                    region_count,
                    p_regions,
                )
            },
        }
    }
    extern "system" fn cmd_copy_image_to_buffer(
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_buffer: vk::Buffer,
        region_count: u32,
        p_regions: *const vk::BufferImageCopy,
    ) {
        let global = Self::instance();
        // vkCmdCopyImageToBuffer
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_copy_image_to_buffer(
            command_buffer,
            src_image,
            src_image_layout,
            dst_buffer,
            unsafe { std::slice::from_raw_parts(p_regions, region_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_image_to_buffer)(
                    command_buffer,
                    src_image,
                    src_image_layout,
                    dst_buffer,
                    region_count,
                    p_regions,
                )
            },
        }
    }
    extern "system" fn cmd_update_buffer(
        command_buffer: vk::CommandBuffer,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        data_size: vk::DeviceSize,
        p_data: *const c_void,
    ) {
        let global = Self::instance();
        // vkCmdUpdateBuffer
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_update_buffer(
            command_buffer,
            dst_buffer,
            dst_offset,
            unsafe { std::slice::from_raw_parts(p_data as *const u8, data_size as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_update_buffer)(
                    command_buffer,
                    dst_buffer,
                    dst_offset,
                    data_size,
                    p_data,
                )
            },
        }
    }
    extern "system" fn cmd_fill_buffer(
        command_buffer: vk::CommandBuffer,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        size: vk::DeviceSize,
        data: u32,
    ) {
        let global = Self::instance();
        // vkCmdFillBuffer
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_fill_buffer(
            command_buffer,
            dst_buffer,
            dst_offset,
            size,
            data,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_fill_buffer)(command_buffer, dst_buffer, dst_offset, size, data)
            },
        }
    }
    extern "system" fn cmd_clear_color_image(
        command_buffer: vk::CommandBuffer,
        image: vk::Image,
        image_layout: vk::ImageLayout,
        p_color: *const vk::ClearColorValue,
        range_count: u32,
        p_ranges: *const vk::ImageSubresourceRange,
    ) {
        let global = Self::instance();
        // vkCmdClearColorImage
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_clear_color_image(
            command_buffer,
            image,
            image_layout,
            unsafe { p_color.as_ref() }.unwrap(),
            unsafe { std::slice::from_raw_parts(p_ranges, range_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_clear_color_image)(
                    command_buffer,
                    image,
                    image_layout,
                    p_color,
                    range_count,
                    p_ranges,
                )
            },
        }
    }
    extern "system" fn cmd_clear_depth_stencil_image(
        command_buffer: vk::CommandBuffer,
        image: vk::Image,
        image_layout: vk::ImageLayout,
        p_depth_stencil: *const vk::ClearDepthStencilValue,
        range_count: u32,
        p_ranges: *const vk::ImageSubresourceRange,
    ) {
        let global = Self::instance();
        // vkCmdClearDepthStencilImage
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_clear_depth_stencil_image(
            command_buffer,
            image,
            image_layout,
            unsafe { p_depth_stencil.as_ref() }.unwrap(),
            unsafe { std::slice::from_raw_parts(p_ranges, range_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_clear_depth_stencil_image)(
                    command_buffer,
                    image,
                    image_layout,
                    p_depth_stencil,
                    range_count,
                    p_ranges,
                )
            },
        }
    }
    extern "system" fn cmd_clear_attachments(
        command_buffer: vk::CommandBuffer,
        attachment_count: u32,
        p_attachments: *const vk::ClearAttachment,
        rect_count: u32,
        p_rects: *const vk::ClearRect,
    ) {
        let global = Self::instance();
        // vkCmdClearAttachments
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_clear_attachments(
            command_buffer,
            unsafe { std::slice::from_raw_parts(p_attachments, attachment_count as usize) },
            unsafe { std::slice::from_raw_parts(p_rects, rect_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_clear_attachments)(
                    command_buffer,
                    attachment_count,
                    p_attachments,
                    rect_count,
                    p_rects,
                )
            },
        }
    }
    extern "system" fn cmd_resolve_image(
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        region_count: u32,
        p_regions: *const vk::ImageResolve,
    ) {
        let global = Self::instance();
        // vkCmdResolveImage
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_resolve_image(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            unsafe { std::slice::from_raw_parts(p_regions, region_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_resolve_image)(
                    command_buffer,
                    src_image,
                    src_image_layout,
                    dst_image,
                    dst_image_layout,
                    region_count,
                    p_regions,
                )
            },
        }
    }
    extern "system" fn cmd_set_event(
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags,
    ) {
        let global = Self::instance();
        // vkCmdSetEvent
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result =
            device_info
                .customized_info
                .cmd_set_event(command_buffer, event, stage_mask);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_event)(command_buffer, event, stage_mask)
            },
        }
    }
    extern "system" fn cmd_reset_event(
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags,
    ) {
        let global = Self::instance();
        // vkCmdResetEvent
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result =
            device_info
                .customized_info
                .cmd_reset_event(command_buffer, event, stage_mask);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_reset_event)(command_buffer, event, stage_mask)
            },
        }
    }
    extern "system" fn cmd_wait_events(
        command_buffer: vk::CommandBuffer,
        event_count: u32,
        p_events: *const vk::Event,
        src_stage_mask: vk::PipelineStageFlags,
        dst_stage_mask: vk::PipelineStageFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const vk::MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const vk::BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const vk::ImageMemoryBarrier,
    ) {
        let global = Self::instance();
        // vkCmdWaitEvents
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_wait_events(
            command_buffer,
            unsafe { std::slice::from_raw_parts(p_events, event_count as usize) },
            src_stage_mask,
            dst_stage_mask,
            unsafe { std::slice::from_raw_parts(p_memory_barriers, memory_barrier_count as usize) },
            unsafe {
                std::slice::from_raw_parts(
                    p_buffer_memory_barriers,
                    buffer_memory_barrier_count as usize,
                )
            },
            unsafe {
                std::slice::from_raw_parts(
                    p_image_memory_barriers,
                    image_memory_barrier_count as usize,
                )
            },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_wait_events)(
                    command_buffer,
                    event_count,
                    p_events,
                    src_stage_mask,
                    dst_stage_mask,
                    memory_barrier_count,
                    p_memory_barriers,
                    buffer_memory_barrier_count,
                    p_buffer_memory_barriers,
                    image_memory_barrier_count,
                    p_image_memory_barriers,
                )
            },
        }
    }
    extern "system" fn cmd_pipeline_barrier(
        command_buffer: vk::CommandBuffer,
        src_stage_mask: vk::PipelineStageFlags,
        dst_stage_mask: vk::PipelineStageFlags,
        dependency_flags: vk::DependencyFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const vk::MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const vk::BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const vk::ImageMemoryBarrier,
    ) {
        let global = Self::instance();
        // vkCmdPipelineBarrier
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_pipeline_barrier(
            command_buffer,
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            unsafe { std::slice::from_raw_parts(p_memory_barriers, memory_barrier_count as usize) },
            unsafe {
                std::slice::from_raw_parts(
                    p_buffer_memory_barriers,
                    buffer_memory_barrier_count as usize,
                )
            },
            unsafe {
                std::slice::from_raw_parts(
                    p_image_memory_barriers,
                    image_memory_barrier_count as usize,
                )
            },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_pipeline_barrier)(
                    command_buffer,
                    src_stage_mask,
                    dst_stage_mask,
                    dependency_flags,
                    memory_barrier_count,
                    p_memory_barriers,
                    buffer_memory_barrier_count,
                    p_buffer_memory_barriers,
                    image_memory_barrier_count,
                    p_image_memory_barriers,
                )
            },
        }
    }
    extern "system" fn cmd_begin_query(
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        flags: vk::QueryControlFlags,
    ) {
        let global = Self::instance();
        // vkCmdBeginQuery
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result =
            device_info
                .customized_info
                .cmd_begin_query(command_buffer, query_pool, query, flags);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_begin_query)(command_buffer, query_pool, query, flags)
            },
        }
    }
    extern "system" fn cmd_end_query(
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        let global = Self::instance();
        // vkCmdEndQuery
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result =
            device_info
                .customized_info
                .cmd_end_query(command_buffer, query_pool, query);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_end_query)(command_buffer, query_pool, query)
            },
        }
    }
    extern "system" fn cmd_reset_query_pool(
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        let global = Self::instance();
        // vkCmdResetQueryPool
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_reset_query_pool(
            command_buffer,
            query_pool,
            first_query,
            query_count,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_reset_query_pool)(
                    command_buffer,
                    query_pool,
                    first_query,
                    query_count,
                )
            },
        }
    }
    extern "system" fn cmd_write_timestamp(
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        let global = Self::instance();
        // vkCmdWriteTimestamp
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_write_timestamp(
            command_buffer,
            pipeline_stage,
            query_pool,
            query,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_write_timestamp)(
                    command_buffer,
                    pipeline_stage,
                    query_pool,
                    query,
                )
            },
        }
    }
    extern "system" fn cmd_copy_query_pool_results(
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        stride: vk::DeviceSize,
        flags: vk::QueryResultFlags,
    ) {
        let global = Self::instance();
        // vkCmdCopyQueryPoolResults
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_copy_query_pool_results(
            command_buffer,
            query_pool,
            first_query,
            query_count,
            dst_buffer,
            dst_offset,
            stride,
            flags,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_query_pool_results)(
                    command_buffer,
                    query_pool,
                    first_query,
                    query_count,
                    dst_buffer,
                    dst_offset,
                    stride,
                    flags,
                )
            },
        }
    }
    extern "system" fn cmd_push_constants(
        command_buffer: vk::CommandBuffer,
        layout: vk::PipelineLayout,
        stage_flags: vk::ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const c_void,
    ) {
        let global = Self::instance();
        // vkCmdPushConstants
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_push_constants(
            command_buffer,
            layout,
            stage_flags,
            offset,
            unsafe { std::slice::from_raw_parts(p_values as *const u8, size as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_push_constants)(
                    command_buffer,
                    layout,
                    stage_flags,
                    offset,
                    size,
                    p_values,
                )
            },
        }
    }
    extern "system" fn cmd_begin_render_pass(
        command_buffer: vk::CommandBuffer,
        p_render_pass_begin: *const vk::RenderPassBeginInfo,
        contents: vk::SubpassContents,
    ) {
        let global = Self::instance();
        // vkCmdBeginRenderPass
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info.customized_info.cmd_begin_render_pass(
            command_buffer,
            unsafe { p_render_pass_begin.as_ref() }.unwrap(),
            contents,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_begin_render_pass)(
                    command_buffer,
                    p_render_pass_begin,
                    contents,
                )
            },
        }
    }
    extern "system" fn cmd_next_subpass(
        command_buffer: vk::CommandBuffer,
        contents: vk::SubpassContents,
    ) {
        let global = Self::instance();
        // vkCmdNextSubpass
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .cmd_next_subpass(command_buffer, contents);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_next_subpass)(command_buffer, contents)
            },
        }
    }
    extern "system" fn cmd_end_render_pass(command_buffer: vk::CommandBuffer) {
        let global = Self::instance();
        // vkCmdEndRenderPass
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        let layer_result = device_info
            .customized_info
            .cmd_end_render_pass(command_buffer);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_end_render_pass)(command_buffer)
            },
        }
    }
    extern "system" fn cmd_execute_commands(
        command_buffer: vk::CommandBuffer,
        command_buffer_count: u32,
        p_command_buffers: *const vk::CommandBuffer,
    ) {
        let global = Self::instance();
        // vkCmdExecuteCommands
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_0();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_execute_commands(command_buffer, unsafe {
                std::slice::from_raw_parts(p_command_buffers, command_buffer_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_execute_commands)(
                    command_buffer,
                    command_buffer_count,
                    p_command_buffers,
                )
            },
        }
    }
    extern "system" fn bind_buffer_memory2(
        device: vk::Device,
        bind_info_count: u32,
        p_bind_infos: *const vk::BindBufferMemoryInfo,
    ) -> vk::Result {
        let global = Self::instance();
        // vkBindBufferMemory2
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.bind_buffer_memory2(unsafe {
            std::slice::from_raw_parts(p_bind_infos, bind_info_count as usize)
        });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.bind_buffer_memory2)(device, bind_info_count, p_bind_infos)
            },
        }
    }
    extern "system" fn bind_image_memory2(
        device: vk::Device,
        bind_info_count: u32,
        p_bind_infos: *const vk::BindImageMemoryInfo,
    ) -> vk::Result {
        let global = Self::instance();
        // vkBindImageMemory2
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.bind_image_memory2(unsafe {
            std::slice::from_raw_parts(p_bind_infos, bind_info_count as usize)
        });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.bind_image_memory2)(device, bind_info_count, p_bind_infos)
            },
        }
    }
    extern "system" fn get_device_group_peer_memory_features(
        device: vk::Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut vk::PeerMemoryFeatureFlags,
    ) {
        let global = Self::instance();
        // vkGetDeviceGroupPeerMemoryFeatures
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .get_device_group_peer_memory_features(
                heap_index,
                local_device_index,
                remote_device_index,
            );
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_peer_memory_features.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_group_peer_memory_features)(
                    device,
                    heap_index,
                    local_device_index,
                    remote_device_index,
                    p_peer_memory_features,
                )
            },
        }
    }
    extern "system" fn cmd_set_device_mask(command_buffer: vk::CommandBuffer, device_mask: u32) {
        let global = Self::instance();
        // vkCmdSetDeviceMask
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .cmd_set_device_mask(command_buffer, device_mask);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_device_mask)(command_buffer, device_mask)
            },
        }
    }
    extern "system" fn cmd_dispatch_base(
        command_buffer: vk::CommandBuffer,
        base_groupx: u32,
        base_groupy: u32,
        base_groupz: u32,
        group_countx: u32,
        group_county: u32,
        group_countz: u32,
    ) {
        let global = Self::instance();
        // vkCmdDispatchBase
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info.customized_info.cmd_dispatch_base(
            command_buffer,
            base_groupx,
            base_groupy,
            base_groupz,
            group_countx,
            group_county,
            group_countz,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_dispatch_base)(
                    command_buffer,
                    base_groupx,
                    base_groupy,
                    base_groupz,
                    group_countx,
                    group_county,
                    group_countz,
                )
            },
        }
    }
    extern "system" fn get_image_memory_requirements2(
        device: vk::Device,
        p_info: *const vk::ImageMemoryRequirementsInfo2,
        p_memory_requirements: *mut vk::MemoryRequirements2,
    ) {
        let global = Self::instance();
        // vkGetImageMemoryRequirements2
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .get_image_memory_requirements2(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_memory_requirements.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_memory_requirements2)(
                    device,
                    p_info,
                    p_memory_requirements,
                )
            },
        }
    }
    extern "system" fn get_buffer_memory_requirements2(
        device: vk::Device,
        p_info: *const vk::BufferMemoryRequirementsInfo2,
        p_memory_requirements: *mut vk::MemoryRequirements2,
    ) {
        let global = Self::instance();
        // vkGetBufferMemoryRequirements2
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .get_buffer_memory_requirements2(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_memory_requirements.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_buffer_memory_requirements2)(
                    device,
                    p_info,
                    p_memory_requirements,
                )
            },
        }
    }
    extern "system" fn get_image_sparse_memory_requirements2(
        device: vk::Device,
        p_info: *const vk::ImageSparseMemoryRequirementsInfo2,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut vk::SparseImageMemoryRequirements2,
    ) {
        let global = Self::instance();
        // vkGetImageSparseMemoryRequirements2
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .get_image_sparse_memory_requirements2(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                // We can't return INCOMPLETE from vkGetImageSparseMemoryRequirements2
                #[allow(unused_must_use)]
                unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_sparse_memory_requirement_count).unwrap(),
                        p_sparse_memory_requirements,
                    );
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_sparse_memory_requirements2)(
                    device,
                    p_info,
                    p_sparse_memory_requirement_count,
                    p_sparse_memory_requirements,
                )
            },
        }
    }
    extern "system" fn trim_command_pool(
        device: vk::Device,
        command_pool: vk::CommandPool,
        flags: vk::CommandPoolTrimFlags,
    ) {
        let global = Self::instance();
        // vkTrimCommandPool
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .trim_command_pool(command_pool, flags);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.trim_command_pool)(device, command_pool, flags)
            },
        }
    }
    extern "system" fn get_device_queue2(
        device: vk::Device,
        p_queue_info: *const vk::DeviceQueueInfo2,
        p_queue: *mut vk::Queue,
    ) {
        let global = Self::instance();
        // vkGetDeviceQueue2
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .get_device_queue2(unsafe { p_queue_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_queue.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_queue2)(device, p_queue_info, p_queue)
            },
        }
    }
    extern "system" fn create_sampler_ycbcr_conversion(
        device: vk::Device,
        p_create_info: *const vk::SamplerYcbcrConversionCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_ycbcr_conversion: *mut vk::SamplerYcbcrConversion,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateSamplerYcbcrConversion
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .create_sampler_ycbcr_conversion(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_ycbcr_conversion.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_sampler_ycbcr_conversion)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_ycbcr_conversion,
                )
            },
        }
    }
    extern "system" fn destroy_sampler_ycbcr_conversion(
        device: vk::Device,
        ycbcr_conversion: vk::SamplerYcbcrConversion,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroySamplerYcbcrConversion
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .destroy_sampler_ycbcr_conversion(ycbcr_conversion, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_sampler_ycbcr_conversion)(
                    device,
                    ycbcr_conversion,
                    p_allocator,
                )
            },
        }
    }
    extern "system" fn create_descriptor_update_template(
        device: vk::Device,
        p_create_info: *const vk::DescriptorUpdateTemplateCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_descriptor_update_template: *mut vk::DescriptorUpdateTemplate,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateDescriptorUpdateTemplate
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .create_descriptor_update_template(
                unsafe { p_create_info.as_ref() }.unwrap(),
                unsafe { p_allocator.as_ref() },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_descriptor_update_template.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_descriptor_update_template)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_descriptor_update_template,
                )
            },
        }
    }
    extern "system" fn destroy_descriptor_update_template(
        device: vk::Device,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyDescriptorUpdateTemplate
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .destroy_descriptor_update_template(descriptor_update_template, unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_descriptor_update_template)(
                    device,
                    descriptor_update_template,
                    p_allocator,
                )
            },
        }
    }
    extern "system" fn update_descriptor_set_with_template(
        device: vk::Device,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        let global = Self::instance();
        // vkUpdateDescriptorSetWithTemplate
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .update_descriptor_set_with_template(
                descriptor_set,
                descriptor_update_template,
                unsafe { p_data.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.update_descriptor_set_with_template)(
                    device,
                    descriptor_set,
                    descriptor_update_template,
                    p_data,
                )
            },
        }
    }
    extern "system" fn get_descriptor_set_layout_support(
        device: vk::Device,
        p_create_info: *const vk::DescriptorSetLayoutCreateInfo,
        p_support: *mut vk::DescriptorSetLayoutSupport,
    ) {
        let global = Self::instance();
        // vkGetDescriptorSetLayoutSupport
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_1();
        let layer_result = device_info
            .customized_info
            .get_descriptor_set_layout_support(unsafe { p_create_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_support.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_descriptor_set_layout_support)(device, p_create_info, p_support)
            },
        }
    }
    extern "system" fn cmd_draw_indirect_count(
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawIndirectCount
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info.customized_info.cmd_draw_indirect_count(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_indirect_count)(
                    command_buffer,
                    buffer,
                    offset,
                    count_buffer,
                    count_buffer_offset,
                    max_draw_count,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_draw_indexed_indirect_count(
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawIndexedIndirectCount
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info.customized_info.cmd_draw_indexed_indirect_count(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_indexed_indirect_count)(
                    command_buffer,
                    buffer,
                    offset,
                    count_buffer,
                    count_buffer_offset,
                    max_draw_count,
                    stride,
                )
            },
        }
    }
    extern "system" fn create_render_pass2(
        device: vk::Device,
        p_create_info: *const vk::RenderPassCreateInfo2,
        p_allocator: *const vk::AllocationCallbacks,
        p_render_pass: *mut vk::RenderPass,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateRenderPass2
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info
            .customized_info
            .create_render_pass2(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_render_pass.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_render_pass2)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_render_pass,
                )
            },
        }
    }
    extern "system" fn cmd_begin_render_pass2(
        command_buffer: vk::CommandBuffer,
        p_render_pass_begin: *const vk::RenderPassBeginInfo,
        p_subpass_begin_info: *const vk::SubpassBeginInfo,
    ) {
        let global = Self::instance();
        // vkCmdBeginRenderPass2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info.customized_info.cmd_begin_render_pass2(
            command_buffer,
            unsafe { p_render_pass_begin.as_ref() }.unwrap(),
            unsafe { p_subpass_begin_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_begin_render_pass2)(
                    command_buffer,
                    p_render_pass_begin,
                    p_subpass_begin_info,
                )
            },
        }
    }
    extern "system" fn cmd_next_subpass2(
        command_buffer: vk::CommandBuffer,
        p_subpass_begin_info: *const vk::SubpassBeginInfo,
        p_subpass_end_info: *const vk::SubpassEndInfo,
    ) {
        let global = Self::instance();
        // vkCmdNextSubpass2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info.customized_info.cmd_next_subpass2(
            command_buffer,
            unsafe { p_subpass_begin_info.as_ref() }.unwrap(),
            unsafe { p_subpass_end_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_next_subpass2)(
                    command_buffer,
                    p_subpass_begin_info,
                    p_subpass_end_info,
                )
            },
        }
    }
    extern "system" fn cmd_end_render_pass2(
        command_buffer: vk::CommandBuffer,
        p_subpass_end_info: *const vk::SubpassEndInfo,
    ) {
        let global = Self::instance();
        // vkCmdEndRenderPass2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info.customized_info.cmd_end_render_pass2(
            command_buffer,
            unsafe { p_subpass_end_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_end_render_pass2)(command_buffer, p_subpass_end_info)
            },
        }
    }
    extern "system" fn reset_query_pool(
        device: vk::Device,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        let global = Self::instance();
        // vkResetQueryPool
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result =
            device_info
                .customized_info
                .reset_query_pool(query_pool, first_query, query_count);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.reset_query_pool)(device, query_pool, first_query, query_count)
            },
        }
    }
    extern "system" fn get_semaphore_counter_value(
        device: vk::Device,
        semaphore: vk::Semaphore,
        p_value: *mut u64,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetSemaphoreCounterValue
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info
            .customized_info
            .get_semaphore_counter_value(semaphore);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_value.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_semaphore_counter_value)(device, semaphore, p_value)
            },
        }
    }
    extern "system" fn wait_semaphores(
        device: vk::Device,
        p_wait_info: *const vk::SemaphoreWaitInfo,
        timeout: u64,
    ) -> vk::Result {
        let global = Self::instance();
        // vkWaitSemaphores
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info
            .customized_info
            .wait_semaphores(unsafe { p_wait_info.as_ref() }.unwrap(), timeout);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.wait_semaphores)(device, p_wait_info, timeout)
            },
        }
    }
    extern "system" fn signal_semaphore(
        device: vk::Device,
        p_signal_info: *const vk::SemaphoreSignalInfo,
    ) -> vk::Result {
        let global = Self::instance();
        // vkSignalSemaphore
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info
            .customized_info
            .signal_semaphore(unsafe { p_signal_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.signal_semaphore)(device, p_signal_info)
            },
        }
    }
    extern "system" fn get_buffer_device_address(
        device: vk::Device,
        p_info: *const vk::BufferDeviceAddressInfo,
    ) -> vk::DeviceAddress {
        let global = Self::instance();
        // vkGetBufferDeviceAddress
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info
            .customized_info
            .get_buffer_device_address(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_buffer_device_address)(device, p_info)
            },
        }
    }
    extern "system" fn get_buffer_opaque_capture_address(
        device: vk::Device,
        p_info: *const vk::BufferDeviceAddressInfo,
    ) -> u64 {
        let global = Self::instance();
        // vkGetBufferOpaqueCaptureAddress
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info
            .customized_info
            .get_buffer_opaque_capture_address(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_buffer_opaque_capture_address)(device, p_info)
            },
        }
    }
    extern "system" fn get_device_memory_opaque_capture_address(
        device: vk::Device,
        p_info: *const vk::DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        let global = Self::instance();
        // vkGetDeviceMemoryOpaqueCaptureAddress
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_2();
        let layer_result = device_info
            .customized_info
            .get_device_memory_opaque_capture_address(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_memory_opaque_capture_address)(device, p_info)
            },
        }
    }
    extern "system" fn create_private_data_slot(
        device: vk::Device,
        p_create_info: *const vk::PrivateDataSlotCreateInfo,
        p_allocator: *const vk::AllocationCallbacks,
        p_private_data_slot: *mut vk::PrivateDataSlot,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreatePrivateDataSlot
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .create_private_data_slot(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_private_data_slot.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_private_data_slot)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_private_data_slot,
                )
            },
        }
    }
    extern "system" fn destroy_private_data_slot(
        device: vk::Device,
        private_data_slot: vk::PrivateDataSlot,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyPrivateDataSlot
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .destroy_private_data_slot(private_data_slot, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_private_data_slot)(device, private_data_slot, p_allocator)
            },
        }
    }
    extern "system" fn set_private_data(
        device: vk::Device,
        object_type: vk::ObjectType,
        object_handle: u64,
        private_data_slot: vk::PrivateDataSlot,
        data: u64,
    ) -> vk::Result {
        let global = Self::instance();
        // vkSetPrivateData
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.set_private_data(
            object_type,
            object_handle,
            private_data_slot,
            data,
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.set_private_data)(
                    device,
                    object_type,
                    object_handle,
                    private_data_slot,
                    data,
                )
            },
        }
    }
    extern "system" fn get_private_data(
        device: vk::Device,
        object_type: vk::ObjectType,
        object_handle: u64,
        private_data_slot: vk::PrivateDataSlot,
        p_data: *mut u64,
    ) {
        let global = Self::instance();
        // vkGetPrivateData
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.get_private_data(
            object_type,
            object_handle,
            private_data_slot,
        );
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_data.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_private_data)(
                    device,
                    object_type,
                    object_handle,
                    private_data_slot,
                    p_data,
                )
            },
        }
    }
    extern "system" fn cmd_set_event2(
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        p_dependency_info: *const vk::DependencyInfo,
    ) {
        let global = Self::instance();
        // vkCmdSetEvent2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_set_event2(
            command_buffer,
            event,
            unsafe { p_dependency_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_event2)(command_buffer, event, p_dependency_info)
            },
        }
    }
    extern "system" fn cmd_reset_event2(
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags2,
    ) {
        let global = Self::instance();
        // vkCmdResetEvent2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result =
            device_info
                .customized_info
                .cmd_reset_event2(command_buffer, event, stage_mask);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_reset_event2)(command_buffer, event, stage_mask)
            },
        }
    }
    extern "system" fn cmd_wait_events2(
        command_buffer: vk::CommandBuffer,
        event_count: u32,
        p_events: *const vk::Event,
        p_dependency_infos: *const vk::DependencyInfo,
    ) {
        let global = Self::instance();
        // vkCmdWaitEvents2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_wait_events2(
            command_buffer,
            unsafe { std::slice::from_raw_parts(p_events, event_count as usize) },
            unsafe { std::slice::from_raw_parts(p_dependency_infos, event_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_wait_events2)(
                    command_buffer,
                    event_count,
                    p_events,
                    p_dependency_infos,
                )
            },
        }
    }
    extern "system" fn cmd_pipeline_barrier2(
        command_buffer: vk::CommandBuffer,
        p_dependency_info: *const vk::DependencyInfo,
    ) {
        let global = Self::instance();
        // vkCmdPipelineBarrier2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_pipeline_barrier2(
            command_buffer,
            unsafe { p_dependency_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_pipeline_barrier2)(command_buffer, p_dependency_info)
            },
        }
    }
    extern "system" fn cmd_write_timestamp2(
        command_buffer: vk::CommandBuffer,
        stage: vk::PipelineStageFlags2,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        let global = Self::instance();
        // vkCmdWriteTimestamp2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_write_timestamp2(
            command_buffer,
            stage,
            query_pool,
            query,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_write_timestamp2)(command_buffer, stage, query_pool, query)
            },
        }
    }
    extern "system" fn queue_submit2(
        queue: vk::Queue,
        submit_count: u32,
        p_submits: *const vk::SubmitInfo2,
        fence: vk::Fence,
    ) -> vk::Result {
        let global = Self::instance();
        // vkQueueSubmit2
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.queue_submit2(
            queue,
            unsafe { std::slice::from_raw_parts(p_submits, submit_count as usize) },
            fence,
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.queue_submit2)(queue, submit_count, p_submits, fence)
            },
        }
    }
    extern "system" fn cmd_copy_buffer2(
        command_buffer: vk::CommandBuffer,
        p_copy_buffer_info: *const vk::CopyBufferInfo2,
    ) {
        let global = Self::instance();
        // vkCmdCopyBuffer2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_copy_buffer2(
            command_buffer,
            unsafe { p_copy_buffer_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_buffer2)(command_buffer, p_copy_buffer_info)
            },
        }
    }
    extern "system" fn cmd_copy_image2(
        command_buffer: vk::CommandBuffer,
        p_copy_image_info: *const vk::CopyImageInfo2,
    ) {
        let global = Self::instance();
        // vkCmdCopyImage2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_copy_image2(
            command_buffer,
            unsafe { p_copy_image_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_image2)(command_buffer, p_copy_image_info)
            },
        }
    }
    extern "system" fn cmd_copy_buffer_to_image2(
        command_buffer: vk::CommandBuffer,
        p_copy_buffer_to_image_info: *const vk::CopyBufferToImageInfo2,
    ) {
        let global = Self::instance();
        // vkCmdCopyBufferToImage2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_copy_buffer_to_image2(
            command_buffer,
            unsafe { p_copy_buffer_to_image_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_buffer_to_image2)(
                    command_buffer,
                    p_copy_buffer_to_image_info,
                )
            },
        }
    }
    extern "system" fn cmd_copy_image_to_buffer2(
        command_buffer: vk::CommandBuffer,
        p_copy_image_to_buffer_info: *const vk::CopyImageToBufferInfo2,
    ) {
        let global = Self::instance();
        // vkCmdCopyImageToBuffer2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_copy_image_to_buffer2(
            command_buffer,
            unsafe { p_copy_image_to_buffer_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_image_to_buffer2)(
                    command_buffer,
                    p_copy_image_to_buffer_info,
                )
            },
        }
    }
    extern "system" fn cmd_blit_image2(
        command_buffer: vk::CommandBuffer,
        p_blit_image_info: *const vk::BlitImageInfo2,
    ) {
        let global = Self::instance();
        // vkCmdBlitImage2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_blit_image2(
            command_buffer,
            unsafe { p_blit_image_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_blit_image2)(command_buffer, p_blit_image_info)
            },
        }
    }
    extern "system" fn cmd_resolve_image2(
        command_buffer: vk::CommandBuffer,
        p_resolve_image_info: *const vk::ResolveImageInfo2,
    ) {
        let global = Self::instance();
        // vkCmdResolveImage2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_resolve_image2(
            command_buffer,
            unsafe { p_resolve_image_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_resolve_image2)(command_buffer, p_resolve_image_info)
            },
        }
    }
    extern "system" fn cmd_begin_rendering(
        command_buffer: vk::CommandBuffer,
        p_rendering_info: *const vk::RenderingInfo,
    ) {
        let global = Self::instance();
        // vkCmdBeginRendering
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_begin_rendering(
            command_buffer,
            unsafe { p_rendering_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_begin_rendering)(command_buffer, p_rendering_info)
            },
        }
    }
    extern "system" fn cmd_end_rendering(command_buffer: vk::CommandBuffer) {
        let global = Self::instance();
        // vkCmdEndRendering
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_end_rendering(command_buffer);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe { (dispatch_table.cmd_end_rendering)(command_buffer) },
        }
    }
    extern "system" fn cmd_set_cull_mode(
        command_buffer: vk::CommandBuffer,
        cull_mode: vk::CullModeFlags,
    ) {
        let global = Self::instance();
        // vkCmdSetCullMode
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_cull_mode(command_buffer, cull_mode);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_cull_mode)(command_buffer, cull_mode)
            },
        }
    }
    extern "system" fn cmd_set_front_face(
        command_buffer: vk::CommandBuffer,
        front_face: vk::FrontFace,
    ) {
        let global = Self::instance();
        // vkCmdSetFrontFace
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_front_face(command_buffer, front_face);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_front_face)(command_buffer, front_face)
            },
        }
    }
    extern "system" fn cmd_set_primitive_topology(
        command_buffer: vk::CommandBuffer,
        primitive_topology: vk::PrimitiveTopology,
    ) {
        let global = Self::instance();
        // vkCmdSetPrimitiveTopology
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_primitive_topology(command_buffer, primitive_topology);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_primitive_topology)(command_buffer, primitive_topology)
            },
        }
    }
    extern "system" fn cmd_set_viewport_with_count(
        command_buffer: vk::CommandBuffer,
        viewport_count: u32,
        p_viewports: *const vk::Viewport,
    ) {
        let global = Self::instance();
        // vkCmdSetViewportWithCount
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_set_viewport_with_count(command_buffer, unsafe {
                std::slice::from_raw_parts(p_viewports, viewport_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_viewport_with_count)(
                    command_buffer,
                    viewport_count,
                    p_viewports,
                )
            },
        }
    }
    extern "system" fn cmd_set_scissor_with_count(
        command_buffer: vk::CommandBuffer,
        scissor_count: u32,
        p_scissors: *const vk::Rect2D,
    ) {
        let global = Self::instance();
        // vkCmdSetScissorWithCount
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_set_scissor_with_count(command_buffer, unsafe {
                std::slice::from_raw_parts(p_scissors, scissor_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_scissor_with_count)(
                    command_buffer,
                    scissor_count,
                    p_scissors,
                )
            },
        }
    }
    extern "system" fn cmd_bind_vertex_buffers2(
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const vk::Buffer,
        p_offsets: *const vk::DeviceSize,
        p_sizes: *const vk::DeviceSize,
        p_strides: *const vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkCmdBindVertexBuffers2
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_bind_vertex_buffers2(
            command_buffer,
            first_binding,
            unsafe { std::slice::from_raw_parts(p_buffers, binding_count as usize) },
            unsafe { std::slice::from_raw_parts(p_offsets, binding_count as usize) },
            if p_sizes.is_null() {
                None
            } else {
                Some(unsafe { std::slice::from_raw_parts(p_sizes, binding_count as usize) })
            },
            if p_strides.is_null() {
                None
            } else {
                Some(unsafe { std::slice::from_raw_parts(p_strides, binding_count as usize) })
            },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_vertex_buffers2)(
                    command_buffer,
                    first_binding,
                    binding_count,
                    p_buffers,
                    p_offsets,
                    p_sizes,
                    p_strides,
                )
            },
        }
    }
    extern "system" fn cmd_set_depth_test_enable(
        command_buffer: vk::CommandBuffer,
        depth_test_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetDepthTestEnable
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_depth_test_enable(command_buffer, depth_test_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_depth_test_enable)(command_buffer, depth_test_enable)
            },
        }
    }
    extern "system" fn cmd_set_depth_write_enable(
        command_buffer: vk::CommandBuffer,
        depth_write_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetDepthWriteEnable
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_depth_write_enable(command_buffer, depth_write_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_depth_write_enable)(command_buffer, depth_write_enable)
            },
        }
    }
    extern "system" fn cmd_set_depth_compare_op(
        command_buffer: vk::CommandBuffer,
        depth_compare_op: vk::CompareOp,
    ) {
        let global = Self::instance();
        // vkCmdSetDepthCompareOp
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_depth_compare_op(command_buffer, depth_compare_op);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_depth_compare_op)(command_buffer, depth_compare_op)
            },
        }
    }
    extern "system" fn cmd_set_depth_bounds_test_enable(
        command_buffer: vk::CommandBuffer,
        depth_bounds_test_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetDepthBoundsTestEnable
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_depth_bounds_test_enable(command_buffer, depth_bounds_test_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_depth_bounds_test_enable)(
                    command_buffer,
                    depth_bounds_test_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_stencil_test_enable(
        command_buffer: vk::CommandBuffer,
        stencil_test_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetStencilTestEnable
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_stencil_test_enable(command_buffer, stencil_test_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_stencil_test_enable)(command_buffer, stencil_test_enable)
            },
        }
    }
    extern "system" fn cmd_set_stencil_op(
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        fail_op: vk::StencilOp,
        pass_op: vk::StencilOp,
        depth_fail_op: vk::StencilOp,
        compare_op: vk::CompareOp,
    ) {
        let global = Self::instance();
        // vkCmdSetStencilOp
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info.customized_info.cmd_set_stencil_op(
            command_buffer,
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_stencil_op)(
                    command_buffer,
                    face_mask,
                    fail_op,
                    pass_op,
                    depth_fail_op,
                    compare_op,
                )
            },
        }
    }
    extern "system" fn cmd_set_rasterizer_discard_enable(
        command_buffer: vk::CommandBuffer,
        rasterizer_discard_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetRasterizerDiscardEnable
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_rasterizer_discard_enable(
                command_buffer,
                rasterizer_discard_enable == vk::TRUE,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_rasterizer_discard_enable)(
                    command_buffer,
                    rasterizer_discard_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_depth_bias_enable(
        command_buffer: vk::CommandBuffer,
        depth_bias_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetDepthBiasEnable
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_depth_bias_enable(command_buffer, depth_bias_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_depth_bias_enable)(command_buffer, depth_bias_enable)
            },
        }
    }
    extern "system" fn cmd_set_primitive_restart_enable(
        command_buffer: vk::CommandBuffer,
        primitive_restart_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetPrimitiveRestartEnable
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .cmd_set_primitive_restart_enable(command_buffer, primitive_restart_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_primitive_restart_enable)(
                    command_buffer,
                    primitive_restart_enable,
                )
            },
        }
    }
    extern "system" fn get_device_buffer_memory_requirements(
        device: vk::Device,
        p_info: *const vk::DeviceBufferMemoryRequirements,
        p_memory_requirements: *mut vk::MemoryRequirements2,
    ) {
        let global = Self::instance();
        // vkGetDeviceBufferMemoryRequirements
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .get_device_buffer_memory_requirements(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_memory_requirements.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_buffer_memory_requirements)(
                    device,
                    p_info,
                    p_memory_requirements,
                )
            },
        }
    }
    extern "system" fn get_device_image_memory_requirements(
        device: vk::Device,
        p_info: *const vk::DeviceImageMemoryRequirements,
        p_memory_requirements: *mut vk::MemoryRequirements2,
    ) {
        let global = Self::instance();
        // vkGetDeviceImageMemoryRequirements
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .get_device_image_memory_requirements(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_memory_requirements.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_image_memory_requirements)(
                    device,
                    p_info,
                    p_memory_requirements,
                )
            },
        }
    }
    extern "system" fn get_device_image_sparse_memory_requirements(
        device: vk::Device,
        p_info: *const vk::DeviceImageMemoryRequirements,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut vk::SparseImageMemoryRequirements2,
    ) {
        let global = Self::instance();
        // vkGetDeviceImageSparseMemoryRequirements
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.core.fp_v1_3();
        let layer_result = device_info
            .customized_info
            .get_device_image_sparse_memory_requirements(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                // We can't return INCOMPLETE from vkGetDeviceImageSparseMemoryRequirements
                #[allow(unused_must_use)]
                unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_sparse_memory_requirement_count).unwrap(),
                        p_sparse_memory_requirements,
                    );
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_image_sparse_memory_requirements)(
                    device,
                    p_info,
                    p_sparse_memory_requirement_count,
                    p_sparse_memory_requirements,
                )
            },
        }
    }
    extern "system" fn create_swapchain_khr(
        device: vk::Device,
        p_create_info: *const vk::SwapchainCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_swapchain: *mut vk::SwapchainKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateSwapchainKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_swapchain;
        let layer_result = device_info
            .customized_info
            .create_swapchain_khr(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_swapchain.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_swapchain_khr)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_swapchain,
                )
            },
        }
    }
    extern "system" fn destroy_swapchain_khr(
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroySwapchainKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_swapchain;
        let layer_result = device_info
            .customized_info
            .destroy_swapchain_khr(swapchain, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_swapchain_khr)(device, swapchain, p_allocator)
            },
        }
    }
    extern "system" fn get_swapchain_images_khr(
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut vk::Image,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetSwapchainImagesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_swapchain;
        let layer_result = device_info
            .customized_info
            .get_swapchain_images_khr(swapchain);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_swapchain_image_count).unwrap(),
                        p_swapchain_images,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_swapchain_images_khr)(
                    device,
                    swapchain,
                    p_swapchain_image_count,
                    p_swapchain_images,
                )
            },
        }
    }
    extern "system" fn acquire_next_image_khr(
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        timeout: u64,
        semaphore: vk::Semaphore,
        fence: vk::Fence,
        p_image_index: *mut u32,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAcquireNextImageKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_swapchain;
        let layer_result = device_info
            .customized_info
            .acquire_next_image_khr(swapchain, timeout, semaphore, fence);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_image_index.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.acquire_next_image_khr)(
                    device,
                    swapchain,
                    timeout,
                    semaphore,
                    fence,
                    p_image_index,
                )
            },
        }
    }
    extern "system" fn queue_present_khr(
        queue: vk::Queue,
        p_present_info: *const vk::PresentInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkQueuePresentKHR
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_swapchain;
        let layer_result = device_info
            .customized_info
            .queue_present_khr(queue, unsafe { p_present_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.queue_present_khr)(queue, p_present_info)
            },
        }
    }
    extern "system" fn get_device_group_present_capabilities_khr(
        device: vk::Device,
        p_device_group_present_capabilities: *mut vk::DeviceGroupPresentCapabilitiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDeviceGroupPresentCapabilitiesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_swapchain;
        let layer_result = device_info
            .customized_info
            .get_device_group_present_capabilities_khr();
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_device_group_present_capabilities.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_group_present_capabilities_khr)(
                    device,
                    p_device_group_present_capabilities,
                )
            },
        }
    }
    extern "system" fn get_device_group_surface_present_modes_khr(
        device: vk::Device,
        surface: vk::SurfaceKHR,
        p_modes: *mut vk::DeviceGroupPresentModeFlagsKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDeviceGroupSurfacePresentModesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_swapchain;
        let layer_result = device_info
            .customized_info
            .get_device_group_surface_present_modes_khr(surface);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_modes.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_group_surface_present_modes_khr)(
                    device, surface, p_modes,
                )
            },
        }
    }
    extern "system" fn acquire_next_image2_khr(
        device: vk::Device,
        p_acquire_info: *const vk::AcquireNextImageInfoKHR,
        p_image_index: *mut u32,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAcquireNextImage2KHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_swapchain;
        let layer_result = device_info
            .customized_info
            .acquire_next_image2_khr(unsafe { p_acquire_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_image_index.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.acquire_next_image2_khr)(device, p_acquire_info, p_image_index)
            },
        }
    }
    extern "system" fn create_shared_swapchains_khr(
        device: vk::Device,
        swapchain_count: u32,
        p_create_infos: *const vk::SwapchainCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_swapchains: *mut vk::SwapchainKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateSharedSwapchainsKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_display_swapchain;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.create_shared_swapchains_khr(
            unsafe { std::slice::from_raw_parts(p_create_infos, swapchain_count as usize) },
            unsafe { p_allocator.as_ref() },
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    unsafe {
                        std::slice::from_raw_parts_mut(
                            p_swapchains,
                            swapchain_count.try_into().unwrap(),
                        )
                    }
                    .copy_from_slice(&res);
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_shared_swapchains_khr)(
                    device,
                    swapchain_count,
                    p_create_infos,
                    p_allocator,
                    p_swapchains,
                )
            },
        }
    }
    extern "system" fn create_video_session_khr(
        device: vk::Device,
        p_create_info: *const vk::VideoSessionCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_video_session: *mut vk::VideoSessionKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateVideoSessionKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_queue;
        let layer_result = device_info
            .customized_info
            .create_video_session_khr(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_video_session.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_video_session_khr)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_video_session,
                )
            },
        }
    }
    extern "system" fn destroy_video_session_khr(
        device: vk::Device,
        video_session: vk::VideoSessionKHR,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyVideoSessionKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_queue;
        let layer_result = device_info
            .customized_info
            .destroy_video_session_khr(video_session, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_video_session_khr)(device, video_session, p_allocator)
            },
        }
    }
    extern "system" fn get_video_session_memory_requirements_khr(
        device: vk::Device,
        video_session: vk::VideoSessionKHR,
        p_memory_requirements_count: *mut u32,
        p_memory_requirements: *mut vk::VideoSessionMemoryRequirementsKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetVideoSessionMemoryRequirementsKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_queue;
        let layer_result = device_info
            .customized_info
            .get_video_session_memory_requirements_khr(video_session);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_memory_requirements_count).unwrap(),
                        p_memory_requirements,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_video_session_memory_requirements_khr)(
                    device,
                    video_session,
                    p_memory_requirements_count,
                    p_memory_requirements,
                )
            },
        }
    }
    extern "system" fn bind_video_session_memory_khr(
        device: vk::Device,
        video_session: vk::VideoSessionKHR,
        bind_session_memory_info_count: u32,
        p_bind_session_memory_infos: *const vk::BindVideoSessionMemoryInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkBindVideoSessionMemoryKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_queue;
        #[allow(clippy::unnecessary_cast)]
        let layer_result =
            device_info
                .customized_info
                .bind_video_session_memory_khr(video_session, unsafe {
                    std::slice::from_raw_parts(
                        p_bind_session_memory_infos,
                        bind_session_memory_info_count as usize,
                    )
                });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.bind_video_session_memory_khr)(
                    device,
                    video_session,
                    bind_session_memory_info_count,
                    p_bind_session_memory_infos,
                )
            },
        }
    }
    extern "system" fn create_video_session_parameters_khr(
        device: vk::Device,
        p_create_info: *const vk::VideoSessionParametersCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_video_session_parameters: *mut vk::VideoSessionParametersKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateVideoSessionParametersKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_queue;
        let layer_result = device_info
            .customized_info
            .create_video_session_parameters_khr(
                unsafe { p_create_info.as_ref() }.unwrap(),
                unsafe { p_allocator.as_ref() },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_video_session_parameters.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_video_session_parameters_khr)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_video_session_parameters,
                )
            },
        }
    }
    extern "system" fn update_video_session_parameters_khr(
        device: vk::Device,
        video_session_parameters: vk::VideoSessionParametersKHR,
        p_update_info: *const vk::VideoSessionParametersUpdateInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkUpdateVideoSessionParametersKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_queue;
        let layer_result = device_info
            .customized_info
            .update_video_session_parameters_khr(
                video_session_parameters,
                unsafe { p_update_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.update_video_session_parameters_khr)(
                    device,
                    video_session_parameters,
                    p_update_info,
                )
            },
        }
    }
    extern "system" fn destroy_video_session_parameters_khr(
        device: vk::Device,
        video_session_parameters: vk::VideoSessionParametersKHR,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyVideoSessionParametersKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_queue;
        let layer_result = device_info
            .customized_info
            .destroy_video_session_parameters_khr(video_session_parameters, unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_video_session_parameters_khr)(
                    device,
                    video_session_parameters,
                    p_allocator,
                )
            },
        }
    }
    extern "system" fn cmd_begin_video_coding_khr(
        command_buffer: vk::CommandBuffer,
        p_begin_info: *const vk::VideoBeginCodingInfoKHR,
    ) {
        let global = Self::instance();
        // vkCmdBeginVideoCodingKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_queue;
        let layer_result = device_info
            .customized_info
            .cmd_begin_video_coding_khr(command_buffer, unsafe { p_begin_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_begin_video_coding_khr)(command_buffer, p_begin_info)
            },
        }
    }
    extern "system" fn cmd_end_video_coding_khr(
        command_buffer: vk::CommandBuffer,
        p_end_coding_info: *const vk::VideoEndCodingInfoKHR,
    ) {
        let global = Self::instance();
        // vkCmdEndVideoCodingKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_queue;
        let layer_result = device_info.customized_info.cmd_end_video_coding_khr(
            command_buffer,
            unsafe { p_end_coding_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_end_video_coding_khr)(command_buffer, p_end_coding_info)
            },
        }
    }
    extern "system" fn cmd_control_video_coding_khr(
        command_buffer: vk::CommandBuffer,
        p_coding_control_info: *const vk::VideoCodingControlInfoKHR,
    ) {
        let global = Self::instance();
        // vkCmdControlVideoCodingKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_queue;
        let layer_result = device_info.customized_info.cmd_control_video_coding_khr(
            command_buffer,
            unsafe { p_coding_control_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_control_video_coding_khr)(command_buffer, p_coding_control_info)
            },
        }
    }
    extern "system" fn cmd_decode_video_khr(
        command_buffer: vk::CommandBuffer,
        p_decode_info: *const vk::VideoDecodeInfoKHR,
    ) {
        let global = Self::instance();
        // vkCmdDecodeVideoKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_decode_queue;
        let layer_result = device_info
            .customized_info
            .cmd_decode_video_khr(command_buffer, unsafe { p_decode_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_decode_video_khr)(command_buffer, p_decode_info)
            },
        }
    }
    extern "system" fn get_memory_win32_handle_khr(
        device: vk::Device,
        p_get_win32_handle_info: *const vk::MemoryGetWin32HandleInfoKHR,
        p_handle: *mut vk::HANDLE,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetMemoryWin32HandleKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_memory_win32;
        let layer_result = device_info
            .customized_info
            .get_memory_win32_handle_khr(unsafe { p_get_win32_handle_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_handle.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_memory_win32_handle_khr)(
                    device,
                    p_get_win32_handle_info,
                    p_handle,
                )
            },
        }
    }
    extern "system" fn get_memory_win32_handle_properties_khr(
        device: vk::Device,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        handle: vk::HANDLE,
        p_memory_win32_handle_properties: *mut vk::MemoryWin32HandlePropertiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetMemoryWin32HandlePropertiesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_memory_win32;
        let layer_result = device_info
            .customized_info
            .get_memory_win32_handle_properties_khr(handle_type, handle);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_memory_win32_handle_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_memory_win32_handle_properties_khr)(
                    device,
                    handle_type,
                    handle,
                    p_memory_win32_handle_properties,
                )
            },
        }
    }
    extern "system" fn get_memory_fd_khr(
        device: vk::Device,
        p_get_fd_info: *const vk::MemoryGetFdInfoKHR,
        p_fd: *mut c_int,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetMemoryFdKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_memory_fd;
        let layer_result = device_info
            .customized_info
            .get_memory_fd_khr(unsafe { p_get_fd_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_fd.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_memory_fd_khr)(device, p_get_fd_info, p_fd)
            },
        }
    }
    extern "system" fn get_memory_fd_properties_khr(
        device: vk::Device,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        fd: c_int,
        p_memory_fd_properties: *mut vk::MemoryFdPropertiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetMemoryFdPropertiesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_memory_fd;
        let layer_result = device_info
            .customized_info
            .get_memory_fd_properties_khr(handle_type, fd);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_memory_fd_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_memory_fd_properties_khr)(
                    device,
                    handle_type,
                    fd,
                    p_memory_fd_properties,
                )
            },
        }
    }
    extern "system" fn import_semaphore_win32_handle_khr(
        device: vk::Device,
        p_import_semaphore_win32_handle_info: *const vk::ImportSemaphoreWin32HandleInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkImportSemaphoreWin32HandleKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_semaphore_win32;
        let layer_result = device_info
            .customized_info
            .import_semaphore_win32_handle_khr(
                unsafe { p_import_semaphore_win32_handle_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.import_semaphore_win32_handle_khr)(
                    device,
                    p_import_semaphore_win32_handle_info,
                )
            },
        }
    }
    extern "system" fn get_semaphore_win32_handle_khr(
        device: vk::Device,
        p_get_win32_handle_info: *const vk::SemaphoreGetWin32HandleInfoKHR,
        p_handle: *mut vk::HANDLE,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetSemaphoreWin32HandleKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_semaphore_win32;
        let layer_result = device_info
            .customized_info
            .get_semaphore_win32_handle_khr(unsafe { p_get_win32_handle_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_handle.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_semaphore_win32_handle_khr)(
                    device,
                    p_get_win32_handle_info,
                    p_handle,
                )
            },
        }
    }
    extern "system" fn import_semaphore_fd_khr(
        device: vk::Device,
        p_import_semaphore_fd_info: *const vk::ImportSemaphoreFdInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkImportSemaphoreFdKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_semaphore_fd;
        let layer_result = device_info
            .customized_info
            .import_semaphore_fd_khr(unsafe { p_import_semaphore_fd_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.import_semaphore_fd_khr)(device, p_import_semaphore_fd_info)
            },
        }
    }
    extern "system" fn get_semaphore_fd_khr(
        device: vk::Device,
        p_get_fd_info: *const vk::SemaphoreGetFdInfoKHR,
        p_fd: *mut c_int,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetSemaphoreFdKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_semaphore_fd;
        let layer_result = device_info
            .customized_info
            .get_semaphore_fd_khr(unsafe { p_get_fd_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_fd.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_semaphore_fd_khr)(device, p_get_fd_info, p_fd)
            },
        }
    }
    extern "system" fn cmd_push_descriptor_set_khr(
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const vk::WriteDescriptorSet,
    ) {
        let global = Self::instance();
        // vkCmdPushDescriptorSetKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_push_descriptor;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_push_descriptor_set_khr(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
            unsafe {
                std::slice::from_raw_parts(p_descriptor_writes, descriptor_write_count as usize)
            },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_push_descriptor_set_khr)(
                    command_buffer,
                    pipeline_bind_point,
                    layout,
                    set,
                    descriptor_write_count,
                    p_descriptor_writes,
                )
            },
        }
    }
    extern "system" fn cmd_push_descriptor_set_with_template_khr(
        command_buffer: vk::CommandBuffer,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        layout: vk::PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ) {
        let global = Self::instance();
        // vkCmdPushDescriptorSetWithTemplateKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_push_descriptor;
        let layer_result = device_info
            .customized_info
            .cmd_push_descriptor_set_with_template_khr(
                command_buffer,
                descriptor_update_template,
                layout,
                set,
                unsafe { p_data.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_push_descriptor_set_with_template_khr)(
                    command_buffer,
                    descriptor_update_template,
                    layout,
                    set,
                    p_data,
                )
            },
        }
    }
    extern "system" fn get_swapchain_status_khr(
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetSwapchainStatusKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_shared_presentable_image;
        let layer_result = device_info
            .customized_info
            .get_swapchain_status_khr(swapchain);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_swapchain_status_khr)(device, swapchain)
            },
        }
    }
    extern "system" fn import_fence_win32_handle_khr(
        device: vk::Device,
        p_import_fence_win32_handle_info: *const vk::ImportFenceWin32HandleInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkImportFenceWin32HandleKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_fence_win32;
        let layer_result = device_info.customized_info.import_fence_win32_handle_khr(
            unsafe { p_import_fence_win32_handle_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.import_fence_win32_handle_khr)(
                    device,
                    p_import_fence_win32_handle_info,
                )
            },
        }
    }
    extern "system" fn get_fence_win32_handle_khr(
        device: vk::Device,
        p_get_win32_handle_info: *const vk::FenceGetWin32HandleInfoKHR,
        p_handle: *mut vk::HANDLE,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetFenceWin32HandleKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_fence_win32;
        let layer_result = device_info
            .customized_info
            .get_fence_win32_handle_khr(unsafe { p_get_win32_handle_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_handle.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_fence_win32_handle_khr)(
                    device,
                    p_get_win32_handle_info,
                    p_handle,
                )
            },
        }
    }
    extern "system" fn import_fence_fd_khr(
        device: vk::Device,
        p_import_fence_fd_info: *const vk::ImportFenceFdInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkImportFenceFdKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_fence_fd;
        let layer_result = device_info
            .customized_info
            .import_fence_fd_khr(unsafe { p_import_fence_fd_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.import_fence_fd_khr)(device, p_import_fence_fd_info)
            },
        }
    }
    extern "system" fn get_fence_fd_khr(
        device: vk::Device,
        p_get_fd_info: *const vk::FenceGetFdInfoKHR,
        p_fd: *mut c_int,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetFenceFdKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_external_fence_fd;
        let layer_result = device_info
            .customized_info
            .get_fence_fd_khr(unsafe { p_get_fd_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_fd.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_fence_fd_khr)(device, p_get_fd_info, p_fd)
            },
        }
    }
    extern "system" fn acquire_profiling_lock_khr(
        device: vk::Device,
        p_info: *const vk::AcquireProfilingLockInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAcquireProfilingLockKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_performance_query;
        let layer_result = device_info
            .customized_info
            .acquire_profiling_lock_khr(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.acquire_profiling_lock_khr)(device, p_info)
            },
        }
    }
    extern "system" fn release_profiling_lock_khr(device: vk::Device) {
        let global = Self::instance();
        // vkReleaseProfilingLockKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_performance_query;
        let layer_result = device_info.customized_info.release_profiling_lock_khr();
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.release_profiling_lock_khr)(device)
            },
        }
    }
    extern "system" fn cmd_set_fragment_shading_rate_khr(
        command_buffer: vk::CommandBuffer,
        p_fragment_size: *const vk::Extent2D,
        combiner_ops: *const [vk::FragmentShadingRateCombinerOpKHR; 2],
    ) {
        let global = Self::instance();
        // vkCmdSetFragmentShadingRateKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_fragment_shading_rate;
        let combiner_ops = unsafe { combiner_ops.as_ref() }.unwrap();
        let layer_result = device_info
            .customized_info
            .cmd_set_fragment_shading_rate_khr(
                command_buffer,
                unsafe { p_fragment_size.as_ref() }.unwrap(),
                combiner_ops,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_fragment_shading_rate_khr)(
                    command_buffer,
                    p_fragment_size,
                    combiner_ops,
                )
            },
        }
    }
    extern "system" fn wait_for_present_khr(
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> vk::Result {
        let global = Self::instance();
        // vkWaitForPresentKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_present_wait;
        let layer_result = device_info
            .customized_info
            .wait_for_present_khr(swapchain, present_id, timeout);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.wait_for_present_khr)(device, swapchain, present_id, timeout)
            },
        }
    }
    extern "system" fn create_deferred_operation_khr(
        device: vk::Device,
        p_allocator: *const vk::AllocationCallbacks,
        p_deferred_operation: *mut vk::DeferredOperationKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateDeferredOperationKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_deferred_host_operations;
        let layer_result = device_info
            .customized_info
            .create_deferred_operation_khr(unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_deferred_operation.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_deferred_operation_khr)(
                    device,
                    p_allocator,
                    p_deferred_operation,
                )
            },
        }
    }
    extern "system" fn destroy_deferred_operation_khr(
        device: vk::Device,
        operation: vk::DeferredOperationKHR,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyDeferredOperationKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_deferred_host_operations;
        let layer_result = device_info
            .customized_info
            .destroy_deferred_operation_khr(operation, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_deferred_operation_khr)(device, operation, p_allocator)
            },
        }
    }
    extern "system" fn get_deferred_operation_max_concurrency_khr(
        device: vk::Device,
        operation: vk::DeferredOperationKHR,
    ) -> u32 {
        let global = Self::instance();
        // vkGetDeferredOperationMaxConcurrencyKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_deferred_host_operations;
        let layer_result = device_info
            .customized_info
            .get_deferred_operation_max_concurrency_khr(operation);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_deferred_operation_max_concurrency_khr)(device, operation)
            },
        }
    }
    extern "system" fn get_deferred_operation_result_khr(
        device: vk::Device,
        operation: vk::DeferredOperationKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDeferredOperationResultKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_deferred_host_operations;
        let layer_result = device_info
            .customized_info
            .get_deferred_operation_result_khr(operation);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_deferred_operation_result_khr)(device, operation)
            },
        }
    }
    extern "system" fn deferred_operation_join_khr(
        device: vk::Device,
        operation: vk::DeferredOperationKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkDeferredOperationJoinKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_deferred_host_operations;
        let layer_result = device_info
            .customized_info
            .deferred_operation_join_khr(operation);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.deferred_operation_join_khr)(device, operation)
            },
        }
    }
    extern "system" fn get_pipeline_executable_properties_khr(
        device: vk::Device,
        p_pipeline_info: *const vk::PipelineInfoKHR,
        p_executable_count: *mut u32,
        p_properties: *mut vk::PipelineExecutablePropertiesKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPipelineExecutablePropertiesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info
            .dispatch_table
            .khr_pipeline_executable_properties;
        let layer_result = device_info
            .customized_info
            .get_pipeline_executable_properties_khr(unsafe { p_pipeline_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_executable_count).unwrap(),
                        p_properties,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_pipeline_executable_properties_khr)(
                    device,
                    p_pipeline_info,
                    p_executable_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_pipeline_executable_statistics_khr(
        device: vk::Device,
        p_executable_info: *const vk::PipelineExecutableInfoKHR,
        p_statistic_count: *mut u32,
        p_statistics: *mut vk::PipelineExecutableStatisticKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPipelineExecutableStatisticsKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info
            .dispatch_table
            .khr_pipeline_executable_properties;
        let layer_result = device_info
            .customized_info
            .get_pipeline_executable_statistics_khr(unsafe { p_executable_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(&res, NonNull::new(p_statistic_count).unwrap(), p_statistics)
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_pipeline_executable_statistics_khr)(
                    device,
                    p_executable_info,
                    p_statistic_count,
                    p_statistics,
                )
            },
        }
    }
    extern "system" fn get_pipeline_executable_internal_representations_khr(
        device: vk::Device,
        p_executable_info: *const vk::PipelineExecutableInfoKHR,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut vk::PipelineExecutableInternalRepresentationKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPipelineExecutableInternalRepresentationsKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info
            .dispatch_table
            .khr_pipeline_executable_properties;
        let layer_result = device_info
            .customized_info
            .get_pipeline_executable_internal_representations_khr(
                unsafe { p_executable_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_internal_representation_count).unwrap(),
                        p_internal_representations,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_pipeline_executable_internal_representations_khr)(
                    device,
                    p_executable_info,
                    p_internal_representation_count,
                    p_internal_representations,
                )
            },
        }
    }
    extern "system" fn cmd_encode_video_khr(
        command_buffer: vk::CommandBuffer,
        p_encode_info: *const vk::VideoEncodeInfoKHR,
    ) {
        let global = Self::instance();
        // vkCmdEncodeVideoKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_video_encode_queue;
        let layer_result = device_info
            .customized_info
            .cmd_encode_video_khr(command_buffer, unsafe { p_encode_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_encode_video_khr)(command_buffer, p_encode_info)
            },
        }
    }
    extern "system" fn cmd_write_buffer_marker2_amd(
        command_buffer: vk::CommandBuffer,
        stage: vk::PipelineStageFlags2,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        marker: u32,
    ) {
        let global = Self::instance();
        // vkCmdWriteBufferMarker2AMD
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_synchronization2;
        let layer_result = device_info.customized_info.cmd_write_buffer_marker2_amd(
            command_buffer,
            stage,
            dst_buffer,
            dst_offset,
            marker,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_write_buffer_marker2_amd)(
                    command_buffer,
                    stage,
                    dst_buffer,
                    dst_offset,
                    marker,
                )
            },
        }
    }
    extern "system" fn get_queue_checkpoint_data2_nv(
        queue: vk::Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut vk::CheckpointData2NV,
    ) {
        let global = Self::instance();
        // vkGetQueueCheckpointData2NV
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_synchronization2;
        let layer_result = device_info
            .customized_info
            .get_queue_checkpoint_data2_nv(queue);
        match layer_result {
            LayerResult::Handled(res) => {
                // We can't return INCOMPLETE from vkGetQueueCheckpointData2NV
                #[allow(unused_must_use)]
                unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_checkpoint_data_count).unwrap(),
                        p_checkpoint_data,
                    );
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_queue_checkpoint_data2_nv)(
                    queue,
                    p_checkpoint_data_count,
                    p_checkpoint_data,
                )
            },
        }
    }
    extern "system" fn cmd_trace_rays_indirect2_khr(
        command_buffer: vk::CommandBuffer,
        indirect_device_address: vk::DeviceAddress,
    ) {
        let global = Self::instance();
        // vkCmdTraceRaysIndirect2KHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_ray_tracing_maintenance1;
        let layer_result = device_info
            .customized_info
            .cmd_trace_rays_indirect2_khr(command_buffer, indirect_device_address);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_trace_rays_indirect2_khr)(
                    command_buffer,
                    indirect_device_address,
                )
            },
        }
    }
    extern "system" fn debug_marker_set_object_tag_ext(
        device: vk::Device,
        p_tag_info: *const vk::DebugMarkerObjectTagInfoEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkDebugMarkerSetObjectTagEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_marker;
        let layer_result = device_info
            .customized_info
            .debug_marker_set_object_tag_ext(unsafe { p_tag_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.debug_marker_set_object_tag_ext)(device, p_tag_info)
            },
        }
    }
    extern "system" fn debug_marker_set_object_name_ext(
        device: vk::Device,
        p_name_info: *const vk::DebugMarkerObjectNameInfoEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkDebugMarkerSetObjectNameEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_marker;
        let layer_result = device_info
            .customized_info
            .debug_marker_set_object_name_ext(unsafe { p_name_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.debug_marker_set_object_name_ext)(device, p_name_info)
            },
        }
    }
    extern "system" fn cmd_debug_marker_begin_ext(
        command_buffer: vk::CommandBuffer,
        p_marker_info: *const vk::DebugMarkerMarkerInfoEXT,
    ) {
        let global = Self::instance();
        // vkCmdDebugMarkerBeginEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_marker;
        let layer_result = device_info
            .customized_info
            .cmd_debug_marker_begin_ext(command_buffer, unsafe { p_marker_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_debug_marker_begin_ext)(command_buffer, p_marker_info)
            },
        }
    }
    extern "system" fn cmd_debug_marker_end_ext(command_buffer: vk::CommandBuffer) {
        let global = Self::instance();
        // vkCmdDebugMarkerEndEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_marker;
        let layer_result = device_info
            .customized_info
            .cmd_debug_marker_end_ext(command_buffer);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_debug_marker_end_ext)(command_buffer)
            },
        }
    }
    extern "system" fn cmd_debug_marker_insert_ext(
        command_buffer: vk::CommandBuffer,
        p_marker_info: *const vk::DebugMarkerMarkerInfoEXT,
    ) {
        let global = Self::instance();
        // vkCmdDebugMarkerInsertEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_marker;
        let layer_result = device_info.customized_info.cmd_debug_marker_insert_ext(
            command_buffer,
            unsafe { p_marker_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_debug_marker_insert_ext)(command_buffer, p_marker_info)
            },
        }
    }
    extern "system" fn cmd_bind_transform_feedback_buffers_ext(
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const vk::Buffer,
        p_offsets: *const vk::DeviceSize,
        p_sizes: *const vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkCmdBindTransformFeedbackBuffersEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_transform_feedback;
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_bind_transform_feedback_buffers_ext(
                command_buffer,
                first_binding,
                unsafe { std::slice::from_raw_parts(p_buffers, binding_count as usize) },
                unsafe { std::slice::from_raw_parts(p_offsets, binding_count as usize) },
                if p_sizes.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(p_sizes, binding_count as usize) })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_transform_feedback_buffers_ext)(
                    command_buffer,
                    first_binding,
                    binding_count,
                    p_buffers,
                    p_offsets,
                    p_sizes,
                )
            },
        }
    }
    extern "system" fn cmd_begin_transform_feedback_ext(
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const vk::Buffer,
        p_counter_buffer_offsets: *const vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkCmdBeginTransformFeedbackEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_transform_feedback;
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_begin_transform_feedback_ext(
                command_buffer,
                first_counter_buffer,
                unsafe {
                    std::slice::from_raw_parts(p_counter_buffers, counter_buffer_count as usize)
                },
                if p_counter_buffer_offsets.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts(
                            p_counter_buffer_offsets,
                            counter_buffer_count as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_begin_transform_feedback_ext)(
                    command_buffer,
                    first_counter_buffer,
                    counter_buffer_count,
                    p_counter_buffers,
                    p_counter_buffer_offsets,
                )
            },
        }
    }
    extern "system" fn cmd_end_transform_feedback_ext(
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const vk::Buffer,
        p_counter_buffer_offsets: *const vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkCmdEndTransformFeedbackEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_transform_feedback;
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_end_transform_feedback_ext(
            command_buffer,
            first_counter_buffer,
            unsafe { std::slice::from_raw_parts(p_counter_buffers, counter_buffer_count as usize) },
            if p_counter_buffer_offsets.is_null() {
                None
            } else {
                Some(unsafe {
                    std::slice::from_raw_parts(
                        p_counter_buffer_offsets,
                        counter_buffer_count as usize,
                    )
                })
            },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_end_transform_feedback_ext)(
                    command_buffer,
                    first_counter_buffer,
                    counter_buffer_count,
                    p_counter_buffers,
                    p_counter_buffer_offsets,
                )
            },
        }
    }
    extern "system" fn cmd_begin_query_indexed_ext(
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        flags: vk::QueryControlFlags,
        index: u32,
    ) {
        let global = Self::instance();
        // vkCmdBeginQueryIndexedEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_transform_feedback;
        let layer_result = device_info.customized_info.cmd_begin_query_indexed_ext(
            command_buffer,
            query_pool,
            query,
            flags,
            index,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_begin_query_indexed_ext)(
                    command_buffer,
                    query_pool,
                    query,
                    flags,
                    index,
                )
            },
        }
    }
    extern "system" fn cmd_end_query_indexed_ext(
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        index: u32,
    ) {
        let global = Self::instance();
        // vkCmdEndQueryIndexedEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_transform_feedback;
        let layer_result = device_info.customized_info.cmd_end_query_indexed_ext(
            command_buffer,
            query_pool,
            query,
            index,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_end_query_indexed_ext)(command_buffer, query_pool, query, index)
            },
        }
    }
    extern "system" fn cmd_draw_indirect_byte_count_ext(
        command_buffer: vk::CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: vk::Buffer,
        counter_buffer_offset: vk::DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawIndirectByteCountEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_transform_feedback;
        let layer_result = device_info
            .customized_info
            .cmd_draw_indirect_byte_count_ext(
                command_buffer,
                instance_count,
                first_instance,
                counter_buffer,
                counter_buffer_offset,
                counter_offset,
                vertex_stride,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_indirect_byte_count_ext)(
                    command_buffer,
                    instance_count,
                    first_instance,
                    counter_buffer,
                    counter_buffer_offset,
                    counter_offset,
                    vertex_stride,
                )
            },
        }
    }
    extern "system" fn create_cu_module_nvx(
        device: vk::Device,
        p_create_info: *const vk::CuModuleCreateInfoNVX,
        p_allocator: *const vk::AllocationCallbacks,
        p_module: *mut vk::CuModuleNVX,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateCuModuleNVX
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nvx_binary_import;
        let layer_result = device_info
            .customized_info
            .create_cu_module_nvx(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_module.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_cu_module_nvx)(device, p_create_info, p_allocator, p_module)
            },
        }
    }
    extern "system" fn create_cu_function_nvx(
        device: vk::Device,
        p_create_info: *const vk::CuFunctionCreateInfoNVX,
        p_allocator: *const vk::AllocationCallbacks,
        p_function: *mut vk::CuFunctionNVX,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateCuFunctionNVX
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nvx_binary_import;
        let layer_result = device_info
            .customized_info
            .create_cu_function_nvx(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_function.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_cu_function_nvx)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_function,
                )
            },
        }
    }
    extern "system" fn destroy_cu_module_nvx(
        device: vk::Device,
        module: vk::CuModuleNVX,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyCuModuleNVX
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nvx_binary_import;
        let layer_result = device_info
            .customized_info
            .destroy_cu_module_nvx(module, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_cu_module_nvx)(device, module, p_allocator)
            },
        }
    }
    extern "system" fn destroy_cu_function_nvx(
        device: vk::Device,
        function: vk::CuFunctionNVX,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyCuFunctionNVX
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nvx_binary_import;
        let layer_result = device_info
            .customized_info
            .destroy_cu_function_nvx(function, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_cu_function_nvx)(device, function, p_allocator)
            },
        }
    }
    extern "system" fn cmd_cu_launch_kernel_nvx(
        command_buffer: vk::CommandBuffer,
        p_launch_info: *const vk::CuLaunchInfoNVX,
    ) {
        let global = Self::instance();
        // vkCmdCuLaunchKernelNVX
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nvx_binary_import;
        let layer_result = device_info
            .customized_info
            .cmd_cu_launch_kernel_nvx(command_buffer, unsafe { p_launch_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_cu_launch_kernel_nvx)(command_buffer, p_launch_info)
            },
        }
    }
    extern "system" fn get_image_view_handle_nvx(
        device: vk::Device,
        p_info: *const vk::ImageViewHandleInfoNVX,
    ) -> u32 {
        let global = Self::instance();
        // vkGetImageViewHandleNVX
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nvx_image_view_handle;
        let layer_result = device_info
            .customized_info
            .get_image_view_handle_nvx(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_view_handle_nvx)(device, p_info)
            },
        }
    }
    extern "system" fn get_image_view_address_nvx(
        device: vk::Device,
        image_view: vk::ImageView,
        p_properties: *mut vk::ImageViewAddressPropertiesNVX,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetImageViewAddressNVX
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nvx_image_view_handle;
        let layer_result = device_info
            .customized_info
            .get_image_view_address_nvx(image_view);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_view_address_nvx)(device, image_view, p_properties)
            },
        }
    }
    extern "system" fn get_shader_info_amd(
        device: vk::Device,
        pipeline: vk::Pipeline,
        shader_stage: vk::ShaderStageFlags,
        info_type: vk::ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetShaderInfoAMD
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.amd_shader_info;
        let layer_result =
            device_info
                .customized_info
                .get_shader_info_amd(pipeline, shader_stage, info_type);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    let p_info = p_info as *mut u8;
                    unsafe { fill_vk_out_array(&res, NonNull::new(p_info_size).unwrap(), p_info) }
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_shader_info_amd)(
                    device,
                    pipeline,
                    shader_stage,
                    info_type,
                    p_info_size,
                    p_info,
                )
            },
        }
    }
    extern "system" fn get_memory_win32_handle_nv(
        device: vk::Device,
        memory: vk::DeviceMemory,
        handle_type: vk::ExternalMemoryHandleTypeFlagsNV,
        p_handle: *mut vk::HANDLE,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetMemoryWin32HandleNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_external_memory_win32;
        let layer_result = device_info
            .customized_info
            .get_memory_win32_handle_nv(memory, handle_type);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_handle.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_memory_win32_handle_nv)(device, memory, handle_type, p_handle)
            },
        }
    }
    extern "system" fn cmd_begin_conditional_rendering_ext(
        command_buffer: vk::CommandBuffer,
        p_conditional_rendering_begin: *const vk::ConditionalRenderingBeginInfoEXT,
    ) {
        let global = Self::instance();
        // vkCmdBeginConditionalRenderingEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_conditional_rendering;
        let layer_result = device_info
            .customized_info
            .cmd_begin_conditional_rendering_ext(
                command_buffer,
                unsafe { p_conditional_rendering_begin.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_begin_conditional_rendering_ext)(
                    command_buffer,
                    p_conditional_rendering_begin,
                )
            },
        }
    }
    extern "system" fn cmd_end_conditional_rendering_ext(command_buffer: vk::CommandBuffer) {
        let global = Self::instance();
        // vkCmdEndConditionalRenderingEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_conditional_rendering;
        let layer_result = device_info
            .customized_info
            .cmd_end_conditional_rendering_ext(command_buffer);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_end_conditional_rendering_ext)(command_buffer)
            },
        }
    }
    extern "system" fn cmd_set_viewport_w_scaling_nv(
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_w_scalings: *const vk::ViewportWScalingNV,
    ) {
        let global = Self::instance();
        // vkCmdSetViewportWScalingNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_clip_space_w_scaling;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_set_viewport_w_scaling_nv(
            command_buffer,
            first_viewport,
            unsafe { std::slice::from_raw_parts(p_viewport_w_scalings, viewport_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_viewport_w_scaling_nv)(
                    command_buffer,
                    first_viewport,
                    viewport_count,
                    p_viewport_w_scalings,
                )
            },
        }
    }
    extern "system" fn display_power_control_ext(
        device: vk::Device,
        display: vk::DisplayKHR,
        p_display_power_info: *const vk::DisplayPowerInfoEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkDisplayPowerControlEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_display_control;
        let layer_result = device_info
            .customized_info
            .display_power_control_ext(display, unsafe { p_display_power_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.display_power_control_ext)(device, display, p_display_power_info)
            },
        }
    }
    extern "system" fn register_device_event_ext(
        device: vk::Device,
        p_device_event_info: *const vk::DeviceEventInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
        p_fence: *mut vk::Fence,
    ) -> vk::Result {
        let global = Self::instance();
        // vkRegisterDeviceEventEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_display_control;
        let layer_result = device_info
            .customized_info
            .register_device_event_ext(unsafe { p_device_event_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_fence.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.register_device_event_ext)(
                    device,
                    p_device_event_info,
                    p_allocator,
                    p_fence,
                )
            },
        }
    }
    extern "system" fn register_display_event_ext(
        device: vk::Device,
        display: vk::DisplayKHR,
        p_display_event_info: *const vk::DisplayEventInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
        p_fence: *mut vk::Fence,
    ) -> vk::Result {
        let global = Self::instance();
        // vkRegisterDisplayEventEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_display_control;
        let layer_result = device_info.customized_info.register_display_event_ext(
            display,
            unsafe { p_display_event_info.as_ref() }.unwrap(),
            unsafe { p_allocator.as_ref() },
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_fence.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.register_display_event_ext)(
                    device,
                    display,
                    p_display_event_info,
                    p_allocator,
                    p_fence,
                )
            },
        }
    }
    extern "system" fn get_swapchain_counter_ext(
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        counter: vk::SurfaceCounterFlagsEXT,
        p_counter_value: *mut u64,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetSwapchainCounterEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_display_control;
        let layer_result = device_info
            .customized_info
            .get_swapchain_counter_ext(swapchain, counter);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_counter_value.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_swapchain_counter_ext)(
                    device,
                    swapchain,
                    counter,
                    p_counter_value,
                )
            },
        }
    }
    extern "system" fn get_refresh_cycle_duration_google(
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        p_display_timing_properties: *mut vk::RefreshCycleDurationGOOGLE,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetRefreshCycleDurationGOOGLE
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.google_display_timing;
        let layer_result = device_info
            .customized_info
            .get_refresh_cycle_duration_google(swapchain);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_display_timing_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_refresh_cycle_duration_google)(
                    device,
                    swapchain,
                    p_display_timing_properties,
                )
            },
        }
    }
    extern "system" fn get_past_presentation_timing_google(
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        p_presentation_timing_count: *mut u32,
        p_presentation_timings: *mut vk::PastPresentationTimingGOOGLE,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPastPresentationTimingGOOGLE
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.google_display_timing;
        let layer_result = device_info
            .customized_info
            .get_past_presentation_timing_google(swapchain);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_presentation_timing_count).unwrap(),
                        p_presentation_timings,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_past_presentation_timing_google)(
                    device,
                    swapchain,
                    p_presentation_timing_count,
                    p_presentation_timings,
                )
            },
        }
    }
    extern "system" fn cmd_set_discard_rectangle_ext(
        command_buffer: vk::CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
        p_discard_rectangles: *const vk::Rect2D,
    ) {
        let global = Self::instance();
        // vkCmdSetDiscardRectangleEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_discard_rectangles;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_set_discard_rectangle_ext(
            command_buffer,
            first_discard_rectangle,
            unsafe {
                std::slice::from_raw_parts(p_discard_rectangles, discard_rectangle_count as usize)
            },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_discard_rectangle_ext)(
                    command_buffer,
                    first_discard_rectangle,
                    discard_rectangle_count,
                    p_discard_rectangles,
                )
            },
        }
    }
    extern "system" fn set_hdr_metadata_ext(
        device: vk::Device,
        swapchain_count: u32,
        p_swapchains: *const vk::SwapchainKHR,
        p_metadata: *const vk::HdrMetadataEXT,
    ) {
        let global = Self::instance();
        // vkSetHdrMetadataEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_hdr_metadata;
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.set_hdr_metadata_ext(
            unsafe { std::slice::from_raw_parts(p_swapchains, swapchain_count as usize) },
            unsafe { std::slice::from_raw_parts(p_metadata, swapchain_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.set_hdr_metadata_ext)(
                    device,
                    swapchain_count,
                    p_swapchains,
                    p_metadata,
                )
            },
        }
    }
    extern "system" fn set_debug_utils_object_name_ext(
        device: vk::Device,
        p_name_info: *const vk::DebugUtilsObjectNameInfoEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkSetDebugUtilsObjectNameEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_utils;
        let layer_result = device_info
            .customized_info
            .set_debug_utils_object_name_ext(unsafe { p_name_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.set_debug_utils_object_name_ext)(device, p_name_info)
            },
        }
    }
    extern "system" fn set_debug_utils_object_tag_ext(
        device: vk::Device,
        p_tag_info: *const vk::DebugUtilsObjectTagInfoEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkSetDebugUtilsObjectTagEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_utils;
        let layer_result = device_info
            .customized_info
            .set_debug_utils_object_tag_ext(unsafe { p_tag_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.set_debug_utils_object_tag_ext)(device, p_tag_info)
            },
        }
    }
    extern "system" fn queue_begin_debug_utils_label_ext(
        queue: vk::Queue,
        p_label_info: *const vk::DebugUtilsLabelEXT,
    ) {
        let global = Self::instance();
        // vkQueueBeginDebugUtilsLabelEXT
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_utils;
        let layer_result = device_info
            .customized_info
            .queue_begin_debug_utils_label_ext(queue, unsafe { p_label_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.queue_begin_debug_utils_label_ext)(queue, p_label_info)
            },
        }
    }
    extern "system" fn queue_end_debug_utils_label_ext(queue: vk::Queue) {
        let global = Self::instance();
        // vkQueueEndDebugUtilsLabelEXT
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_utils;
        let layer_result = device_info
            .customized_info
            .queue_end_debug_utils_label_ext(queue);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.queue_end_debug_utils_label_ext)(queue)
            },
        }
    }
    extern "system" fn queue_insert_debug_utils_label_ext(
        queue: vk::Queue,
        p_label_info: *const vk::DebugUtilsLabelEXT,
    ) {
        let global = Self::instance();
        // vkQueueInsertDebugUtilsLabelEXT
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_utils;
        let layer_result = device_info
            .customized_info
            .queue_insert_debug_utils_label_ext(queue, unsafe { p_label_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.queue_insert_debug_utils_label_ext)(queue, p_label_info)
            },
        }
    }
    extern "system" fn cmd_begin_debug_utils_label_ext(
        command_buffer: vk::CommandBuffer,
        p_label_info: *const vk::DebugUtilsLabelEXT,
    ) {
        let global = Self::instance();
        // vkCmdBeginDebugUtilsLabelEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_utils;
        let layer_result = device_info.customized_info.cmd_begin_debug_utils_label_ext(
            command_buffer,
            unsafe { p_label_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_begin_debug_utils_label_ext)(command_buffer, p_label_info)
            },
        }
    }
    extern "system" fn cmd_end_debug_utils_label_ext(command_buffer: vk::CommandBuffer) {
        let global = Self::instance();
        // vkCmdEndDebugUtilsLabelEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_utils;
        let layer_result = device_info
            .customized_info
            .cmd_end_debug_utils_label_ext(command_buffer);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_end_debug_utils_label_ext)(command_buffer)
            },
        }
    }
    extern "system" fn cmd_insert_debug_utils_label_ext(
        command_buffer: vk::CommandBuffer,
        p_label_info: *const vk::DebugUtilsLabelEXT,
    ) {
        let global = Self::instance();
        // vkCmdInsertDebugUtilsLabelEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_debug_utils;
        let layer_result = device_info
            .customized_info
            .cmd_insert_debug_utils_label_ext(
                command_buffer,
                unsafe { p_label_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_insert_debug_utils_label_ext)(command_buffer, p_label_info)
            },
        }
    }
    extern "system" fn get_android_hardware_buffer_properties_android(
        device: vk::Device,
        buffer: *const vk::AHardwareBuffer,
        p_properties: *mut vk::AndroidHardwareBufferPropertiesANDROID,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetAndroidHardwareBufferPropertiesANDROID
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info
            .dispatch_table
            .android_external_memory_android_hardware_buffer;
        let layer_result = device_info
            .customized_info
            .get_android_hardware_buffer_properties_android(unsafe { buffer.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_android_hardware_buffer_properties_android)(
                    device,
                    buffer,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_memory_android_hardware_buffer_android(
        device: vk::Device,
        p_info: *const vk::MemoryGetAndroidHardwareBufferInfoANDROID,
        p_buffer: *mut *mut vk::AHardwareBuffer,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetMemoryAndroidHardwareBufferANDROID
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info
            .dispatch_table
            .android_external_memory_android_hardware_buffer;
        let layer_result = device_info
            .customized_info
            .get_memory_android_hardware_buffer_android(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_buffer.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_memory_android_hardware_buffer_android)(
                    device, p_info, p_buffer,
                )
            },
        }
    }
    extern "system" fn cmd_set_sample_locations_ext(
        command_buffer: vk::CommandBuffer,
        p_sample_locations_info: *const vk::SampleLocationsInfoEXT,
    ) {
        let global = Self::instance();
        // vkCmdSetSampleLocationsEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_sample_locations;
        let layer_result = device_info.customized_info.cmd_set_sample_locations_ext(
            command_buffer,
            unsafe { p_sample_locations_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_sample_locations_ext)(
                    command_buffer,
                    p_sample_locations_info,
                )
            },
        }
    }
    extern "system" fn get_image_drm_format_modifier_properties_ext(
        device: vk::Device,
        image: vk::Image,
        p_properties: *mut vk::ImageDrmFormatModifierPropertiesEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetImageDrmFormatModifierPropertiesEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_image_drm_format_modifier;
        let layer_result = device_info
            .customized_info
            .get_image_drm_format_modifier_properties_ext(image);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_drm_format_modifier_properties_ext)(
                    device,
                    image,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn create_validation_cache_ext(
        device: vk::Device,
        p_create_info: *const vk::ValidationCacheCreateInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
        p_validation_cache: *mut vk::ValidationCacheEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateValidationCacheEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_validation_cache;
        let layer_result = device_info
            .customized_info
            .create_validation_cache_ext(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_validation_cache.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_validation_cache_ext)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_validation_cache,
                )
            },
        }
    }
    extern "system" fn destroy_validation_cache_ext(
        device: vk::Device,
        validation_cache: vk::ValidationCacheEXT,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyValidationCacheEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_validation_cache;
        let layer_result = device_info
            .customized_info
            .destroy_validation_cache_ext(validation_cache, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_validation_cache_ext)(device, validation_cache, p_allocator)
            },
        }
    }
    extern "system" fn merge_validation_caches_ext(
        device: vk::Device,
        dst_cache: vk::ValidationCacheEXT,
        src_cache_count: u32,
        p_src_caches: *const vk::ValidationCacheEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkMergeValidationCachesEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_validation_cache;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .merge_validation_caches_ext(dst_cache, unsafe {
                std::slice::from_raw_parts(p_src_caches, src_cache_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.merge_validation_caches_ext)(
                    device,
                    dst_cache,
                    src_cache_count,
                    p_src_caches,
                )
            },
        }
    }
    extern "system" fn get_validation_cache_data_ext(
        device: vk::Device,
        validation_cache: vk::ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetValidationCacheDataEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_validation_cache;
        let layer_result = device_info
            .customized_info
            .get_validation_cache_data_ext(validation_cache);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    let p_data = p_data as *mut u8;
                    unsafe { fill_vk_out_array(&res, NonNull::new(p_data_size).unwrap(), p_data) }
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_validation_cache_data_ext)(
                    device,
                    validation_cache,
                    p_data_size,
                    p_data,
                )
            },
        }
    }
    extern "system" fn cmd_bind_shading_rate_image_nv(
        command_buffer: vk::CommandBuffer,
        image_view: vk::ImageView,
        image_layout: vk::ImageLayout,
    ) {
        let global = Self::instance();
        // vkCmdBindShadingRateImageNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_shading_rate_image;
        let layer_result = device_info.customized_info.cmd_bind_shading_rate_image_nv(
            command_buffer,
            image_view,
            image_layout,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_shading_rate_image_nv)(
                    command_buffer,
                    image_view,
                    image_layout,
                )
            },
        }
    }
    extern "system" fn cmd_set_viewport_shading_rate_palette_nv(
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_shading_rate_palettes: *const vk::ShadingRatePaletteNV,
    ) {
        let global = Self::instance();
        // vkCmdSetViewportShadingRatePaletteNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_shading_rate_image;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_set_viewport_shading_rate_palette_nv(command_buffer, first_viewport, unsafe {
                std::slice::from_raw_parts(p_shading_rate_palettes, viewport_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_viewport_shading_rate_palette_nv)(
                    command_buffer,
                    first_viewport,
                    viewport_count,
                    p_shading_rate_palettes,
                )
            },
        }
    }
    extern "system" fn cmd_set_coarse_sample_order_nv(
        command_buffer: vk::CommandBuffer,
        sample_order_type: vk::CoarseSampleOrderTypeNV,
        custom_sample_order_count: u32,
        p_custom_sample_orders: *const vk::CoarseSampleOrderCustomNV,
    ) {
        let global = Self::instance();
        // vkCmdSetCoarseSampleOrderNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_shading_rate_image;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_set_coarse_sample_order_nv(
            command_buffer,
            sample_order_type,
            unsafe {
                std::slice::from_raw_parts(
                    p_custom_sample_orders,
                    custom_sample_order_count as usize,
                )
            },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_coarse_sample_order_nv)(
                    command_buffer,
                    sample_order_type,
                    custom_sample_order_count,
                    p_custom_sample_orders,
                )
            },
        }
    }
    extern "system" fn create_acceleration_structure_nv(
        device: vk::Device,
        p_create_info: *const vk::AccelerationStructureCreateInfoNV,
        p_allocator: *const vk::AllocationCallbacks,
        p_acceleration_structure: *mut vk::AccelerationStructureNV,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateAccelerationStructureNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        let layer_result = device_info
            .customized_info
            .create_acceleration_structure_nv(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_acceleration_structure.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_acceleration_structure_nv)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_acceleration_structure,
                )
            },
        }
    }
    extern "system" fn destroy_acceleration_structure_nv(
        device: vk::Device,
        acceleration_structure: vk::AccelerationStructureNV,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyAccelerationStructureNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        let layer_result = device_info
            .customized_info
            .destroy_acceleration_structure_nv(acceleration_structure, unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_acceleration_structure_nv)(
                    device,
                    acceleration_structure,
                    p_allocator,
                )
            },
        }
    }
    extern "system" fn get_acceleration_structure_memory_requirements_nv(
        device: vk::Device,
        p_info: *const vk::AccelerationStructureMemoryRequirementsInfoNV,
        p_memory_requirements: *mut vk::MemoryRequirements2KHR,
    ) {
        let global = Self::instance();
        // vkGetAccelerationStructureMemoryRequirementsNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        let layer_result = device_info
            .customized_info
            .get_acceleration_structure_memory_requirements_nv(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_memory_requirements.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_acceleration_structure_memory_requirements_nv)(
                    device,
                    p_info,
                    p_memory_requirements,
                )
            },
        }
    }
    extern "system" fn bind_acceleration_structure_memory_nv(
        device: vk::Device,
        bind_info_count: u32,
        p_bind_infos: *const vk::BindAccelerationStructureMemoryInfoNV,
    ) -> vk::Result {
        let global = Self::instance();
        // vkBindAccelerationStructureMemoryNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .bind_acceleration_structure_memory_nv(unsafe {
                std::slice::from_raw_parts(p_bind_infos, bind_info_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.bind_acceleration_structure_memory_nv)(
                    device,
                    bind_info_count,
                    p_bind_infos,
                )
            },
        }
    }
    extern "system" fn cmd_build_acceleration_structure_nv(
        command_buffer: vk::CommandBuffer,
        p_info: *const vk::AccelerationStructureInfoNV,
        instance_data: vk::Buffer,
        instance_offset: vk::DeviceSize,
        update: vk::Bool32,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        scratch: vk::Buffer,
        scratch_offset: vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkCmdBuildAccelerationStructureNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        let layer_result = device_info
            .customized_info
            .cmd_build_acceleration_structure_nv(
                command_buffer,
                unsafe { p_info.as_ref() }.unwrap(),
                instance_data,
                instance_offset,
                update == vk::TRUE,
                dst,
                src,
                scratch,
                scratch_offset,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_build_acceleration_structure_nv)(
                    command_buffer,
                    p_info,
                    instance_data,
                    instance_offset,
                    update,
                    dst,
                    src,
                    scratch,
                    scratch_offset,
                )
            },
        }
    }
    extern "system" fn cmd_copy_acceleration_structure_nv(
        command_buffer: vk::CommandBuffer,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        mode: vk::CopyAccelerationStructureModeKHR,
    ) {
        let global = Self::instance();
        // vkCmdCopyAccelerationStructureNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        let layer_result = device_info
            .customized_info
            .cmd_copy_acceleration_structure_nv(command_buffer, dst, src, mode);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_acceleration_structure_nv)(command_buffer, dst, src, mode)
            },
        }
    }
    extern "system" fn cmd_trace_rays_nv(
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_table_buffer: vk::Buffer,
        raygen_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_table_buffer: vk::Buffer,
        miss_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_stride: vk::DeviceSize,
        hit_shader_binding_table_buffer: vk::Buffer,
        hit_shader_binding_offset: vk::DeviceSize,
        hit_shader_binding_stride: vk::DeviceSize,
        callable_shader_binding_table_buffer: vk::Buffer,
        callable_shader_binding_offset: vk::DeviceSize,
        callable_shader_binding_stride: vk::DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        let global = Self::instance();
        // vkCmdTraceRaysNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        let layer_result = device_info.customized_info.cmd_trace_rays_nv(
            command_buffer,
            raygen_shader_binding_table_buffer,
            raygen_shader_binding_offset,
            miss_shader_binding_table_buffer,
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table_buffer,
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table_buffer,
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width,
            height,
            depth,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_trace_rays_nv)(
                    command_buffer,
                    raygen_shader_binding_table_buffer,
                    raygen_shader_binding_offset,
                    miss_shader_binding_table_buffer,
                    miss_shader_binding_offset,
                    miss_shader_binding_stride,
                    hit_shader_binding_table_buffer,
                    hit_shader_binding_offset,
                    hit_shader_binding_stride,
                    callable_shader_binding_table_buffer,
                    callable_shader_binding_offset,
                    callable_shader_binding_stride,
                    width,
                    height,
                    depth,
                )
            },
        }
    }
    extern "system" fn create_ray_tracing_pipelines_nv(
        device: vk::Device,
        pipeline_cache: vk::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const vk::RayTracingPipelineCreateInfoNV,
        p_allocator: *const vk::AllocationCallbacks,
        p_pipelines: *mut vk::Pipeline,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateRayTracingPipelinesNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.create_ray_tracing_pipelines_nv(
            pipeline_cache,
            unsafe { std::slice::from_raw_parts(p_create_infos, create_info_count as usize) },
            unsafe { p_allocator.as_ref() },
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    unsafe {
                        std::slice::from_raw_parts_mut(
                            p_pipelines,
                            create_info_count.try_into().unwrap(),
                        )
                    }
                    .copy_from_slice(&res);
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_ray_tracing_pipelines_nv)(
                    device,
                    pipeline_cache,
                    create_info_count,
                    p_create_infos,
                    p_allocator,
                    p_pipelines,
                )
            },
        }
    }
    extern "system" fn get_ray_tracing_shader_group_handles_khr(
        device: vk::Device,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetRayTracingShaderGroupHandlesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_ray_tracing_pipeline;
        let layer_result = device_info
            .customized_info
            .get_ray_tracing_shader_group_handles_khr(pipeline, first_group, group_count);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    let p_data = p_data as *mut u8;
                    unsafe { std::slice::from_raw_parts_mut(p_data, data_size) }
                        .copy_from_slice(&res);
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_ray_tracing_shader_group_handles_khr)(
                    device,
                    pipeline,
                    first_group,
                    group_count,
                    data_size,
                    p_data,
                )
            },
        }
    }
    extern "system" fn get_acceleration_structure_handle_nv(
        device: vk::Device,
        acceleration_structure: vk::AccelerationStructureNV,
        data_size: usize,
        p_data: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetAccelerationStructureHandleNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        let layer_result = device_info
            .customized_info
            .get_acceleration_structure_handle_nv(acceleration_structure);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    let p_data = p_data as *mut u8;
                    unsafe { std::slice::from_raw_parts_mut(p_data, data_size) }
                        .copy_from_slice(&res);
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_acceleration_structure_handle_nv)(
                    device,
                    acceleration_structure,
                    data_size,
                    p_data,
                )
            },
        }
    }
    extern "system" fn cmd_write_acceleration_structures_properties_nv(
        command_buffer: vk::CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const vk::AccelerationStructureNV,
        query_type: vk::QueryType,
        query_pool: vk::QueryPool,
        first_query: u32,
    ) {
        let global = Self::instance();
        // vkCmdWriteAccelerationStructuresPropertiesNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_write_acceleration_structures_properties_nv(
                command_buffer,
                unsafe {
                    std::slice::from_raw_parts(
                        p_acceleration_structures,
                        acceleration_structure_count as usize,
                    )
                },
                query_type,
                query_pool,
                first_query,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_write_acceleration_structures_properties_nv)(
                    command_buffer,
                    acceleration_structure_count,
                    p_acceleration_structures,
                    query_type,
                    query_pool,
                    first_query,
                )
            },
        }
    }
    extern "system" fn compile_deferred_nv(
        device: vk::Device,
        pipeline: vk::Pipeline,
        shader: u32,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCompileDeferredNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_ray_tracing;
        let layer_result = device_info
            .customized_info
            .compile_deferred_nv(pipeline, shader);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.compile_deferred_nv)(device, pipeline, shader)
            },
        }
    }
    extern "system" fn get_memory_host_pointer_properties_ext(
        device: vk::Device,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: *mut vk::MemoryHostPointerPropertiesEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetMemoryHostPointerPropertiesEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_external_memory_host;
        let layer_result = device_info
            .customized_info
            .get_memory_host_pointer_properties_ext(
                handle_type,
                unsafe { p_host_pointer.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_memory_host_pointer_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_memory_host_pointer_properties_ext)(
                    device,
                    handle_type,
                    p_host_pointer,
                    p_memory_host_pointer_properties,
                )
            },
        }
    }
    extern "system" fn cmd_write_buffer_marker_amd(
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        marker: u32,
    ) {
        let global = Self::instance();
        // vkCmdWriteBufferMarkerAMD
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.amd_buffer_marker;
        let layer_result = device_info.customized_info.cmd_write_buffer_marker_amd(
            command_buffer,
            pipeline_stage,
            dst_buffer,
            dst_offset,
            marker,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_write_buffer_marker_amd)(
                    command_buffer,
                    pipeline_stage,
                    dst_buffer,
                    dst_offset,
                    marker,
                )
            },
        }
    }
    extern "system" fn get_calibrated_timestamps_ext(
        device: vk::Device,
        timestamp_count: u32,
        p_timestamp_infos: *const vk::CalibratedTimestampInfoEXT,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetCalibratedTimestampsEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_calibrated_timestamps;
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.get_calibrated_timestamps_ext(
            unsafe { std::slice::from_raw_parts(p_timestamp_infos, timestamp_count as usize) },
            unsafe { std::slice::from_raw_parts_mut(p_timestamps, timestamp_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_max_deviation.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_calibrated_timestamps_ext)(
                    device,
                    timestamp_count,
                    p_timestamp_infos,
                    p_timestamps,
                    p_max_deviation,
                )
            },
        }
    }
    extern "system" fn cmd_draw_mesh_tasks_nv(
        command_buffer: vk::CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawMeshTasksNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_mesh_shader;
        let layer_result = device_info.customized_info.cmd_draw_mesh_tasks_nv(
            command_buffer,
            task_count,
            first_task,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_mesh_tasks_nv)(command_buffer, task_count, first_task)
            },
        }
    }
    extern "system" fn cmd_draw_mesh_tasks_indirect_nv(
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawMeshTasksIndirectNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_mesh_shader;
        let layer_result = device_info.customized_info.cmd_draw_mesh_tasks_indirect_nv(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_mesh_tasks_indirect_nv)(
                    command_buffer,
                    buffer,
                    offset,
                    draw_count,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_draw_mesh_tasks_indirect_count_nv(
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawMeshTasksIndirectCountNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_mesh_shader;
        let layer_result = device_info
            .customized_info
            .cmd_draw_mesh_tasks_indirect_count_nv(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_mesh_tasks_indirect_count_nv)(
                    command_buffer,
                    buffer,
                    offset,
                    count_buffer,
                    count_buffer_offset,
                    max_draw_count,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_set_exclusive_scissor_nv(
        command_buffer: vk::CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissors: *const vk::Rect2D,
    ) {
        let global = Self::instance();
        // vkCmdSetExclusiveScissorNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_scissor_exclusive;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_set_exclusive_scissor_nv(
            command_buffer,
            first_exclusive_scissor,
            unsafe {
                std::slice::from_raw_parts(p_exclusive_scissors, exclusive_scissor_count as usize)
            },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_exclusive_scissor_nv)(
                    command_buffer,
                    first_exclusive_scissor,
                    exclusive_scissor_count,
                    p_exclusive_scissors,
                )
            },
        }
    }
    extern "system" fn cmd_set_checkpoint_nv(
        command_buffer: vk::CommandBuffer,
        p_checkpoint_marker: *const c_void,
    ) {
        let global = Self::instance();
        // vkCmdSetCheckpointNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_device_diagnostic_checkpoints;
        let layer_result = device_info.customized_info.cmd_set_checkpoint_nv(
            command_buffer,
            unsafe { p_checkpoint_marker.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_checkpoint_nv)(command_buffer, p_checkpoint_marker)
            },
        }
    }
    extern "system" fn get_queue_checkpoint_data_nv(
        queue: vk::Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut vk::CheckpointDataNV,
    ) {
        let global = Self::instance();
        // vkGetQueueCheckpointDataNV
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_device_diagnostic_checkpoints;
        let layer_result = device_info
            .customized_info
            .get_queue_checkpoint_data_nv(queue);
        match layer_result {
            LayerResult::Handled(res) => {
                // We can't return INCOMPLETE from vkGetQueueCheckpointDataNV
                #[allow(unused_must_use)]
                unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_checkpoint_data_count).unwrap(),
                        p_checkpoint_data,
                    );
                }
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_queue_checkpoint_data_nv)(
                    queue,
                    p_checkpoint_data_count,
                    p_checkpoint_data,
                )
            },
        }
    }
    extern "system" fn initialize_performance_api_intel(
        device: vk::Device,
        p_initialize_info: *const vk::InitializePerformanceApiInfoINTEL,
    ) -> vk::Result {
        let global = Self::instance();
        // vkInitializePerformanceApiINTEL
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.intel_performance_query;
        let layer_result = device_info
            .customized_info
            .initialize_performance_api_intel(unsafe { p_initialize_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.initialize_performance_api_intel)(device, p_initialize_info)
            },
        }
    }
    extern "system" fn uninitialize_performance_api_intel(device: vk::Device) {
        let global = Self::instance();
        // vkUninitializePerformanceApiINTEL
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.intel_performance_query;
        let layer_result = device_info
            .customized_info
            .uninitialize_performance_api_intel();
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.uninitialize_performance_api_intel)(device)
            },
        }
    }
    extern "system" fn cmd_set_performance_marker_intel(
        command_buffer: vk::CommandBuffer,
        p_marker_info: *const vk::PerformanceMarkerInfoINTEL,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCmdSetPerformanceMarkerINTEL
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.intel_performance_query;
        let layer_result = device_info
            .customized_info
            .cmd_set_performance_marker_intel(
                command_buffer,
                unsafe { p_marker_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_performance_marker_intel)(command_buffer, p_marker_info)
            },
        }
    }
    extern "system" fn cmd_set_performance_stream_marker_intel(
        command_buffer: vk::CommandBuffer,
        p_marker_info: *const vk::PerformanceStreamMarkerInfoINTEL,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCmdSetPerformanceStreamMarkerINTEL
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.intel_performance_query;
        let layer_result = device_info
            .customized_info
            .cmd_set_performance_stream_marker_intel(
                command_buffer,
                unsafe { p_marker_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_performance_stream_marker_intel)(
                    command_buffer,
                    p_marker_info,
                )
            },
        }
    }
    extern "system" fn cmd_set_performance_override_intel(
        command_buffer: vk::CommandBuffer,
        p_override_info: *const vk::PerformanceOverrideInfoINTEL,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCmdSetPerformanceOverrideINTEL
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.intel_performance_query;
        let layer_result = device_info
            .customized_info
            .cmd_set_performance_override_intel(
                command_buffer,
                unsafe { p_override_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_performance_override_intel)(command_buffer, p_override_info)
            },
        }
    }
    extern "system" fn acquire_performance_configuration_intel(
        device: vk::Device,
        p_acquire_info: *const vk::PerformanceConfigurationAcquireInfoINTEL,
        p_configuration: *mut vk::PerformanceConfigurationINTEL,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAcquirePerformanceConfigurationINTEL
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.intel_performance_query;
        let layer_result = device_info
            .customized_info
            .acquire_performance_configuration_intel(unsafe { p_acquire_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_configuration.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.acquire_performance_configuration_intel)(
                    device,
                    p_acquire_info,
                    p_configuration,
                )
            },
        }
    }
    extern "system" fn release_performance_configuration_intel(
        device: vk::Device,
        configuration: vk::PerformanceConfigurationINTEL,
    ) -> vk::Result {
        let global = Self::instance();
        // vkReleasePerformanceConfigurationINTEL
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.intel_performance_query;
        let layer_result = device_info
            .customized_info
            .release_performance_configuration_intel(configuration);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.release_performance_configuration_intel)(device, configuration)
            },
        }
    }
    extern "system" fn queue_set_performance_configuration_intel(
        queue: vk::Queue,
        configuration: vk::PerformanceConfigurationINTEL,
    ) -> vk::Result {
        let global = Self::instance();
        // vkQueueSetPerformanceConfigurationINTEL
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.intel_performance_query;
        let layer_result = device_info
            .customized_info
            .queue_set_performance_configuration_intel(queue, configuration);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.queue_set_performance_configuration_intel)(queue, configuration)
            },
        }
    }
    extern "system" fn get_performance_parameter_intel(
        device: vk::Device,
        parameter: vk::PerformanceParameterTypeINTEL,
        p_value: *mut vk::PerformanceValueINTEL,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPerformanceParameterINTEL
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.intel_performance_query;
        let layer_result = device_info
            .customized_info
            .get_performance_parameter_intel(parameter);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_value.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_performance_parameter_intel)(device, parameter, p_value)
            },
        }
    }
    extern "system" fn set_local_dimming_amd(
        device: vk::Device,
        swap_chain: vk::SwapchainKHR,
        local_dimming_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkSetLocalDimmingAMD
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.amd_display_native_hdr;
        let layer_result = device_info
            .customized_info
            .set_local_dimming_amd(swap_chain, local_dimming_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.set_local_dimming_amd)(device, swap_chain, local_dimming_enable)
            },
        }
    }
    extern "system" fn acquire_full_screen_exclusive_mode_ext(
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAcquireFullScreenExclusiveModeEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_full_screen_exclusive;
        let layer_result = device_info
            .customized_info
            .acquire_full_screen_exclusive_mode_ext(swapchain);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.acquire_full_screen_exclusive_mode_ext)(device, swapchain)
            },
        }
    }
    extern "system" fn release_full_screen_exclusive_mode_ext(
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkReleaseFullScreenExclusiveModeEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_full_screen_exclusive;
        let layer_result = device_info
            .customized_info
            .release_full_screen_exclusive_mode_ext(swapchain);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.release_full_screen_exclusive_mode_ext)(device, swapchain)
            },
        }
    }
    extern "system" fn get_device_group_surface_present_modes2_ext(
        device: vk::Device,
        p_surface_info: *const vk::PhysicalDeviceSurfaceInfo2KHR,
        p_modes: *mut vk::DeviceGroupPresentModeFlagsKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDeviceGroupSurfacePresentModes2EXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_full_screen_exclusive;
        let layer_result = device_info
            .customized_info
            .get_device_group_surface_present_modes2_ext(
                unsafe { p_surface_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_modes.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_group_surface_present_modes2_ext)(
                    device,
                    p_surface_info,
                    p_modes,
                )
            },
        }
    }
    extern "system" fn cmd_set_line_stipple_ext(
        command_buffer: vk::CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        let global = Self::instance();
        // vkCmdSetLineStippleEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_line_rasterization;
        let layer_result = device_info.customized_info.cmd_set_line_stipple_ext(
            command_buffer,
            line_stipple_factor,
            line_stipple_pattern,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_line_stipple_ext)(
                    command_buffer,
                    line_stipple_factor,
                    line_stipple_pattern,
                )
            },
        }
    }
    extern "system" fn release_swapchain_images_ext(
        device: vk::Device,
        p_release_info: *const vk::ReleaseSwapchainImagesInfoEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkReleaseSwapchainImagesEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_swapchain_maintenance1;
        let layer_result = device_info
            .customized_info
            .release_swapchain_images_ext(unsafe { p_release_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.release_swapchain_images_ext)(device, p_release_info)
            },
        }
    }
    extern "system" fn get_generated_commands_memory_requirements_nv(
        device: vk::Device,
        p_info: *const vk::GeneratedCommandsMemoryRequirementsInfoNV,
        p_memory_requirements: *mut vk::MemoryRequirements2,
    ) {
        let global = Self::instance();
        // vkGetGeneratedCommandsMemoryRequirementsNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_device_generated_commands;
        let layer_result = device_info
            .customized_info
            .get_generated_commands_memory_requirements_nv(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_memory_requirements.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_generated_commands_memory_requirements_nv)(
                    device,
                    p_info,
                    p_memory_requirements,
                )
            },
        }
    }
    extern "system" fn cmd_preprocess_generated_commands_nv(
        command_buffer: vk::CommandBuffer,
        p_generated_commands_info: *const vk::GeneratedCommandsInfoNV,
    ) {
        let global = Self::instance();
        // vkCmdPreprocessGeneratedCommandsNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_device_generated_commands;
        let layer_result = device_info
            .customized_info
            .cmd_preprocess_generated_commands_nv(
                command_buffer,
                unsafe { p_generated_commands_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_preprocess_generated_commands_nv)(
                    command_buffer,
                    p_generated_commands_info,
                )
            },
        }
    }
    extern "system" fn cmd_execute_generated_commands_nv(
        command_buffer: vk::CommandBuffer,
        is_preprocessed: vk::Bool32,
        p_generated_commands_info: *const vk::GeneratedCommandsInfoNV,
    ) {
        let global = Self::instance();
        // vkCmdExecuteGeneratedCommandsNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_device_generated_commands;
        let layer_result = device_info
            .customized_info
            .cmd_execute_generated_commands_nv(
                command_buffer,
                is_preprocessed == vk::TRUE,
                unsafe { p_generated_commands_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_execute_generated_commands_nv)(
                    command_buffer,
                    is_preprocessed,
                    p_generated_commands_info,
                )
            },
        }
    }
    extern "system" fn cmd_bind_pipeline_shader_group_nv(
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
        group_index: u32,
    ) {
        let global = Self::instance();
        // vkCmdBindPipelineShaderGroupNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_device_generated_commands;
        let layer_result = device_info
            .customized_info
            .cmd_bind_pipeline_shader_group_nv(
                command_buffer,
                pipeline_bind_point,
                pipeline,
                group_index,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_pipeline_shader_group_nv)(
                    command_buffer,
                    pipeline_bind_point,
                    pipeline,
                    group_index,
                )
            },
        }
    }
    extern "system" fn create_indirect_commands_layout_nv(
        device: vk::Device,
        p_create_info: *const vk::IndirectCommandsLayoutCreateInfoNV,
        p_allocator: *const vk::AllocationCallbacks,
        p_indirect_commands_layout: *mut vk::IndirectCommandsLayoutNV,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateIndirectCommandsLayoutNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_device_generated_commands;
        let layer_result = device_info
            .customized_info
            .create_indirect_commands_layout_nv(
                unsafe { p_create_info.as_ref() }.unwrap(),
                unsafe { p_allocator.as_ref() },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_indirect_commands_layout.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_indirect_commands_layout_nv)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_indirect_commands_layout,
                )
            },
        }
    }
    extern "system" fn destroy_indirect_commands_layout_nv(
        device: vk::Device,
        indirect_commands_layout: vk::IndirectCommandsLayoutNV,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyIndirectCommandsLayoutNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_device_generated_commands;
        let layer_result = device_info
            .customized_info
            .destroy_indirect_commands_layout_nv(indirect_commands_layout, unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_indirect_commands_layout_nv)(
                    device,
                    indirect_commands_layout,
                    p_allocator,
                )
            },
        }
    }
    extern "system" fn export_metal_objects_ext(
        device: vk::Device,
        p_metal_objects_info: *mut vk::ExportMetalObjectsInfoEXT,
    ) {
        let global = Self::instance();
        // vkExportMetalObjectsEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_metal_objects;
        let layer_result = device_info.customized_info.export_metal_objects_ext();
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_metal_objects_info.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.export_metal_objects_ext)(device, p_metal_objects_info)
            },
        }
    }
    extern "system" fn get_descriptor_set_layout_size_ext(
        device: vk::Device,
        layout: vk::DescriptorSetLayout,
        p_layout_size_in_bytes: *mut vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkGetDescriptorSetLayoutSizeEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        let layer_result = device_info
            .customized_info
            .get_descriptor_set_layout_size_ext(layout);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_layout_size_in_bytes.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_descriptor_set_layout_size_ext)(
                    device,
                    layout,
                    p_layout_size_in_bytes,
                )
            },
        }
    }
    extern "system" fn get_descriptor_set_layout_binding_offset_ext(
        device: vk::Device,
        layout: vk::DescriptorSetLayout,
        binding: u32,
        p_offset: *mut vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkGetDescriptorSetLayoutBindingOffsetEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        let layer_result = device_info
            .customized_info
            .get_descriptor_set_layout_binding_offset_ext(layout, binding);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_offset.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_descriptor_set_layout_binding_offset_ext)(
                    device, layout, binding, p_offset,
                )
            },
        }
    }
    extern "system" fn get_descriptor_ext(
        device: vk::Device,
        p_descriptor_info: *const vk::DescriptorGetInfoEXT,
        data_size: usize,
        p_descriptor: *mut c_void,
    ) {
        let global = Self::instance();
        // vkGetDescriptorEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        let layer_result = device_info
            .customized_info
            .get_descriptor_ext(unsafe { p_descriptor_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                let p_descriptor = p_descriptor as *mut u8;
                unsafe { std::slice::from_raw_parts_mut(p_descriptor, data_size) }
                    .copy_from_slice(&res);
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_descriptor_ext)(
                    device,
                    p_descriptor_info,
                    data_size,
                    p_descriptor,
                )
            },
        }
    }
    extern "system" fn cmd_bind_descriptor_buffers_ext(
        command_buffer: vk::CommandBuffer,
        buffer_count: u32,
        p_binding_infos: *const vk::DescriptorBufferBindingInfoEXT,
    ) {
        let global = Self::instance();
        // vkCmdBindDescriptorBuffersEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_bind_descriptor_buffers_ext(command_buffer, unsafe {
                std::slice::from_raw_parts(p_binding_infos, buffer_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_descriptor_buffers_ext)(
                    command_buffer,
                    buffer_count,
                    p_binding_infos,
                )
            },
        }
    }
    extern "system" fn cmd_set_descriptor_buffer_offsets_ext(
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        set_count: u32,
        p_buffer_indices: *const u32,
        p_offsets: *const vk::DeviceSize,
    ) {
        let global = Self::instance();
        // vkCmdSetDescriptorBufferOffsetsEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_set_descriptor_buffer_offsets_ext(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                unsafe { std::slice::from_raw_parts(p_buffer_indices, set_count as usize) },
                unsafe { std::slice::from_raw_parts(p_offsets, set_count as usize) },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_descriptor_buffer_offsets_ext)(
                    command_buffer,
                    pipeline_bind_point,
                    layout,
                    first_set,
                    set_count,
                    p_buffer_indices,
                    p_offsets,
                )
            },
        }
    }
    extern "system" fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: u32,
    ) {
        let global = Self::instance();
        // vkCmdBindDescriptorBufferEmbeddedSamplersEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        let layer_result = device_info
            .customized_info
            .cmd_bind_descriptor_buffer_embedded_samplers_ext(
                command_buffer,
                pipeline_bind_point,
                layout,
                set,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_descriptor_buffer_embedded_samplers_ext)(
                    command_buffer,
                    pipeline_bind_point,
                    layout,
                    set,
                )
            },
        }
    }
    extern "system" fn get_buffer_opaque_capture_descriptor_data_ext(
        device: vk::Device,
        p_info: *const vk::BufferCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetBufferOpaqueCaptureDescriptorDataEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        let layer_result = device_info
            .customized_info
            .get_buffer_opaque_capture_descriptor_data_ext(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_data.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_buffer_opaque_capture_descriptor_data_ext)(
                    device, p_info, p_data,
                )
            },
        }
    }
    extern "system" fn get_image_opaque_capture_descriptor_data_ext(
        device: vk::Device,
        p_info: *const vk::ImageCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetImageOpaqueCaptureDescriptorDataEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        let layer_result = device_info
            .customized_info
            .get_image_opaque_capture_descriptor_data_ext(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_data.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_opaque_capture_descriptor_data_ext)(
                    device, p_info, p_data,
                )
            },
        }
    }
    extern "system" fn get_image_view_opaque_capture_descriptor_data_ext(
        device: vk::Device,
        p_info: *const vk::ImageViewCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetImageViewOpaqueCaptureDescriptorDataEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        let layer_result = device_info
            .customized_info
            .get_image_view_opaque_capture_descriptor_data_ext(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_data.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_view_opaque_capture_descriptor_data_ext)(
                    device, p_info, p_data,
                )
            },
        }
    }
    extern "system" fn get_sampler_opaque_capture_descriptor_data_ext(
        device: vk::Device,
        p_info: *const vk::SamplerCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetSamplerOpaqueCaptureDescriptorDataEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        let layer_result = device_info
            .customized_info
            .get_sampler_opaque_capture_descriptor_data_ext(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_data.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_sampler_opaque_capture_descriptor_data_ext)(
                    device, p_info, p_data,
                )
            },
        }
    }
    extern "system" fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
        device: vk::Device,
        p_info: *const vk::AccelerationStructureCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_descriptor_buffer;
        let layer_result = device_info
            .customized_info
            .get_acceleration_structure_opaque_capture_descriptor_data_ext(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_data.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_acceleration_structure_opaque_capture_descriptor_data_ext)(
                    device, p_info, p_data,
                )
            },
        }
    }
    extern "system" fn cmd_set_fragment_shading_rate_enum_nv(
        command_buffer: vk::CommandBuffer,
        shading_rate: vk::FragmentShadingRateNV,
        combiner_ops: *const [vk::FragmentShadingRateCombinerOpKHR; 2],
    ) {
        let global = Self::instance();
        // vkCmdSetFragmentShadingRateEnumNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_fragment_shading_rate_enums;
        let combiner_ops = unsafe { combiner_ops.as_ref() }.unwrap();
        let layer_result = device_info
            .customized_info
            .cmd_set_fragment_shading_rate_enum_nv(command_buffer, shading_rate, combiner_ops);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_fragment_shading_rate_enum_nv)(
                    command_buffer,
                    shading_rate,
                    combiner_ops,
                )
            },
        }
    }
    extern "system" fn get_image_subresource_layout2_ext(
        device: vk::Device,
        image: vk::Image,
        p_subresource: *const vk::ImageSubresource2EXT,
        p_layout: *mut vk::SubresourceLayout2EXT,
    ) {
        let global = Self::instance();
        // vkGetImageSubresourceLayout2EXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_image_compression_control;
        let layer_result = device_info
            .customized_info
            .get_image_subresource_layout2_ext(image, unsafe { p_subresource.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_layout.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_image_subresource_layout2_ext)(
                    device,
                    image,
                    p_subresource,
                    p_layout,
                )
            },
        }
    }
    extern "system" fn cmd_set_vertex_input_ext(
        command_buffer: vk::CommandBuffer,
        vertex_binding_description_count: u32,
        p_vertex_binding_descriptions: *const vk::VertexInputBindingDescription2EXT,
        vertex_attribute_description_count: u32,
        p_vertex_attribute_descriptions: *const vk::VertexInputAttributeDescription2EXT,
    ) {
        let global = Self::instance();
        // vkCmdSetVertexInputEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_vertex_input_dynamic_state;
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_set_vertex_input_ext(
            command_buffer,
            unsafe {
                std::slice::from_raw_parts(
                    p_vertex_binding_descriptions,
                    vertex_binding_description_count as usize,
                )
            },
            unsafe {
                std::slice::from_raw_parts(
                    p_vertex_attribute_descriptions,
                    vertex_attribute_description_count as usize,
                )
            },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_vertex_input_ext)(
                    command_buffer,
                    vertex_binding_description_count,
                    p_vertex_binding_descriptions,
                    vertex_attribute_description_count,
                    p_vertex_attribute_descriptions,
                )
            },
        }
    }
    extern "system" fn get_memory_zircon_handle_fuchsia(
        device: vk::Device,
        p_get_zircon_handle_info: *const vk::MemoryGetZirconHandleInfoFUCHSIA,
        p_zircon_handle: *mut vk::zx_handle_t,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetMemoryZirconHandleFUCHSIA
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.fuchsia_external_memory;
        let layer_result = device_info
            .customized_info
            .get_memory_zircon_handle_fuchsia(
                unsafe { p_get_zircon_handle_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_zircon_handle.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_memory_zircon_handle_fuchsia)(
                    device,
                    p_get_zircon_handle_info,
                    p_zircon_handle,
                )
            },
        }
    }
    extern "system" fn get_memory_zircon_handle_properties_fuchsia(
        device: vk::Device,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        zircon_handle: vk::zx_handle_t,
        p_memory_zircon_handle_properties: *mut vk::MemoryZirconHandlePropertiesFUCHSIA,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetMemoryZirconHandlePropertiesFUCHSIA
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.fuchsia_external_memory;
        let layer_result = device_info
            .customized_info
            .get_memory_zircon_handle_properties_fuchsia(handle_type, zircon_handle);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_memory_zircon_handle_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_memory_zircon_handle_properties_fuchsia)(
                    device,
                    handle_type,
                    zircon_handle,
                    p_memory_zircon_handle_properties,
                )
            },
        }
    }
    extern "system" fn import_semaphore_zircon_handle_fuchsia(
        device: vk::Device,
        p_import_semaphore_zircon_handle_info: *const vk::ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> vk::Result {
        let global = Self::instance();
        // vkImportSemaphoreZirconHandleFUCHSIA
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.fuchsia_external_semaphore;
        let layer_result = device_info
            .customized_info
            .import_semaphore_zircon_handle_fuchsia(
                unsafe { p_import_semaphore_zircon_handle_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.import_semaphore_zircon_handle_fuchsia)(
                    device,
                    p_import_semaphore_zircon_handle_info,
                )
            },
        }
    }
    extern "system" fn get_semaphore_zircon_handle_fuchsia(
        device: vk::Device,
        p_get_zircon_handle_info: *const vk::SemaphoreGetZirconHandleInfoFUCHSIA,
        p_zircon_handle: *mut vk::zx_handle_t,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetSemaphoreZirconHandleFUCHSIA
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.fuchsia_external_semaphore;
        let layer_result = device_info
            .customized_info
            .get_semaphore_zircon_handle_fuchsia(
                unsafe { p_get_zircon_handle_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_zircon_handle.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_semaphore_zircon_handle_fuchsia)(
                    device,
                    p_get_zircon_handle_info,
                    p_zircon_handle,
                )
            },
        }
    }
    extern "system" fn create_buffer_collection_fuchsia(
        device: vk::Device,
        p_create_info: *const vk::BufferCollectionCreateInfoFUCHSIA,
        p_allocator: *const vk::AllocationCallbacks,
        p_collection: *mut vk::BufferCollectionFUCHSIA,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateBufferCollectionFUCHSIA
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.fuchsia_buffer_collection;
        let layer_result = device_info
            .customized_info
            .create_buffer_collection_fuchsia(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_collection.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_buffer_collection_fuchsia)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_collection,
                )
            },
        }
    }
    extern "system" fn set_buffer_collection_image_constraints_fuchsia(
        device: vk::Device,
        collection: vk::BufferCollectionFUCHSIA,
        p_image_constraints_info: *const vk::ImageConstraintsInfoFUCHSIA,
    ) -> vk::Result {
        let global = Self::instance();
        // vkSetBufferCollectionImageConstraintsFUCHSIA
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.fuchsia_buffer_collection;
        let layer_result = device_info
            .customized_info
            .set_buffer_collection_image_constraints_fuchsia(
                collection,
                unsafe { p_image_constraints_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.set_buffer_collection_image_constraints_fuchsia)(
                    device,
                    collection,
                    p_image_constraints_info,
                )
            },
        }
    }
    extern "system" fn set_buffer_collection_buffer_constraints_fuchsia(
        device: vk::Device,
        collection: vk::BufferCollectionFUCHSIA,
        p_buffer_constraints_info: *const vk::BufferConstraintsInfoFUCHSIA,
    ) -> vk::Result {
        let global = Self::instance();
        // vkSetBufferCollectionBufferConstraintsFUCHSIA
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.fuchsia_buffer_collection;
        let layer_result = device_info
            .customized_info
            .set_buffer_collection_buffer_constraints_fuchsia(
                collection,
                unsafe { p_buffer_constraints_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.set_buffer_collection_buffer_constraints_fuchsia)(
                    device,
                    collection,
                    p_buffer_constraints_info,
                )
            },
        }
    }
    extern "system" fn destroy_buffer_collection_fuchsia(
        device: vk::Device,
        collection: vk::BufferCollectionFUCHSIA,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyBufferCollectionFUCHSIA
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.fuchsia_buffer_collection;
        let layer_result = device_info
            .customized_info
            .destroy_buffer_collection_fuchsia(collection, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_buffer_collection_fuchsia)(device, collection, p_allocator)
            },
        }
    }
    extern "system" fn get_buffer_collection_properties_fuchsia(
        device: vk::Device,
        collection: vk::BufferCollectionFUCHSIA,
        p_properties: *mut vk::BufferCollectionPropertiesFUCHSIA,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetBufferCollectionPropertiesFUCHSIA
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.fuchsia_buffer_collection;
        let layer_result = device_info
            .customized_info
            .get_buffer_collection_properties_fuchsia(collection);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_buffer_collection_properties_fuchsia)(
                    device,
                    collection,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_device_subpass_shading_max_workgroup_size_huawei(
        device: vk::Device,
        renderpass: vk::RenderPass,
        p_max_workgroup_size: *mut vk::Extent2D,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.huawei_subpass_shading;
        let layer_result = device_info
            .customized_info
            .get_device_subpass_shading_max_workgroup_size_huawei(renderpass);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_max_workgroup_size.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_subpass_shading_max_workgroup_size_huawei)(
                    device,
                    renderpass,
                    p_max_workgroup_size,
                )
            },
        }
    }
    extern "system" fn cmd_subpass_shading_huawei(command_buffer: vk::CommandBuffer) {
        let global = Self::instance();
        // vkCmdSubpassShadingHUAWEI
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.huawei_subpass_shading;
        let layer_result = device_info
            .customized_info
            .cmd_subpass_shading_huawei(command_buffer);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_subpass_shading_huawei)(command_buffer)
            },
        }
    }
    extern "system" fn cmd_bind_invocation_mask_huawei(
        command_buffer: vk::CommandBuffer,
        image_view: vk::ImageView,
        image_layout: vk::ImageLayout,
    ) {
        let global = Self::instance();
        // vkCmdBindInvocationMaskHUAWEI
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.huawei_invocation_mask;
        let layer_result = device_info.customized_info.cmd_bind_invocation_mask_huawei(
            command_buffer,
            image_view,
            image_layout,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_bind_invocation_mask_huawei)(
                    command_buffer,
                    image_view,
                    image_layout,
                )
            },
        }
    }
    extern "system" fn get_memory_remote_address_nv(
        device: vk::Device,
        p_memory_get_remote_address_info: *const vk::MemoryGetRemoteAddressInfoNV,
        p_address: *mut vk::RemoteAddressNV,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetMemoryRemoteAddressNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_external_memory_rdma;
        let layer_result = device_info.customized_info.get_memory_remote_address_nv(
            unsafe { p_memory_get_remote_address_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_address.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_memory_remote_address_nv)(
                    device,
                    p_memory_get_remote_address_info,
                    p_address,
                )
            },
        }
    }
    extern "system" fn get_pipeline_properties_ext(
        device: vk::Device,
        p_pipeline_info: *const vk::PipelineInfoEXT,
        p_pipeline_properties: *mut vk::BaseOutStructure,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetPipelinePropertiesEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_pipeline_properties;
        let layer_result = device_info
            .customized_info
            .get_pipeline_properties_ext(unsafe { p_pipeline_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_pipeline_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_pipeline_properties_ext)(
                    device,
                    p_pipeline_info,
                    p_pipeline_properties,
                )
            },
        }
    }
    extern "system" fn cmd_set_patch_control_points_ext(
        command_buffer: vk::CommandBuffer,
        patch_control_points: u32,
    ) {
        let global = Self::instance();
        // vkCmdSetPatchControlPointsEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state2;
        let layer_result = device_info
            .customized_info
            .cmd_set_patch_control_points_ext(command_buffer, patch_control_points);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_patch_control_points_ext)(
                    command_buffer,
                    patch_control_points,
                )
            },
        }
    }
    extern "system" fn cmd_set_logic_op_ext(
        command_buffer: vk::CommandBuffer,
        logic_op: vk::LogicOp,
    ) {
        let global = Self::instance();
        // vkCmdSetLogicOpEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state2;
        let layer_result = device_info
            .customized_info
            .cmd_set_logic_op_ext(command_buffer, logic_op);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_logic_op_ext)(command_buffer, logic_op)
            },
        }
    }
    extern "system" fn cmd_set_color_write_enable_ext(
        command_buffer: vk::CommandBuffer,
        attachment_count: u32,
        p_color_write_enables: *const vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetColorWriteEnableEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_color_write_enable;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_set_color_write_enable_ext(
            command_buffer,
            unsafe { std::slice::from_raw_parts(p_color_write_enables, attachment_count as usize) }
                .iter()
                .map(|v| *v == vk::TRUE),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_color_write_enable_ext)(
                    command_buffer,
                    attachment_count,
                    p_color_write_enables,
                )
            },
        }
    }
    extern "system" fn cmd_draw_multi_ext(
        command_buffer: vk::CommandBuffer,
        draw_count: u32,
        p_vertex_info: *const vk::MultiDrawInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawMultiEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_multi_draw;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_draw_multi_ext(
            command_buffer,
            unsafe { std::slice::from_raw_parts(p_vertex_info, draw_count as usize) },
            instance_count,
            first_instance,
            stride,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_multi_ext)(
                    command_buffer,
                    draw_count,
                    p_vertex_info,
                    instance_count,
                    first_instance,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_draw_multi_indexed_ext(
        command_buffer: vk::CommandBuffer,
        draw_count: u32,
        p_index_info: *const vk::MultiDrawIndexedInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        p_vertex_offset: *const i32,
    ) {
        let global = Self::instance();
        // vkCmdDrawMultiIndexedEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_multi_draw;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_draw_multi_indexed_ext(
            command_buffer,
            unsafe { std::slice::from_raw_parts(p_index_info, draw_count as usize) },
            instance_count,
            first_instance,
            stride,
            unsafe { p_vertex_offset.as_ref() },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_multi_indexed_ext)(
                    command_buffer,
                    draw_count,
                    p_index_info,
                    instance_count,
                    first_instance,
                    stride,
                    p_vertex_offset,
                )
            },
        }
    }
    extern "system" fn create_micromap_ext(
        device: vk::Device,
        p_create_info: *const vk::MicromapCreateInfoEXT,
        p_allocator: *const vk::AllocationCallbacks,
        p_micromap: *mut vk::MicromapEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateMicromapEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        let layer_result = device_info
            .customized_info
            .create_micromap_ext(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_micromap.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_micromap_ext)(device, p_create_info, p_allocator, p_micromap)
            },
        }
    }
    extern "system" fn destroy_micromap_ext(
        device: vk::Device,
        micromap: vk::MicromapEXT,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyMicromapEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        let layer_result = device_info
            .customized_info
            .destroy_micromap_ext(micromap, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_micromap_ext)(device, micromap, p_allocator)
            },
        }
    }
    extern "system" fn cmd_build_micromaps_ext(
        command_buffer: vk::CommandBuffer,
        info_count: u32,
        p_infos: *const vk::MicromapBuildInfoEXT,
    ) {
        let global = Self::instance();
        // vkCmdBuildMicromapsEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_build_micromaps_ext(command_buffer, unsafe {
                std::slice::from_raw_parts(p_infos, info_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_build_micromaps_ext)(command_buffer, info_count, p_infos)
            },
        }
    }
    extern "system" fn build_micromaps_ext(
        device: vk::Device,
        deferred_operation: vk::DeferredOperationKHR,
        info_count: u32,
        p_infos: *const vk::MicromapBuildInfoEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkBuildMicromapsEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .build_micromaps_ext(deferred_operation, unsafe {
                std::slice::from_raw_parts(p_infos, info_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.build_micromaps_ext)(
                    device,
                    deferred_operation,
                    info_count,
                    p_infos,
                )
            },
        }
    }
    extern "system" fn copy_micromap_ext(
        device: vk::Device,
        deferred_operation: vk::DeferredOperationKHR,
        p_info: *const vk::CopyMicromapInfoEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCopyMicromapEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        let layer_result = device_info
            .customized_info
            .copy_micromap_ext(deferred_operation, unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.copy_micromap_ext)(device, deferred_operation, p_info)
            },
        }
    }
    extern "system" fn copy_micromap_to_memory_ext(
        device: vk::Device,
        deferred_operation: vk::DeferredOperationKHR,
        p_info: *const vk::CopyMicromapToMemoryInfoEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCopyMicromapToMemoryEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        let layer_result = device_info
            .customized_info
            .copy_micromap_to_memory_ext(deferred_operation, unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.copy_micromap_to_memory_ext)(device, deferred_operation, p_info)
            },
        }
    }
    extern "system" fn copy_memory_to_micromap_ext(
        device: vk::Device,
        deferred_operation: vk::DeferredOperationKHR,
        p_info: *const vk::CopyMemoryToMicromapInfoEXT,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCopyMemoryToMicromapEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        let layer_result = device_info
            .customized_info
            .copy_memory_to_micromap_ext(deferred_operation, unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.copy_memory_to_micromap_ext)(device, deferred_operation, p_info)
            },
        }
    }
    extern "system" fn write_micromaps_properties_ext(
        device: vk::Device,
        micromap_count: u32,
        p_micromaps: *const vk::MicromapEXT,
        query_type: vk::QueryType,
        data_size: usize,
        p_data: *mut c_void,
        stride: usize,
    ) -> vk::Result {
        let global = Self::instance();
        // vkWriteMicromapsPropertiesEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.write_micromaps_properties_ext(
            unsafe { std::slice::from_raw_parts(p_micromaps, micromap_count as usize) },
            query_type,
            unsafe { std::slice::from_raw_parts_mut(p_data as *mut u8, data_size as usize) },
            stride,
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.write_micromaps_properties_ext)(
                    device,
                    micromap_count,
                    p_micromaps,
                    query_type,
                    data_size,
                    p_data,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_copy_micromap_ext(
        command_buffer: vk::CommandBuffer,
        p_info: *const vk::CopyMicromapInfoEXT,
    ) {
        let global = Self::instance();
        // vkCmdCopyMicromapEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        let layer_result = device_info
            .customized_info
            .cmd_copy_micromap_ext(command_buffer, unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_micromap_ext)(command_buffer, p_info)
            },
        }
    }
    extern "system" fn cmd_copy_micromap_to_memory_ext(
        command_buffer: vk::CommandBuffer,
        p_info: *const vk::CopyMicromapToMemoryInfoEXT,
    ) {
        let global = Self::instance();
        // vkCmdCopyMicromapToMemoryEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        let layer_result = device_info
            .customized_info
            .cmd_copy_micromap_to_memory_ext(command_buffer, unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_micromap_to_memory_ext)(command_buffer, p_info)
            },
        }
    }
    extern "system" fn cmd_copy_memory_to_micromap_ext(
        command_buffer: vk::CommandBuffer,
        p_info: *const vk::CopyMemoryToMicromapInfoEXT,
    ) {
        let global = Self::instance();
        // vkCmdCopyMemoryToMicromapEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        let layer_result = device_info
            .customized_info
            .cmd_copy_memory_to_micromap_ext(command_buffer, unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_memory_to_micromap_ext)(command_buffer, p_info)
            },
        }
    }
    extern "system" fn cmd_write_micromaps_properties_ext(
        command_buffer: vk::CommandBuffer,
        micromap_count: u32,
        p_micromaps: *const vk::MicromapEXT,
        query_type: vk::QueryType,
        query_pool: vk::QueryPool,
        first_query: u32,
    ) {
        let global = Self::instance();
        // vkCmdWriteMicromapsPropertiesEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_write_micromaps_properties_ext(
                command_buffer,
                unsafe { std::slice::from_raw_parts(p_micromaps, micromap_count as usize) },
                query_type,
                query_pool,
                first_query,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_write_micromaps_properties_ext)(
                    command_buffer,
                    micromap_count,
                    p_micromaps,
                    query_type,
                    query_pool,
                    first_query,
                )
            },
        }
    }
    extern "system" fn get_device_micromap_compatibility_ext(
        device: vk::Device,
        p_version_info: *const vk::MicromapVersionInfoEXT,
        p_compatibility: *mut vk::AccelerationStructureCompatibilityKHR,
    ) {
        let global = Self::instance();
        // vkGetDeviceMicromapCompatibilityEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        let layer_result = device_info
            .customized_info
            .get_device_micromap_compatibility_ext(unsafe { p_version_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_compatibility.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_micromap_compatibility_ext)(
                    device,
                    p_version_info,
                    p_compatibility,
                )
            },
        }
    }
    extern "system" fn get_micromap_build_sizes_ext(
        device: vk::Device,
        build_type: vk::AccelerationStructureBuildTypeKHR,
        p_build_info: *const vk::MicromapBuildInfoEXT,
        p_size_info: *mut vk::MicromapBuildSizesInfoEXT,
    ) {
        let global = Self::instance();
        // vkGetMicromapBuildSizesEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_opacity_micromap;
        let layer_result = device_info
            .customized_info
            .get_micromap_build_sizes_ext(build_type, unsafe { p_build_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_size_info.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_micromap_build_sizes_ext)(
                    device,
                    build_type,
                    p_build_info,
                    p_size_info,
                )
            },
        }
    }
    extern "system" fn set_device_memory_priority_ext(
        device: vk::Device,
        memory: vk::DeviceMemory,
        priority: f32,
    ) {
        let global = Self::instance();
        // vkSetDeviceMemoryPriorityEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_pageable_device_local_memory;
        let layer_result = device_info
            .customized_info
            .set_device_memory_priority_ext(memory, priority);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.set_device_memory_priority_ext)(device, memory, priority)
            },
        }
    }
    extern "system" fn get_descriptor_set_layout_host_mapping_info_valve(
        device: vk::Device,
        p_binding_reference: *const vk::DescriptorSetBindingReferenceVALVE,
        p_host_mapping: *mut vk::DescriptorSetLayoutHostMappingInfoVALVE,
    ) {
        let global = Self::instance();
        // vkGetDescriptorSetLayoutHostMappingInfoVALVE
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.valve_descriptor_set_host_mapping;
        let layer_result = device_info
            .customized_info
            .get_descriptor_set_layout_host_mapping_info_valve(
                unsafe { p_binding_reference.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_host_mapping.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_descriptor_set_layout_host_mapping_info_valve)(
                    device,
                    p_binding_reference,
                    p_host_mapping,
                )
            },
        }
    }
    extern "system" fn get_descriptor_set_host_mapping_valve(
        device: vk::Device,
        descriptor_set: vk::DescriptorSet,
        pp_data: *mut *mut c_void,
    ) {
        let global = Self::instance();
        // vkGetDescriptorSetHostMappingVALVE
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.valve_descriptor_set_host_mapping;
        let layer_result = device_info
            .customized_info
            .get_descriptor_set_host_mapping_valve(descriptor_set);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { pp_data.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_descriptor_set_host_mapping_valve)(
                    device,
                    descriptor_set,
                    pp_data,
                )
            },
        }
    }
    extern "system" fn cmd_copy_memory_indirect_nv(
        command_buffer: vk::CommandBuffer,
        copy_buffer_address: vk::DeviceAddress,
        copy_count: u32,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdCopyMemoryIndirectNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_copy_memory_indirect;
        let layer_result = device_info.customized_info.cmd_copy_memory_indirect_nv(
            command_buffer,
            copy_buffer_address,
            copy_count,
            stride,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_memory_indirect_nv)(
                    command_buffer,
                    copy_buffer_address,
                    copy_count,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_copy_memory_to_image_indirect_nv(
        command_buffer: vk::CommandBuffer,
        copy_buffer_address: vk::DeviceAddress,
        copy_count: u32,
        stride: u32,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        p_image_subresources: *const vk::ImageSubresourceLayers,
    ) {
        let global = Self::instance();
        // vkCmdCopyMemoryToImageIndirectNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_copy_memory_indirect;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_copy_memory_to_image_indirect_nv(
                command_buffer,
                copy_buffer_address,
                stride,
                dst_image,
                dst_image_layout,
                unsafe { std::slice::from_raw_parts(p_image_subresources, copy_count as usize) },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_memory_to_image_indirect_nv)(
                    command_buffer,
                    copy_buffer_address,
                    copy_count,
                    stride,
                    dst_image,
                    dst_image_layout,
                    p_image_subresources,
                )
            },
        }
    }
    extern "system" fn cmd_decompress_memory_nv(
        command_buffer: vk::CommandBuffer,
        decompress_region_count: u32,
        p_decompress_memory_regions: *const vk::DecompressMemoryRegionNV,
    ) {
        let global = Self::instance();
        // vkCmdDecompressMemoryNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_memory_decompression;
        #[allow(clippy::unnecessary_cast)]
        let layer_result =
            device_info
                .customized_info
                .cmd_decompress_memory_nv(command_buffer, unsafe {
                    std::slice::from_raw_parts(
                        p_decompress_memory_regions,
                        decompress_region_count as usize,
                    )
                });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_decompress_memory_nv)(
                    command_buffer,
                    decompress_region_count,
                    p_decompress_memory_regions,
                )
            },
        }
    }
    extern "system" fn cmd_decompress_memory_indirect_count_nv(
        command_buffer: vk::CommandBuffer,
        indirect_commands_address: vk::DeviceAddress,
        indirect_commands_count_address: vk::DeviceAddress,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDecompressMemoryIndirectCountNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_memory_decompression;
        let layer_result = device_info
            .customized_info
            .cmd_decompress_memory_indirect_count_nv(
                command_buffer,
                indirect_commands_address,
                indirect_commands_count_address,
                stride,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_decompress_memory_indirect_count_nv)(
                    command_buffer,
                    indirect_commands_address,
                    indirect_commands_count_address,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_set_tessellation_domain_origin_ext(
        command_buffer: vk::CommandBuffer,
        domain_origin: vk::TessellationDomainOrigin,
    ) {
        let global = Self::instance();
        // vkCmdSetTessellationDomainOriginEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_tessellation_domain_origin_ext(command_buffer, domain_origin);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_tessellation_domain_origin_ext)(
                    command_buffer,
                    domain_origin,
                )
            },
        }
    }
    extern "system" fn cmd_set_depth_clamp_enable_ext(
        command_buffer: vk::CommandBuffer,
        depth_clamp_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetDepthClampEnableEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_depth_clamp_enable_ext(command_buffer, depth_clamp_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_depth_clamp_enable_ext)(command_buffer, depth_clamp_enable)
            },
        }
    }
    extern "system" fn cmd_set_polygon_mode_ext(
        command_buffer: vk::CommandBuffer,
        polygon_mode: vk::PolygonMode,
    ) {
        let global = Self::instance();
        // vkCmdSetPolygonModeEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_polygon_mode_ext(command_buffer, polygon_mode);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_polygon_mode_ext)(command_buffer, polygon_mode)
            },
        }
    }
    extern "system" fn cmd_set_rasterization_samples_ext(
        command_buffer: vk::CommandBuffer,
        rasterization_samples: vk::SampleCountFlags,
    ) {
        let global = Self::instance();
        // vkCmdSetRasterizationSamplesEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_rasterization_samples_ext(command_buffer, rasterization_samples);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_rasterization_samples_ext)(
                    command_buffer,
                    rasterization_samples,
                )
            },
        }
    }
    extern "system" fn cmd_set_sample_mask_ext(
        command_buffer: vk::CommandBuffer,
        samples: vk::SampleCountFlags,
        p_sample_mask: *const vk::SampleMask,
    ) {
        let global = Self::instance();
        // vkCmdSetSampleMaskEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        #[allow(clippy::unnecessary_cast)]
        let layer_result =
            device_info
                .customized_info
                .cmd_set_sample_mask_ext(command_buffer, samples, unsafe {
                    std::slice::from_raw_parts(
                        p_sample_mask,
                        ((samples.as_raw() + 31) / 32) as usize,
                    )
                });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_sample_mask_ext)(command_buffer, samples, p_sample_mask)
            },
        }
    }
    extern "system" fn cmd_set_alpha_to_coverage_enable_ext(
        command_buffer: vk::CommandBuffer,
        alpha_to_coverage_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetAlphaToCoverageEnableEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_alpha_to_coverage_enable_ext(
                command_buffer,
                alpha_to_coverage_enable == vk::TRUE,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_alpha_to_coverage_enable_ext)(
                    command_buffer,
                    alpha_to_coverage_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_alpha_to_one_enable_ext(
        command_buffer: vk::CommandBuffer,
        alpha_to_one_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetAlphaToOneEnableEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_alpha_to_one_enable_ext(command_buffer, alpha_to_one_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_alpha_to_one_enable_ext)(
                    command_buffer,
                    alpha_to_one_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_logic_op_enable_ext(
        command_buffer: vk::CommandBuffer,
        logic_op_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetLogicOpEnableEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_logic_op_enable_ext(command_buffer, logic_op_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_logic_op_enable_ext)(command_buffer, logic_op_enable)
            },
        }
    }
    extern "system" fn cmd_set_color_blend_enable_ext(
        command_buffer: vk::CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_enables: *const vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetColorBlendEnableEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_set_color_blend_enable_ext(
            command_buffer,
            first_attachment,
            unsafe { std::slice::from_raw_parts(p_color_blend_enables, attachment_count as usize) }
                .iter()
                .map(|v| *v == vk::TRUE),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_color_blend_enable_ext)(
                    command_buffer,
                    first_attachment,
                    attachment_count,
                    p_color_blend_enables,
                )
            },
        }
    }
    extern "system" fn cmd_set_color_blend_equation_ext(
        command_buffer: vk::CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_equations: *const vk::ColorBlendEquationEXT,
    ) {
        let global = Self::instance();
        // vkCmdSetColorBlendEquationEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_set_color_blend_equation_ext(command_buffer, first_attachment, unsafe {
                std::slice::from_raw_parts(p_color_blend_equations, attachment_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_color_blend_equation_ext)(
                    command_buffer,
                    first_attachment,
                    attachment_count,
                    p_color_blend_equations,
                )
            },
        }
    }
    extern "system" fn cmd_set_color_write_mask_ext(
        command_buffer: vk::CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_write_masks: *const vk::ColorComponentFlags,
    ) {
        let global = Self::instance();
        // vkCmdSetColorWriteMaskEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_set_color_write_mask_ext(
            command_buffer,
            first_attachment,
            unsafe { std::slice::from_raw_parts(p_color_write_masks, attachment_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_color_write_mask_ext)(
                    command_buffer,
                    first_attachment,
                    attachment_count,
                    p_color_write_masks,
                )
            },
        }
    }
    extern "system" fn cmd_set_rasterization_stream_ext(
        command_buffer: vk::CommandBuffer,
        rasterization_stream: u32,
    ) {
        let global = Self::instance();
        // vkCmdSetRasterizationStreamEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_rasterization_stream_ext(command_buffer, rasterization_stream);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_rasterization_stream_ext)(
                    command_buffer,
                    rasterization_stream,
                )
            },
        }
    }
    extern "system" fn cmd_set_conservative_rasterization_mode_ext(
        command_buffer: vk::CommandBuffer,
        conservative_rasterization_mode: vk::ConservativeRasterizationModeEXT,
    ) {
        let global = Self::instance();
        // vkCmdSetConservativeRasterizationModeEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_conservative_rasterization_mode_ext(
                command_buffer,
                conservative_rasterization_mode,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_conservative_rasterization_mode_ext)(
                    command_buffer,
                    conservative_rasterization_mode,
                )
            },
        }
    }
    extern "system" fn cmd_set_extra_primitive_overestimation_size_ext(
        command_buffer: vk::CommandBuffer,
        extra_primitive_overestimation_size: f32,
    ) {
        let global = Self::instance();
        // vkCmdSetExtraPrimitiveOverestimationSizeEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_extra_primitive_overestimation_size_ext(
                command_buffer,
                extra_primitive_overestimation_size,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_extra_primitive_overestimation_size_ext)(
                    command_buffer,
                    extra_primitive_overestimation_size,
                )
            },
        }
    }
    extern "system" fn cmd_set_depth_clip_enable_ext(
        command_buffer: vk::CommandBuffer,
        depth_clip_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetDepthClipEnableEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_depth_clip_enable_ext(command_buffer, depth_clip_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_depth_clip_enable_ext)(command_buffer, depth_clip_enable)
            },
        }
    }
    extern "system" fn cmd_set_sample_locations_enable_ext(
        command_buffer: vk::CommandBuffer,
        sample_locations_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetSampleLocationsEnableEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_sample_locations_enable_ext(
                command_buffer,
                sample_locations_enable == vk::TRUE,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_sample_locations_enable_ext)(
                    command_buffer,
                    sample_locations_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_color_blend_advanced_ext(
        command_buffer: vk::CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_advanced: *const vk::ColorBlendAdvancedEXT,
    ) {
        let global = Self::instance();
        // vkCmdSetColorBlendAdvancedEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_set_color_blend_advanced_ext(command_buffer, first_attachment, unsafe {
                std::slice::from_raw_parts(p_color_blend_advanced, attachment_count as usize)
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_color_blend_advanced_ext)(
                    command_buffer,
                    first_attachment,
                    attachment_count,
                    p_color_blend_advanced,
                )
            },
        }
    }
    extern "system" fn cmd_set_provoking_vertex_mode_ext(
        command_buffer: vk::CommandBuffer,
        provoking_vertex_mode: vk::ProvokingVertexModeEXT,
    ) {
        let global = Self::instance();
        // vkCmdSetProvokingVertexModeEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_provoking_vertex_mode_ext(command_buffer, provoking_vertex_mode);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_provoking_vertex_mode_ext)(
                    command_buffer,
                    provoking_vertex_mode,
                )
            },
        }
    }
    extern "system" fn cmd_set_line_rasterization_mode_ext(
        command_buffer: vk::CommandBuffer,
        line_rasterization_mode: vk::LineRasterizationModeEXT,
    ) {
        let global = Self::instance();
        // vkCmdSetLineRasterizationModeEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_line_rasterization_mode_ext(command_buffer, line_rasterization_mode);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_line_rasterization_mode_ext)(
                    command_buffer,
                    line_rasterization_mode,
                )
            },
        }
    }
    extern "system" fn cmd_set_line_stipple_enable_ext(
        command_buffer: vk::CommandBuffer,
        stippled_line_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetLineStippleEnableEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_line_stipple_enable_ext(command_buffer, stippled_line_enable == vk::TRUE);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_line_stipple_enable_ext)(
                    command_buffer,
                    stippled_line_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_depth_clip_negative_one_to_one_ext(
        command_buffer: vk::CommandBuffer,
        negative_one_to_one: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetDepthClipNegativeOneToOneEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_depth_clip_negative_one_to_one_ext(
                command_buffer,
                negative_one_to_one == vk::TRUE,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_depth_clip_negative_one_to_one_ext)(
                    command_buffer,
                    negative_one_to_one,
                )
            },
        }
    }
    extern "system" fn cmd_set_viewport_w_scaling_enable_nv(
        command_buffer: vk::CommandBuffer,
        viewport_w_scaling_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetViewportWScalingEnableNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_viewport_w_scaling_enable_nv(
                command_buffer,
                viewport_w_scaling_enable == vk::TRUE,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_viewport_w_scaling_enable_nv)(
                    command_buffer,
                    viewport_w_scaling_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_viewport_swizzle_nv(
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_swizzles: *const vk::ViewportSwizzleNV,
    ) {
        let global = Self::instance();
        // vkCmdSetViewportSwizzleNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info.customized_info.cmd_set_viewport_swizzle_nv(
            command_buffer,
            first_viewport,
            unsafe { std::slice::from_raw_parts(p_viewport_swizzles, viewport_count as usize) },
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_viewport_swizzle_nv)(
                    command_buffer,
                    first_viewport,
                    viewport_count,
                    p_viewport_swizzles,
                )
            },
        }
    }
    extern "system" fn cmd_set_coverage_to_color_enable_nv(
        command_buffer: vk::CommandBuffer,
        coverage_to_color_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetCoverageToColorEnableNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_coverage_to_color_enable_nv(
                command_buffer,
                coverage_to_color_enable == vk::TRUE,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_coverage_to_color_enable_nv)(
                    command_buffer,
                    coverage_to_color_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_coverage_to_color_location_nv(
        command_buffer: vk::CommandBuffer,
        coverage_to_color_location: u32,
    ) {
        let global = Self::instance();
        // vkCmdSetCoverageToColorLocationNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_coverage_to_color_location_nv(command_buffer, coverage_to_color_location);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_coverage_to_color_location_nv)(
                    command_buffer,
                    coverage_to_color_location,
                )
            },
        }
    }
    extern "system" fn cmd_set_coverage_modulation_mode_nv(
        command_buffer: vk::CommandBuffer,
        coverage_modulation_mode: vk::CoverageModulationModeNV,
    ) {
        let global = Self::instance();
        // vkCmdSetCoverageModulationModeNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_coverage_modulation_mode_nv(command_buffer, coverage_modulation_mode);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_coverage_modulation_mode_nv)(
                    command_buffer,
                    coverage_modulation_mode,
                )
            },
        }
    }
    extern "system" fn cmd_set_coverage_modulation_table_enable_nv(
        command_buffer: vk::CommandBuffer,
        coverage_modulation_table_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetCoverageModulationTableEnableNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_coverage_modulation_table_enable_nv(
                command_buffer,
                coverage_modulation_table_enable == vk::TRUE,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_coverage_modulation_table_enable_nv)(
                    command_buffer,
                    coverage_modulation_table_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_coverage_modulation_table_nv(
        command_buffer: vk::CommandBuffer,
        coverage_modulation_table_count: u32,
        p_coverage_modulation_table: *const f32,
    ) {
        let global = Self::instance();
        // vkCmdSetCoverageModulationTableNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_set_coverage_modulation_table_nv(command_buffer, unsafe {
                std::slice::from_raw_parts(
                    p_coverage_modulation_table,
                    coverage_modulation_table_count as usize,
                )
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_coverage_modulation_table_nv)(
                    command_buffer,
                    coverage_modulation_table_count,
                    p_coverage_modulation_table,
                )
            },
        }
    }
    extern "system" fn cmd_set_shading_rate_image_enable_nv(
        command_buffer: vk::CommandBuffer,
        shading_rate_image_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetShadingRateImageEnableNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_shading_rate_image_enable_nv(
                command_buffer,
                shading_rate_image_enable == vk::TRUE,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_shading_rate_image_enable_nv)(
                    command_buffer,
                    shading_rate_image_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_representative_fragment_test_enable_nv(
        command_buffer: vk::CommandBuffer,
        representative_fragment_test_enable: vk::Bool32,
    ) {
        let global = Self::instance();
        // vkCmdSetRepresentativeFragmentTestEnableNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_representative_fragment_test_enable_nv(
                command_buffer,
                representative_fragment_test_enable == vk::TRUE,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_representative_fragment_test_enable_nv)(
                    command_buffer,
                    representative_fragment_test_enable,
                )
            },
        }
    }
    extern "system" fn cmd_set_coverage_reduction_mode_nv(
        command_buffer: vk::CommandBuffer,
        coverage_reduction_mode: vk::CoverageReductionModeNV,
    ) {
        let global = Self::instance();
        // vkCmdSetCoverageReductionModeNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_extended_dynamic_state3;
        let layer_result = device_info
            .customized_info
            .cmd_set_coverage_reduction_mode_nv(command_buffer, coverage_reduction_mode);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_coverage_reduction_mode_nv)(
                    command_buffer,
                    coverage_reduction_mode,
                )
            },
        }
    }
    extern "system" fn get_shader_module_identifier_ext(
        device: vk::Device,
        shader_module: vk::ShaderModule,
        p_identifier: *mut vk::ShaderModuleIdentifierEXT,
    ) {
        let global = Self::instance();
        // vkGetShaderModuleIdentifierEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_shader_module_identifier;
        let layer_result = device_info
            .customized_info
            .get_shader_module_identifier_ext(shader_module);
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_identifier.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_shader_module_identifier_ext)(
                    device,
                    shader_module,
                    p_identifier,
                )
            },
        }
    }
    extern "system" fn get_shader_module_create_info_identifier_ext(
        device: vk::Device,
        p_create_info: *const vk::ShaderModuleCreateInfo,
        p_identifier: *mut vk::ShaderModuleIdentifierEXT,
    ) {
        let global = Self::instance();
        // vkGetShaderModuleCreateInfoIdentifierEXT
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_shader_module_identifier;
        let layer_result = device_info
            .customized_info
            .get_shader_module_create_info_identifier_ext(
                unsafe { p_create_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_identifier.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_shader_module_create_info_identifier_ext)(
                    device,
                    p_create_info,
                    p_identifier,
                )
            },
        }
    }
    extern "system" fn create_optical_flow_session_nv(
        device: vk::Device,
        p_create_info: *const vk::OpticalFlowSessionCreateInfoNV,
        p_allocator: *const vk::AllocationCallbacks,
        p_session: *mut vk::OpticalFlowSessionNV,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateOpticalFlowSessionNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_optical_flow;
        let layer_result = device_info
            .customized_info
            .create_optical_flow_session_nv(unsafe { p_create_info.as_ref() }.unwrap(), unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_session.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_optical_flow_session_nv)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_session,
                )
            },
        }
    }
    extern "system" fn destroy_optical_flow_session_nv(
        device: vk::Device,
        session: vk::OpticalFlowSessionNV,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyOpticalFlowSessionNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_optical_flow;
        let layer_result = device_info
            .customized_info
            .destroy_optical_flow_session_nv(session, unsafe { p_allocator.as_ref() });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_optical_flow_session_nv)(device, session, p_allocator)
            },
        }
    }
    extern "system" fn bind_optical_flow_session_image_nv(
        device: vk::Device,
        session: vk::OpticalFlowSessionNV,
        binding_point: vk::OpticalFlowSessionBindingPointNV,
        view: vk::ImageView,
        layout: vk::ImageLayout,
    ) -> vk::Result {
        let global = Self::instance();
        // vkBindOpticalFlowSessionImageNV
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_optical_flow;
        let layer_result = device_info
            .customized_info
            .bind_optical_flow_session_image_nv(session, binding_point, view, layout);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.bind_optical_flow_session_image_nv)(
                    device,
                    session,
                    binding_point,
                    view,
                    layout,
                )
            },
        }
    }
    extern "system" fn cmd_optical_flow_execute_nv(
        command_buffer: vk::CommandBuffer,
        session: vk::OpticalFlowSessionNV,
        p_execute_info: *const vk::OpticalFlowExecuteInfoNV,
    ) {
        let global = Self::instance();
        // vkCmdOpticalFlowExecuteNV
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.nv_optical_flow;
        let layer_result = device_info.customized_info.cmd_optical_flow_execute_nv(
            command_buffer,
            session,
            unsafe { p_execute_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_optical_flow_execute_nv)(
                    command_buffer,
                    session,
                    p_execute_info,
                )
            },
        }
    }
    extern "system" fn get_framebuffer_tile_properties_qcom(
        device: vk::Device,
        framebuffer: vk::Framebuffer,
        p_properties_count: *mut u32,
        p_properties: *mut vk::TilePropertiesQCOM,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetFramebufferTilePropertiesQCOM
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.qcom_tile_properties;
        let layer_result = device_info
            .customized_info
            .get_framebuffer_tile_properties_qcom(framebuffer);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => unsafe {
                    fill_vk_out_array(
                        &res,
                        NonNull::new(p_properties_count).unwrap(),
                        p_properties,
                    )
                },
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_framebuffer_tile_properties_qcom)(
                    device,
                    framebuffer,
                    p_properties_count,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn get_dynamic_rendering_tile_properties_qcom(
        device: vk::Device,
        p_rendering_info: *const vk::RenderingInfo,
        p_properties: *mut vk::TilePropertiesQCOM,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetDynamicRenderingTilePropertiesQCOM
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.qcom_tile_properties;
        let layer_result = device_info
            .customized_info
            .get_dynamic_rendering_tile_properties_qcom(
                unsafe { p_rendering_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_properties.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_dynamic_rendering_tile_properties_qcom)(
                    device,
                    p_rendering_info,
                    p_properties,
                )
            },
        }
    }
    extern "system" fn create_acceleration_structure_khr(
        device: vk::Device,
        p_create_info: *const vk::AccelerationStructureCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_acceleration_structure: *mut vk::AccelerationStructureKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateAccelerationStructureKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        let layer_result = device_info
            .customized_info
            .create_acceleration_structure_khr(
                unsafe { p_create_info.as_ref() }.unwrap(),
                unsafe { p_allocator.as_ref() },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_acceleration_structure.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_acceleration_structure_khr)(
                    device,
                    p_create_info,
                    p_allocator,
                    p_acceleration_structure,
                )
            },
        }
    }
    extern "system" fn destroy_acceleration_structure_khr(
        device: vk::Device,
        acceleration_structure: vk::AccelerationStructureKHR,
        p_allocator: *const vk::AllocationCallbacks,
    ) {
        let global = Self::instance();
        // vkDestroyAccelerationStructureKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        let layer_result = device_info
            .customized_info
            .destroy_acceleration_structure_khr(acceleration_structure, unsafe {
                p_allocator.as_ref()
            });
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.destroy_acceleration_structure_khr)(
                    device,
                    acceleration_structure,
                    p_allocator,
                )
            },
        }
    }
    extern "system" fn copy_acceleration_structure_khr(
        device: vk::Device,
        deferred_operation: vk::DeferredOperationKHR,
        p_info: *const vk::CopyAccelerationStructureInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCopyAccelerationStructureKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        let layer_result = device_info.customized_info.copy_acceleration_structure_khr(
            deferred_operation,
            unsafe { p_info.as_ref() }.unwrap(),
        );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.copy_acceleration_structure_khr)(device, deferred_operation, p_info)
            },
        }
    }
    extern "system" fn copy_acceleration_structure_to_memory_khr(
        device: vk::Device,
        deferred_operation: vk::DeferredOperationKHR,
        p_info: *const vk::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCopyAccelerationStructureToMemoryKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        let layer_result = device_info
            .customized_info
            .copy_acceleration_structure_to_memory_khr(
                deferred_operation,
                unsafe { p_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.copy_acceleration_structure_to_memory_khr)(
                    device,
                    deferred_operation,
                    p_info,
                )
            },
        }
    }
    extern "system" fn copy_memory_to_acceleration_structure_khr(
        device: vk::Device,
        deferred_operation: vk::DeferredOperationKHR,
        p_info: *const vk::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCopyMemoryToAccelerationStructureKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        let layer_result = device_info
            .customized_info
            .copy_memory_to_acceleration_structure_khr(
                deferred_operation,
                unsafe { p_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.copy_memory_to_acceleration_structure_khr)(
                    device,
                    deferred_operation,
                    p_info,
                )
            },
        }
    }
    extern "system" fn write_acceleration_structures_properties_khr(
        device: vk::Device,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const vk::AccelerationStructureKHR,
        query_type: vk::QueryType,
        data_size: usize,
        p_data: *mut c_void,
        stride: usize,
    ) -> vk::Result {
        let global = Self::instance();
        // vkWriteAccelerationStructuresPropertiesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        #[allow(clippy::unnecessary_cast)]
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .write_acceleration_structures_properties_khr(
                unsafe {
                    std::slice::from_raw_parts(
                        p_acceleration_structures,
                        acceleration_structure_count as usize,
                    )
                },
                query_type,
                unsafe { std::slice::from_raw_parts_mut(p_data as *mut u8, data_size as usize) },
                stride,
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.write_acceleration_structures_properties_khr)(
                    device,
                    acceleration_structure_count,
                    p_acceleration_structures,
                    query_type,
                    data_size,
                    p_data,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_copy_acceleration_structure_khr(
        command_buffer: vk::CommandBuffer,
        p_info: *const vk::CopyAccelerationStructureInfoKHR,
    ) {
        let global = Self::instance();
        // vkCmdCopyAccelerationStructureKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        let layer_result = device_info
            .customized_info
            .cmd_copy_acceleration_structure_khr(
                command_buffer,
                unsafe { p_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_acceleration_structure_khr)(command_buffer, p_info)
            },
        }
    }
    extern "system" fn cmd_copy_acceleration_structure_to_memory_khr(
        command_buffer: vk::CommandBuffer,
        p_info: *const vk::CopyAccelerationStructureToMemoryInfoKHR,
    ) {
        let global = Self::instance();
        // vkCmdCopyAccelerationStructureToMemoryKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        let layer_result = device_info
            .customized_info
            .cmd_copy_acceleration_structure_to_memory_khr(
                command_buffer,
                unsafe { p_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_acceleration_structure_to_memory_khr)(
                    command_buffer,
                    p_info,
                )
            },
        }
    }
    extern "system" fn cmd_copy_memory_to_acceleration_structure_khr(
        command_buffer: vk::CommandBuffer,
        p_info: *const vk::CopyMemoryToAccelerationStructureInfoKHR,
    ) {
        let global = Self::instance();
        // vkCmdCopyMemoryToAccelerationStructureKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        let layer_result = device_info
            .customized_info
            .cmd_copy_memory_to_acceleration_structure_khr(
                command_buffer,
                unsafe { p_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_copy_memory_to_acceleration_structure_khr)(
                    command_buffer,
                    p_info,
                )
            },
        }
    }
    extern "system" fn get_acceleration_structure_device_address_khr(
        device: vk::Device,
        p_info: *const vk::AccelerationStructureDeviceAddressInfoKHR,
    ) -> vk::DeviceAddress {
        let global = Self::instance();
        // vkGetAccelerationStructureDeviceAddressKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        let layer_result = device_info
            .customized_info
            .get_acceleration_structure_device_address_khr(unsafe { p_info.as_ref() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_acceleration_structure_device_address_khr)(device, p_info)
            },
        }
    }
    extern "system" fn cmd_write_acceleration_structures_properties_khr(
        command_buffer: vk::CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const vk::AccelerationStructureKHR,
        query_type: vk::QueryType,
        query_pool: vk::QueryPool,
        first_query: u32,
    ) {
        let global = Self::instance();
        // vkCmdWriteAccelerationStructuresPropertiesKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .cmd_write_acceleration_structures_properties_khr(
                command_buffer,
                unsafe {
                    std::slice::from_raw_parts(
                        p_acceleration_structures,
                        acceleration_structure_count as usize,
                    )
                },
                query_type,
                query_pool,
                first_query,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_write_acceleration_structures_properties_khr)(
                    command_buffer,
                    acceleration_structure_count,
                    p_acceleration_structures,
                    query_type,
                    query_pool,
                    first_query,
                )
            },
        }
    }
    extern "system" fn get_device_acceleration_structure_compatibility_khr(
        device: vk::Device,
        p_version_info: *const vk::AccelerationStructureVersionInfoKHR,
        p_compatibility: *mut vk::AccelerationStructureCompatibilityKHR,
    ) {
        let global = Self::instance();
        // vkGetDeviceAccelerationStructureCompatibilityKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        let layer_result = device_info
            .customized_info
            .get_device_acceleration_structure_compatibility_khr(
                unsafe { p_version_info.as_ref() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_compatibility.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_device_acceleration_structure_compatibility_khr)(
                    device,
                    p_version_info,
                    p_compatibility,
                )
            },
        }
    }
    extern "system" fn get_acceleration_structure_build_sizes_khr(
        device: vk::Device,
        build_type: vk::AccelerationStructureBuildTypeKHR,
        p_build_info: *const vk::AccelerationStructureBuildGeometryInfoKHR,
        p_max_primitive_counts: *const u32,
        p_size_info: *mut vk::AccelerationStructureBuildSizesInfoKHR,
    ) {
        let global = Self::instance();
        // vkGetAccelerationStructureBuildSizesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_acceleration_structure;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .get_acceleration_structure_build_sizes_khr(
                build_type,
                unsafe { p_build_info.as_ref() }.unwrap(),
                if p_max_primitive_counts.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts(
                            p_max_primitive_counts,
                            unsafe { p_build_info.as_ref() }.unwrap().geometry_count as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => {
                *unsafe { p_size_info.as_mut() }.unwrap() = res;
            }
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_acceleration_structure_build_sizes_khr)(
                    device,
                    build_type,
                    p_build_info,
                    p_max_primitive_counts,
                    p_size_info,
                )
            },
        }
    }
    extern "system" fn cmd_trace_rays_khr(
        command_buffer: vk::CommandBuffer,
        p_raygen_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        let global = Self::instance();
        // vkCmdTraceRaysKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_ray_tracing_pipeline;
        let layer_result = device_info.customized_info.cmd_trace_rays_khr(
            command_buffer,
            unsafe { p_raygen_shader_binding_table.as_ref() }.unwrap(),
            unsafe { p_miss_shader_binding_table.as_ref() }.unwrap(),
            unsafe { p_hit_shader_binding_table.as_ref() }.unwrap(),
            unsafe { p_callable_shader_binding_table.as_ref() }.unwrap(),
            width,
            height,
            depth,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_trace_rays_khr)(
                    command_buffer,
                    p_raygen_shader_binding_table,
                    p_miss_shader_binding_table,
                    p_hit_shader_binding_table,
                    p_callable_shader_binding_table,
                    width,
                    height,
                    depth,
                )
            },
        }
    }
    extern "system" fn create_ray_tracing_pipelines_khr(
        device: vk::Device,
        deferred_operation: vk::DeferredOperationKHR,
        pipeline_cache: vk::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const vk::RayTracingPipelineCreateInfoKHR,
        p_allocator: *const vk::AllocationCallbacks,
        p_pipelines: *mut vk::Pipeline,
    ) -> vk::Result {
        let global = Self::instance();
        // vkCreateRayTracingPipelinesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_ray_tracing_pipeline;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .create_ray_tracing_pipelines_khr(
                deferred_operation,
                pipeline_cache,
                unsafe { std::slice::from_raw_parts(p_create_infos, create_info_count as usize) },
                unsafe { p_allocator.as_ref() },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    unsafe {
                        std::slice::from_raw_parts_mut(
                            p_pipelines,
                            create_info_count.try_into().unwrap(),
                        )
                    }
                    .copy_from_slice(&res);
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.create_ray_tracing_pipelines_khr)(
                    device,
                    deferred_operation,
                    pipeline_cache,
                    create_info_count,
                    p_create_infos,
                    p_allocator,
                    p_pipelines,
                )
            },
        }
    }
    extern "system" fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        device: vk::Device,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetRayTracingCaptureReplayShaderGroupHandlesKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_ray_tracing_pipeline;
        let layer_result = device_info
            .customized_info
            .get_ray_tracing_capture_replay_shader_group_handles_khr(
                pipeline,
                first_group,
                group_count,
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    let p_data = p_data as *mut u8;
                    unsafe { std::slice::from_raw_parts_mut(p_data, data_size) }
                        .copy_from_slice(&res);
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_ray_tracing_capture_replay_shader_group_handles_khr)(
                    device,
                    pipeline,
                    first_group,
                    group_count,
                    data_size,
                    p_data,
                )
            },
        }
    }
    extern "system" fn cmd_trace_rays_indirect_khr(
        command_buffer: vk::CommandBuffer,
        p_raygen_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const vk::StridedDeviceAddressRegionKHR,
        indirect_device_address: vk::DeviceAddress,
    ) {
        let global = Self::instance();
        // vkCmdTraceRaysIndirectKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_ray_tracing_pipeline;
        let layer_result = device_info.customized_info.cmd_trace_rays_indirect_khr(
            command_buffer,
            unsafe { p_raygen_shader_binding_table.as_ref() }.unwrap(),
            unsafe { p_miss_shader_binding_table.as_ref() }.unwrap(),
            unsafe { p_hit_shader_binding_table.as_ref() }.unwrap(),
            unsafe { p_callable_shader_binding_table.as_ref() }.unwrap(),
            indirect_device_address,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_trace_rays_indirect_khr)(
                    command_buffer,
                    p_raygen_shader_binding_table,
                    p_miss_shader_binding_table,
                    p_hit_shader_binding_table,
                    p_callable_shader_binding_table,
                    indirect_device_address,
                )
            },
        }
    }
    extern "system" fn get_ray_tracing_shader_group_stack_size_khr(
        device: vk::Device,
        pipeline: vk::Pipeline,
        group: u32,
        group_shader: vk::ShaderGroupShaderKHR,
    ) -> vk::DeviceSize {
        let global = Self::instance();
        // vkGetRayTracingShaderGroupStackSizeKHR
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_ray_tracing_pipeline;
        let layer_result = device_info
            .customized_info
            .get_ray_tracing_shader_group_stack_size_khr(pipeline, group, group_shader);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_ray_tracing_shader_group_stack_size_khr)(
                    device,
                    pipeline,
                    group,
                    group_shader,
                )
            },
        }
    }
    extern "system" fn cmd_set_ray_tracing_pipeline_stack_size_khr(
        command_buffer: vk::CommandBuffer,
        pipeline_stack_size: u32,
    ) {
        let global = Self::instance();
        // vkCmdSetRayTracingPipelineStackSizeKHR
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.khr_ray_tracing_pipeline;
        let layer_result = device_info
            .customized_info
            .cmd_set_ray_tracing_pipeline_stack_size_khr(command_buffer, pipeline_stack_size);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_set_ray_tracing_pipeline_stack_size_khr)(
                    command_buffer,
                    pipeline_stack_size,
                )
            },
        }
    }
    extern "system" fn cmd_draw_mesh_tasks_ext(
        command_buffer: vk::CommandBuffer,
        group_countx: u32,
        group_county: u32,
        group_countz: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawMeshTasksEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_mesh_shader;
        let layer_result = device_info.customized_info.cmd_draw_mesh_tasks_ext(
            command_buffer,
            group_countx,
            group_county,
            group_countz,
        );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_mesh_tasks_ext)(
                    command_buffer,
                    group_countx,
                    group_county,
                    group_countz,
                )
            },
        }
    }
    extern "system" fn cmd_draw_mesh_tasks_indirect_ext(
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawMeshTasksIndirectEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_mesh_shader;
        let layer_result = device_info
            .customized_info
            .cmd_draw_mesh_tasks_indirect_ext(command_buffer, buffer, offset, draw_count, stride);
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_mesh_tasks_indirect_ext)(
                    command_buffer,
                    buffer,
                    offset,
                    draw_count,
                    stride,
                )
            },
        }
    }
    extern "system" fn cmd_draw_mesh_tasks_indirect_count_ext(
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let global = Self::instance();
        // vkCmdDrawMeshTasksIndirectCountEXT
        let device_info = global.get_device_info(command_buffer).unwrap();
        let dispatch_table = &device_info.dispatch_table.ext_mesh_shader;
        let layer_result = device_info
            .customized_info
            .cmd_draw_mesh_tasks_indirect_count_ext(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            );
        match layer_result {
            LayerResult::Handled(res) => res,
            LayerResult::Unhandled => unsafe {
                (dispatch_table.cmd_draw_mesh_tasks_indirect_count_ext)(
                    command_buffer,
                    buffer,
                    offset,
                    count_buffer,
                    count_buffer_offset,
                    max_draw_count,
                    stride,
                )
            },
        }
    }
}
