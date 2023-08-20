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
use smallvec::smallvec;
use std::{
    borrow::Borrow,
    collections::HashSet,
    ffi::{c_char, c_int, c_void, CStr},
    ptr::NonNull,
    sync::Arc,
};

use super::{
    get_device_proc_addr_loader, get_instance_proc_addr_loader, ApiVersion, Feature,
    TryFromExtensionError, VulkanCommand,
};
use crate::{
    fill_vk_out_array, DeviceHooks, DeviceInfo, Global, InstanceHooks, InstanceInfo, Layer,
    LayerResult, LayerVulkanCommand,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Extension {
    KHRSurface,
    KHRSwapchain,
    KHRDisplay,
    KHRDisplaySwapchain,
    KHRXlibSurface,
    KHRXcbSurface,
    KHRWaylandSurface,
    KHRAndroidSurface,
    KHRWin32Surface,
    KHRSamplerMirrorClampToEdge,
    KHRVideoQueue,
    KHRVideoDecodeQueue,
    KHRVideoDecodeH264,
    KHRDynamicRendering,
    KHRMultiview,
    KHRGetPhysicalDeviceProperties2,
    KHRDeviceGroup,
    KHRShaderDrawParameters,
    KHRMaintenance1,
    KHRDeviceGroupCreation,
    KHRExternalMemoryCapabilities,
    KHRExternalMemory,
    KHRExternalMemoryWin32,
    KHRExternalMemoryFd,
    KHRWin32KeyedMutex,
    KHRExternalSemaphoreCapabilities,
    KHRExternalSemaphore,
    KHRExternalSemaphoreWin32,
    KHRExternalSemaphoreFd,
    KHRPushDescriptor,
    KHRShaderFloat16Int8,
    KHR16bitStorage,
    KHRIncrementalPresent,
    KHRDescriptorUpdateTemplate,
    KHRImagelessFramebuffer,
    KHRCreateRenderpass2,
    KHRSharedPresentableImage,
    KHRExternalFenceCapabilities,
    KHRExternalFence,
    KHRExternalFenceWin32,
    KHRExternalFenceFd,
    KHRPerformanceQuery,
    KHRMaintenance2,
    KHRGetSurfaceCapabilities2,
    KHRVariablePointers,
    KHRGetDisplayProperties2,
    KHRDedicatedAllocation,
    KHRStorageBufferStorageClass,
    KHRRelaxedBlockLayout,
    KHRGetMemoryRequirements2,
    KHRImageFormatList,
    KHRSamplerYcbcrConversion,
    KHRBindMemory2,
    KHRPortabilitySubset,
    KHRMaintenance3,
    KHRDrawIndirectCount,
    KHRShaderSubgroupExtendedTypes,
    KHR8bitStorage,
    KHRShaderAtomicInt64,
    KHRShaderClock,
    KHRVideoDecodeH265,
    KHRGlobalPriority,
    KHRDriverProperties,
    KHRShaderFloatControls,
    KHRDepthStencilResolve,
    KHRSwapchainMutableFormat,
    KHRTimelineSemaphore,
    KHRVulkanMemoryModel,
    KHRShaderTerminateInvocation,
    KHRFragmentShadingRate,
    KHRSpirv14,
    KHRSurfaceProtectedCapabilities,
    KHRSeparateDepthStencilLayouts,
    KHRPresentWait,
    KHRUniformBufferStandardLayout,
    KHRBufferDeviceAddress,
    KHRDeferredHostOperations,
    KHRPipelineExecutableProperties,
    KHRMapMemory2,
    KHRShaderIntegerDotProduct,
    KHRPipelineLibrary,
    KHRShaderNonSemanticInfo,
    KHRPresentId,
    KHRVideoEncodeQueue,
    KHRSynchronization2,
    KHRFragmentShaderBarycentric,
    KHRShaderSubgroupUniformControlFlow,
    KHRZeroInitializeWorkgroupMemory,
    KHRWorkgroupMemoryExplicitLayout,
    KHRCopyCommands2,
    KHRFormatFeatureFlags2,
    KHRRayTracingMaintenance1,
    KHRPortabilityEnumeration,
    KHRMaintenance4,
    KHRRayTracingPositionFetch,
    ANDROIDNativeBuffer,
    EXTDebugReport,
    NVGlslShader,
    EXTDepthRangeUnrestricted,
    IMGFilterCubic,
    AMDRasterizationOrder,
    AMDShaderTrinaryMinmax,
    AMDShaderExplicitVertexParameter,
    EXTDebugMarker,
    AMDGcnShader,
    NVDedicatedAllocation,
    EXTTransformFeedback,
    NVXBinaryImport,
    NVXImageViewHandle,
    AMDDrawIndirectCount,
    AMDNegativeViewportHeight,
    AMDGpuShaderHalfFloat,
    AMDShaderBallot,
    EXTVideoEncodeH264,
    EXTVideoEncodeH265,
    AMDTextureGatherBiasLod,
    AMDShaderInfo,
    AMDShaderImageLoadStoreLod,
    GGPStreamDescriptorSurface,
    NVCornerSampledImage,
    IMGFormatPvrtc,
    NVExternalMemoryCapabilities,
    NVExternalMemory,
    NVExternalMemoryWin32,
    NVWin32KeyedMutex,
    EXTValidationFlags,
    NNViSurface,
    EXTShaderSubgroupBallot,
    EXTShaderSubgroupVote,
    EXTTextureCompressionAstcHdr,
    EXTAstcDecodeMode,
    EXTPipelineRobustness,
    EXTConditionalRendering,
    NVClipSpaceWScaling,
    EXTDirectModeDisplay,
    EXTAcquireXlibDisplay,
    EXTDisplaySurfaceCounter,
    EXTDisplayControl,
    GOOGLEDisplayTiming,
    NVSampleMaskOverrideCoverage,
    NVGeometryShaderPassthrough,
    NVViewportArray2,
    NVXMultiviewPerViewAttributes,
    NVViewportSwizzle,
    EXTDiscardRectangles,
    EXTConservativeRasterization,
    EXTDepthClipEnable,
    EXTSwapchainColorspace,
    EXTHdrMetadata,
    MVKIosSurface,
    MVKMacosSurface,
    EXTExternalMemoryDmaBuf,
    EXTQueueFamilyForeign,
    EXTDebugUtils,
    ANDROIDExternalMemoryAndroidHardwareBuffer,
    EXTSamplerFilterMinmax,
    AMDGpuShaderInt16,
    AMDMixedAttachmentSamples,
    AMDShaderFragmentMask,
    EXTInlineUniformBlock,
    EXTShaderStencilExport,
    EXTSampleLocations,
    EXTBlendOperationAdvanced,
    NVFragmentCoverageToColor,
    NVFramebufferMixedSamples,
    NVFillRectangle,
    NVShaderSmBuiltins,
    EXTPostDepthCoverage,
    EXTImageDrmFormatModifier,
    EXTValidationCache,
    EXTDescriptorIndexing,
    EXTShaderViewportIndexLayer,
    NVShadingRateImage,
    NVRayTracing,
    NVRepresentativeFragmentTest,
    EXTFilterCubic,
    QCOMRenderPassShaderResolve,
    EXTGlobalPriority,
    EXTExternalMemoryHost,
    AMDBufferMarker,
    AMDPipelineCompilerControl,
    EXTCalibratedTimestamps,
    AMDShaderCoreProperties,
    AMDMemoryOverallocationBehavior,
    EXTVertexAttributeDivisor,
    GGPFrameToken,
    EXTPipelineCreationFeedback,
    NVShaderSubgroupPartitioned,
    NVComputeShaderDerivatives,
    NVMeshShader,
    NVFragmentShaderBarycentric,
    NVShaderImageFootprint,
    NVScissorExclusive,
    NVDeviceDiagnosticCheckpoints,
    INTELShaderIntegerFunctions2,
    INTELPerformanceQuery,
    EXTPciBusInfo,
    AMDDisplayNativeHdr,
    FUCHSIAImagepipeSurface,
    EXTMetalSurface,
    EXTFragmentDensityMap,
    EXTScalarBlockLayout,
    GOOGLEHlslFunctionality1,
    GOOGLEDecorateString,
    EXTSubgroupSizeControl,
    AMDShaderCoreProperties2,
    AMDDeviceCoherentMemory,
    EXTShaderImageAtomicInt64,
    EXTMemoryBudget,
    EXTMemoryPriority,
    NVDedicatedAllocationImageAliasing,
    EXTBufferDeviceAddress,
    EXTToolingInfo,
    EXTSeparateStencilUsage,
    EXTValidationFeatures,
    NVCooperativeMatrix,
    NVCoverageReductionMode,
    EXTFragmentShaderInterlock,
    EXTYcbcrImageArrays,
    EXTProvokingVertex,
    EXTFullScreenExclusive,
    EXTHeadlessSurface,
    EXTLineRasterization,
    EXTShaderAtomicFloat,
    EXTHostQueryReset,
    EXTIndexTypeUint8,
    EXTExtendedDynamicState,
    EXTShaderAtomicFloat2,
    EXTSurfaceMaintenance1,
    EXTSwapchainMaintenance1,
    EXTShaderDemoteToHelperInvocation,
    NVDeviceGeneratedCommands,
    NVInheritedViewportScissor,
    EXTTexelBufferAlignment,
    QCOMRenderPassTransform,
    EXTDeviceMemoryReport,
    EXTAcquireDrmDisplay,
    EXTRobustness2,
    EXTCustomBorderColor,
    GOOGLEUserType,
    NVPresentBarrier,
    EXTPrivateData,
    EXTPipelineCreationCacheControl,
    NVDeviceDiagnosticsConfig,
    QCOMRenderPassStoreOps,
    NVLowLatency,
    EXTMetalObjects,
    EXTDescriptorBuffer,
    EXTGraphicsPipelineLibrary,
    AMDShaderEarlyAndLateFragmentTests,
    NVFragmentShadingRateEnums,
    NVRayTracingMotionBlur,
    EXTYcbcr2plane444Formats,
    EXTFragmentDensityMap2,
    QCOMRotatedCopyCommands,
    EXTImageRobustness,
    EXTImageCompressionControl,
    EXTAttachmentFeedbackLoopLayout,
    EXT4444Formats,
    EXTDeviceFault,
    ARMRasterizationOrderAttachmentAccess,
    EXTRgba10x6Formats,
    NVAcquireWinrtDisplay,
    EXTDirectfbSurface,
    VALVEMutableDescriptorType,
    EXTVertexInputDynamicState,
    EXTPhysicalDeviceDrm,
    EXTDeviceAddressBindingReport,
    EXTDepthClipControl,
    EXTPrimitiveTopologyListRestart,
    FUCHSIAExternalMemory,
    FUCHSIAExternalSemaphore,
    FUCHSIABufferCollection,
    HUAWEISubpassShading,
    HUAWEIInvocationMask,
    NVExternalMemoryRdma,
    EXTPipelineProperties,
    EXTMultisampledRenderToSingleSampled,
    EXTExtendedDynamicState2,
    QNXScreenSurface,
    EXTColorWriteEnable,
    EXTPrimitivesGeneratedQuery,
    EXTGlobalPriorityQuery,
    EXTImageViewMinLod,
    EXTMultiDraw,
    EXTImage2dViewOf3d,
    EXTShaderTileImage,
    EXTOpacityMicromap,
    NVDisplacementMicromap,
    EXTLoadStoreOpNone,
    HUAWEIClusterCullingShader,
    EXTBorderColorSwizzle,
    EXTPageableDeviceLocalMemory,
    ARMShaderCoreProperties,
    EXTImageSlicedViewOf3d,
    VALVEDescriptorSetHostMapping,
    EXTDepthClampZeroOne,
    EXTNonSeamlessCubeMap,
    QCOMFragmentDensityMapOffset,
    NVCopyMemoryIndirect,
    NVMemoryDecompression,
    NVLinearColorAttachment,
    GOOGLESurfacelessQuery,
    EXTImageCompressionControlSwapchain,
    QCOMImageProcessing,
    EXTExtendedDynamicState3,
    EXTSubpassMergeFeedback,
    LUNARGDirectDriverLoading,
    EXTShaderModuleIdentifier,
    EXTRasterizationOrderAttachmentAccess,
    NVOpticalFlow,
    EXTLegacyDithering,
    EXTPipelineProtectedAccess,
    EXTShaderObject,
    QCOMTileProperties,
    SECAmigoProfiling,
    QCOMMultiviewPerViewViewports,
    NVRayTracingInvocationReorder,
    EXTMutableDescriptorType,
    ARMShaderCoreBuiltins,
    EXTPipelineLibraryGroupHandles,
    QCOMMultiviewPerViewRenderAreas,
    EXTAttachmentFeedbackLoopDynamicState,
    KHRAccelerationStructure,
    KHRRayTracingPipeline,
    KHRRayQuery,
    EXTMeshShader,
}

impl TryFrom<&str> for Extension {
    type Error = TryFromExtensionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "VK_KHR_surface" => Ok(Extension::KHRSurface),
            "VK_KHR_swapchain" => Ok(Extension::KHRSwapchain),
            "VK_KHR_display" => Ok(Extension::KHRDisplay),
            "VK_KHR_display_swapchain" => Ok(Extension::KHRDisplaySwapchain),
            "VK_KHR_xlib_surface" => Ok(Extension::KHRXlibSurface),
            "VK_KHR_xcb_surface" => Ok(Extension::KHRXcbSurface),
            "VK_KHR_wayland_surface" => Ok(Extension::KHRWaylandSurface),
            "VK_KHR_android_surface" => Ok(Extension::KHRAndroidSurface),
            "VK_KHR_win32_surface" => Ok(Extension::KHRWin32Surface),
            "VK_KHR_sampler_mirror_clamp_to_edge" => Ok(Extension::KHRSamplerMirrorClampToEdge),
            "VK_KHR_video_queue" => Ok(Extension::KHRVideoQueue),
            "VK_KHR_video_decode_queue" => Ok(Extension::KHRVideoDecodeQueue),
            "VK_KHR_video_decode_h264" => Ok(Extension::KHRVideoDecodeH264),
            "VK_KHR_dynamic_rendering" => Ok(Extension::KHRDynamicRendering),
            "VK_KHR_multiview" => Ok(Extension::KHRMultiview),
            "VK_KHR_get_physical_device_properties2" => {
                Ok(Extension::KHRGetPhysicalDeviceProperties2)
            }
            "VK_KHR_device_group" => Ok(Extension::KHRDeviceGroup),
            "VK_KHR_shader_draw_parameters" => Ok(Extension::KHRShaderDrawParameters),
            "VK_KHR_maintenance1" => Ok(Extension::KHRMaintenance1),
            "VK_KHR_device_group_creation" => Ok(Extension::KHRDeviceGroupCreation),
            "VK_KHR_external_memory_capabilities" => Ok(Extension::KHRExternalMemoryCapabilities),
            "VK_KHR_external_memory" => Ok(Extension::KHRExternalMemory),
            "VK_KHR_external_memory_win32" => Ok(Extension::KHRExternalMemoryWin32),
            "VK_KHR_external_memory_fd" => Ok(Extension::KHRExternalMemoryFd),
            "VK_KHR_win32_keyed_mutex" => Ok(Extension::KHRWin32KeyedMutex),
            "VK_KHR_external_semaphore_capabilities" => {
                Ok(Extension::KHRExternalSemaphoreCapabilities)
            }
            "VK_KHR_external_semaphore" => Ok(Extension::KHRExternalSemaphore),
            "VK_KHR_external_semaphore_win32" => Ok(Extension::KHRExternalSemaphoreWin32),
            "VK_KHR_external_semaphore_fd" => Ok(Extension::KHRExternalSemaphoreFd),
            "VK_KHR_push_descriptor" => Ok(Extension::KHRPushDescriptor),
            "VK_KHR_shader_float16_int8" => Ok(Extension::KHRShaderFloat16Int8),
            "VK_KHR_16bit_storage" => Ok(Extension::KHR16bitStorage),
            "VK_KHR_incremental_present" => Ok(Extension::KHRIncrementalPresent),
            "VK_KHR_descriptor_update_template" => Ok(Extension::KHRDescriptorUpdateTemplate),
            "VK_KHR_imageless_framebuffer" => Ok(Extension::KHRImagelessFramebuffer),
            "VK_KHR_create_renderpass2" => Ok(Extension::KHRCreateRenderpass2),
            "VK_KHR_shared_presentable_image" => Ok(Extension::KHRSharedPresentableImage),
            "VK_KHR_external_fence_capabilities" => Ok(Extension::KHRExternalFenceCapabilities),
            "VK_KHR_external_fence" => Ok(Extension::KHRExternalFence),
            "VK_KHR_external_fence_win32" => Ok(Extension::KHRExternalFenceWin32),
            "VK_KHR_external_fence_fd" => Ok(Extension::KHRExternalFenceFd),
            "VK_KHR_performance_query" => Ok(Extension::KHRPerformanceQuery),
            "VK_KHR_maintenance2" => Ok(Extension::KHRMaintenance2),
            "VK_KHR_get_surface_capabilities2" => Ok(Extension::KHRGetSurfaceCapabilities2),
            "VK_KHR_variable_pointers" => Ok(Extension::KHRVariablePointers),
            "VK_KHR_get_display_properties2" => Ok(Extension::KHRGetDisplayProperties2),
            "VK_KHR_dedicated_allocation" => Ok(Extension::KHRDedicatedAllocation),
            "VK_KHR_storage_buffer_storage_class" => Ok(Extension::KHRStorageBufferStorageClass),
            "VK_KHR_relaxed_block_layout" => Ok(Extension::KHRRelaxedBlockLayout),
            "VK_KHR_get_memory_requirements2" => Ok(Extension::KHRGetMemoryRequirements2),
            "VK_KHR_image_format_list" => Ok(Extension::KHRImageFormatList),
            "VK_KHR_sampler_ycbcr_conversion" => Ok(Extension::KHRSamplerYcbcrConversion),
            "VK_KHR_bind_memory2" => Ok(Extension::KHRBindMemory2),
            "VK_KHR_portability_subset" => Ok(Extension::KHRPortabilitySubset),
            "VK_KHR_maintenance3" => Ok(Extension::KHRMaintenance3),
            "VK_KHR_draw_indirect_count" => Ok(Extension::KHRDrawIndirectCount),
            "VK_KHR_shader_subgroup_extended_types" => {
                Ok(Extension::KHRShaderSubgroupExtendedTypes)
            }
            "VK_KHR_8bit_storage" => Ok(Extension::KHR8bitStorage),
            "VK_KHR_shader_atomic_int64" => Ok(Extension::KHRShaderAtomicInt64),
            "VK_KHR_shader_clock" => Ok(Extension::KHRShaderClock),
            "VK_KHR_video_decode_h265" => Ok(Extension::KHRVideoDecodeH265),
            "VK_KHR_global_priority" => Ok(Extension::KHRGlobalPriority),
            "VK_KHR_driver_properties" => Ok(Extension::KHRDriverProperties),
            "VK_KHR_shader_float_controls" => Ok(Extension::KHRShaderFloatControls),
            "VK_KHR_depth_stencil_resolve" => Ok(Extension::KHRDepthStencilResolve),
            "VK_KHR_swapchain_mutable_format" => Ok(Extension::KHRSwapchainMutableFormat),
            "VK_KHR_timeline_semaphore" => Ok(Extension::KHRTimelineSemaphore),
            "VK_KHR_vulkan_memory_model" => Ok(Extension::KHRVulkanMemoryModel),
            "VK_KHR_shader_terminate_invocation" => Ok(Extension::KHRShaderTerminateInvocation),
            "VK_KHR_fragment_shading_rate" => Ok(Extension::KHRFragmentShadingRate),
            "VK_KHR_spirv_1_4" => Ok(Extension::KHRSpirv14),
            "VK_KHR_surface_protected_capabilities" => {
                Ok(Extension::KHRSurfaceProtectedCapabilities)
            }
            "VK_KHR_separate_depth_stencil_layouts" => {
                Ok(Extension::KHRSeparateDepthStencilLayouts)
            }
            "VK_KHR_present_wait" => Ok(Extension::KHRPresentWait),
            "VK_KHR_uniform_buffer_standard_layout" => {
                Ok(Extension::KHRUniformBufferStandardLayout)
            }
            "VK_KHR_buffer_device_address" => Ok(Extension::KHRBufferDeviceAddress),
            "VK_KHR_deferred_host_operations" => Ok(Extension::KHRDeferredHostOperations),
            "VK_KHR_pipeline_executable_properties" => {
                Ok(Extension::KHRPipelineExecutableProperties)
            }
            "VK_KHR_map_memory2" => Ok(Extension::KHRMapMemory2),
            "VK_KHR_shader_integer_dot_product" => Ok(Extension::KHRShaderIntegerDotProduct),
            "VK_KHR_pipeline_library" => Ok(Extension::KHRPipelineLibrary),
            "VK_KHR_shader_non_semantic_info" => Ok(Extension::KHRShaderNonSemanticInfo),
            "VK_KHR_present_id" => Ok(Extension::KHRPresentId),
            "VK_KHR_video_encode_queue" => Ok(Extension::KHRVideoEncodeQueue),
            "VK_KHR_synchronization2" => Ok(Extension::KHRSynchronization2),
            "VK_KHR_fragment_shader_barycentric" => Ok(Extension::KHRFragmentShaderBarycentric),
            "VK_KHR_shader_subgroup_uniform_control_flow" => {
                Ok(Extension::KHRShaderSubgroupUniformControlFlow)
            }
            "VK_KHR_zero_initialize_workgroup_memory" => {
                Ok(Extension::KHRZeroInitializeWorkgroupMemory)
            }
            "VK_KHR_workgroup_memory_explicit_layout" => {
                Ok(Extension::KHRWorkgroupMemoryExplicitLayout)
            }
            "VK_KHR_copy_commands2" => Ok(Extension::KHRCopyCommands2),
            "VK_KHR_format_feature_flags2" => Ok(Extension::KHRFormatFeatureFlags2),
            "VK_KHR_ray_tracing_maintenance1" => Ok(Extension::KHRRayTracingMaintenance1),
            "VK_KHR_portability_enumeration" => Ok(Extension::KHRPortabilityEnumeration),
            "VK_KHR_maintenance4" => Ok(Extension::KHRMaintenance4),
            "VK_KHR_ray_tracing_position_fetch" => Ok(Extension::KHRRayTracingPositionFetch),
            "VK_ANDROID_native_buffer" => Ok(Extension::ANDROIDNativeBuffer),
            "VK_EXT_debug_report" => Ok(Extension::EXTDebugReport),
            "VK_NV_glsl_shader" => Ok(Extension::NVGlslShader),
            "VK_EXT_depth_range_unrestricted" => Ok(Extension::EXTDepthRangeUnrestricted),
            "VK_IMG_filter_cubic" => Ok(Extension::IMGFilterCubic),
            "VK_AMD_rasterization_order" => Ok(Extension::AMDRasterizationOrder),
            "VK_AMD_shader_trinary_minmax" => Ok(Extension::AMDShaderTrinaryMinmax),
            "VK_AMD_shader_explicit_vertex_parameter" => {
                Ok(Extension::AMDShaderExplicitVertexParameter)
            }
            "VK_EXT_debug_marker" => Ok(Extension::EXTDebugMarker),
            "VK_AMD_gcn_shader" => Ok(Extension::AMDGcnShader),
            "VK_NV_dedicated_allocation" => Ok(Extension::NVDedicatedAllocation),
            "VK_EXT_transform_feedback" => Ok(Extension::EXTTransformFeedback),
            "VK_NVX_binary_import" => Ok(Extension::NVXBinaryImport),
            "VK_NVX_image_view_handle" => Ok(Extension::NVXImageViewHandle),
            "VK_AMD_draw_indirect_count" => Ok(Extension::AMDDrawIndirectCount),
            "VK_AMD_negative_viewport_height" => Ok(Extension::AMDNegativeViewportHeight),
            "VK_AMD_gpu_shader_half_float" => Ok(Extension::AMDGpuShaderHalfFloat),
            "VK_AMD_shader_ballot" => Ok(Extension::AMDShaderBallot),
            "VK_EXT_video_encode_h264" => Ok(Extension::EXTVideoEncodeH264),
            "VK_EXT_video_encode_h265" => Ok(Extension::EXTVideoEncodeH265),
            "VK_AMD_texture_gather_bias_lod" => Ok(Extension::AMDTextureGatherBiasLod),
            "VK_AMD_shader_info" => Ok(Extension::AMDShaderInfo),
            "VK_AMD_shader_image_load_store_lod" => Ok(Extension::AMDShaderImageLoadStoreLod),
            "VK_GGP_stream_descriptor_surface" => Ok(Extension::GGPStreamDescriptorSurface),
            "VK_NV_corner_sampled_image" => Ok(Extension::NVCornerSampledImage),
            "VK_IMG_format_pvrtc" => Ok(Extension::IMGFormatPvrtc),
            "VK_NV_external_memory_capabilities" => Ok(Extension::NVExternalMemoryCapabilities),
            "VK_NV_external_memory" => Ok(Extension::NVExternalMemory),
            "VK_NV_external_memory_win32" => Ok(Extension::NVExternalMemoryWin32),
            "VK_NV_win32_keyed_mutex" => Ok(Extension::NVWin32KeyedMutex),
            "VK_EXT_validation_flags" => Ok(Extension::EXTValidationFlags),
            "VK_NN_vi_surface" => Ok(Extension::NNViSurface),
            "VK_EXT_shader_subgroup_ballot" => Ok(Extension::EXTShaderSubgroupBallot),
            "VK_EXT_shader_subgroup_vote" => Ok(Extension::EXTShaderSubgroupVote),
            "VK_EXT_texture_compression_astc_hdr" => Ok(Extension::EXTTextureCompressionAstcHdr),
            "VK_EXT_astc_decode_mode" => Ok(Extension::EXTAstcDecodeMode),
            "VK_EXT_pipeline_robustness" => Ok(Extension::EXTPipelineRobustness),
            "VK_EXT_conditional_rendering" => Ok(Extension::EXTConditionalRendering),
            "VK_NV_clip_space_w_scaling" => Ok(Extension::NVClipSpaceWScaling),
            "VK_EXT_direct_mode_display" => Ok(Extension::EXTDirectModeDisplay),
            "VK_EXT_acquire_xlib_display" => Ok(Extension::EXTAcquireXlibDisplay),
            "VK_EXT_display_surface_counter" => Ok(Extension::EXTDisplaySurfaceCounter),
            "VK_EXT_display_control" => Ok(Extension::EXTDisplayControl),
            "VK_GOOGLE_display_timing" => Ok(Extension::GOOGLEDisplayTiming),
            "VK_NV_sample_mask_override_coverage" => Ok(Extension::NVSampleMaskOverrideCoverage),
            "VK_NV_geometry_shader_passthrough" => Ok(Extension::NVGeometryShaderPassthrough),
            "VK_NV_viewport_array2" => Ok(Extension::NVViewportArray2),
            "VK_NVX_multiview_per_view_attributes" => Ok(Extension::NVXMultiviewPerViewAttributes),
            "VK_NV_viewport_swizzle" => Ok(Extension::NVViewportSwizzle),
            "VK_EXT_discard_rectangles" => Ok(Extension::EXTDiscardRectangles),
            "VK_EXT_conservative_rasterization" => Ok(Extension::EXTConservativeRasterization),
            "VK_EXT_depth_clip_enable" => Ok(Extension::EXTDepthClipEnable),
            "VK_EXT_swapchain_colorspace" => Ok(Extension::EXTSwapchainColorspace),
            "VK_EXT_hdr_metadata" => Ok(Extension::EXTHdrMetadata),
            "VK_MVK_ios_surface" => Ok(Extension::MVKIosSurface),
            "VK_MVK_macos_surface" => Ok(Extension::MVKMacosSurface),
            "VK_EXT_external_memory_dma_buf" => Ok(Extension::EXTExternalMemoryDmaBuf),
            "VK_EXT_queue_family_foreign" => Ok(Extension::EXTQueueFamilyForeign),
            "VK_EXT_debug_utils" => Ok(Extension::EXTDebugUtils),
            "VK_ANDROID_external_memory_android_hardware_buffer" => {
                Ok(Extension::ANDROIDExternalMemoryAndroidHardwareBuffer)
            }
            "VK_EXT_sampler_filter_minmax" => Ok(Extension::EXTSamplerFilterMinmax),
            "VK_AMD_gpu_shader_int16" => Ok(Extension::AMDGpuShaderInt16),
            "VK_AMD_mixed_attachment_samples" => Ok(Extension::AMDMixedAttachmentSamples),
            "VK_AMD_shader_fragment_mask" => Ok(Extension::AMDShaderFragmentMask),
            "VK_EXT_inline_uniform_block" => Ok(Extension::EXTInlineUniformBlock),
            "VK_EXT_shader_stencil_export" => Ok(Extension::EXTShaderStencilExport),
            "VK_EXT_sample_locations" => Ok(Extension::EXTSampleLocations),
            "VK_EXT_blend_operation_advanced" => Ok(Extension::EXTBlendOperationAdvanced),
            "VK_NV_fragment_coverage_to_color" => Ok(Extension::NVFragmentCoverageToColor),
            "VK_NV_framebuffer_mixed_samples" => Ok(Extension::NVFramebufferMixedSamples),
            "VK_NV_fill_rectangle" => Ok(Extension::NVFillRectangle),
            "VK_NV_shader_sm_builtins" => Ok(Extension::NVShaderSmBuiltins),
            "VK_EXT_post_depth_coverage" => Ok(Extension::EXTPostDepthCoverage),
            "VK_EXT_image_drm_format_modifier" => Ok(Extension::EXTImageDrmFormatModifier),
            "VK_EXT_validation_cache" => Ok(Extension::EXTValidationCache),
            "VK_EXT_descriptor_indexing" => Ok(Extension::EXTDescriptorIndexing),
            "VK_EXT_shader_viewport_index_layer" => Ok(Extension::EXTShaderViewportIndexLayer),
            "VK_NV_shading_rate_image" => Ok(Extension::NVShadingRateImage),
            "VK_NV_ray_tracing" => Ok(Extension::NVRayTracing),
            "VK_NV_representative_fragment_test" => Ok(Extension::NVRepresentativeFragmentTest),
            "VK_EXT_filter_cubic" => Ok(Extension::EXTFilterCubic),
            "VK_QCOM_render_pass_shader_resolve" => Ok(Extension::QCOMRenderPassShaderResolve),
            "VK_EXT_global_priority" => Ok(Extension::EXTGlobalPriority),
            "VK_EXT_external_memory_host" => Ok(Extension::EXTExternalMemoryHost),
            "VK_AMD_buffer_marker" => Ok(Extension::AMDBufferMarker),
            "VK_AMD_pipeline_compiler_control" => Ok(Extension::AMDPipelineCompilerControl),
            "VK_EXT_calibrated_timestamps" => Ok(Extension::EXTCalibratedTimestamps),
            "VK_AMD_shader_core_properties" => Ok(Extension::AMDShaderCoreProperties),
            "VK_AMD_memory_overallocation_behavior" => {
                Ok(Extension::AMDMemoryOverallocationBehavior)
            }
            "VK_EXT_vertex_attribute_divisor" => Ok(Extension::EXTVertexAttributeDivisor),
            "VK_GGP_frame_token" => Ok(Extension::GGPFrameToken),
            "VK_EXT_pipeline_creation_feedback" => Ok(Extension::EXTPipelineCreationFeedback),
            "VK_NV_shader_subgroup_partitioned" => Ok(Extension::NVShaderSubgroupPartitioned),
            "VK_NV_compute_shader_derivatives" => Ok(Extension::NVComputeShaderDerivatives),
            "VK_NV_mesh_shader" => Ok(Extension::NVMeshShader),
            "VK_NV_fragment_shader_barycentric" => Ok(Extension::NVFragmentShaderBarycentric),
            "VK_NV_shader_image_footprint" => Ok(Extension::NVShaderImageFootprint),
            "VK_NV_scissor_exclusive" => Ok(Extension::NVScissorExclusive),
            "VK_NV_device_diagnostic_checkpoints" => Ok(Extension::NVDeviceDiagnosticCheckpoints),
            "VK_INTEL_shader_integer_functions2" => Ok(Extension::INTELShaderIntegerFunctions2),
            "VK_INTEL_performance_query" => Ok(Extension::INTELPerformanceQuery),
            "VK_EXT_pci_bus_info" => Ok(Extension::EXTPciBusInfo),
            "VK_AMD_display_native_hdr" => Ok(Extension::AMDDisplayNativeHdr),
            "VK_FUCHSIA_imagepipe_surface" => Ok(Extension::FUCHSIAImagepipeSurface),
            "VK_EXT_metal_surface" => Ok(Extension::EXTMetalSurface),
            "VK_EXT_fragment_density_map" => Ok(Extension::EXTFragmentDensityMap),
            "VK_EXT_scalar_block_layout" => Ok(Extension::EXTScalarBlockLayout),
            "VK_GOOGLE_hlsl_functionality1" => Ok(Extension::GOOGLEHlslFunctionality1),
            "VK_GOOGLE_decorate_string" => Ok(Extension::GOOGLEDecorateString),
            "VK_EXT_subgroup_size_control" => Ok(Extension::EXTSubgroupSizeControl),
            "VK_AMD_shader_core_properties2" => Ok(Extension::AMDShaderCoreProperties2),
            "VK_AMD_device_coherent_memory" => Ok(Extension::AMDDeviceCoherentMemory),
            "VK_EXT_shader_image_atomic_int64" => Ok(Extension::EXTShaderImageAtomicInt64),
            "VK_EXT_memory_budget" => Ok(Extension::EXTMemoryBudget),
            "VK_EXT_memory_priority" => Ok(Extension::EXTMemoryPriority),
            "VK_NV_dedicated_allocation_image_aliasing" => {
                Ok(Extension::NVDedicatedAllocationImageAliasing)
            }
            "VK_EXT_buffer_device_address" => Ok(Extension::EXTBufferDeviceAddress),
            "VK_EXT_tooling_info" => Ok(Extension::EXTToolingInfo),
            "VK_EXT_separate_stencil_usage" => Ok(Extension::EXTSeparateStencilUsage),
            "VK_EXT_validation_features" => Ok(Extension::EXTValidationFeatures),
            "VK_NV_cooperative_matrix" => Ok(Extension::NVCooperativeMatrix),
            "VK_NV_coverage_reduction_mode" => Ok(Extension::NVCoverageReductionMode),
            "VK_EXT_fragment_shader_interlock" => Ok(Extension::EXTFragmentShaderInterlock),
            "VK_EXT_ycbcr_image_arrays" => Ok(Extension::EXTYcbcrImageArrays),
            "VK_EXT_provoking_vertex" => Ok(Extension::EXTProvokingVertex),
            "VK_EXT_full_screen_exclusive" => Ok(Extension::EXTFullScreenExclusive),
            "VK_EXT_headless_surface" => Ok(Extension::EXTHeadlessSurface),
            "VK_EXT_line_rasterization" => Ok(Extension::EXTLineRasterization),
            "VK_EXT_shader_atomic_float" => Ok(Extension::EXTShaderAtomicFloat),
            "VK_EXT_host_query_reset" => Ok(Extension::EXTHostQueryReset),
            "VK_EXT_index_type_uint8" => Ok(Extension::EXTIndexTypeUint8),
            "VK_EXT_extended_dynamic_state" => Ok(Extension::EXTExtendedDynamicState),
            "VK_EXT_shader_atomic_float2" => Ok(Extension::EXTShaderAtomicFloat2),
            "VK_EXT_surface_maintenance1" => Ok(Extension::EXTSurfaceMaintenance1),
            "VK_EXT_swapchain_maintenance1" => Ok(Extension::EXTSwapchainMaintenance1),
            "VK_EXT_shader_demote_to_helper_invocation" => {
                Ok(Extension::EXTShaderDemoteToHelperInvocation)
            }
            "VK_NV_device_generated_commands" => Ok(Extension::NVDeviceGeneratedCommands),
            "VK_NV_inherited_viewport_scissor" => Ok(Extension::NVInheritedViewportScissor),
            "VK_EXT_texel_buffer_alignment" => Ok(Extension::EXTTexelBufferAlignment),
            "VK_QCOM_render_pass_transform" => Ok(Extension::QCOMRenderPassTransform),
            "VK_EXT_device_memory_report" => Ok(Extension::EXTDeviceMemoryReport),
            "VK_EXT_acquire_drm_display" => Ok(Extension::EXTAcquireDrmDisplay),
            "VK_EXT_robustness2" => Ok(Extension::EXTRobustness2),
            "VK_EXT_custom_border_color" => Ok(Extension::EXTCustomBorderColor),
            "VK_GOOGLE_user_type" => Ok(Extension::GOOGLEUserType),
            "VK_NV_present_barrier" => Ok(Extension::NVPresentBarrier),
            "VK_EXT_private_data" => Ok(Extension::EXTPrivateData),
            "VK_EXT_pipeline_creation_cache_control" => {
                Ok(Extension::EXTPipelineCreationCacheControl)
            }
            "VK_NV_device_diagnostics_config" => Ok(Extension::NVDeviceDiagnosticsConfig),
            "VK_QCOM_render_pass_store_ops" => Ok(Extension::QCOMRenderPassStoreOps),
            "VK_NV_low_latency" => Ok(Extension::NVLowLatency),
            "VK_EXT_metal_objects" => Ok(Extension::EXTMetalObjects),
            "VK_EXT_descriptor_buffer" => Ok(Extension::EXTDescriptorBuffer),
            "VK_EXT_graphics_pipeline_library" => Ok(Extension::EXTGraphicsPipelineLibrary),
            "VK_AMD_shader_early_and_late_fragment_tests" => {
                Ok(Extension::AMDShaderEarlyAndLateFragmentTests)
            }
            "VK_NV_fragment_shading_rate_enums" => Ok(Extension::NVFragmentShadingRateEnums),
            "VK_NV_ray_tracing_motion_blur" => Ok(Extension::NVRayTracingMotionBlur),
            "VK_EXT_ycbcr_2plane_444_formats" => Ok(Extension::EXTYcbcr2plane444Formats),
            "VK_EXT_fragment_density_map2" => Ok(Extension::EXTFragmentDensityMap2),
            "VK_QCOM_rotated_copy_commands" => Ok(Extension::QCOMRotatedCopyCommands),
            "VK_EXT_image_robustness" => Ok(Extension::EXTImageRobustness),
            "VK_EXT_image_compression_control" => Ok(Extension::EXTImageCompressionControl),
            "VK_EXT_attachment_feedback_loop_layout" => {
                Ok(Extension::EXTAttachmentFeedbackLoopLayout)
            }
            "VK_EXT_4444_formats" => Ok(Extension::EXT4444Formats),
            "VK_EXT_device_fault" => Ok(Extension::EXTDeviceFault),
            "VK_ARM_rasterization_order_attachment_access" => {
                Ok(Extension::ARMRasterizationOrderAttachmentAccess)
            }
            "VK_EXT_rgba10x6_formats" => Ok(Extension::EXTRgba10x6Formats),
            "VK_NV_acquire_winrt_display" => Ok(Extension::NVAcquireWinrtDisplay),
            "VK_EXT_directfb_surface" => Ok(Extension::EXTDirectfbSurface),
            "VK_VALVE_mutable_descriptor_type" => Ok(Extension::VALVEMutableDescriptorType),
            "VK_EXT_vertex_input_dynamic_state" => Ok(Extension::EXTVertexInputDynamicState),
            "VK_EXT_physical_device_drm" => Ok(Extension::EXTPhysicalDeviceDrm),
            "VK_EXT_device_address_binding_report" => Ok(Extension::EXTDeviceAddressBindingReport),
            "VK_EXT_depth_clip_control" => Ok(Extension::EXTDepthClipControl),
            "VK_EXT_primitive_topology_list_restart" => {
                Ok(Extension::EXTPrimitiveTopologyListRestart)
            }
            "VK_FUCHSIA_external_memory" => Ok(Extension::FUCHSIAExternalMemory),
            "VK_FUCHSIA_external_semaphore" => Ok(Extension::FUCHSIAExternalSemaphore),
            "VK_FUCHSIA_buffer_collection" => Ok(Extension::FUCHSIABufferCollection),
            "VK_HUAWEI_subpass_shading" => Ok(Extension::HUAWEISubpassShading),
            "VK_HUAWEI_invocation_mask" => Ok(Extension::HUAWEIInvocationMask),
            "VK_NV_external_memory_rdma" => Ok(Extension::NVExternalMemoryRdma),
            "VK_EXT_pipeline_properties" => Ok(Extension::EXTPipelineProperties),
            "VK_EXT_multisampled_render_to_single_sampled" => {
                Ok(Extension::EXTMultisampledRenderToSingleSampled)
            }
            "VK_EXT_extended_dynamic_state2" => Ok(Extension::EXTExtendedDynamicState2),
            "VK_QNX_screen_surface" => Ok(Extension::QNXScreenSurface),
            "VK_EXT_color_write_enable" => Ok(Extension::EXTColorWriteEnable),
            "VK_EXT_primitives_generated_query" => Ok(Extension::EXTPrimitivesGeneratedQuery),
            "VK_EXT_global_priority_query" => Ok(Extension::EXTGlobalPriorityQuery),
            "VK_EXT_image_view_min_lod" => Ok(Extension::EXTImageViewMinLod),
            "VK_EXT_multi_draw" => Ok(Extension::EXTMultiDraw),
            "VK_EXT_image_2d_view_of_3d" => Ok(Extension::EXTImage2dViewOf3d),
            "VK_EXT_shader_tile_image" => Ok(Extension::EXTShaderTileImage),
            "VK_EXT_opacity_micromap" => Ok(Extension::EXTOpacityMicromap),
            "VK_NV_displacement_micromap" => Ok(Extension::NVDisplacementMicromap),
            "VK_EXT_load_store_op_none" => Ok(Extension::EXTLoadStoreOpNone),
            "VK_HUAWEI_cluster_culling_shader" => Ok(Extension::HUAWEIClusterCullingShader),
            "VK_EXT_border_color_swizzle" => Ok(Extension::EXTBorderColorSwizzle),
            "VK_EXT_pageable_device_local_memory" => Ok(Extension::EXTPageableDeviceLocalMemory),
            "VK_ARM_shader_core_properties" => Ok(Extension::ARMShaderCoreProperties),
            "VK_EXT_image_sliced_view_of_3d" => Ok(Extension::EXTImageSlicedViewOf3d),
            "VK_VALVE_descriptor_set_host_mapping" => Ok(Extension::VALVEDescriptorSetHostMapping),
            "VK_EXT_depth_clamp_zero_one" => Ok(Extension::EXTDepthClampZeroOne),
            "VK_EXT_non_seamless_cube_map" => Ok(Extension::EXTNonSeamlessCubeMap),
            "VK_QCOM_fragment_density_map_offset" => Ok(Extension::QCOMFragmentDensityMapOffset),
            "VK_NV_copy_memory_indirect" => Ok(Extension::NVCopyMemoryIndirect),
            "VK_NV_memory_decompression" => Ok(Extension::NVMemoryDecompression),
            "VK_NV_linear_color_attachment" => Ok(Extension::NVLinearColorAttachment),
            "VK_GOOGLE_surfaceless_query" => Ok(Extension::GOOGLESurfacelessQuery),
            "VK_EXT_image_compression_control_swapchain" => {
                Ok(Extension::EXTImageCompressionControlSwapchain)
            }
            "VK_QCOM_image_processing" => Ok(Extension::QCOMImageProcessing),
            "VK_EXT_extended_dynamic_state3" => Ok(Extension::EXTExtendedDynamicState3),
            "VK_EXT_subpass_merge_feedback" => Ok(Extension::EXTSubpassMergeFeedback),
            "VK_LUNARG_direct_driver_loading" => Ok(Extension::LUNARGDirectDriverLoading),
            "VK_EXT_shader_module_identifier" => Ok(Extension::EXTShaderModuleIdentifier),
            "VK_EXT_rasterization_order_attachment_access" => {
                Ok(Extension::EXTRasterizationOrderAttachmentAccess)
            }
            "VK_NV_optical_flow" => Ok(Extension::NVOpticalFlow),
            "VK_EXT_legacy_dithering" => Ok(Extension::EXTLegacyDithering),
            "VK_EXT_pipeline_protected_access" => Ok(Extension::EXTPipelineProtectedAccess),
            "VK_EXT_shader_object" => Ok(Extension::EXTShaderObject),
            "VK_QCOM_tile_properties" => Ok(Extension::QCOMTileProperties),
            "VK_SEC_amigo_profiling" => Ok(Extension::SECAmigoProfiling),
            "VK_QCOM_multiview_per_view_viewports" => Ok(Extension::QCOMMultiviewPerViewViewports),
            "VK_NV_ray_tracing_invocation_reorder" => Ok(Extension::NVRayTracingInvocationReorder),
            "VK_EXT_mutable_descriptor_type" => Ok(Extension::EXTMutableDescriptorType),
            "VK_ARM_shader_core_builtins" => Ok(Extension::ARMShaderCoreBuiltins),
            "VK_EXT_pipeline_library_group_handles" => {
                Ok(Extension::EXTPipelineLibraryGroupHandles)
            }
            "VK_QCOM_multiview_per_view_render_areas" => {
                Ok(Extension::QCOMMultiviewPerViewRenderAreas)
            }
            "VK_EXT_attachment_feedback_loop_dynamic_state" => {
                Ok(Extension::EXTAttachmentFeedbackLoopDynamicState)
            }
            "VK_KHR_acceleration_structure" => Ok(Extension::KHRAccelerationStructure),
            "VK_KHR_ray_tracing_pipeline" => Ok(Extension::KHRRayTracingPipeline),
            "VK_KHR_ray_query" => Ok(Extension::KHRRayQuery),
            "VK_EXT_mesh_shader" => Ok(Extension::EXTMeshShader),
            _ => Err(TryFromExtensionError::UnknownExtension(value.to_owned())),
        }
    }
}

impl From<Extension> for &'static str {
    fn from(value: Extension) -> &'static str {
        match value {
            Extension::KHRSurface => "VK_KHR_surface",
            Extension::KHRSwapchain => "VK_KHR_swapchain",
            Extension::KHRDisplay => "VK_KHR_display",
            Extension::KHRDisplaySwapchain => "VK_KHR_display_swapchain",
            Extension::KHRXlibSurface => "VK_KHR_xlib_surface",
            Extension::KHRXcbSurface => "VK_KHR_xcb_surface",
            Extension::KHRWaylandSurface => "VK_KHR_wayland_surface",
            Extension::KHRAndroidSurface => "VK_KHR_android_surface",
            Extension::KHRWin32Surface => "VK_KHR_win32_surface",
            Extension::KHRSamplerMirrorClampToEdge => "VK_KHR_sampler_mirror_clamp_to_edge",
            Extension::KHRVideoQueue => "VK_KHR_video_queue",
            Extension::KHRVideoDecodeQueue => "VK_KHR_video_decode_queue",
            Extension::KHRVideoDecodeH264 => "VK_KHR_video_decode_h264",
            Extension::KHRDynamicRendering => "VK_KHR_dynamic_rendering",
            Extension::KHRMultiview => "VK_KHR_multiview",
            Extension::KHRGetPhysicalDeviceProperties2 => "VK_KHR_get_physical_device_properties2",
            Extension::KHRDeviceGroup => "VK_KHR_device_group",
            Extension::KHRShaderDrawParameters => "VK_KHR_shader_draw_parameters",
            Extension::KHRMaintenance1 => "VK_KHR_maintenance1",
            Extension::KHRDeviceGroupCreation => "VK_KHR_device_group_creation",
            Extension::KHRExternalMemoryCapabilities => "VK_KHR_external_memory_capabilities",
            Extension::KHRExternalMemory => "VK_KHR_external_memory",
            Extension::KHRExternalMemoryWin32 => "VK_KHR_external_memory_win32",
            Extension::KHRExternalMemoryFd => "VK_KHR_external_memory_fd",
            Extension::KHRWin32KeyedMutex => "VK_KHR_win32_keyed_mutex",
            Extension::KHRExternalSemaphoreCapabilities => "VK_KHR_external_semaphore_capabilities",
            Extension::KHRExternalSemaphore => "VK_KHR_external_semaphore",
            Extension::KHRExternalSemaphoreWin32 => "VK_KHR_external_semaphore_win32",
            Extension::KHRExternalSemaphoreFd => "VK_KHR_external_semaphore_fd",
            Extension::KHRPushDescriptor => "VK_KHR_push_descriptor",
            Extension::KHRShaderFloat16Int8 => "VK_KHR_shader_float16_int8",
            Extension::KHR16bitStorage => "VK_KHR_16bit_storage",
            Extension::KHRIncrementalPresent => "VK_KHR_incremental_present",
            Extension::KHRDescriptorUpdateTemplate => "VK_KHR_descriptor_update_template",
            Extension::KHRImagelessFramebuffer => "VK_KHR_imageless_framebuffer",
            Extension::KHRCreateRenderpass2 => "VK_KHR_create_renderpass2",
            Extension::KHRSharedPresentableImage => "VK_KHR_shared_presentable_image",
            Extension::KHRExternalFenceCapabilities => "VK_KHR_external_fence_capabilities",
            Extension::KHRExternalFence => "VK_KHR_external_fence",
            Extension::KHRExternalFenceWin32 => "VK_KHR_external_fence_win32",
            Extension::KHRExternalFenceFd => "VK_KHR_external_fence_fd",
            Extension::KHRPerformanceQuery => "VK_KHR_performance_query",
            Extension::KHRMaintenance2 => "VK_KHR_maintenance2",
            Extension::KHRGetSurfaceCapabilities2 => "VK_KHR_get_surface_capabilities2",
            Extension::KHRVariablePointers => "VK_KHR_variable_pointers",
            Extension::KHRGetDisplayProperties2 => "VK_KHR_get_display_properties2",
            Extension::KHRDedicatedAllocation => "VK_KHR_dedicated_allocation",
            Extension::KHRStorageBufferStorageClass => "VK_KHR_storage_buffer_storage_class",
            Extension::KHRRelaxedBlockLayout => "VK_KHR_relaxed_block_layout",
            Extension::KHRGetMemoryRequirements2 => "VK_KHR_get_memory_requirements2",
            Extension::KHRImageFormatList => "VK_KHR_image_format_list",
            Extension::KHRSamplerYcbcrConversion => "VK_KHR_sampler_ycbcr_conversion",
            Extension::KHRBindMemory2 => "VK_KHR_bind_memory2",
            Extension::KHRPortabilitySubset => "VK_KHR_portability_subset",
            Extension::KHRMaintenance3 => "VK_KHR_maintenance3",
            Extension::KHRDrawIndirectCount => "VK_KHR_draw_indirect_count",
            Extension::KHRShaderSubgroupExtendedTypes => "VK_KHR_shader_subgroup_extended_types",
            Extension::KHR8bitStorage => "VK_KHR_8bit_storage",
            Extension::KHRShaderAtomicInt64 => "VK_KHR_shader_atomic_int64",
            Extension::KHRShaderClock => "VK_KHR_shader_clock",
            Extension::KHRVideoDecodeH265 => "VK_KHR_video_decode_h265",
            Extension::KHRGlobalPriority => "VK_KHR_global_priority",
            Extension::KHRDriverProperties => "VK_KHR_driver_properties",
            Extension::KHRShaderFloatControls => "VK_KHR_shader_float_controls",
            Extension::KHRDepthStencilResolve => "VK_KHR_depth_stencil_resolve",
            Extension::KHRSwapchainMutableFormat => "VK_KHR_swapchain_mutable_format",
            Extension::KHRTimelineSemaphore => "VK_KHR_timeline_semaphore",
            Extension::KHRVulkanMemoryModel => "VK_KHR_vulkan_memory_model",
            Extension::KHRShaderTerminateInvocation => "VK_KHR_shader_terminate_invocation",
            Extension::KHRFragmentShadingRate => "VK_KHR_fragment_shading_rate",
            Extension::KHRSpirv14 => "VK_KHR_spirv_1_4",
            Extension::KHRSurfaceProtectedCapabilities => "VK_KHR_surface_protected_capabilities",
            Extension::KHRSeparateDepthStencilLayouts => "VK_KHR_separate_depth_stencil_layouts",
            Extension::KHRPresentWait => "VK_KHR_present_wait",
            Extension::KHRUniformBufferStandardLayout => "VK_KHR_uniform_buffer_standard_layout",
            Extension::KHRBufferDeviceAddress => "VK_KHR_buffer_device_address",
            Extension::KHRDeferredHostOperations => "VK_KHR_deferred_host_operations",
            Extension::KHRPipelineExecutableProperties => "VK_KHR_pipeline_executable_properties",
            Extension::KHRMapMemory2 => "VK_KHR_map_memory2",
            Extension::KHRShaderIntegerDotProduct => "VK_KHR_shader_integer_dot_product",
            Extension::KHRPipelineLibrary => "VK_KHR_pipeline_library",
            Extension::KHRShaderNonSemanticInfo => "VK_KHR_shader_non_semantic_info",
            Extension::KHRPresentId => "VK_KHR_present_id",
            Extension::KHRVideoEncodeQueue => "VK_KHR_video_encode_queue",
            Extension::KHRSynchronization2 => "VK_KHR_synchronization2",
            Extension::KHRFragmentShaderBarycentric => "VK_KHR_fragment_shader_barycentric",
            Extension::KHRShaderSubgroupUniformControlFlow => {
                "VK_KHR_shader_subgroup_uniform_control_flow"
            }
            Extension::KHRZeroInitializeWorkgroupMemory => {
                "VK_KHR_zero_initialize_workgroup_memory"
            }
            Extension::KHRWorkgroupMemoryExplicitLayout => {
                "VK_KHR_workgroup_memory_explicit_layout"
            }
            Extension::KHRCopyCommands2 => "VK_KHR_copy_commands2",
            Extension::KHRFormatFeatureFlags2 => "VK_KHR_format_feature_flags2",
            Extension::KHRRayTracingMaintenance1 => "VK_KHR_ray_tracing_maintenance1",
            Extension::KHRPortabilityEnumeration => "VK_KHR_portability_enumeration",
            Extension::KHRMaintenance4 => "VK_KHR_maintenance4",
            Extension::KHRRayTracingPositionFetch => "VK_KHR_ray_tracing_position_fetch",
            Extension::ANDROIDNativeBuffer => "VK_ANDROID_native_buffer",
            Extension::EXTDebugReport => "VK_EXT_debug_report",
            Extension::NVGlslShader => "VK_NV_glsl_shader",
            Extension::EXTDepthRangeUnrestricted => "VK_EXT_depth_range_unrestricted",
            Extension::IMGFilterCubic => "VK_IMG_filter_cubic",
            Extension::AMDRasterizationOrder => "VK_AMD_rasterization_order",
            Extension::AMDShaderTrinaryMinmax => "VK_AMD_shader_trinary_minmax",
            Extension::AMDShaderExplicitVertexParameter => {
                "VK_AMD_shader_explicit_vertex_parameter"
            }
            Extension::EXTDebugMarker => "VK_EXT_debug_marker",
            Extension::AMDGcnShader => "VK_AMD_gcn_shader",
            Extension::NVDedicatedAllocation => "VK_NV_dedicated_allocation",
            Extension::EXTTransformFeedback => "VK_EXT_transform_feedback",
            Extension::NVXBinaryImport => "VK_NVX_binary_import",
            Extension::NVXImageViewHandle => "VK_NVX_image_view_handle",
            Extension::AMDDrawIndirectCount => "VK_AMD_draw_indirect_count",
            Extension::AMDNegativeViewportHeight => "VK_AMD_negative_viewport_height",
            Extension::AMDGpuShaderHalfFloat => "VK_AMD_gpu_shader_half_float",
            Extension::AMDShaderBallot => "VK_AMD_shader_ballot",
            Extension::EXTVideoEncodeH264 => "VK_EXT_video_encode_h264",
            Extension::EXTVideoEncodeH265 => "VK_EXT_video_encode_h265",
            Extension::AMDTextureGatherBiasLod => "VK_AMD_texture_gather_bias_lod",
            Extension::AMDShaderInfo => "VK_AMD_shader_info",
            Extension::AMDShaderImageLoadStoreLod => "VK_AMD_shader_image_load_store_lod",
            Extension::GGPStreamDescriptorSurface => "VK_GGP_stream_descriptor_surface",
            Extension::NVCornerSampledImage => "VK_NV_corner_sampled_image",
            Extension::IMGFormatPvrtc => "VK_IMG_format_pvrtc",
            Extension::NVExternalMemoryCapabilities => "VK_NV_external_memory_capabilities",
            Extension::NVExternalMemory => "VK_NV_external_memory",
            Extension::NVExternalMemoryWin32 => "VK_NV_external_memory_win32",
            Extension::NVWin32KeyedMutex => "VK_NV_win32_keyed_mutex",
            Extension::EXTValidationFlags => "VK_EXT_validation_flags",
            Extension::NNViSurface => "VK_NN_vi_surface",
            Extension::EXTShaderSubgroupBallot => "VK_EXT_shader_subgroup_ballot",
            Extension::EXTShaderSubgroupVote => "VK_EXT_shader_subgroup_vote",
            Extension::EXTTextureCompressionAstcHdr => "VK_EXT_texture_compression_astc_hdr",
            Extension::EXTAstcDecodeMode => "VK_EXT_astc_decode_mode",
            Extension::EXTPipelineRobustness => "VK_EXT_pipeline_robustness",
            Extension::EXTConditionalRendering => "VK_EXT_conditional_rendering",
            Extension::NVClipSpaceWScaling => "VK_NV_clip_space_w_scaling",
            Extension::EXTDirectModeDisplay => "VK_EXT_direct_mode_display",
            Extension::EXTAcquireXlibDisplay => "VK_EXT_acquire_xlib_display",
            Extension::EXTDisplaySurfaceCounter => "VK_EXT_display_surface_counter",
            Extension::EXTDisplayControl => "VK_EXT_display_control",
            Extension::GOOGLEDisplayTiming => "VK_GOOGLE_display_timing",
            Extension::NVSampleMaskOverrideCoverage => "VK_NV_sample_mask_override_coverage",
            Extension::NVGeometryShaderPassthrough => "VK_NV_geometry_shader_passthrough",
            Extension::NVViewportArray2 => "VK_NV_viewport_array2",
            Extension::NVXMultiviewPerViewAttributes => "VK_NVX_multiview_per_view_attributes",
            Extension::NVViewportSwizzle => "VK_NV_viewport_swizzle",
            Extension::EXTDiscardRectangles => "VK_EXT_discard_rectangles",
            Extension::EXTConservativeRasterization => "VK_EXT_conservative_rasterization",
            Extension::EXTDepthClipEnable => "VK_EXT_depth_clip_enable",
            Extension::EXTSwapchainColorspace => "VK_EXT_swapchain_colorspace",
            Extension::EXTHdrMetadata => "VK_EXT_hdr_metadata",
            Extension::MVKIosSurface => "VK_MVK_ios_surface",
            Extension::MVKMacosSurface => "VK_MVK_macos_surface",
            Extension::EXTExternalMemoryDmaBuf => "VK_EXT_external_memory_dma_buf",
            Extension::EXTQueueFamilyForeign => "VK_EXT_queue_family_foreign",
            Extension::EXTDebugUtils => "VK_EXT_debug_utils",
            Extension::ANDROIDExternalMemoryAndroidHardwareBuffer => {
                "VK_ANDROID_external_memory_android_hardware_buffer"
            }
            Extension::EXTSamplerFilterMinmax => "VK_EXT_sampler_filter_minmax",
            Extension::AMDGpuShaderInt16 => "VK_AMD_gpu_shader_int16",
            Extension::AMDMixedAttachmentSamples => "VK_AMD_mixed_attachment_samples",
            Extension::AMDShaderFragmentMask => "VK_AMD_shader_fragment_mask",
            Extension::EXTInlineUniformBlock => "VK_EXT_inline_uniform_block",
            Extension::EXTShaderStencilExport => "VK_EXT_shader_stencil_export",
            Extension::EXTSampleLocations => "VK_EXT_sample_locations",
            Extension::EXTBlendOperationAdvanced => "VK_EXT_blend_operation_advanced",
            Extension::NVFragmentCoverageToColor => "VK_NV_fragment_coverage_to_color",
            Extension::NVFramebufferMixedSamples => "VK_NV_framebuffer_mixed_samples",
            Extension::NVFillRectangle => "VK_NV_fill_rectangle",
            Extension::NVShaderSmBuiltins => "VK_NV_shader_sm_builtins",
            Extension::EXTPostDepthCoverage => "VK_EXT_post_depth_coverage",
            Extension::EXTImageDrmFormatModifier => "VK_EXT_image_drm_format_modifier",
            Extension::EXTValidationCache => "VK_EXT_validation_cache",
            Extension::EXTDescriptorIndexing => "VK_EXT_descriptor_indexing",
            Extension::EXTShaderViewportIndexLayer => "VK_EXT_shader_viewport_index_layer",
            Extension::NVShadingRateImage => "VK_NV_shading_rate_image",
            Extension::NVRayTracing => "VK_NV_ray_tracing",
            Extension::NVRepresentativeFragmentTest => "VK_NV_representative_fragment_test",
            Extension::EXTFilterCubic => "VK_EXT_filter_cubic",
            Extension::QCOMRenderPassShaderResolve => "VK_QCOM_render_pass_shader_resolve",
            Extension::EXTGlobalPriority => "VK_EXT_global_priority",
            Extension::EXTExternalMemoryHost => "VK_EXT_external_memory_host",
            Extension::AMDBufferMarker => "VK_AMD_buffer_marker",
            Extension::AMDPipelineCompilerControl => "VK_AMD_pipeline_compiler_control",
            Extension::EXTCalibratedTimestamps => "VK_EXT_calibrated_timestamps",
            Extension::AMDShaderCoreProperties => "VK_AMD_shader_core_properties",
            Extension::AMDMemoryOverallocationBehavior => "VK_AMD_memory_overallocation_behavior",
            Extension::EXTVertexAttributeDivisor => "VK_EXT_vertex_attribute_divisor",
            Extension::GGPFrameToken => "VK_GGP_frame_token",
            Extension::EXTPipelineCreationFeedback => "VK_EXT_pipeline_creation_feedback",
            Extension::NVShaderSubgroupPartitioned => "VK_NV_shader_subgroup_partitioned",
            Extension::NVComputeShaderDerivatives => "VK_NV_compute_shader_derivatives",
            Extension::NVMeshShader => "VK_NV_mesh_shader",
            Extension::NVFragmentShaderBarycentric => "VK_NV_fragment_shader_barycentric",
            Extension::NVShaderImageFootprint => "VK_NV_shader_image_footprint",
            Extension::NVScissorExclusive => "VK_NV_scissor_exclusive",
            Extension::NVDeviceDiagnosticCheckpoints => "VK_NV_device_diagnostic_checkpoints",
            Extension::INTELShaderIntegerFunctions2 => "VK_INTEL_shader_integer_functions2",
            Extension::INTELPerformanceQuery => "VK_INTEL_performance_query",
            Extension::EXTPciBusInfo => "VK_EXT_pci_bus_info",
            Extension::AMDDisplayNativeHdr => "VK_AMD_display_native_hdr",
            Extension::FUCHSIAImagepipeSurface => "VK_FUCHSIA_imagepipe_surface",
            Extension::EXTMetalSurface => "VK_EXT_metal_surface",
            Extension::EXTFragmentDensityMap => "VK_EXT_fragment_density_map",
            Extension::EXTScalarBlockLayout => "VK_EXT_scalar_block_layout",
            Extension::GOOGLEHlslFunctionality1 => "VK_GOOGLE_hlsl_functionality1",
            Extension::GOOGLEDecorateString => "VK_GOOGLE_decorate_string",
            Extension::EXTSubgroupSizeControl => "VK_EXT_subgroup_size_control",
            Extension::AMDShaderCoreProperties2 => "VK_AMD_shader_core_properties2",
            Extension::AMDDeviceCoherentMemory => "VK_AMD_device_coherent_memory",
            Extension::EXTShaderImageAtomicInt64 => "VK_EXT_shader_image_atomic_int64",
            Extension::EXTMemoryBudget => "VK_EXT_memory_budget",
            Extension::EXTMemoryPriority => "VK_EXT_memory_priority",
            Extension::NVDedicatedAllocationImageAliasing => {
                "VK_NV_dedicated_allocation_image_aliasing"
            }
            Extension::EXTBufferDeviceAddress => "VK_EXT_buffer_device_address",
            Extension::EXTToolingInfo => "VK_EXT_tooling_info",
            Extension::EXTSeparateStencilUsage => "VK_EXT_separate_stencil_usage",
            Extension::EXTValidationFeatures => "VK_EXT_validation_features",
            Extension::NVCooperativeMatrix => "VK_NV_cooperative_matrix",
            Extension::NVCoverageReductionMode => "VK_NV_coverage_reduction_mode",
            Extension::EXTFragmentShaderInterlock => "VK_EXT_fragment_shader_interlock",
            Extension::EXTYcbcrImageArrays => "VK_EXT_ycbcr_image_arrays",
            Extension::EXTProvokingVertex => "VK_EXT_provoking_vertex",
            Extension::EXTFullScreenExclusive => "VK_EXT_full_screen_exclusive",
            Extension::EXTHeadlessSurface => "VK_EXT_headless_surface",
            Extension::EXTLineRasterization => "VK_EXT_line_rasterization",
            Extension::EXTShaderAtomicFloat => "VK_EXT_shader_atomic_float",
            Extension::EXTHostQueryReset => "VK_EXT_host_query_reset",
            Extension::EXTIndexTypeUint8 => "VK_EXT_index_type_uint8",
            Extension::EXTExtendedDynamicState => "VK_EXT_extended_dynamic_state",
            Extension::EXTShaderAtomicFloat2 => "VK_EXT_shader_atomic_float2",
            Extension::EXTSurfaceMaintenance1 => "VK_EXT_surface_maintenance1",
            Extension::EXTSwapchainMaintenance1 => "VK_EXT_swapchain_maintenance1",
            Extension::EXTShaderDemoteToHelperInvocation => {
                "VK_EXT_shader_demote_to_helper_invocation"
            }
            Extension::NVDeviceGeneratedCommands => "VK_NV_device_generated_commands",
            Extension::NVInheritedViewportScissor => "VK_NV_inherited_viewport_scissor",
            Extension::EXTTexelBufferAlignment => "VK_EXT_texel_buffer_alignment",
            Extension::QCOMRenderPassTransform => "VK_QCOM_render_pass_transform",
            Extension::EXTDeviceMemoryReport => "VK_EXT_device_memory_report",
            Extension::EXTAcquireDrmDisplay => "VK_EXT_acquire_drm_display",
            Extension::EXTRobustness2 => "VK_EXT_robustness2",
            Extension::EXTCustomBorderColor => "VK_EXT_custom_border_color",
            Extension::GOOGLEUserType => "VK_GOOGLE_user_type",
            Extension::NVPresentBarrier => "VK_NV_present_barrier",
            Extension::EXTPrivateData => "VK_EXT_private_data",
            Extension::EXTPipelineCreationCacheControl => "VK_EXT_pipeline_creation_cache_control",
            Extension::NVDeviceDiagnosticsConfig => "VK_NV_device_diagnostics_config",
            Extension::QCOMRenderPassStoreOps => "VK_QCOM_render_pass_store_ops",
            Extension::NVLowLatency => "VK_NV_low_latency",
            Extension::EXTMetalObjects => "VK_EXT_metal_objects",
            Extension::EXTDescriptorBuffer => "VK_EXT_descriptor_buffer",
            Extension::EXTGraphicsPipelineLibrary => "VK_EXT_graphics_pipeline_library",
            Extension::AMDShaderEarlyAndLateFragmentTests => {
                "VK_AMD_shader_early_and_late_fragment_tests"
            }
            Extension::NVFragmentShadingRateEnums => "VK_NV_fragment_shading_rate_enums",
            Extension::NVRayTracingMotionBlur => "VK_NV_ray_tracing_motion_blur",
            Extension::EXTYcbcr2plane444Formats => "VK_EXT_ycbcr_2plane_444_formats",
            Extension::EXTFragmentDensityMap2 => "VK_EXT_fragment_density_map2",
            Extension::QCOMRotatedCopyCommands => "VK_QCOM_rotated_copy_commands",
            Extension::EXTImageRobustness => "VK_EXT_image_robustness",
            Extension::EXTImageCompressionControl => "VK_EXT_image_compression_control",
            Extension::EXTAttachmentFeedbackLoopLayout => "VK_EXT_attachment_feedback_loop_layout",
            Extension::EXT4444Formats => "VK_EXT_4444_formats",
            Extension::EXTDeviceFault => "VK_EXT_device_fault",
            Extension::ARMRasterizationOrderAttachmentAccess => {
                "VK_ARM_rasterization_order_attachment_access"
            }
            Extension::EXTRgba10x6Formats => "VK_EXT_rgba10x6_formats",
            Extension::NVAcquireWinrtDisplay => "VK_NV_acquire_winrt_display",
            Extension::EXTDirectfbSurface => "VK_EXT_directfb_surface",
            Extension::VALVEMutableDescriptorType => "VK_VALVE_mutable_descriptor_type",
            Extension::EXTVertexInputDynamicState => "VK_EXT_vertex_input_dynamic_state",
            Extension::EXTPhysicalDeviceDrm => "VK_EXT_physical_device_drm",
            Extension::EXTDeviceAddressBindingReport => "VK_EXT_device_address_binding_report",
            Extension::EXTDepthClipControl => "VK_EXT_depth_clip_control",
            Extension::EXTPrimitiveTopologyListRestart => "VK_EXT_primitive_topology_list_restart",
            Extension::FUCHSIAExternalMemory => "VK_FUCHSIA_external_memory",
            Extension::FUCHSIAExternalSemaphore => "VK_FUCHSIA_external_semaphore",
            Extension::FUCHSIABufferCollection => "VK_FUCHSIA_buffer_collection",
            Extension::HUAWEISubpassShading => "VK_HUAWEI_subpass_shading",
            Extension::HUAWEIInvocationMask => "VK_HUAWEI_invocation_mask",
            Extension::NVExternalMemoryRdma => "VK_NV_external_memory_rdma",
            Extension::EXTPipelineProperties => "VK_EXT_pipeline_properties",
            Extension::EXTMultisampledRenderToSingleSampled => {
                "VK_EXT_multisampled_render_to_single_sampled"
            }
            Extension::EXTExtendedDynamicState2 => "VK_EXT_extended_dynamic_state2",
            Extension::QNXScreenSurface => "VK_QNX_screen_surface",
            Extension::EXTColorWriteEnable => "VK_EXT_color_write_enable",
            Extension::EXTPrimitivesGeneratedQuery => "VK_EXT_primitives_generated_query",
            Extension::EXTGlobalPriorityQuery => "VK_EXT_global_priority_query",
            Extension::EXTImageViewMinLod => "VK_EXT_image_view_min_lod",
            Extension::EXTMultiDraw => "VK_EXT_multi_draw",
            Extension::EXTImage2dViewOf3d => "VK_EXT_image_2d_view_of_3d",
            Extension::EXTShaderTileImage => "VK_EXT_shader_tile_image",
            Extension::EXTOpacityMicromap => "VK_EXT_opacity_micromap",
            Extension::NVDisplacementMicromap => "VK_NV_displacement_micromap",
            Extension::EXTLoadStoreOpNone => "VK_EXT_load_store_op_none",
            Extension::HUAWEIClusterCullingShader => "VK_HUAWEI_cluster_culling_shader",
            Extension::EXTBorderColorSwizzle => "VK_EXT_border_color_swizzle",
            Extension::EXTPageableDeviceLocalMemory => "VK_EXT_pageable_device_local_memory",
            Extension::ARMShaderCoreProperties => "VK_ARM_shader_core_properties",
            Extension::EXTImageSlicedViewOf3d => "VK_EXT_image_sliced_view_of_3d",
            Extension::VALVEDescriptorSetHostMapping => "VK_VALVE_descriptor_set_host_mapping",
            Extension::EXTDepthClampZeroOne => "VK_EXT_depth_clamp_zero_one",
            Extension::EXTNonSeamlessCubeMap => "VK_EXT_non_seamless_cube_map",
            Extension::QCOMFragmentDensityMapOffset => "VK_QCOM_fragment_density_map_offset",
            Extension::NVCopyMemoryIndirect => "VK_NV_copy_memory_indirect",
            Extension::NVMemoryDecompression => "VK_NV_memory_decompression",
            Extension::NVLinearColorAttachment => "VK_NV_linear_color_attachment",
            Extension::GOOGLESurfacelessQuery => "VK_GOOGLE_surfaceless_query",
            Extension::EXTImageCompressionControlSwapchain => {
                "VK_EXT_image_compression_control_swapchain"
            }
            Extension::QCOMImageProcessing => "VK_QCOM_image_processing",
            Extension::EXTExtendedDynamicState3 => "VK_EXT_extended_dynamic_state3",
            Extension::EXTSubpassMergeFeedback => "VK_EXT_subpass_merge_feedback",
            Extension::LUNARGDirectDriverLoading => "VK_LUNARG_direct_driver_loading",
            Extension::EXTShaderModuleIdentifier => "VK_EXT_shader_module_identifier",
            Extension::EXTRasterizationOrderAttachmentAccess => {
                "VK_EXT_rasterization_order_attachment_access"
            }
            Extension::NVOpticalFlow => "VK_NV_optical_flow",
            Extension::EXTLegacyDithering => "VK_EXT_legacy_dithering",
            Extension::EXTPipelineProtectedAccess => "VK_EXT_pipeline_protected_access",
            Extension::EXTShaderObject => "VK_EXT_shader_object",
            Extension::QCOMTileProperties => "VK_QCOM_tile_properties",
            Extension::SECAmigoProfiling => "VK_SEC_amigo_profiling",
            Extension::QCOMMultiviewPerViewViewports => "VK_QCOM_multiview_per_view_viewports",
            Extension::NVRayTracingInvocationReorder => "VK_NV_ray_tracing_invocation_reorder",
            Extension::EXTMutableDescriptorType => "VK_EXT_mutable_descriptor_type",
            Extension::ARMShaderCoreBuiltins => "VK_ARM_shader_core_builtins",
            Extension::EXTPipelineLibraryGroupHandles => "VK_EXT_pipeline_library_group_handles",
            Extension::QCOMMultiviewPerViewRenderAreas => "VK_QCOM_multiview_per_view_render_areas",
            Extension::EXTAttachmentFeedbackLoopDynamicState => {
                "VK_EXT_attachment_feedback_loop_dynamic_state"
            }
            Extension::KHRAccelerationStructure => "VK_KHR_acceleration_structure",
            Extension::KHRRayTracingPipeline => "VK_KHR_ray_tracing_pipeline",
            Extension::KHRRayQuery => "VK_KHR_ray_query",
            Extension::EXTMeshShader => "VK_EXT_mesh_shader",
        }
    }
}

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
    android_native_buffer: Arc<ash::vk::AndroidNativeBufferFn>,
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
            android_native_buffer: Arc::new(ash::vk::AndroidNativeBufferFn::load(
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
    pub(crate) fn create_device_commands(
        &self,
        instance_info: &T::InstanceInfo,
        device_info: Option<&T::DeviceInfo>,
    ) -> Box<[VulkanCommand]> {
        let hooked_commands = self
            .layer_info
            .hooked_device_commands(instance_info, device_info)
            .collect::<HashSet<_>>();
        Box::new([
            VulkanCommand {
                name: "vkAcquireFullScreenExclusiveModeEXT",
                features: smallvec![Feature::Extension(Extension::EXTFullScreenExclusive)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::AcquireFullScreenExclusiveModeExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::acquire_full_screen_exclusive_mode_ext
                            as vk::PFN_vkAcquireFullScreenExclusiveModeEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkAcquireImageANDROID",
                features: smallvec![Feature::Extension(Extension::ANDROIDNativeBuffer)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::AcquireImageAndroid),
                proc: unsafe {
                    std::mem::transmute(
                        Self::acquire_image_android as vk::PFN_vkAcquireImageANDROID,
                    )
                },
            },
            VulkanCommand {
                name: "vkAcquireNextImage2KHR",
                features: smallvec![
                    Feature::Extension(Extension::KHRSwapchain),
                    Feature::Extension(Extension::KHRDeviceGroup)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::AcquireNextImage2Khr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::acquire_next_image2_khr as vk::PFN_vkAcquireNextImage2KHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkAcquireNextImageKHR",
                features: smallvec![Feature::Extension(Extension::KHRSwapchain)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::AcquireNextImageKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::acquire_next_image_khr as vk::PFN_vkAcquireNextImageKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkAcquirePerformanceConfigurationINTEL",
                features: smallvec![Feature::Extension(Extension::INTELPerformanceQuery)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::AcquirePerformanceConfigurationIntel),
                proc: unsafe {
                    std::mem::transmute(
                        Self::acquire_performance_configuration_intel
                            as vk::PFN_vkAcquirePerformanceConfigurationINTEL,
                    )
                },
            },
            VulkanCommand {
                name: "vkAcquireProfilingLockKHR",
                features: smallvec![Feature::Extension(Extension::KHRPerformanceQuery)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::AcquireProfilingLockKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::acquire_profiling_lock_khr as vk::PFN_vkAcquireProfilingLockKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkAllocateCommandBuffers",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::AllocateCommandBuffers),
                proc: unsafe {
                    std::mem::transmute(
                        Self::allocate_command_buffers as vk::PFN_vkAllocateCommandBuffers,
                    )
                },
            },
            VulkanCommand {
                name: "vkAllocateDescriptorSets",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::AllocateDescriptorSets),
                proc: unsafe {
                    std::mem::transmute(
                        Self::allocate_descriptor_sets as vk::PFN_vkAllocateDescriptorSets,
                    )
                },
            },
            VulkanCommand {
                name: "vkAllocateMemory",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::AllocateMemory),
                proc: unsafe {
                    std::mem::transmute(Self::allocate_memory as vk::PFN_vkAllocateMemory)
                },
            },
            VulkanCommand {
                name: "vkBeginCommandBuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::BeginCommandBuffer),
                proc: unsafe {
                    std::mem::transmute(Self::begin_command_buffer as vk::PFN_vkBeginCommandBuffer)
                },
            },
            VulkanCommand {
                name: "vkBindAccelerationStructureMemoryNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::BindAccelerationStructureMemoryNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::bind_acceleration_structure_memory_nv
                            as vk::PFN_vkBindAccelerationStructureMemoryNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkBindBufferMemory",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::BindBufferMemory),
                proc: unsafe {
                    std::mem::transmute(Self::bind_buffer_memory as vk::PFN_vkBindBufferMemory)
                },
            },
            VulkanCommand {
                name: "vkBindBufferMemory2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::BindBufferMemory2),
                proc: unsafe {
                    std::mem::transmute(Self::bind_buffer_memory2 as vk::PFN_vkBindBufferMemory2)
                },
            },
            VulkanCommand {
                name: "vkBindBufferMemory2KHR",
                features: smallvec![Feature::Extension(Extension::KHRBindMemory2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::BindBufferMemory2),
                proc: unsafe {
                    std::mem::transmute(Self::bind_buffer_memory2 as vk::PFN_vkBindBufferMemory2)
                },
            },
            VulkanCommand {
                name: "vkBindImageMemory",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::BindImageMemory),
                proc: unsafe {
                    std::mem::transmute(Self::bind_image_memory as vk::PFN_vkBindImageMemory)
                },
            },
            VulkanCommand {
                name: "vkBindImageMemory2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::BindImageMemory2),
                proc: unsafe {
                    std::mem::transmute(Self::bind_image_memory2 as vk::PFN_vkBindImageMemory2)
                },
            },
            VulkanCommand {
                name: "vkBindImageMemory2KHR",
                features: smallvec![Feature::Extension(Extension::KHRBindMemory2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::BindImageMemory2),
                proc: unsafe {
                    std::mem::transmute(Self::bind_image_memory2 as vk::PFN_vkBindImageMemory2)
                },
            },
            VulkanCommand {
                name: "vkBindOpticalFlowSessionImageNV",
                features: smallvec![Feature::Extension(Extension::NVOpticalFlow)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::BindOpticalFlowSessionImageNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::bind_optical_flow_session_image_nv
                            as vk::PFN_vkBindOpticalFlowSessionImageNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkBindVideoSessionMemoryKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::BindVideoSessionMemoryKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::bind_video_session_memory_khr as vk::PFN_vkBindVideoSessionMemoryKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkBuildMicromapsEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::BuildMicromapsExt),
                proc: unsafe {
                    std::mem::transmute(Self::build_micromaps_ext as vk::PFN_vkBuildMicromapsEXT)
                },
            },
            VulkanCommand {
                name: "vkCmdBeginConditionalRenderingEXT",
                features: smallvec![Feature::Extension(Extension::EXTConditionalRendering)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdBeginConditionalRenderingExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_begin_conditional_rendering_ext
                            as vk::PFN_vkCmdBeginConditionalRenderingEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBeginDebugUtilsLabelEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBeginDebugUtilsLabelExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_begin_debug_utils_label_ext
                            as vk::PFN_vkCmdBeginDebugUtilsLabelEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBeginQuery",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBeginQuery),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_begin_query as vk::PFN_vkCmdBeginQuery)
                },
            },
            VulkanCommand {
                name: "vkCmdBeginQueryIndexedEXT",
                features: smallvec![Feature::Extension(Extension::EXTTransformFeedback)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBeginQueryIndexedExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_begin_query_indexed_ext as vk::PFN_vkCmdBeginQueryIndexedEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBeginRenderPass",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBeginRenderPass),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_begin_render_pass as vk::PFN_vkCmdBeginRenderPass)
                },
            },
            VulkanCommand {
                name: "vkCmdBeginRenderPass2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBeginRenderPass2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_begin_render_pass2 as vk::PFN_vkCmdBeginRenderPass2,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBeginRenderPass2KHR",
                features: smallvec![Feature::Extension(Extension::KHRCreateRenderpass2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBeginRenderPass2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_begin_render_pass2 as vk::PFN_vkCmdBeginRenderPass2,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBeginRendering",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBeginRendering),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_begin_rendering as vk::PFN_vkCmdBeginRendering)
                },
            },
            VulkanCommand {
                name: "vkCmdBeginRenderingKHR",
                features: smallvec![Feature::Extension(Extension::KHRDynamicRendering)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBeginRendering),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_begin_rendering as vk::PFN_vkCmdBeginRendering)
                },
            },
            VulkanCommand {
                name: "vkCmdBeginTransformFeedbackEXT",
                features: smallvec![Feature::Extension(Extension::EXTTransformFeedback)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBeginTransformFeedbackExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_begin_transform_feedback_ext
                            as vk::PFN_vkCmdBeginTransformFeedbackEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBeginVideoCodingKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBeginVideoCodingKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_begin_video_coding_khr as vk::PFN_vkCmdBeginVideoCodingKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBindDescriptorBufferEmbeddedSamplersEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdBindDescriptorBufferEmbeddedSamplersExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_bind_descriptor_buffer_embedded_samplers_ext
                            as vk::PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBindDescriptorBuffersEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBindDescriptorBuffersExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_bind_descriptor_buffers_ext
                            as vk::PFN_vkCmdBindDescriptorBuffersEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBindDescriptorSets",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBindDescriptorSets),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_bind_descriptor_sets as vk::PFN_vkCmdBindDescriptorSets,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBindIndexBuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBindIndexBuffer),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_bind_index_buffer as vk::PFN_vkCmdBindIndexBuffer)
                },
            },
            VulkanCommand {
                name: "vkCmdBindInvocationMaskHUAWEI",
                features: smallvec![Feature::Extension(Extension::HUAWEIInvocationMask)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBindInvocationMaskHuawei),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_bind_invocation_mask_huawei
                            as vk::PFN_vkCmdBindInvocationMaskHUAWEI,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBindPipeline",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBindPipeline),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_bind_pipeline as vk::PFN_vkCmdBindPipeline)
                },
            },
            VulkanCommand {
                name: "vkCmdBindPipelineShaderGroupNV",
                features: smallvec![Feature::Extension(Extension::NVDeviceGeneratedCommands)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBindPipelineShaderGroupNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_bind_pipeline_shader_group_nv
                            as vk::PFN_vkCmdBindPipelineShaderGroupNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBindShadingRateImageNV",
                features: smallvec![Feature::Extension(Extension::NVShadingRateImage)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBindShadingRateImageNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_bind_shading_rate_image_nv as vk::PFN_vkCmdBindShadingRateImageNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBindTransformFeedbackBuffersEXT",
                features: smallvec![Feature::Extension(Extension::EXTTransformFeedback)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdBindTransformFeedbackBuffersExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_bind_transform_feedback_buffers_ext
                            as vk::PFN_vkCmdBindTransformFeedbackBuffersEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBindVertexBuffers",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBindVertexBuffers),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_bind_vertex_buffers as vk::PFN_vkCmdBindVertexBuffers,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBindVertexBuffers2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBindVertexBuffers2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_bind_vertex_buffers2 as vk::PFN_vkCmdBindVertexBuffers2,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBindVertexBuffers2EXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBindVertexBuffers2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_bind_vertex_buffers2 as vk::PFN_vkCmdBindVertexBuffers2,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBlitImage",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBlitImage),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_blit_image as vk::PFN_vkCmdBlitImage)
                },
            },
            VulkanCommand {
                name: "vkCmdBlitImage2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBlitImage2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_blit_image2 as vk::PFN_vkCmdBlitImage2)
                },
            },
            VulkanCommand {
                name: "vkCmdBlitImage2KHR",
                features: smallvec![Feature::Extension(Extension::KHRCopyCommands2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBlitImage2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_blit_image2 as vk::PFN_vkCmdBlitImage2)
                },
            },
            VulkanCommand {
                name: "vkCmdBuildAccelerationStructureNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdBuildAccelerationStructureNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_build_acceleration_structure_nv
                            as vk::PFN_vkCmdBuildAccelerationStructureNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdBuildMicromapsEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdBuildMicromapsExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_build_micromaps_ext as vk::PFN_vkCmdBuildMicromapsEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdClearAttachments",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdClearAttachments),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_clear_attachments as vk::PFN_vkCmdClearAttachments,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdClearColorImage",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdClearColorImage),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_clear_color_image as vk::PFN_vkCmdClearColorImage)
                },
            },
            VulkanCommand {
                name: "vkCmdClearDepthStencilImage",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdClearDepthStencilImage),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_clear_depth_stencil_image as vk::PFN_vkCmdClearDepthStencilImage,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdControlVideoCodingKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdControlVideoCodingKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_control_video_coding_khr as vk::PFN_vkCmdControlVideoCodingKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyAccelerationStructureKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdCopyAccelerationStructureKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_acceleration_structure_khr
                            as vk::PFN_vkCmdCopyAccelerationStructureKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyAccelerationStructureNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdCopyAccelerationStructureNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_acceleration_structure_nv
                            as vk::PFN_vkCmdCopyAccelerationStructureNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyAccelerationStructureToMemoryKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdCopyAccelerationStructureToMemoryKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_acceleration_structure_to_memory_khr
                            as vk::PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyBuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyBuffer),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_copy_buffer as vk::PFN_vkCmdCopyBuffer)
                },
            },
            VulkanCommand {
                name: "vkCmdCopyBuffer2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyBuffer2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_copy_buffer2 as vk::PFN_vkCmdCopyBuffer2)
                },
            },
            VulkanCommand {
                name: "vkCmdCopyBuffer2KHR",
                features: smallvec![Feature::Extension(Extension::KHRCopyCommands2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyBuffer2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_copy_buffer2 as vk::PFN_vkCmdCopyBuffer2)
                },
            },
            VulkanCommand {
                name: "vkCmdCopyBufferToImage",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyBufferToImage),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_buffer_to_image as vk::PFN_vkCmdCopyBufferToImage,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyBufferToImage2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyBufferToImage2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_buffer_to_image2 as vk::PFN_vkCmdCopyBufferToImage2,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyBufferToImage2KHR",
                features: smallvec![Feature::Extension(Extension::KHRCopyCommands2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyBufferToImage2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_buffer_to_image2 as vk::PFN_vkCmdCopyBufferToImage2,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyImage",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyImage),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_copy_image as vk::PFN_vkCmdCopyImage)
                },
            },
            VulkanCommand {
                name: "vkCmdCopyImage2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyImage2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_copy_image2 as vk::PFN_vkCmdCopyImage2)
                },
            },
            VulkanCommand {
                name: "vkCmdCopyImage2KHR",
                features: smallvec![Feature::Extension(Extension::KHRCopyCommands2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyImage2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_copy_image2 as vk::PFN_vkCmdCopyImage2)
                },
            },
            VulkanCommand {
                name: "vkCmdCopyImageToBuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyImageToBuffer),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_image_to_buffer as vk::PFN_vkCmdCopyImageToBuffer,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyImageToBuffer2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyImageToBuffer2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_image_to_buffer2 as vk::PFN_vkCmdCopyImageToBuffer2,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyImageToBuffer2KHR",
                features: smallvec![Feature::Extension(Extension::KHRCopyCommands2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyImageToBuffer2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_image_to_buffer2 as vk::PFN_vkCmdCopyImageToBuffer2,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyMemoryIndirectNV",
                features: smallvec![Feature::Extension(Extension::NVCopyMemoryIndirect)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyMemoryIndirectNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_memory_indirect_nv as vk::PFN_vkCmdCopyMemoryIndirectNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyMemoryToAccelerationStructureKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdCopyMemoryToAccelerationStructureKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_memory_to_acceleration_structure_khr
                            as vk::PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyMemoryToImageIndirectNV",
                features: smallvec![Feature::Extension(Extension::NVCopyMemoryIndirect)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdCopyMemoryToImageIndirectNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_memory_to_image_indirect_nv
                            as vk::PFN_vkCmdCopyMemoryToImageIndirectNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyMemoryToMicromapEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyMemoryToMicromapExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_memory_to_micromap_ext
                            as vk::PFN_vkCmdCopyMemoryToMicromapEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyMicromapEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyMicromapExt),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_copy_micromap_ext as vk::PFN_vkCmdCopyMicromapEXT)
                },
            },
            VulkanCommand {
                name: "vkCmdCopyMicromapToMemoryEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyMicromapToMemoryExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_micromap_to_memory_ext
                            as vk::PFN_vkCmdCopyMicromapToMemoryEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCopyQueryPoolResults",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCopyQueryPoolResults),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_copy_query_pool_results as vk::PFN_vkCmdCopyQueryPoolResults,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdCuLaunchKernelNVX",
                features: smallvec![Feature::Extension(Extension::NVXBinaryImport)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdCuLaunchKernelNvx),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_cu_launch_kernel_nvx as vk::PFN_vkCmdCuLaunchKernelNVX,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDebugMarkerBeginEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugMarker)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDebugMarkerBeginExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_debug_marker_begin_ext as vk::PFN_vkCmdDebugMarkerBeginEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDebugMarkerEndEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugMarker)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDebugMarkerEndExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_debug_marker_end_ext as vk::PFN_vkCmdDebugMarkerEndEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDebugMarkerInsertEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugMarker)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDebugMarkerInsertExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_debug_marker_insert_ext as vk::PFN_vkCmdDebugMarkerInsertEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDecodeVideoKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoDecodeQueue)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDecodeVideoKhr),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_decode_video_khr as vk::PFN_vkCmdDecodeVideoKHR)
                },
            },
            VulkanCommand {
                name: "vkCmdDecompressMemoryIndirectCountNV",
                features: smallvec![Feature::Extension(Extension::NVMemoryDecompression)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdDecompressMemoryIndirectCountNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_decompress_memory_indirect_count_nv
                            as vk::PFN_vkCmdDecompressMemoryIndirectCountNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDecompressMemoryNV",
                features: smallvec![Feature::Extension(Extension::NVMemoryDecompression)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDecompressMemoryNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_decompress_memory_nv as vk::PFN_vkCmdDecompressMemoryNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDispatch",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDispatch),
                proc: unsafe { std::mem::transmute(Self::cmd_dispatch as vk::PFN_vkCmdDispatch) },
            },
            VulkanCommand {
                name: "vkCmdDispatchBase",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDispatchBase),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_dispatch_base as vk::PFN_vkCmdDispatchBase)
                },
            },
            VulkanCommand {
                name: "vkCmdDispatchBaseKHR",
                features: smallvec![Feature::Extension(Extension::KHRDeviceGroup)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDispatchBase),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_dispatch_base as vk::PFN_vkCmdDispatchBase)
                },
            },
            VulkanCommand {
                name: "vkCmdDispatchIndirect",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDispatchIndirect),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_dispatch_indirect as vk::PFN_vkCmdDispatchIndirect,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDraw",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDraw),
                proc: unsafe { std::mem::transmute(Self::cmd_draw as vk::PFN_vkCmdDraw) },
            },
            VulkanCommand {
                name: "vkCmdDrawIndexed",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawIndexed),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_draw_indexed as vk::PFN_vkCmdDrawIndexed)
                },
            },
            VulkanCommand {
                name: "vkCmdDrawIndexedIndirect",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawIndexedIndirect),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_indexed_indirect as vk::PFN_vkCmdDrawIndexedIndirect,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawIndexedIndirectCount",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawIndexedIndirectCount),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_indexed_indirect_count
                            as vk::PFN_vkCmdDrawIndexedIndirectCount,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawIndexedIndirectCountAMD",
                features: smallvec![Feature::Extension(Extension::AMDDrawIndirectCount)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawIndexedIndirectCount),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_indexed_indirect_count
                            as vk::PFN_vkCmdDrawIndexedIndirectCount,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawIndexedIndirectCountKHR",
                features: smallvec![Feature::Extension(Extension::KHRDrawIndirectCount)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawIndexedIndirectCount),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_indexed_indirect_count
                            as vk::PFN_vkCmdDrawIndexedIndirectCount,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawIndirect",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawIndirect),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_draw_indirect as vk::PFN_vkCmdDrawIndirect)
                },
            },
            VulkanCommand {
                name: "vkCmdDrawIndirectByteCountEXT",
                features: smallvec![Feature::Extension(Extension::EXTTransformFeedback)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawIndirectByteCountExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_indirect_byte_count_ext
                            as vk::PFN_vkCmdDrawIndirectByteCountEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawIndirectCount",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawIndirectCount),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_indirect_count as vk::PFN_vkCmdDrawIndirectCount,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawIndirectCountAMD",
                features: smallvec![Feature::Extension(Extension::AMDDrawIndirectCount)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawIndirectCount),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_indirect_count as vk::PFN_vkCmdDrawIndirectCount,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawIndirectCountKHR",
                features: smallvec![Feature::Extension(Extension::KHRDrawIndirectCount)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawIndirectCount),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_indirect_count as vk::PFN_vkCmdDrawIndirectCount,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawMeshTasksEXT",
                features: smallvec![Feature::Extension(Extension::EXTMeshShader)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawMeshTasksExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_mesh_tasks_ext as vk::PFN_vkCmdDrawMeshTasksEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawMeshTasksIndirectCountEXT",
                features: smallvec![Feature::Extension(Extension::EXTMeshShader)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdDrawMeshTasksIndirectCountExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_mesh_tasks_indirect_count_ext
                            as vk::PFN_vkCmdDrawMeshTasksIndirectCountEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawMeshTasksIndirectCountNV",
                features: smallvec![Feature::Extension(Extension::NVMeshShader)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdDrawMeshTasksIndirectCountNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_mesh_tasks_indirect_count_nv
                            as vk::PFN_vkCmdDrawMeshTasksIndirectCountNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawMeshTasksIndirectEXT",
                features: smallvec![Feature::Extension(Extension::EXTMeshShader)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawMeshTasksIndirectExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_mesh_tasks_indirect_ext
                            as vk::PFN_vkCmdDrawMeshTasksIndirectEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawMeshTasksIndirectNV",
                features: smallvec![Feature::Extension(Extension::NVMeshShader)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawMeshTasksIndirectNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_mesh_tasks_indirect_nv
                            as vk::PFN_vkCmdDrawMeshTasksIndirectNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawMeshTasksNV",
                features: smallvec![Feature::Extension(Extension::NVMeshShader)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawMeshTasksNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_mesh_tasks_nv as vk::PFN_vkCmdDrawMeshTasksNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdDrawMultiEXT",
                features: smallvec![Feature::Extension(Extension::EXTMultiDraw)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawMultiExt),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_draw_multi_ext as vk::PFN_vkCmdDrawMultiEXT)
                },
            },
            VulkanCommand {
                name: "vkCmdDrawMultiIndexedEXT",
                features: smallvec![Feature::Extension(Extension::EXTMultiDraw)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdDrawMultiIndexedExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_draw_multi_indexed_ext as vk::PFN_vkCmdDrawMultiIndexedEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdEncodeVideoKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoEncodeQueue)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEncodeVideoKhr),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_encode_video_khr as vk::PFN_vkCmdEncodeVideoKHR)
                },
            },
            VulkanCommand {
                name: "vkCmdEndConditionalRenderingEXT",
                features: smallvec![Feature::Extension(Extension::EXTConditionalRendering)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdEndConditionalRenderingExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_end_conditional_rendering_ext
                            as vk::PFN_vkCmdEndConditionalRenderingEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdEndDebugUtilsLabelEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEndDebugUtilsLabelExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_end_debug_utils_label_ext as vk::PFN_vkCmdEndDebugUtilsLabelEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdEndQuery",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEndQuery),
                proc: unsafe { std::mem::transmute(Self::cmd_end_query as vk::PFN_vkCmdEndQuery) },
            },
            VulkanCommand {
                name: "vkCmdEndQueryIndexedEXT",
                features: smallvec![Feature::Extension(Extension::EXTTransformFeedback)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEndQueryIndexedExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_end_query_indexed_ext as vk::PFN_vkCmdEndQueryIndexedEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdEndRenderPass",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEndRenderPass),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_end_render_pass as vk::PFN_vkCmdEndRenderPass)
                },
            },
            VulkanCommand {
                name: "vkCmdEndRenderPass2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEndRenderPass2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_end_render_pass2 as vk::PFN_vkCmdEndRenderPass2)
                },
            },
            VulkanCommand {
                name: "vkCmdEndRenderPass2KHR",
                features: smallvec![Feature::Extension(Extension::KHRCreateRenderpass2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEndRenderPass2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_end_render_pass2 as vk::PFN_vkCmdEndRenderPass2)
                },
            },
            VulkanCommand {
                name: "vkCmdEndRendering",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEndRendering),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_end_rendering as vk::PFN_vkCmdEndRendering)
                },
            },
            VulkanCommand {
                name: "vkCmdEndRenderingKHR",
                features: smallvec![Feature::Extension(Extension::KHRDynamicRendering)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEndRendering),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_end_rendering as vk::PFN_vkCmdEndRendering)
                },
            },
            VulkanCommand {
                name: "vkCmdEndTransformFeedbackEXT",
                features: smallvec![Feature::Extension(Extension::EXTTransformFeedback)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEndTransformFeedbackExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_end_transform_feedback_ext
                            as vk::PFN_vkCmdEndTransformFeedbackEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdEndVideoCodingKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdEndVideoCodingKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_end_video_coding_khr as vk::PFN_vkCmdEndVideoCodingKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdExecuteCommands",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdExecuteCommands),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_execute_commands as vk::PFN_vkCmdExecuteCommands)
                },
            },
            VulkanCommand {
                name: "vkCmdExecuteGeneratedCommandsNV",
                features: smallvec![Feature::Extension(Extension::NVDeviceGeneratedCommands)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdExecuteGeneratedCommandsNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_execute_generated_commands_nv
                            as vk::PFN_vkCmdExecuteGeneratedCommandsNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdFillBuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdFillBuffer),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_fill_buffer as vk::PFN_vkCmdFillBuffer)
                },
            },
            VulkanCommand {
                name: "vkCmdInsertDebugUtilsLabelEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdInsertDebugUtilsLabelExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_insert_debug_utils_label_ext
                            as vk::PFN_vkCmdInsertDebugUtilsLabelEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdNextSubpass",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdNextSubpass),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_next_subpass as vk::PFN_vkCmdNextSubpass)
                },
            },
            VulkanCommand {
                name: "vkCmdNextSubpass2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdNextSubpass2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_next_subpass2 as vk::PFN_vkCmdNextSubpass2)
                },
            },
            VulkanCommand {
                name: "vkCmdNextSubpass2KHR",
                features: smallvec![Feature::Extension(Extension::KHRCreateRenderpass2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdNextSubpass2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_next_subpass2 as vk::PFN_vkCmdNextSubpass2)
                },
            },
            VulkanCommand {
                name: "vkCmdOpticalFlowExecuteNV",
                features: smallvec![Feature::Extension(Extension::NVOpticalFlow)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdOpticalFlowExecuteNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_optical_flow_execute_nv as vk::PFN_vkCmdOpticalFlowExecuteNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdPipelineBarrier",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdPipelineBarrier),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_pipeline_barrier as vk::PFN_vkCmdPipelineBarrier)
                },
            },
            VulkanCommand {
                name: "vkCmdPipelineBarrier2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdPipelineBarrier2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_pipeline_barrier2 as vk::PFN_vkCmdPipelineBarrier2,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdPipelineBarrier2KHR",
                features: smallvec![Feature::Extension(Extension::KHRSynchronization2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdPipelineBarrier2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_pipeline_barrier2 as vk::PFN_vkCmdPipelineBarrier2,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdPreprocessGeneratedCommandsNV",
                features: smallvec![Feature::Extension(Extension::NVDeviceGeneratedCommands)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdPreprocessGeneratedCommandsNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_preprocess_generated_commands_nv
                            as vk::PFN_vkCmdPreprocessGeneratedCommandsNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdPushConstants",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdPushConstants),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_push_constants as vk::PFN_vkCmdPushConstants)
                },
            },
            VulkanCommand {
                name: "vkCmdPushDescriptorSetKHR",
                features: smallvec![Feature::Extension(Extension::KHRPushDescriptor)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdPushDescriptorSetKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_push_descriptor_set_khr as vk::PFN_vkCmdPushDescriptorSetKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdPushDescriptorSetWithTemplateKHR",
                features: smallvec![
                    Feature::Extension(Extension::KHRPushDescriptor),
                    Feature::Extension(Extension::KHRDescriptorUpdateTemplate)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdPushDescriptorSetWithTemplateKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_push_descriptor_set_with_template_khr
                            as vk::PFN_vkCmdPushDescriptorSetWithTemplateKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdResetEvent",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdResetEvent),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_reset_event as vk::PFN_vkCmdResetEvent)
                },
            },
            VulkanCommand {
                name: "vkCmdResetEvent2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdResetEvent2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_reset_event2 as vk::PFN_vkCmdResetEvent2)
                },
            },
            VulkanCommand {
                name: "vkCmdResetEvent2KHR",
                features: smallvec![Feature::Extension(Extension::KHRSynchronization2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdResetEvent2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_reset_event2 as vk::PFN_vkCmdResetEvent2)
                },
            },
            VulkanCommand {
                name: "vkCmdResetQueryPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdResetQueryPool),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_reset_query_pool as vk::PFN_vkCmdResetQueryPool)
                },
            },
            VulkanCommand {
                name: "vkCmdResolveImage",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdResolveImage),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_resolve_image as vk::PFN_vkCmdResolveImage)
                },
            },
            VulkanCommand {
                name: "vkCmdResolveImage2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdResolveImage2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_resolve_image2 as vk::PFN_vkCmdResolveImage2)
                },
            },
            VulkanCommand {
                name: "vkCmdResolveImage2KHR",
                features: smallvec![Feature::Extension(Extension::KHRCopyCommands2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdResolveImage2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_resolve_image2 as vk::PFN_vkCmdResolveImage2)
                },
            },
            VulkanCommand {
                name: "vkCmdSetAlphaToCoverageEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetAlphaToCoverageEnableExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_alpha_to_coverage_enable_ext
                            as vk::PFN_vkCmdSetAlphaToCoverageEnableEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetAlphaToOneEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetAlphaToOneEnableExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_alpha_to_one_enable_ext
                            as vk::PFN_vkCmdSetAlphaToOneEnableEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetBlendConstants",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetBlendConstants),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_blend_constants as vk::PFN_vkCmdSetBlendConstants,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetCheckpointNV",
                features: smallvec![Feature::Extension(Extension::NVDeviceDiagnosticCheckpoints)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetCheckpointNv),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_checkpoint_nv as vk::PFN_vkCmdSetCheckpointNV)
                },
            },
            VulkanCommand {
                name: "vkCmdSetCoarseSampleOrderNV",
                features: smallvec![Feature::Extension(Extension::NVShadingRateImage)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetCoarseSampleOrderNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_coarse_sample_order_nv as vk::PFN_vkCmdSetCoarseSampleOrderNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetColorBlendAdvancedEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetColorBlendAdvancedExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_color_blend_advanced_ext
                            as vk::PFN_vkCmdSetColorBlendAdvancedEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetColorBlendEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetColorBlendEnableExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_color_blend_enable_ext as vk::PFN_vkCmdSetColorBlendEnableEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetColorBlendEquationEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetColorBlendEquationExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_color_blend_equation_ext
                            as vk::PFN_vkCmdSetColorBlendEquationEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetColorWriteEnableEXT",
                features: smallvec![Feature::Extension(Extension::EXTColorWriteEnable)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetColorWriteEnableExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_color_write_enable_ext as vk::PFN_vkCmdSetColorWriteEnableEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetColorWriteMaskEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetColorWriteMaskExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_color_write_mask_ext as vk::PFN_vkCmdSetColorWriteMaskEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetConservativeRasterizationModeEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetConservativeRasterizationModeExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_conservative_rasterization_mode_ext
                            as vk::PFN_vkCmdSetConservativeRasterizationModeEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetCoverageModulationModeNV",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetCoverageModulationModeNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_coverage_modulation_mode_nv
                            as vk::PFN_vkCmdSetCoverageModulationModeNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetCoverageModulationTableEnableNV",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetCoverageModulationTableEnableNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_coverage_modulation_table_enable_nv
                            as vk::PFN_vkCmdSetCoverageModulationTableEnableNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetCoverageModulationTableNV",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetCoverageModulationTableNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_coverage_modulation_table_nv
                            as vk::PFN_vkCmdSetCoverageModulationTableNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetCoverageReductionModeNV",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetCoverageReductionModeNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_coverage_reduction_mode_nv
                            as vk::PFN_vkCmdSetCoverageReductionModeNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetCoverageToColorEnableNV",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetCoverageToColorEnableNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_coverage_to_color_enable_nv
                            as vk::PFN_vkCmdSetCoverageToColorEnableNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetCoverageToColorLocationNV",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetCoverageToColorLocationNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_coverage_to_color_location_nv
                            as vk::PFN_vkCmdSetCoverageToColorLocationNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetCullMode",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetCullMode),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_cull_mode as vk::PFN_vkCmdSetCullMode)
                },
            },
            VulkanCommand {
                name: "vkCmdSetCullModeEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetCullMode),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_cull_mode as vk::PFN_vkCmdSetCullMode)
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthBias",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthBias),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_depth_bias as vk::PFN_vkCmdSetDepthBias)
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthBiasEnable",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthBiasEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_bias_enable as vk::PFN_vkCmdSetDepthBiasEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthBiasEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState2),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthBiasEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_bias_enable as vk::PFN_vkCmdSetDepthBiasEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthBounds",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthBounds),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_depth_bounds as vk::PFN_vkCmdSetDepthBounds)
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthBoundsTestEnable",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthBoundsTestEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_bounds_test_enable
                            as vk::PFN_vkCmdSetDepthBoundsTestEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthBoundsTestEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthBoundsTestEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_bounds_test_enable
                            as vk::PFN_vkCmdSetDepthBoundsTestEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthClampEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthClampEnableExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_clamp_enable_ext as vk::PFN_vkCmdSetDepthClampEnableEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthClipEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthClipEnableExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_clip_enable_ext as vk::PFN_vkCmdSetDepthClipEnableEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthClipNegativeOneToOneEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetDepthClipNegativeOneToOneExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_clip_negative_one_to_one_ext
                            as vk::PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthCompareOp",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthCompareOp),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_compare_op as vk::PFN_vkCmdSetDepthCompareOp,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthCompareOpEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthCompareOp),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_compare_op as vk::PFN_vkCmdSetDepthCompareOp,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthTestEnable",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthTestEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_test_enable as vk::PFN_vkCmdSetDepthTestEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthTestEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthTestEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_test_enable as vk::PFN_vkCmdSetDepthTestEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthWriteEnable",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthWriteEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_write_enable as vk::PFN_vkCmdSetDepthWriteEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDepthWriteEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDepthWriteEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_depth_write_enable as vk::PFN_vkCmdSetDepthWriteEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDescriptorBufferOffsetsEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetDescriptorBufferOffsetsExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_descriptor_buffer_offsets_ext
                            as vk::PFN_vkCmdSetDescriptorBufferOffsetsEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetDeviceMask",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDeviceMask),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_device_mask as vk::PFN_vkCmdSetDeviceMask)
                },
            },
            VulkanCommand {
                name: "vkCmdSetDeviceMaskKHR",
                features: smallvec![Feature::Extension(Extension::KHRDeviceGroup)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDeviceMask),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_device_mask as vk::PFN_vkCmdSetDeviceMask)
                },
            },
            VulkanCommand {
                name: "vkCmdSetDiscardRectangleEXT",
                features: smallvec![Feature::Extension(Extension::EXTDiscardRectangles)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetDiscardRectangleExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_discard_rectangle_ext as vk::PFN_vkCmdSetDiscardRectangleEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetEvent",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetEvent),
                proc: unsafe { std::mem::transmute(Self::cmd_set_event as vk::PFN_vkCmdSetEvent) },
            },
            VulkanCommand {
                name: "vkCmdSetEvent2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetEvent2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_event2 as vk::PFN_vkCmdSetEvent2)
                },
            },
            VulkanCommand {
                name: "vkCmdSetEvent2KHR",
                features: smallvec![Feature::Extension(Extension::KHRSynchronization2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetEvent2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_event2 as vk::PFN_vkCmdSetEvent2)
                },
            },
            VulkanCommand {
                name: "vkCmdSetExclusiveScissorNV",
                features: smallvec![Feature::Extension(Extension::NVScissorExclusive)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetExclusiveScissorNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_exclusive_scissor_nv as vk::PFN_vkCmdSetExclusiveScissorNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetExtraPrimitiveOverestimationSizeEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetExtraPrimitiveOverestimationSizeExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_extra_primitive_overestimation_size_ext
                            as vk::PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetFragmentShadingRateEnumNV",
                features: smallvec![Feature::Extension(Extension::NVFragmentShadingRateEnums)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetFragmentShadingRateEnumNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_fragment_shading_rate_enum_nv
                            as vk::PFN_vkCmdSetFragmentShadingRateEnumNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetFragmentShadingRateKHR",
                features: smallvec![Feature::Extension(Extension::KHRFragmentShadingRate)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetFragmentShadingRateKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_fragment_shading_rate_khr
                            as vk::PFN_vkCmdSetFragmentShadingRateKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetFrontFace",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetFrontFace),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_front_face as vk::PFN_vkCmdSetFrontFace)
                },
            },
            VulkanCommand {
                name: "vkCmdSetFrontFaceEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetFrontFace),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_front_face as vk::PFN_vkCmdSetFrontFace)
                },
            },
            VulkanCommand {
                name: "vkCmdSetLineRasterizationModeEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetLineRasterizationModeExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_line_rasterization_mode_ext
                            as vk::PFN_vkCmdSetLineRasterizationModeEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetLineStippleEXT",
                features: smallvec![Feature::Extension(Extension::EXTLineRasterization)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetLineStippleExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_line_stipple_ext as vk::PFN_vkCmdSetLineStippleEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetLineStippleEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetLineStippleEnableExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_line_stipple_enable_ext
                            as vk::PFN_vkCmdSetLineStippleEnableEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetLineWidth",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetLineWidth),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_line_width as vk::PFN_vkCmdSetLineWidth)
                },
            },
            VulkanCommand {
                name: "vkCmdSetLogicOpEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState2),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetLogicOpExt),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_logic_op_ext as vk::PFN_vkCmdSetLogicOpEXT)
                },
            },
            VulkanCommand {
                name: "vkCmdSetLogicOpEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetLogicOpEnableExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_logic_op_enable_ext as vk::PFN_vkCmdSetLogicOpEnableEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetPatchControlPointsEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState2),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetPatchControlPointsExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_patch_control_points_ext
                            as vk::PFN_vkCmdSetPatchControlPointsEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetPerformanceMarkerINTEL",
                features: smallvec![Feature::Extension(Extension::INTELPerformanceQuery)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetPerformanceMarkerIntel),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_performance_marker_intel
                            as vk::PFN_vkCmdSetPerformanceMarkerINTEL,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetPerformanceOverrideINTEL",
                features: smallvec![Feature::Extension(Extension::INTELPerformanceQuery)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetPerformanceOverrideIntel),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_performance_override_intel
                            as vk::PFN_vkCmdSetPerformanceOverrideINTEL,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetPerformanceStreamMarkerINTEL",
                features: smallvec![Feature::Extension(Extension::INTELPerformanceQuery)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetPerformanceStreamMarkerIntel),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_performance_stream_marker_intel
                            as vk::PFN_vkCmdSetPerformanceStreamMarkerINTEL,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetPolygonModeEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetPolygonModeExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_polygon_mode_ext as vk::PFN_vkCmdSetPolygonModeEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetPrimitiveRestartEnable",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetPrimitiveRestartEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_primitive_restart_enable
                            as vk::PFN_vkCmdSetPrimitiveRestartEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetPrimitiveRestartEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState2),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetPrimitiveRestartEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_primitive_restart_enable
                            as vk::PFN_vkCmdSetPrimitiveRestartEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetPrimitiveTopology",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetPrimitiveTopology),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_primitive_topology as vk::PFN_vkCmdSetPrimitiveTopology,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetPrimitiveTopologyEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetPrimitiveTopology),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_primitive_topology as vk::PFN_vkCmdSetPrimitiveTopology,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetProvokingVertexModeEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetProvokingVertexModeExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_provoking_vertex_mode_ext
                            as vk::PFN_vkCmdSetProvokingVertexModeEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetRasterizationSamplesEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetRasterizationSamplesExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_rasterization_samples_ext
                            as vk::PFN_vkCmdSetRasterizationSamplesEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetRasterizationStreamEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetRasterizationStreamExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_rasterization_stream_ext
                            as vk::PFN_vkCmdSetRasterizationStreamEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetRasterizerDiscardEnable",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetRasterizerDiscardEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_rasterizer_discard_enable
                            as vk::PFN_vkCmdSetRasterizerDiscardEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetRasterizerDiscardEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState2),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetRasterizerDiscardEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_rasterizer_discard_enable
                            as vk::PFN_vkCmdSetRasterizerDiscardEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetRayTracingPipelineStackSizeKHR",
                features: smallvec![Feature::Extension(Extension::KHRRayTracingPipeline)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetRayTracingPipelineStackSizeKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_ray_tracing_pipeline_stack_size_khr
                            as vk::PFN_vkCmdSetRayTracingPipelineStackSizeKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetRepresentativeFragmentTestEnableNV",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetRepresentativeFragmentTestEnableNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_representative_fragment_test_enable_nv
                            as vk::PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetSampleLocationsEXT",
                features: smallvec![Feature::Extension(Extension::EXTSampleLocations)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetSampleLocationsExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_sample_locations_ext as vk::PFN_vkCmdSetSampleLocationsEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetSampleLocationsEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetSampleLocationsEnableExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_sample_locations_enable_ext
                            as vk::PFN_vkCmdSetSampleLocationsEnableEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetSampleMaskEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetSampleMaskExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_sample_mask_ext as vk::PFN_vkCmdSetSampleMaskEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetScissor",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetScissor),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_scissor as vk::PFN_vkCmdSetScissor)
                },
            },
            VulkanCommand {
                name: "vkCmdSetScissorWithCount",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetScissorWithCount),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_scissor_with_count as vk::PFN_vkCmdSetScissorWithCount,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetScissorWithCountEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetScissorWithCount),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_scissor_with_count as vk::PFN_vkCmdSetScissorWithCount,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetShadingRateImageEnableNV",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetShadingRateImageEnableNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_shading_rate_image_enable_nv
                            as vk::PFN_vkCmdSetShadingRateImageEnableNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetStencilCompareMask",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetStencilCompareMask),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_stencil_compare_mask as vk::PFN_vkCmdSetStencilCompareMask,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetStencilOp",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetStencilOp),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_stencil_op as vk::PFN_vkCmdSetStencilOp)
                },
            },
            VulkanCommand {
                name: "vkCmdSetStencilOpEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetStencilOp),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_stencil_op as vk::PFN_vkCmdSetStencilOp)
                },
            },
            VulkanCommand {
                name: "vkCmdSetStencilReference",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetStencilReference),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_stencil_reference as vk::PFN_vkCmdSetStencilReference,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetStencilTestEnable",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetStencilTestEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_stencil_test_enable as vk::PFN_vkCmdSetStencilTestEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetStencilTestEnableEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetStencilTestEnable),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_stencil_test_enable as vk::PFN_vkCmdSetStencilTestEnable,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetStencilWriteMask",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetStencilWriteMask),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_stencil_write_mask as vk::PFN_vkCmdSetStencilWriteMask,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetTessellationDomainOriginEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetTessellationDomainOriginExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_tessellation_domain_origin_ext
                            as vk::PFN_vkCmdSetTessellationDomainOriginEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetVertexInputEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTVertexInputDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetVertexInputExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_vertex_input_ext as vk::PFN_vkCmdSetVertexInputEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetViewport",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetViewport),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_set_viewport as vk::PFN_vkCmdSetViewport)
                },
            },
            VulkanCommand {
                name: "vkCmdSetViewportShadingRatePaletteNV",
                features: smallvec![Feature::Extension(Extension::NVShadingRateImage)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetViewportShadingRatePaletteNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_viewport_shading_rate_palette_nv
                            as vk::PFN_vkCmdSetViewportShadingRatePaletteNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetViewportSwizzleNV",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetViewportSwizzleNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_viewport_swizzle_nv as vk::PFN_vkCmdSetViewportSwizzleNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetViewportWScalingEnableNV",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState3),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdSetViewportWScalingEnableNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_viewport_w_scaling_enable_nv
                            as vk::PFN_vkCmdSetViewportWScalingEnableNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetViewportWScalingNV",
                features: smallvec![Feature::Extension(Extension::NVClipSpaceWScaling)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetViewportWScalingNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_viewport_w_scaling_nv as vk::PFN_vkCmdSetViewportWScalingNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetViewportWithCount",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetViewportWithCount),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_viewport_with_count as vk::PFN_vkCmdSetViewportWithCount,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSetViewportWithCountEXT",
                features: smallvec![
                    Feature::Extension(Extension::EXTExtendedDynamicState),
                    Feature::Extension(Extension::EXTShaderObject)
                ],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSetViewportWithCount),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_set_viewport_with_count as vk::PFN_vkCmdSetViewportWithCount,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdSubpassShadingHUAWEI",
                features: smallvec![Feature::Extension(Extension::HUAWEISubpassShading)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdSubpassShadingHuawei),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_subpass_shading_huawei as vk::PFN_vkCmdSubpassShadingHUAWEI,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdTraceRaysIndirect2KHR",
                features: smallvec![Feature::Extension(Extension::KHRRayTracingMaintenance1)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdTraceRaysIndirect2Khr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_trace_rays_indirect2_khr as vk::PFN_vkCmdTraceRaysIndirect2KHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdTraceRaysIndirectKHR",
                features: smallvec![Feature::Extension(Extension::KHRRayTracingPipeline)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdTraceRaysIndirectKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_trace_rays_indirect_khr as vk::PFN_vkCmdTraceRaysIndirectKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdTraceRaysKHR",
                features: smallvec![Feature::Extension(Extension::KHRRayTracingPipeline)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdTraceRaysKhr),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_trace_rays_khr as vk::PFN_vkCmdTraceRaysKHR)
                },
            },
            VulkanCommand {
                name: "vkCmdTraceRaysNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdTraceRaysNv),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_trace_rays_nv as vk::PFN_vkCmdTraceRaysNV)
                },
            },
            VulkanCommand {
                name: "vkCmdUpdateBuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdUpdateBuffer),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_update_buffer as vk::PFN_vkCmdUpdateBuffer)
                },
            },
            VulkanCommand {
                name: "vkCmdWaitEvents",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdWaitEvents),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_wait_events as vk::PFN_vkCmdWaitEvents)
                },
            },
            VulkanCommand {
                name: "vkCmdWaitEvents2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdWaitEvents2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_wait_events2 as vk::PFN_vkCmdWaitEvents2)
                },
            },
            VulkanCommand {
                name: "vkCmdWaitEvents2KHR",
                features: smallvec![Feature::Extension(Extension::KHRSynchronization2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdWaitEvents2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_wait_events2 as vk::PFN_vkCmdWaitEvents2)
                },
            },
            VulkanCommand {
                name: "vkCmdWriteAccelerationStructuresPropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdWriteAccelerationStructuresPropertiesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_write_acceleration_structures_properties_khr
                            as vk::PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdWriteAccelerationStructuresPropertiesNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdWriteAccelerationStructuresPropertiesNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_write_acceleration_structures_properties_nv
                            as vk::PFN_vkCmdWriteAccelerationStructuresPropertiesNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdWriteBufferMarker2AMD",
                features: smallvec![Feature::Extension(Extension::KHRSynchronization2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdWriteBufferMarker2Amd),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_write_buffer_marker2_amd as vk::PFN_vkCmdWriteBufferMarker2AMD,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdWriteBufferMarkerAMD",
                features: smallvec![Feature::Extension(Extension::AMDBufferMarker)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdWriteBufferMarkerAmd),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_write_buffer_marker_amd as vk::PFN_vkCmdWriteBufferMarkerAMD,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdWriteMicromapsPropertiesEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CmdWriteMicromapsPropertiesExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::cmd_write_micromaps_properties_ext
                            as vk::PFN_vkCmdWriteMicromapsPropertiesEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCmdWriteTimestamp",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdWriteTimestamp),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_write_timestamp as vk::PFN_vkCmdWriteTimestamp)
                },
            },
            VulkanCommand {
                name: "vkCmdWriteTimestamp2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdWriteTimestamp2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_write_timestamp2 as vk::PFN_vkCmdWriteTimestamp2)
                },
            },
            VulkanCommand {
                name: "vkCmdWriteTimestamp2KHR",
                features: smallvec![Feature::Extension(Extension::KHRSynchronization2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CmdWriteTimestamp2),
                proc: unsafe {
                    std::mem::transmute(Self::cmd_write_timestamp2 as vk::PFN_vkCmdWriteTimestamp2)
                },
            },
            VulkanCommand {
                name: "vkCompileDeferredNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CompileDeferredNv),
                proc: unsafe {
                    std::mem::transmute(Self::compile_deferred_nv as vk::PFN_vkCompileDeferredNV)
                },
            },
            VulkanCommand {
                name: "vkCopyAccelerationStructureKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CopyAccelerationStructureKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::copy_acceleration_structure_khr
                            as vk::PFN_vkCopyAccelerationStructureKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCopyAccelerationStructureToMemoryKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CopyAccelerationStructureToMemoryKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::copy_acceleration_structure_to_memory_khr
                            as vk::PFN_vkCopyAccelerationStructureToMemoryKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCopyMemoryToAccelerationStructureKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CopyMemoryToAccelerationStructureKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::copy_memory_to_acceleration_structure_khr
                            as vk::PFN_vkCopyMemoryToAccelerationStructureKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCopyMemoryToMicromapEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CopyMemoryToMicromapExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::copy_memory_to_micromap_ext as vk::PFN_vkCopyMemoryToMicromapEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCopyMicromapEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CopyMicromapExt),
                proc: unsafe {
                    std::mem::transmute(Self::copy_micromap_ext as vk::PFN_vkCopyMicromapEXT)
                },
            },
            VulkanCommand {
                name: "vkCopyMicromapToMemoryEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CopyMicromapToMemoryExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::copy_micromap_to_memory_ext as vk::PFN_vkCopyMicromapToMemoryEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateAccelerationStructureKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CreateAccelerationStructureKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_acceleration_structure_khr
                            as vk::PFN_vkCreateAccelerationStructureKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateAccelerationStructureNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CreateAccelerationStructureNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_acceleration_structure_nv
                            as vk::PFN_vkCreateAccelerationStructureNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateBuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateBuffer),
                proc: unsafe { std::mem::transmute(Self::create_buffer as vk::PFN_vkCreateBuffer) },
            },
            VulkanCommand {
                name: "vkCreateBufferCollectionFUCHSIA",
                features: smallvec![Feature::Extension(Extension::FUCHSIABufferCollection)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CreateBufferCollectionFuchsia),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_buffer_collection_fuchsia
                            as vk::PFN_vkCreateBufferCollectionFUCHSIA,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateBufferView",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateBufferView),
                proc: unsafe {
                    std::mem::transmute(Self::create_buffer_view as vk::PFN_vkCreateBufferView)
                },
            },
            VulkanCommand {
                name: "vkCreateCommandPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateCommandPool),
                proc: unsafe {
                    std::mem::transmute(Self::create_command_pool as vk::PFN_vkCreateCommandPool)
                },
            },
            VulkanCommand {
                name: "vkCreateComputePipelines",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateComputePipelines),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_compute_pipelines as vk::PFN_vkCreateComputePipelines,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateCuFunctionNVX",
                features: smallvec![Feature::Extension(Extension::NVXBinaryImport)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateCuFunctionNvx),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_cu_function_nvx as vk::PFN_vkCreateCuFunctionNVX,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateCuModuleNVX",
                features: smallvec![Feature::Extension(Extension::NVXBinaryImport)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateCuModuleNvx),
                proc: unsafe {
                    std::mem::transmute(Self::create_cu_module_nvx as vk::PFN_vkCreateCuModuleNVX)
                },
            },
            VulkanCommand {
                name: "vkCreateDeferredOperationKHR",
                features: smallvec![Feature::Extension(Extension::KHRDeferredHostOperations)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateDeferredOperationKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_deferred_operation_khr as vk::PFN_vkCreateDeferredOperationKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateDescriptorPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateDescriptorPool),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_descriptor_pool as vk::PFN_vkCreateDescriptorPool,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateDescriptorSetLayout",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateDescriptorSetLayout),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_descriptor_set_layout as vk::PFN_vkCreateDescriptorSetLayout,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateDescriptorUpdateTemplate",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CreateDescriptorUpdateTemplate),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_descriptor_update_template
                            as vk::PFN_vkCreateDescriptorUpdateTemplate,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateDescriptorUpdateTemplateKHR",
                features: smallvec![Feature::Extension(Extension::KHRDescriptorUpdateTemplate)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CreateDescriptorUpdateTemplate),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_descriptor_update_template
                            as vk::PFN_vkCreateDescriptorUpdateTemplate,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateEvent",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateEvent),
                proc: unsafe { std::mem::transmute(Self::create_event as vk::PFN_vkCreateEvent) },
            },
            VulkanCommand {
                name: "vkCreateFence",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateFence),
                proc: unsafe { std::mem::transmute(Self::create_fence as vk::PFN_vkCreateFence) },
            },
            VulkanCommand {
                name: "vkCreateFramebuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateFramebuffer),
                proc: unsafe {
                    std::mem::transmute(Self::create_framebuffer as vk::PFN_vkCreateFramebuffer)
                },
            },
            VulkanCommand {
                name: "vkCreateGraphicsPipelines",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateGraphicsPipelines),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_graphics_pipelines as vk::PFN_vkCreateGraphicsPipelines,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateImage",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateImage),
                proc: unsafe { std::mem::transmute(Self::create_image as vk::PFN_vkCreateImage) },
            },
            VulkanCommand {
                name: "vkCreateImageView",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateImageView),
                proc: unsafe {
                    std::mem::transmute(Self::create_image_view as vk::PFN_vkCreateImageView)
                },
            },
            VulkanCommand {
                name: "vkCreateIndirectCommandsLayoutNV",
                features: smallvec![Feature::Extension(Extension::NVDeviceGeneratedCommands)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CreateIndirectCommandsLayoutNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_indirect_commands_layout_nv
                            as vk::PFN_vkCreateIndirectCommandsLayoutNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateMicromapEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateMicromapExt),
                proc: unsafe {
                    std::mem::transmute(Self::create_micromap_ext as vk::PFN_vkCreateMicromapEXT)
                },
            },
            VulkanCommand {
                name: "vkCreateOpticalFlowSessionNV",
                features: smallvec![Feature::Extension(Extension::NVOpticalFlow)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateOpticalFlowSessionNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_optical_flow_session_nv
                            as vk::PFN_vkCreateOpticalFlowSessionNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreatePipelineCache",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreatePipelineCache),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_pipeline_cache as vk::PFN_vkCreatePipelineCache,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreatePipelineLayout",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreatePipelineLayout),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_pipeline_layout as vk::PFN_vkCreatePipelineLayout,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreatePrivateDataSlot",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreatePrivateDataSlot),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_private_data_slot as vk::PFN_vkCreatePrivateDataSlot,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreatePrivateDataSlotEXT",
                features: smallvec![Feature::Extension(Extension::EXTPrivateData)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreatePrivateDataSlot),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_private_data_slot as vk::PFN_vkCreatePrivateDataSlot,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateQueryPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateQueryPool),
                proc: unsafe {
                    std::mem::transmute(Self::create_query_pool as vk::PFN_vkCreateQueryPool)
                },
            },
            VulkanCommand {
                name: "vkCreateRayTracingPipelinesKHR",
                features: smallvec![Feature::Extension(Extension::KHRRayTracingPipeline)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateRayTracingPipelinesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_ray_tracing_pipelines_khr
                            as vk::PFN_vkCreateRayTracingPipelinesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateRayTracingPipelinesNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateRayTracingPipelinesNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_ray_tracing_pipelines_nv
                            as vk::PFN_vkCreateRayTracingPipelinesNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateRenderPass",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateRenderPass),
                proc: unsafe {
                    std::mem::transmute(Self::create_render_pass as vk::PFN_vkCreateRenderPass)
                },
            },
            VulkanCommand {
                name: "vkCreateRenderPass2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateRenderPass2),
                proc: unsafe {
                    std::mem::transmute(Self::create_render_pass2 as vk::PFN_vkCreateRenderPass2)
                },
            },
            VulkanCommand {
                name: "vkCreateRenderPass2KHR",
                features: smallvec![Feature::Extension(Extension::KHRCreateRenderpass2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateRenderPass2),
                proc: unsafe {
                    std::mem::transmute(Self::create_render_pass2 as vk::PFN_vkCreateRenderPass2)
                },
            },
            VulkanCommand {
                name: "vkCreateSampler",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateSampler),
                proc: unsafe {
                    std::mem::transmute(Self::create_sampler as vk::PFN_vkCreateSampler)
                },
            },
            VulkanCommand {
                name: "vkCreateSamplerYcbcrConversion",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateSamplerYcbcrConversion),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_sampler_ycbcr_conversion
                            as vk::PFN_vkCreateSamplerYcbcrConversion,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateSamplerYcbcrConversionKHR",
                features: smallvec![Feature::Extension(Extension::KHRSamplerYcbcrConversion)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateSamplerYcbcrConversion),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_sampler_ycbcr_conversion
                            as vk::PFN_vkCreateSamplerYcbcrConversion,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateSemaphore",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateSemaphore),
                proc: unsafe {
                    std::mem::transmute(Self::create_semaphore as vk::PFN_vkCreateSemaphore)
                },
            },
            VulkanCommand {
                name: "vkCreateShaderModule",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateShaderModule),
                proc: unsafe {
                    std::mem::transmute(Self::create_shader_module as vk::PFN_vkCreateShaderModule)
                },
            },
            VulkanCommand {
                name: "vkCreateSharedSwapchainsKHR",
                features: smallvec![Feature::Extension(Extension::KHRDisplaySwapchain)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateSharedSwapchainsKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_shared_swapchains_khr as vk::PFN_vkCreateSharedSwapchainsKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateSwapchainKHR",
                features: smallvec![Feature::Extension(Extension::KHRSwapchain)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateSwapchainKhr),
                proc: unsafe {
                    std::mem::transmute(Self::create_swapchain_khr as vk::PFN_vkCreateSwapchainKHR)
                },
            },
            VulkanCommand {
                name: "vkCreateValidationCacheEXT",
                features: smallvec![Feature::Extension(Extension::EXTValidationCache)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateValidationCacheExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_validation_cache_ext as vk::PFN_vkCreateValidationCacheEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateVideoSessionKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateVideoSessionKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_video_session_khr as vk::PFN_vkCreateVideoSessionKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkCreateVideoSessionParametersKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::CreateVideoSessionParametersKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::create_video_session_parameters_khr
                            as vk::PFN_vkCreateVideoSessionParametersKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkDebugMarkerSetObjectNameEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugMarker)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DebugMarkerSetObjectNameExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::debug_marker_set_object_name_ext
                            as vk::PFN_vkDebugMarkerSetObjectNameEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkDebugMarkerSetObjectTagEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugMarker)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DebugMarkerSetObjectTagExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::debug_marker_set_object_tag_ext
                            as vk::PFN_vkDebugMarkerSetObjectTagEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkDeferredOperationJoinKHR",
                features: smallvec![Feature::Extension(Extension::KHRDeferredHostOperations)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DeferredOperationJoinKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::deferred_operation_join_khr as vk::PFN_vkDeferredOperationJoinKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyAccelerationStructureKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::DestroyAccelerationStructureKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_acceleration_structure_khr
                            as vk::PFN_vkDestroyAccelerationStructureKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyAccelerationStructureNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::DestroyAccelerationStructureNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_acceleration_structure_nv
                            as vk::PFN_vkDestroyAccelerationStructureNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyBuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyBuffer),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_buffer as vk::PFN_vkDestroyBuffer)
                },
            },
            VulkanCommand {
                name: "vkDestroyBufferCollectionFUCHSIA",
                features: smallvec![Feature::Extension(Extension::FUCHSIABufferCollection)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::DestroyBufferCollectionFuchsia),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_buffer_collection_fuchsia
                            as vk::PFN_vkDestroyBufferCollectionFUCHSIA,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyBufferView",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyBufferView),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_buffer_view as vk::PFN_vkDestroyBufferView)
                },
            },
            VulkanCommand {
                name: "vkDestroyCommandPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyCommandPool),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_command_pool as vk::PFN_vkDestroyCommandPool)
                },
            },
            VulkanCommand {
                name: "vkDestroyCuFunctionNVX",
                features: smallvec![Feature::Extension(Extension::NVXBinaryImport)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyCuFunctionNvx),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_cu_function_nvx as vk::PFN_vkDestroyCuFunctionNVX,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyCuModuleNVX",
                features: smallvec![Feature::Extension(Extension::NVXBinaryImport)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyCuModuleNvx),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_cu_module_nvx as vk::PFN_vkDestroyCuModuleNVX)
                },
            },
            VulkanCommand {
                name: "vkDestroyDeferredOperationKHR",
                features: smallvec![Feature::Extension(Extension::KHRDeferredHostOperations)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyDeferredOperationKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_deferred_operation_khr
                            as vk::PFN_vkDestroyDeferredOperationKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyDescriptorPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyDescriptorPool),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_descriptor_pool as vk::PFN_vkDestroyDescriptorPool,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyDescriptorSetLayout",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyDescriptorSetLayout),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_descriptor_set_layout as vk::PFN_vkDestroyDescriptorSetLayout,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyDescriptorUpdateTemplate",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::DestroyDescriptorUpdateTemplate),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_descriptor_update_template
                            as vk::PFN_vkDestroyDescriptorUpdateTemplate,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyDescriptorUpdateTemplateKHR",
                features: smallvec![Feature::Extension(Extension::KHRDescriptorUpdateTemplate)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::DestroyDescriptorUpdateTemplate),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_descriptor_update_template
                            as vk::PFN_vkDestroyDescriptorUpdateTemplate,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyDevice",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: true,
                proc: unsafe {
                    std::mem::transmute(Self::destroy_device as vk::PFN_vkDestroyDevice)
                },
            },
            VulkanCommand {
                name: "vkDestroyEvent",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyEvent),
                proc: unsafe { std::mem::transmute(Self::destroy_event as vk::PFN_vkDestroyEvent) },
            },
            VulkanCommand {
                name: "vkDestroyFence",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyFence),
                proc: unsafe { std::mem::transmute(Self::destroy_fence as vk::PFN_vkDestroyFence) },
            },
            VulkanCommand {
                name: "vkDestroyFramebuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyFramebuffer),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_framebuffer as vk::PFN_vkDestroyFramebuffer)
                },
            },
            VulkanCommand {
                name: "vkDestroyImage",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyImage),
                proc: unsafe { std::mem::transmute(Self::destroy_image as vk::PFN_vkDestroyImage) },
            },
            VulkanCommand {
                name: "vkDestroyImageView",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyImageView),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_image_view as vk::PFN_vkDestroyImageView)
                },
            },
            VulkanCommand {
                name: "vkDestroyIndirectCommandsLayoutNV",
                features: smallvec![Feature::Extension(Extension::NVDeviceGeneratedCommands)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::DestroyIndirectCommandsLayoutNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_indirect_commands_layout_nv
                            as vk::PFN_vkDestroyIndirectCommandsLayoutNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyMicromapEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyMicromapExt),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_micromap_ext as vk::PFN_vkDestroyMicromapEXT)
                },
            },
            VulkanCommand {
                name: "vkDestroyOpticalFlowSessionNV",
                features: smallvec![Feature::Extension(Extension::NVOpticalFlow)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyOpticalFlowSessionNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_optical_flow_session_nv
                            as vk::PFN_vkDestroyOpticalFlowSessionNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyPipeline",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyPipeline),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_pipeline as vk::PFN_vkDestroyPipeline)
                },
            },
            VulkanCommand {
                name: "vkDestroyPipelineCache",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyPipelineCache),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_pipeline_cache as vk::PFN_vkDestroyPipelineCache,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyPipelineLayout",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyPipelineLayout),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_pipeline_layout as vk::PFN_vkDestroyPipelineLayout,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyPrivateDataSlot",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyPrivateDataSlot),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_private_data_slot as vk::PFN_vkDestroyPrivateDataSlot,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyPrivateDataSlotEXT",
                features: smallvec![Feature::Extension(Extension::EXTPrivateData)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyPrivateDataSlot),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_private_data_slot as vk::PFN_vkDestroyPrivateDataSlot,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyQueryPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyQueryPool),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_query_pool as vk::PFN_vkDestroyQueryPool)
                },
            },
            VulkanCommand {
                name: "vkDestroyRenderPass",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyRenderPass),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_render_pass as vk::PFN_vkDestroyRenderPass)
                },
            },
            VulkanCommand {
                name: "vkDestroySampler",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroySampler),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_sampler as vk::PFN_vkDestroySampler)
                },
            },
            VulkanCommand {
                name: "vkDestroySamplerYcbcrConversion",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::DestroySamplerYcbcrConversion),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_sampler_ycbcr_conversion
                            as vk::PFN_vkDestroySamplerYcbcrConversion,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroySamplerYcbcrConversionKHR",
                features: smallvec![Feature::Extension(Extension::KHRSamplerYcbcrConversion)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::DestroySamplerYcbcrConversion),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_sampler_ycbcr_conversion
                            as vk::PFN_vkDestroySamplerYcbcrConversion,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroySemaphore",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroySemaphore),
                proc: unsafe {
                    std::mem::transmute(Self::destroy_semaphore as vk::PFN_vkDestroySemaphore)
                },
            },
            VulkanCommand {
                name: "vkDestroyShaderModule",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyShaderModule),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_shader_module as vk::PFN_vkDestroyShaderModule,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroySwapchainKHR",
                features: smallvec![Feature::Extension(Extension::KHRSwapchain)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroySwapchainKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_swapchain_khr as vk::PFN_vkDestroySwapchainKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyValidationCacheEXT",
                features: smallvec![Feature::Extension(Extension::EXTValidationCache)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyValidationCacheExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_validation_cache_ext as vk::PFN_vkDestroyValidationCacheEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyVideoSessionKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyVideoSessionKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_video_session_khr as vk::PFN_vkDestroyVideoSessionKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkDestroyVideoSessionParametersKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::DestroyVideoSessionParametersKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::destroy_video_session_parameters_khr
                            as vk::PFN_vkDestroyVideoSessionParametersKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkDeviceWaitIdle",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DeviceWaitIdle),
                proc: unsafe {
                    std::mem::transmute(Self::device_wait_idle as vk::PFN_vkDeviceWaitIdle)
                },
            },
            VulkanCommand {
                name: "vkDisplayPowerControlEXT",
                features: smallvec![Feature::Extension(Extension::EXTDisplayControl)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DisplayPowerControlExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::display_power_control_ext as vk::PFN_vkDisplayPowerControlEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkEndCommandBuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::EndCommandBuffer),
                proc: unsafe {
                    std::mem::transmute(Self::end_command_buffer as vk::PFN_vkEndCommandBuffer)
                },
            },
            VulkanCommand {
                name: "vkExportMetalObjectsEXT",
                features: smallvec![Feature::Extension(Extension::EXTMetalObjects)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ExportMetalObjectsExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::export_metal_objects_ext as vk::PFN_vkExportMetalObjectsEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkFlushMappedMemoryRanges",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::FlushMappedMemoryRanges),
                proc: unsafe {
                    std::mem::transmute(
                        Self::flush_mapped_memory_ranges as vk::PFN_vkFlushMappedMemoryRanges,
                    )
                },
            },
            VulkanCommand {
                name: "vkFreeCommandBuffers",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::FreeCommandBuffers),
                proc: unsafe {
                    std::mem::transmute(Self::free_command_buffers as vk::PFN_vkFreeCommandBuffers)
                },
            },
            VulkanCommand {
                name: "vkFreeDescriptorSets",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::FreeDescriptorSets),
                proc: unsafe {
                    std::mem::transmute(Self::free_descriptor_sets as vk::PFN_vkFreeDescriptorSets)
                },
            },
            VulkanCommand {
                name: "vkFreeMemory",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::FreeMemory),
                proc: unsafe { std::mem::transmute(Self::free_memory as vk::PFN_vkFreeMemory) },
            },
            VulkanCommand {
                name: "vkGetAccelerationStructureBuildSizesKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetAccelerationStructureBuildSizesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_acceleration_structure_build_sizes_khr
                            as vk::PFN_vkGetAccelerationStructureBuildSizesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetAccelerationStructureDeviceAddressKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetAccelerationStructureDeviceAddressKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_acceleration_structure_device_address_khr
                            as vk::PFN_vkGetAccelerationStructureDeviceAddressKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetAccelerationStructureHandleNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetAccelerationStructureHandleNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_acceleration_structure_handle_nv
                            as vk::PFN_vkGetAccelerationStructureHandleNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetAccelerationStructureMemoryRequirementsNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetAccelerationStructureMemoryRequirementsNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_acceleration_structure_memory_requirements_nv
                            as vk::PFN_vkGetAccelerationStructureMemoryRequirementsNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands.contains(
                    &LayerVulkanCommand::GetAccelerationStructureOpaqueCaptureDescriptorDataExt,
                ),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_acceleration_structure_opaque_capture_descriptor_data_ext
                            as vk::PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetAndroidHardwareBufferPropertiesANDROID",
                features: smallvec![Feature::Extension(
                    Extension::ANDROIDExternalMemoryAndroidHardwareBuffer
                )],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetAndroidHardwareBufferPropertiesAndroid),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_android_hardware_buffer_properties_android
                            as vk::PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetBufferCollectionPropertiesFUCHSIA",
                features: smallvec![Feature::Extension(Extension::FUCHSIABufferCollection)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetBufferCollectionPropertiesFuchsia),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_buffer_collection_properties_fuchsia
                            as vk::PFN_vkGetBufferCollectionPropertiesFUCHSIA,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetBufferDeviceAddress",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetBufferDeviceAddress),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_buffer_device_address as vk::PFN_vkGetBufferDeviceAddress,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetBufferDeviceAddressEXT",
                features: smallvec![Feature::Extension(Extension::EXTBufferDeviceAddress)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetBufferDeviceAddress),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_buffer_device_address as vk::PFN_vkGetBufferDeviceAddress,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetBufferDeviceAddressKHR",
                features: smallvec![Feature::Extension(Extension::KHRBufferDeviceAddress)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetBufferDeviceAddress),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_buffer_device_address as vk::PFN_vkGetBufferDeviceAddress,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetBufferMemoryRequirements",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetBufferMemoryRequirements),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_buffer_memory_requirements
                            as vk::PFN_vkGetBufferMemoryRequirements,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetBufferMemoryRequirements2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetBufferMemoryRequirements2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_buffer_memory_requirements2
                            as vk::PFN_vkGetBufferMemoryRequirements2,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetBufferMemoryRequirements2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetMemoryRequirements2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetBufferMemoryRequirements2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_buffer_memory_requirements2
                            as vk::PFN_vkGetBufferMemoryRequirements2,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetBufferOpaqueCaptureAddress",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetBufferOpaqueCaptureAddress),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_buffer_opaque_capture_address
                            as vk::PFN_vkGetBufferOpaqueCaptureAddress,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetBufferOpaqueCaptureAddressKHR",
                features: smallvec![Feature::Extension(Extension::KHRBufferDeviceAddress)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetBufferOpaqueCaptureAddress),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_buffer_opaque_capture_address
                            as vk::PFN_vkGetBufferOpaqueCaptureAddress,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetBufferOpaqueCaptureDescriptorDataEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetBufferOpaqueCaptureDescriptorDataExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_buffer_opaque_capture_descriptor_data_ext
                            as vk::PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetCalibratedTimestampsEXT",
                features: smallvec![Feature::Extension(Extension::EXTCalibratedTimestamps)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetCalibratedTimestampsExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_calibrated_timestamps_ext as vk::PFN_vkGetCalibratedTimestampsEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeferredOperationMaxConcurrencyKHR",
                features: smallvec![Feature::Extension(Extension::KHRDeferredHostOperations)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeferredOperationMaxConcurrencyKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_deferred_operation_max_concurrency_khr
                            as vk::PFN_vkGetDeferredOperationMaxConcurrencyKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeferredOperationResultKHR",
                features: smallvec![Feature::Extension(Extension::KHRDeferredHostOperations)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeferredOperationResultKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_deferred_operation_result_khr
                            as vk::PFN_vkGetDeferredOperationResultKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDescriptorEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetDescriptorExt),
                proc: unsafe {
                    std::mem::transmute(Self::get_descriptor_ext as vk::PFN_vkGetDescriptorEXT)
                },
            },
            VulkanCommand {
                name: "vkGetDescriptorSetHostMappingVALVE",
                features: smallvec![Feature::Extension(Extension::VALVEDescriptorSetHostMapping)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDescriptorSetHostMappingValve),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_descriptor_set_host_mapping_valve
                            as vk::PFN_vkGetDescriptorSetHostMappingVALVE,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDescriptorSetLayoutBindingOffsetEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDescriptorSetLayoutBindingOffsetExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_descriptor_set_layout_binding_offset_ext
                            as vk::PFN_vkGetDescriptorSetLayoutBindingOffsetEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDescriptorSetLayoutHostMappingInfoVALVE",
                features: smallvec![Feature::Extension(Extension::VALVEDescriptorSetHostMapping)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDescriptorSetLayoutHostMappingInfoValve),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_descriptor_set_layout_host_mapping_info_valve
                            as vk::PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDescriptorSetLayoutSizeEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDescriptorSetLayoutSizeExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_descriptor_set_layout_size_ext
                            as vk::PFN_vkGetDescriptorSetLayoutSizeEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDescriptorSetLayoutSupport",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDescriptorSetLayoutSupport),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_descriptor_set_layout_support
                            as vk::PFN_vkGetDescriptorSetLayoutSupport,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDescriptorSetLayoutSupportKHR",
                features: smallvec![Feature::Extension(Extension::KHRMaintenance3)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDescriptorSetLayoutSupport),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_descriptor_set_layout_support
                            as vk::PFN_vkGetDescriptorSetLayoutSupport,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceAccelerationStructureCompatibilityKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceAccelerationStructureCompatibilityKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_acceleration_structure_compatibility_khr
                            as vk::PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceBufferMemoryRequirements",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceBufferMemoryRequirements),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_buffer_memory_requirements
                            as vk::PFN_vkGetDeviceBufferMemoryRequirements,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceBufferMemoryRequirementsKHR",
                features: smallvec![Feature::Extension(Extension::KHRMaintenance4)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceBufferMemoryRequirements),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_buffer_memory_requirements
                            as vk::PFN_vkGetDeviceBufferMemoryRequirements,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceGroupPeerMemoryFeatures",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceGroupPeerMemoryFeatures),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_group_peer_memory_features
                            as vk::PFN_vkGetDeviceGroupPeerMemoryFeatures,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceGroupPeerMemoryFeaturesKHR",
                features: smallvec![Feature::Extension(Extension::KHRDeviceGroup)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceGroupPeerMemoryFeatures),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_group_peer_memory_features
                            as vk::PFN_vkGetDeviceGroupPeerMemoryFeatures,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceGroupPresentCapabilitiesKHR",
                features: smallvec![
                    Feature::Extension(Extension::KHRSwapchain),
                    Feature::Extension(Extension::KHRDeviceGroup)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceGroupPresentCapabilitiesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_group_present_capabilities_khr
                            as vk::PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceGroupSurfacePresentModes2EXT",
                features: smallvec![Feature::Extension(Extension::EXTFullScreenExclusive)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceGroupSurfacePresentModes2Ext),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_group_surface_present_modes2_ext
                            as vk::PFN_vkGetDeviceGroupSurfacePresentModes2EXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceGroupSurfacePresentModesKHR",
                features: smallvec![
                    Feature::Extension(Extension::KHRSwapchain),
                    Feature::Extension(Extension::KHRDeviceGroup)
                ],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceGroupSurfacePresentModesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_group_surface_present_modes_khr
                            as vk::PFN_vkGetDeviceGroupSurfacePresentModesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceImageMemoryRequirements",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceImageMemoryRequirements),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_image_memory_requirements
                            as vk::PFN_vkGetDeviceImageMemoryRequirements,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceImageMemoryRequirementsKHR",
                features: smallvec![Feature::Extension(Extension::KHRMaintenance4)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceImageMemoryRequirements),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_image_memory_requirements
                            as vk::PFN_vkGetDeviceImageMemoryRequirements,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceImageSparseMemoryRequirements",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceImageSparseMemoryRequirements),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_image_sparse_memory_requirements
                            as vk::PFN_vkGetDeviceImageSparseMemoryRequirements,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceImageSparseMemoryRequirementsKHR",
                features: smallvec![Feature::Extension(Extension::KHRMaintenance4)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceImageSparseMemoryRequirements),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_image_sparse_memory_requirements
                            as vk::PFN_vkGetDeviceImageSparseMemoryRequirements,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceMemoryCommitment",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetDeviceMemoryCommitment),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_memory_commitment as vk::PFN_vkGetDeviceMemoryCommitment,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceMemoryOpaqueCaptureAddress",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceMemoryOpaqueCaptureAddress),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_memory_opaque_capture_address
                            as vk::PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceMemoryOpaqueCaptureAddressKHR",
                features: smallvec![Feature::Extension(Extension::KHRBufferDeviceAddress)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceMemoryOpaqueCaptureAddress),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_memory_opaque_capture_address
                            as vk::PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceMicromapCompatibilityEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceMicromapCompatibilityExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_micromap_compatibility_ext
                            as vk::PFN_vkGetDeviceMicromapCompatibilityEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDeviceProcAddr",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: true,
                proc: unsafe {
                    std::mem::transmute(Self::get_device_proc_addr as vk::PFN_vkGetDeviceProcAddr)
                },
            },
            VulkanCommand {
                name: "vkGetDeviceQueue",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetDeviceQueue),
                proc: unsafe {
                    std::mem::transmute(Self::get_device_queue as vk::PFN_vkGetDeviceQueue)
                },
            },
            VulkanCommand {
                name: "vkGetDeviceQueue2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetDeviceQueue2),
                proc: unsafe {
                    std::mem::transmute(Self::get_device_queue2 as vk::PFN_vkGetDeviceQueue2)
                },
            },
            VulkanCommand {
                name: "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI",
                features: smallvec![Feature::Extension(Extension::HUAWEISubpassShading)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDeviceSubpassShadingMaxWorkgroupSizeHuawei),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_device_subpass_shading_max_workgroup_size_huawei
                            as vk::PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetDynamicRenderingTilePropertiesQCOM",
                features: smallvec![Feature::Extension(Extension::QCOMTileProperties)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetDynamicRenderingTilePropertiesQcom),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_dynamic_rendering_tile_properties_qcom
                            as vk::PFN_vkGetDynamicRenderingTilePropertiesQCOM,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetEventStatus",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetEventStatus),
                proc: unsafe {
                    std::mem::transmute(Self::get_event_status as vk::PFN_vkGetEventStatus)
                },
            },
            VulkanCommand {
                name: "vkGetFenceFdKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalFenceFd)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetFenceFdKhr),
                proc: unsafe {
                    std::mem::transmute(Self::get_fence_fd_khr as vk::PFN_vkGetFenceFdKHR)
                },
            },
            VulkanCommand {
                name: "vkGetFenceStatus",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetFenceStatus),
                proc: unsafe {
                    std::mem::transmute(Self::get_fence_status as vk::PFN_vkGetFenceStatus)
                },
            },
            VulkanCommand {
                name: "vkGetFenceWin32HandleKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalFenceWin32)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetFenceWin32HandleKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_fence_win32_handle_khr as vk::PFN_vkGetFenceWin32HandleKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetFramebufferTilePropertiesQCOM",
                features: smallvec![Feature::Extension(Extension::QCOMTileProperties)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetFramebufferTilePropertiesQcom),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_framebuffer_tile_properties_qcom
                            as vk::PFN_vkGetFramebufferTilePropertiesQCOM,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetGeneratedCommandsMemoryRequirementsNV",
                features: smallvec![Feature::Extension(Extension::NVDeviceGeneratedCommands)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetGeneratedCommandsMemoryRequirementsNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_generated_commands_memory_requirements_nv
                            as vk::PFN_vkGetGeneratedCommandsMemoryRequirementsNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageDrmFormatModifierPropertiesEXT",
                features: smallvec![Feature::Extension(Extension::EXTImageDrmFormatModifier)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetImageDrmFormatModifierPropertiesExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_drm_format_modifier_properties_ext
                            as vk::PFN_vkGetImageDrmFormatModifierPropertiesEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageMemoryRequirements",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetImageMemoryRequirements),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_memory_requirements as vk::PFN_vkGetImageMemoryRequirements,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageMemoryRequirements2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetImageMemoryRequirements2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_memory_requirements2
                            as vk::PFN_vkGetImageMemoryRequirements2,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageMemoryRequirements2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetMemoryRequirements2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetImageMemoryRequirements2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_memory_requirements2
                            as vk::PFN_vkGetImageMemoryRequirements2,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageOpaqueCaptureDescriptorDataEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetImageOpaqueCaptureDescriptorDataExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_opaque_capture_descriptor_data_ext
                            as vk::PFN_vkGetImageOpaqueCaptureDescriptorDataEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageSparseMemoryRequirements",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetImageSparseMemoryRequirements),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_sparse_memory_requirements
                            as vk::PFN_vkGetImageSparseMemoryRequirements,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageSparseMemoryRequirements2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetImageSparseMemoryRequirements2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_sparse_memory_requirements2
                            as vk::PFN_vkGetImageSparseMemoryRequirements2,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageSparseMemoryRequirements2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetMemoryRequirements2)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetImageSparseMemoryRequirements2),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_sparse_memory_requirements2
                            as vk::PFN_vkGetImageSparseMemoryRequirements2,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageSubresourceLayout",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetImageSubresourceLayout),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_subresource_layout as vk::PFN_vkGetImageSubresourceLayout,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageSubresourceLayout2EXT",
                features: smallvec![Feature::Extension(Extension::EXTImageCompressionControl)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetImageSubresourceLayout2Ext),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_subresource_layout2_ext
                            as vk::PFN_vkGetImageSubresourceLayout2EXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageViewAddressNVX",
                features: smallvec![Feature::Extension(Extension::NVXImageViewHandle)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetImageViewAddressNvx),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_view_address_nvx as vk::PFN_vkGetImageViewAddressNVX,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageViewHandleNVX",
                features: smallvec![Feature::Extension(Extension::NVXImageViewHandle)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetImageViewHandleNvx),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_view_handle_nvx as vk::PFN_vkGetImageViewHandleNVX,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetImageViewOpaqueCaptureDescriptorDataEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetImageViewOpaqueCaptureDescriptorDataExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_image_view_opaque_capture_descriptor_data_ext
                            as vk::PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetMemoryAndroidHardwareBufferANDROID",
                features: smallvec![Feature::Extension(
                    Extension::ANDROIDExternalMemoryAndroidHardwareBuffer
                )],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetMemoryAndroidHardwareBufferAndroid),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_memory_android_hardware_buffer_android
                            as vk::PFN_vkGetMemoryAndroidHardwareBufferANDROID,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetMemoryFdKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalMemoryFd)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetMemoryFdKhr),
                proc: unsafe {
                    std::mem::transmute(Self::get_memory_fd_khr as vk::PFN_vkGetMemoryFdKHR)
                },
            },
            VulkanCommand {
                name: "vkGetMemoryFdPropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalMemoryFd)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetMemoryFdPropertiesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_memory_fd_properties_khr as vk::PFN_vkGetMemoryFdPropertiesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetMemoryHostPointerPropertiesEXT",
                features: smallvec![Feature::Extension(Extension::EXTExternalMemoryHost)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetMemoryHostPointerPropertiesExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_memory_host_pointer_properties_ext
                            as vk::PFN_vkGetMemoryHostPointerPropertiesEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetMemoryRemoteAddressNV",
                features: smallvec![Feature::Extension(Extension::NVExternalMemoryRdma)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetMemoryRemoteAddressNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_memory_remote_address_nv as vk::PFN_vkGetMemoryRemoteAddressNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetMemoryWin32HandleKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalMemoryWin32)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetMemoryWin32HandleKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_memory_win32_handle_khr as vk::PFN_vkGetMemoryWin32HandleKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetMemoryWin32HandleNV",
                features: smallvec![Feature::Extension(Extension::NVExternalMemoryWin32)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetMemoryWin32HandleNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_memory_win32_handle_nv as vk::PFN_vkGetMemoryWin32HandleNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetMemoryWin32HandlePropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalMemoryWin32)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetMemoryWin32HandlePropertiesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_memory_win32_handle_properties_khr
                            as vk::PFN_vkGetMemoryWin32HandlePropertiesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetMemoryZirconHandleFUCHSIA",
                features: smallvec![Feature::Extension(Extension::FUCHSIAExternalMemory)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetMemoryZirconHandleFuchsia),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_memory_zircon_handle_fuchsia
                            as vk::PFN_vkGetMemoryZirconHandleFUCHSIA,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetMemoryZirconHandlePropertiesFUCHSIA",
                features: smallvec![Feature::Extension(Extension::FUCHSIAExternalMemory)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetMemoryZirconHandlePropertiesFuchsia),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_memory_zircon_handle_properties_fuchsia
                            as vk::PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetMicromapBuildSizesEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetMicromapBuildSizesExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_micromap_build_sizes_ext as vk::PFN_vkGetMicromapBuildSizesEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetPastPresentationTimingGOOGLE",
                features: smallvec![Feature::Extension(Extension::GOOGLEDisplayTiming)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetPastPresentationTimingGoogle),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_past_presentation_timing_google
                            as vk::PFN_vkGetPastPresentationTimingGOOGLE,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetPerformanceParameterINTEL",
                features: smallvec![Feature::Extension(Extension::INTELPerformanceQuery)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPerformanceParameterIntel),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_performance_parameter_intel
                            as vk::PFN_vkGetPerformanceParameterINTEL,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetPipelineCacheData",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPipelineCacheData),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_pipeline_cache_data as vk::PFN_vkGetPipelineCacheData,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetPipelineExecutableInternalRepresentationsKHR",
                features: smallvec![Feature::Extension(
                    Extension::KHRPipelineExecutableProperties
                )],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetPipelineExecutableInternalRepresentationsKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_pipeline_executable_internal_representations_khr
                            as vk::PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetPipelineExecutablePropertiesKHR",
                features: smallvec![Feature::Extension(
                    Extension::KHRPipelineExecutableProperties
                )],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetPipelineExecutablePropertiesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_pipeline_executable_properties_khr
                            as vk::PFN_vkGetPipelineExecutablePropertiesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetPipelineExecutableStatisticsKHR",
                features: smallvec![Feature::Extension(
                    Extension::KHRPipelineExecutableProperties
                )],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetPipelineExecutableStatisticsKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_pipeline_executable_statistics_khr
                            as vk::PFN_vkGetPipelineExecutableStatisticsKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetPipelinePropertiesEXT",
                features: smallvec![Feature::Extension(Extension::EXTPipelineProperties)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPipelinePropertiesExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_pipeline_properties_ext as vk::PFN_vkGetPipelinePropertiesEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetPrivateData",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPrivateData),
                proc: unsafe {
                    std::mem::transmute(Self::get_private_data as vk::PFN_vkGetPrivateData)
                },
            },
            VulkanCommand {
                name: "vkGetPrivateDataEXT",
                features: smallvec![Feature::Extension(Extension::EXTPrivateData)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPrivateData),
                proc: unsafe {
                    std::mem::transmute(Self::get_private_data as vk::PFN_vkGetPrivateData)
                },
            },
            VulkanCommand {
                name: "vkGetQueryPoolResults",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetQueryPoolResults),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_query_pool_results as vk::PFN_vkGetQueryPoolResults,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetQueueCheckpointData2NV",
                features: smallvec![Feature::Extension(Extension::KHRSynchronization2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetQueueCheckpointData2Nv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_queue_checkpoint_data2_nv as vk::PFN_vkGetQueueCheckpointData2NV,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetQueueCheckpointDataNV",
                features: smallvec![Feature::Extension(Extension::NVDeviceDiagnosticCheckpoints)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetQueueCheckpointDataNv),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_queue_checkpoint_data_nv as vk::PFN_vkGetQueueCheckpointDataNV,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR",
                features: smallvec![Feature::Extension(Extension::KHRRayTracingPipeline)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetRayTracingCaptureReplayShaderGroupHandlesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_ray_tracing_capture_replay_shader_group_handles_khr
                            as vk::PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetRayTracingShaderGroupHandlesKHR",
                features: smallvec![Feature::Extension(Extension::KHRRayTracingPipeline)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetRayTracingShaderGroupHandlesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_ray_tracing_shader_group_handles_khr
                            as vk::PFN_vkGetRayTracingShaderGroupHandlesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetRayTracingShaderGroupHandlesNV",
                features: smallvec![Feature::Extension(Extension::NVRayTracing)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetRayTracingShaderGroupHandlesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_ray_tracing_shader_group_handles_khr
                            as vk::PFN_vkGetRayTracingShaderGroupHandlesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetRayTracingShaderGroupStackSizeKHR",
                features: smallvec![Feature::Extension(Extension::KHRRayTracingPipeline)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetRayTracingShaderGroupStackSizeKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_ray_tracing_shader_group_stack_size_khr
                            as vk::PFN_vkGetRayTracingShaderGroupStackSizeKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetRefreshCycleDurationGOOGLE",
                features: smallvec![Feature::Extension(Extension::GOOGLEDisplayTiming)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetRefreshCycleDurationGoogle),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_refresh_cycle_duration_google
                            as vk::PFN_vkGetRefreshCycleDurationGOOGLE,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetRenderAreaGranularity",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetRenderAreaGranularity),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_render_area_granularity as vk::PFN_vkGetRenderAreaGranularity,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetSamplerOpaqueCaptureDescriptorDataEXT",
                features: smallvec![Feature::Extension(Extension::EXTDescriptorBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetSamplerOpaqueCaptureDescriptorDataExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_sampler_opaque_capture_descriptor_data_ext
                            as vk::PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetSemaphoreCounterValue",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetSemaphoreCounterValue),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_semaphore_counter_value as vk::PFN_vkGetSemaphoreCounterValue,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetSemaphoreCounterValueKHR",
                features: smallvec![Feature::Extension(Extension::KHRTimelineSemaphore)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetSemaphoreCounterValue),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_semaphore_counter_value as vk::PFN_vkGetSemaphoreCounterValue,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetSemaphoreFdKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalSemaphoreFd)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetSemaphoreFdKhr),
                proc: unsafe {
                    std::mem::transmute(Self::get_semaphore_fd_khr as vk::PFN_vkGetSemaphoreFdKHR)
                },
            },
            VulkanCommand {
                name: "vkGetSemaphoreWin32HandleKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalSemaphoreWin32)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetSemaphoreWin32HandleKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_semaphore_win32_handle_khr
                            as vk::PFN_vkGetSemaphoreWin32HandleKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetSemaphoreZirconHandleFUCHSIA",
                features: smallvec![Feature::Extension(Extension::FUCHSIAExternalSemaphore)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetSemaphoreZirconHandleFuchsia),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_semaphore_zircon_handle_fuchsia
                            as vk::PFN_vkGetSemaphoreZirconHandleFUCHSIA,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetShaderInfoAMD",
                features: smallvec![Feature::Extension(Extension::AMDShaderInfo)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetShaderInfoAmd),
                proc: unsafe {
                    std::mem::transmute(Self::get_shader_info_amd as vk::PFN_vkGetShaderInfoAMD)
                },
            },
            VulkanCommand {
                name: "vkGetShaderModuleCreateInfoIdentifierEXT",
                features: smallvec![Feature::Extension(Extension::EXTShaderModuleIdentifier)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetShaderModuleCreateInfoIdentifierExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_shader_module_create_info_identifier_ext
                            as vk::PFN_vkGetShaderModuleCreateInfoIdentifierEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetShaderModuleIdentifierEXT",
                features: smallvec![Feature::Extension(Extension::EXTShaderModuleIdentifier)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetShaderModuleIdentifierExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_shader_module_identifier_ext
                            as vk::PFN_vkGetShaderModuleIdentifierEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetSwapchainCounterEXT",
                features: smallvec![Feature::Extension(Extension::EXTDisplayControl)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetSwapchainCounterExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_swapchain_counter_ext as vk::PFN_vkGetSwapchainCounterEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetSwapchainGrallocUsage2ANDROID",
                features: smallvec![Feature::Extension(Extension::ANDROIDNativeBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetSwapchainGrallocUsage2Android),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_swapchain_gralloc_usage2_android
                            as vk::PFN_vkGetSwapchainGrallocUsage2ANDROID,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetSwapchainGrallocUsageANDROID",
                features: smallvec![Feature::Extension(Extension::ANDROIDNativeBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetSwapchainGrallocUsageAndroid),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_swapchain_gralloc_usage_android
                            as vk::PFN_vkGetSwapchainGrallocUsageANDROID,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetSwapchainImagesKHR",
                features: smallvec![Feature::Extension(Extension::KHRSwapchain)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetSwapchainImagesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_swapchain_images_khr as vk::PFN_vkGetSwapchainImagesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetSwapchainStatusKHR",
                features: smallvec![Feature::Extension(Extension::KHRSharedPresentableImage)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetSwapchainStatusKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_swapchain_status_khr as vk::PFN_vkGetSwapchainStatusKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetValidationCacheDataEXT",
                features: smallvec![Feature::Extension(Extension::EXTValidationCache)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetValidationCacheDataExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_validation_cache_data_ext as vk::PFN_vkGetValidationCacheDataEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkGetVideoSessionMemoryRequirementsKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::GetVideoSessionMemoryRequirementsKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::get_video_session_memory_requirements_khr
                            as vk::PFN_vkGetVideoSessionMemoryRequirementsKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkImportFenceFdKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalFenceFd)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ImportFenceFdKhr),
                proc: unsafe {
                    std::mem::transmute(Self::import_fence_fd_khr as vk::PFN_vkImportFenceFdKHR)
                },
            },
            VulkanCommand {
                name: "vkImportFenceWin32HandleKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalFenceWin32)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ImportFenceWin32HandleKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::import_fence_win32_handle_khr as vk::PFN_vkImportFenceWin32HandleKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkImportSemaphoreFdKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalSemaphoreFd)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ImportSemaphoreFdKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::import_semaphore_fd_khr as vk::PFN_vkImportSemaphoreFdKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkImportSemaphoreWin32HandleKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalSemaphoreWin32)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::ImportSemaphoreWin32HandleKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::import_semaphore_win32_handle_khr
                            as vk::PFN_vkImportSemaphoreWin32HandleKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkImportSemaphoreZirconHandleFUCHSIA",
                features: smallvec![Feature::Extension(Extension::FUCHSIAExternalSemaphore)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::ImportSemaphoreZirconHandleFuchsia),
                proc: unsafe {
                    std::mem::transmute(
                        Self::import_semaphore_zircon_handle_fuchsia
                            as vk::PFN_vkImportSemaphoreZirconHandleFUCHSIA,
                    )
                },
            },
            VulkanCommand {
                name: "vkInitializePerformanceApiINTEL",
                features: smallvec![Feature::Extension(Extension::INTELPerformanceQuery)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::InitializePerformanceApiIntel),
                proc: unsafe {
                    std::mem::transmute(
                        Self::initialize_performance_api_intel
                            as vk::PFN_vkInitializePerformanceApiINTEL,
                    )
                },
            },
            VulkanCommand {
                name: "vkInvalidateMappedMemoryRanges",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::InvalidateMappedMemoryRanges),
                proc: unsafe {
                    std::mem::transmute(
                        Self::invalidate_mapped_memory_ranges
                            as vk::PFN_vkInvalidateMappedMemoryRanges,
                    )
                },
            },
            VulkanCommand {
                name: "vkMapMemory",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::MapMemory),
                proc: unsafe { std::mem::transmute(Self::map_memory as vk::PFN_vkMapMemory) },
            },
            VulkanCommand {
                name: "vkMergePipelineCaches",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::MergePipelineCaches),
                proc: unsafe {
                    std::mem::transmute(
                        Self::merge_pipeline_caches as vk::PFN_vkMergePipelineCaches,
                    )
                },
            },
            VulkanCommand {
                name: "vkMergeValidationCachesEXT",
                features: smallvec![Feature::Extension(Extension::EXTValidationCache)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::MergeValidationCachesExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::merge_validation_caches_ext as vk::PFN_vkMergeValidationCachesEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkQueueBeginDebugUtilsLabelEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::QueueBeginDebugUtilsLabelExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::queue_begin_debug_utils_label_ext
                            as vk::PFN_vkQueueBeginDebugUtilsLabelEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkQueueBindSparse",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::QueueBindSparse),
                proc: unsafe {
                    std::mem::transmute(Self::queue_bind_sparse as vk::PFN_vkQueueBindSparse)
                },
            },
            VulkanCommand {
                name: "vkQueueEndDebugUtilsLabelEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::QueueEndDebugUtilsLabelExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::queue_end_debug_utils_label_ext
                            as vk::PFN_vkQueueEndDebugUtilsLabelEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkQueueInsertDebugUtilsLabelEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::QueueInsertDebugUtilsLabelExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::queue_insert_debug_utils_label_ext
                            as vk::PFN_vkQueueInsertDebugUtilsLabelEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkQueuePresentKHR",
                features: smallvec![Feature::Extension(Extension::KHRSwapchain)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::QueuePresentKhr),
                proc: unsafe {
                    std::mem::transmute(Self::queue_present_khr as vk::PFN_vkQueuePresentKHR)
                },
            },
            VulkanCommand {
                name: "vkQueueSetPerformanceConfigurationINTEL",
                features: smallvec![Feature::Extension(Extension::INTELPerformanceQuery)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::QueueSetPerformanceConfigurationIntel),
                proc: unsafe {
                    std::mem::transmute(
                        Self::queue_set_performance_configuration_intel
                            as vk::PFN_vkQueueSetPerformanceConfigurationINTEL,
                    )
                },
            },
            VulkanCommand {
                name: "vkQueueSignalReleaseImageANDROID",
                features: smallvec![Feature::Extension(Extension::ANDROIDNativeBuffer)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::QueueSignalReleaseImageAndroid),
                proc: unsafe {
                    std::mem::transmute(
                        Self::queue_signal_release_image_android
                            as vk::PFN_vkQueueSignalReleaseImageANDROID,
                    )
                },
            },
            VulkanCommand {
                name: "vkQueueSubmit",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::QueueSubmit),
                proc: unsafe { std::mem::transmute(Self::queue_submit as vk::PFN_vkQueueSubmit) },
            },
            VulkanCommand {
                name: "vkQueueSubmit2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::QueueSubmit2),
                proc: unsafe { std::mem::transmute(Self::queue_submit2 as vk::PFN_vkQueueSubmit2) },
            },
            VulkanCommand {
                name: "vkQueueSubmit2KHR",
                features: smallvec![Feature::Extension(Extension::KHRSynchronization2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::QueueSubmit2),
                proc: unsafe { std::mem::transmute(Self::queue_submit2 as vk::PFN_vkQueueSubmit2) },
            },
            VulkanCommand {
                name: "vkQueueWaitIdle",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::QueueWaitIdle),
                proc: unsafe {
                    std::mem::transmute(Self::queue_wait_idle as vk::PFN_vkQueueWaitIdle)
                },
            },
            VulkanCommand {
                name: "vkRegisterDeviceEventEXT",
                features: smallvec![Feature::Extension(Extension::EXTDisplayControl)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::RegisterDeviceEventExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::register_device_event_ext as vk::PFN_vkRegisterDeviceEventEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkRegisterDisplayEventEXT",
                features: smallvec![Feature::Extension(Extension::EXTDisplayControl)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::RegisterDisplayEventExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::register_display_event_ext as vk::PFN_vkRegisterDisplayEventEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkReleaseFullScreenExclusiveModeEXT",
                features: smallvec![Feature::Extension(Extension::EXTFullScreenExclusive)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::ReleaseFullScreenExclusiveModeExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::release_full_screen_exclusive_mode_ext
                            as vk::PFN_vkReleaseFullScreenExclusiveModeEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkReleasePerformanceConfigurationINTEL",
                features: smallvec![Feature::Extension(Extension::INTELPerformanceQuery)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::ReleasePerformanceConfigurationIntel),
                proc: unsafe {
                    std::mem::transmute(
                        Self::release_performance_configuration_intel
                            as vk::PFN_vkReleasePerformanceConfigurationINTEL,
                    )
                },
            },
            VulkanCommand {
                name: "vkReleaseProfilingLockKHR",
                features: smallvec![Feature::Extension(Extension::KHRPerformanceQuery)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ReleaseProfilingLockKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::release_profiling_lock_khr as vk::PFN_vkReleaseProfilingLockKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkReleaseSwapchainImagesEXT",
                features: smallvec![Feature::Extension(Extension::EXTSwapchainMaintenance1)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ReleaseSwapchainImagesExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::release_swapchain_images_ext as vk::PFN_vkReleaseSwapchainImagesEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkResetCommandBuffer",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ResetCommandBuffer),
                proc: unsafe {
                    std::mem::transmute(Self::reset_command_buffer as vk::PFN_vkResetCommandBuffer)
                },
            },
            VulkanCommand {
                name: "vkResetCommandPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ResetCommandPool),
                proc: unsafe {
                    std::mem::transmute(Self::reset_command_pool as vk::PFN_vkResetCommandPool)
                },
            },
            VulkanCommand {
                name: "vkResetDescriptorPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ResetDescriptorPool),
                proc: unsafe {
                    std::mem::transmute(
                        Self::reset_descriptor_pool as vk::PFN_vkResetDescriptorPool,
                    )
                },
            },
            VulkanCommand {
                name: "vkResetEvent",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ResetEvent),
                proc: unsafe { std::mem::transmute(Self::reset_event as vk::PFN_vkResetEvent) },
            },
            VulkanCommand {
                name: "vkResetFences",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ResetFences),
                proc: unsafe { std::mem::transmute(Self::reset_fences as vk::PFN_vkResetFences) },
            },
            VulkanCommand {
                name: "vkResetQueryPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ResetQueryPool),
                proc: unsafe {
                    std::mem::transmute(Self::reset_query_pool as vk::PFN_vkResetQueryPool)
                },
            },
            VulkanCommand {
                name: "vkResetQueryPoolEXT",
                features: smallvec![Feature::Extension(Extension::EXTHostQueryReset)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ResetQueryPool),
                proc: unsafe {
                    std::mem::transmute(Self::reset_query_pool as vk::PFN_vkResetQueryPool)
                },
            },
            VulkanCommand {
                name: "vkSetBufferCollectionBufferConstraintsFUCHSIA",
                features: smallvec![Feature::Extension(Extension::FUCHSIABufferCollection)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::SetBufferCollectionBufferConstraintsFuchsia),
                proc: unsafe {
                    std::mem::transmute(
                        Self::set_buffer_collection_buffer_constraints_fuchsia
                            as vk::PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,
                    )
                },
            },
            VulkanCommand {
                name: "vkSetBufferCollectionImageConstraintsFUCHSIA",
                features: smallvec![Feature::Extension(Extension::FUCHSIABufferCollection)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::SetBufferCollectionImageConstraintsFuchsia),
                proc: unsafe {
                    std::mem::transmute(
                        Self::set_buffer_collection_image_constraints_fuchsia
                            as vk::PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,
                    )
                },
            },
            VulkanCommand {
                name: "vkSetDebugUtilsObjectNameEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SetDebugUtilsObjectNameExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::set_debug_utils_object_name_ext
                            as vk::PFN_vkSetDebugUtilsObjectNameEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkSetDebugUtilsObjectTagEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SetDebugUtilsObjectTagExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::set_debug_utils_object_tag_ext as vk::PFN_vkSetDebugUtilsObjectTagEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkSetDeviceMemoryPriorityEXT",
                features: smallvec![Feature::Extension(Extension::EXTPageableDeviceLocalMemory)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SetDeviceMemoryPriorityExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::set_device_memory_priority_ext
                            as vk::PFN_vkSetDeviceMemoryPriorityEXT,
                    )
                },
            },
            VulkanCommand {
                name: "vkSetEvent",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SetEvent),
                proc: unsafe { std::mem::transmute(Self::set_event as vk::PFN_vkSetEvent) },
            },
            VulkanCommand {
                name: "vkSetHdrMetadataEXT",
                features: smallvec![Feature::Extension(Extension::EXTHdrMetadata)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SetHdrMetadataExt),
                proc: unsafe {
                    std::mem::transmute(Self::set_hdr_metadata_ext as vk::PFN_vkSetHdrMetadataEXT)
                },
            },
            VulkanCommand {
                name: "vkSetLocalDimmingAMD",
                features: smallvec![Feature::Extension(Extension::AMDDisplayNativeHdr)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SetLocalDimmingAmd),
                proc: unsafe {
                    std::mem::transmute(Self::set_local_dimming_amd as vk::PFN_vkSetLocalDimmingAMD)
                },
            },
            VulkanCommand {
                name: "vkSetPrivateData",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SetPrivateData),
                proc: unsafe {
                    std::mem::transmute(Self::set_private_data as vk::PFN_vkSetPrivateData)
                },
            },
            VulkanCommand {
                name: "vkSetPrivateDataEXT",
                features: smallvec![Feature::Extension(Extension::EXTPrivateData)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SetPrivateData),
                proc: unsafe {
                    std::mem::transmute(Self::set_private_data as vk::PFN_vkSetPrivateData)
                },
            },
            VulkanCommand {
                name: "vkSignalSemaphore",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SignalSemaphore),
                proc: unsafe {
                    std::mem::transmute(Self::signal_semaphore as vk::PFN_vkSignalSemaphore)
                },
            },
            VulkanCommand {
                name: "vkSignalSemaphoreKHR",
                features: smallvec![Feature::Extension(Extension::KHRTimelineSemaphore)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SignalSemaphore),
                proc: unsafe {
                    std::mem::transmute(Self::signal_semaphore as vk::PFN_vkSignalSemaphore)
                },
            },
            VulkanCommand {
                name: "vkTrimCommandPool",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::TrimCommandPool),
                proc: unsafe {
                    std::mem::transmute(Self::trim_command_pool as vk::PFN_vkTrimCommandPool)
                },
            },
            VulkanCommand {
                name: "vkTrimCommandPoolKHR",
                features: smallvec![Feature::Extension(Extension::KHRMaintenance1)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::TrimCommandPool),
                proc: unsafe {
                    std::mem::transmute(Self::trim_command_pool as vk::PFN_vkTrimCommandPool)
                },
            },
            VulkanCommand {
                name: "vkUninitializePerformanceApiINTEL",
                features: smallvec![Feature::Extension(Extension::INTELPerformanceQuery)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::UninitializePerformanceApiIntel),
                proc: unsafe {
                    std::mem::transmute(
                        Self::uninitialize_performance_api_intel
                            as vk::PFN_vkUninitializePerformanceApiINTEL,
                    )
                },
            },
            VulkanCommand {
                name: "vkUnmapMemory",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::UnmapMemory),
                proc: unsafe { std::mem::transmute(Self::unmap_memory as vk::PFN_vkUnmapMemory) },
            },
            VulkanCommand {
                name: "vkUpdateDescriptorSetWithTemplate",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1 })],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::UpdateDescriptorSetWithTemplate),
                proc: unsafe {
                    std::mem::transmute(
                        Self::update_descriptor_set_with_template
                            as vk::PFN_vkUpdateDescriptorSetWithTemplate,
                    )
                },
            },
            VulkanCommand {
                name: "vkUpdateDescriptorSetWithTemplateKHR",
                features: smallvec![Feature::Extension(Extension::KHRDescriptorUpdateTemplate)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::UpdateDescriptorSetWithTemplate),
                proc: unsafe {
                    std::mem::transmute(
                        Self::update_descriptor_set_with_template
                            as vk::PFN_vkUpdateDescriptorSetWithTemplate,
                    )
                },
            },
            VulkanCommand {
                name: "vkUpdateDescriptorSets",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::UpdateDescriptorSets),
                proc: unsafe {
                    std::mem::transmute(
                        Self::update_descriptor_sets as vk::PFN_vkUpdateDescriptorSets,
                    )
                },
            },
            VulkanCommand {
                name: "vkUpdateVideoSessionParametersKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::UpdateVideoSessionParametersKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::update_video_session_parameters_khr
                            as vk::PFN_vkUpdateVideoSessionParametersKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkWaitForFences",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::WaitForFences),
                proc: unsafe {
                    std::mem::transmute(Self::wait_for_fences as vk::PFN_vkWaitForFences)
                },
            },
            VulkanCommand {
                name: "vkWaitForPresentKHR",
                features: smallvec![Feature::Extension(Extension::KHRPresentWait)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::WaitForPresentKhr),
                proc: unsafe {
                    std::mem::transmute(Self::wait_for_present_khr as vk::PFN_vkWaitForPresentKHR)
                },
            },
            VulkanCommand {
                name: "vkWaitSemaphores",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 2 })],
                hooked: hooked_commands.contains(&LayerVulkanCommand::WaitSemaphores),
                proc: unsafe {
                    std::mem::transmute(Self::wait_semaphores as vk::PFN_vkWaitSemaphores)
                },
            },
            VulkanCommand {
                name: "vkWaitSemaphoresKHR",
                features: smallvec![Feature::Extension(Extension::KHRTimelineSemaphore)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::WaitSemaphores),
                proc: unsafe {
                    std::mem::transmute(Self::wait_semaphores as vk::PFN_vkWaitSemaphores)
                },
            },
            VulkanCommand {
                name: "vkWriteAccelerationStructuresPropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRAccelerationStructure)],
                hooked: hooked_commands
                    .contains(&LayerVulkanCommand::WriteAccelerationStructuresPropertiesKhr),
                proc: unsafe {
                    std::mem::transmute(
                        Self::write_acceleration_structures_properties_khr
                            as vk::PFN_vkWriteAccelerationStructuresPropertiesKHR,
                    )
                },
            },
            VulkanCommand {
                name: "vkWriteMicromapsPropertiesEXT",
                features: smallvec![Feature::Extension(Extension::EXTOpacityMicromap)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::WriteMicromapsPropertiesExt),
                proc: unsafe {
                    std::mem::transmute(
                        Self::write_micromaps_properties_ext
                            as vk::PFN_vkWriteMicromapsPropertiesEXT,
                    )
                },
            },
        ])
    }
    pub(crate) fn create_instance_commands(
        &self,
        instance_info: &T::InstanceInfo,
    ) -> Box<[VulkanCommand]> {
        let hooked_commands = self
            .layer_info
            .hooked_instance_commands(instance_info)
            .collect::<HashSet<_>>();
        Box::new([
            VulkanCommand {
                name: "vkAcquireDrmDisplayEXT",
                features: smallvec![Feature::Extension(Extension::EXTAcquireDrmDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::AcquireDrmDisplayExt),
                proc: unsafe { std::mem::transmute(Self::acquire_drm_display_ext as vk::PFN_vkAcquireDrmDisplayEXT)},
            },
            VulkanCommand {
                name: "vkAcquireWinrtDisplayNV",
                features: smallvec![Feature::Extension(Extension::NVAcquireWinrtDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::AcquireWinrtDisplayNv),
                proc: unsafe { std::mem::transmute(Self::acquire_winrt_display_nv as vk::PFN_vkAcquireWinrtDisplayNV)},
            },
            VulkanCommand {
                name: "vkAcquireXlibDisplayEXT",
                features: smallvec![Feature::Extension(Extension::EXTAcquireXlibDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::AcquireXlibDisplayExt),
                proc: unsafe { std::mem::transmute(Self::acquire_xlib_display_ext as vk::PFN_vkAcquireXlibDisplayEXT)},
            },
            VulkanCommand {
                name: "vkCreateAndroidSurfaceKHR",
                features: smallvec![Feature::Extension(Extension::KHRAndroidSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateAndroidSurfaceKhr),
                proc: unsafe { std::mem::transmute(Self::create_android_surface_khr as vk::PFN_vkCreateAndroidSurfaceKHR)},
            },
            VulkanCommand {
                name: "vkCreateDebugReportCallbackEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugReport)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateDebugReportCallbackExt),
                proc: unsafe { std::mem::transmute(Self::create_debug_report_callback_ext as vk::PFN_vkCreateDebugReportCallbackEXT)},
            },
            VulkanCommand {
                name: "vkCreateDebugUtilsMessengerEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateDebugUtilsMessengerExt),
                proc: unsafe { std::mem::transmute(Self::create_debug_utils_messenger_ext as vk::PFN_vkCreateDebugUtilsMessengerEXT)},
            },
            VulkanCommand {
                name: "vkCreateDevice",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: true,
                proc: unsafe { std::mem::transmute(Self::create_device as vk::PFN_vkCreateDevice)},
            },
            VulkanCommand {
                name: "vkCreateDirectFBSurfaceEXT",
                features: smallvec![Feature::Extension(Extension::EXTDirectfbSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateDirectFbSurfaceExt),
                proc: unsafe { std::mem::transmute(Self::create_direct_fb_surface_ext as vk::PFN_vkCreateDirectFBSurfaceEXT)},
            },
            VulkanCommand {
                name: "vkCreateDisplayModeKHR",
                features: smallvec![Feature::Extension(Extension::KHRDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateDisplayModeKhr),
                proc: unsafe { std::mem::transmute(Self::create_display_mode_khr as vk::PFN_vkCreateDisplayModeKHR)},
            },
            VulkanCommand {
                name: "vkCreateDisplayPlaneSurfaceKHR",
                features: smallvec![Feature::Extension(Extension::KHRDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateDisplayPlaneSurfaceKhr),
                proc: unsafe { std::mem::transmute(Self::create_display_plane_surface_khr as vk::PFN_vkCreateDisplayPlaneSurfaceKHR)},
            },
            VulkanCommand {
                name: "vkCreateHeadlessSurfaceEXT",
                features: smallvec![Feature::Extension(Extension::EXTHeadlessSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateHeadlessSurfaceExt),
                proc: unsafe { std::mem::transmute(Self::create_headless_surface_ext as vk::PFN_vkCreateHeadlessSurfaceEXT)},
            },
            VulkanCommand {
                name: "vkCreateIOSSurfaceMVK",
                features: smallvec![Feature::Extension(Extension::MVKIosSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateIosSurfaceMvk),
                proc: unsafe { std::mem::transmute(Self::create_ios_surface_mvk as vk::PFN_vkCreateIOSSurfaceMVK)},
            },
            VulkanCommand {
                name: "vkCreateImagePipeSurfaceFUCHSIA",
                features: smallvec![Feature::Extension(Extension::FUCHSIAImagepipeSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateImagePipeSurfaceFuchsia),
                proc: unsafe { std::mem::transmute(Self::create_image_pipe_surface_fuchsia as vk::PFN_vkCreateImagePipeSurfaceFUCHSIA)},
            },
            VulkanCommand {
                name: "vkCreateMacOSSurfaceMVK",
                features: smallvec![Feature::Extension(Extension::MVKMacosSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateMacOsSurfaceMvk),
                proc: unsafe { std::mem::transmute(Self::create_mac_os_surface_mvk as vk::PFN_vkCreateMacOSSurfaceMVK)},
            },
            VulkanCommand {
                name: "vkCreateMetalSurfaceEXT",
                features: smallvec![Feature::Extension(Extension::EXTMetalSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateMetalSurfaceExt),
                proc: unsafe { std::mem::transmute(Self::create_metal_surface_ext as vk::PFN_vkCreateMetalSurfaceEXT)},
            },
            VulkanCommand {
                name: "vkCreateScreenSurfaceQNX",
                features: smallvec![Feature::Extension(Extension::QNXScreenSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateScreenSurfaceQnx),
                proc: unsafe { std::mem::transmute(Self::create_screen_surface_qnx as vk::PFN_vkCreateScreenSurfaceQNX)},
            },
            VulkanCommand {
                name: "vkCreateStreamDescriptorSurfaceGGP",
                features: smallvec![Feature::Extension(Extension::GGPStreamDescriptorSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateStreamDescriptorSurfaceGgp),
                proc: unsafe { std::mem::transmute(Self::create_stream_descriptor_surface_ggp as vk::PFN_vkCreateStreamDescriptorSurfaceGGP)},
            },
            VulkanCommand {
                name: "vkCreateViSurfaceNN",
                features: smallvec![Feature::Extension(Extension::NNViSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateViSurfaceNn),
                proc: unsafe { std::mem::transmute(Self::create_vi_surface_nn as vk::PFN_vkCreateViSurfaceNN)},
            },
            VulkanCommand {
                name: "vkCreateWaylandSurfaceKHR",
                features: smallvec![Feature::Extension(Extension::KHRWaylandSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateWaylandSurfaceKhr),
                proc: unsafe { std::mem::transmute(Self::create_wayland_surface_khr as vk::PFN_vkCreateWaylandSurfaceKHR)},
            },
            VulkanCommand {
                name: "vkCreateWin32SurfaceKHR",
                features: smallvec![Feature::Extension(Extension::KHRWin32Surface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateWin32SurfaceKhr),
                proc: unsafe { std::mem::transmute(Self::create_win32_surface_khr as vk::PFN_vkCreateWin32SurfaceKHR)},
            },
            VulkanCommand {
                name: "vkCreateXcbSurfaceKHR",
                features: smallvec![Feature::Extension(Extension::KHRXcbSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateXcbSurfaceKhr),
                proc: unsafe { std::mem::transmute(Self::create_xcb_surface_khr as vk::PFN_vkCreateXcbSurfaceKHR)},
            },
            VulkanCommand {
                name: "vkCreateXlibSurfaceKHR",
                features: smallvec![Feature::Extension(Extension::KHRXlibSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::CreateXlibSurfaceKhr),
                proc: unsafe { std::mem::transmute(Self::create_xlib_surface_khr as vk::PFN_vkCreateXlibSurfaceKHR)},
            },
            VulkanCommand {
                name: "vkDebugReportMessageEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugReport)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DebugReportMessageExt),
                proc: unsafe { std::mem::transmute(Self::debug_report_message_ext as vk::PFN_vkDebugReportMessageEXT)},
            },
            VulkanCommand {
                name: "vkDestroyDebugReportCallbackEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugReport)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyDebugReportCallbackExt),
                proc: unsafe { std::mem::transmute(Self::destroy_debug_report_callback_ext as vk::PFN_vkDestroyDebugReportCallbackEXT)},
            },
            VulkanCommand {
                name: "vkDestroyDebugUtilsMessengerEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroyDebugUtilsMessengerExt),
                proc: unsafe { std::mem::transmute(Self::destroy_debug_utils_messenger_ext as vk::PFN_vkDestroyDebugUtilsMessengerEXT)},
            },
            VulkanCommand {
                name: "vkDestroyInstance",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: true,
                proc: unsafe { std::mem::transmute(Self::destroy_instance as vk::PFN_vkDestroyInstance)},
            },
            VulkanCommand {
                name: "vkDestroySurfaceKHR",
                features: smallvec![Feature::Extension(Extension::KHRSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::DestroySurfaceKhr),
                proc: unsafe { std::mem::transmute(Self::destroy_surface_khr as vk::PFN_vkDestroySurfaceKHR)},
            },
            VulkanCommand {
                name: "vkEnumerateDeviceExtensionProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: true,
                proc: unsafe { std::mem::transmute(Self::enumerate_device_extension_properties as vk::PFN_vkEnumerateDeviceExtensionProperties)},
            },
            VulkanCommand {
                name: "vkEnumerateDeviceLayerProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: true,
                proc: unsafe { std::mem::transmute(Self::enumerate_device_layer_properties as vk::PFN_vkEnumerateDeviceLayerProperties)},
            },
            VulkanCommand {
                name: "vkEnumeratePhysicalDeviceGroups",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: true,
                proc: unsafe { std::mem::transmute(Self::enumerate_physical_device_groups as vk::PFN_vkEnumeratePhysicalDeviceGroups)},
            },
            VulkanCommand {
                name: "vkEnumeratePhysicalDeviceGroupsKHR",
                features: smallvec![Feature::Extension(Extension::KHRDeviceGroupCreation)],
                hooked: true,
                proc: unsafe { std::mem::transmute(Self::enumerate_physical_device_groups as vk::PFN_vkEnumeratePhysicalDeviceGroups)},
            },
            VulkanCommand {
                name: "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR",
                features: smallvec![Feature::Extension(Extension::KHRPerformanceQuery)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKhr),
                proc: unsafe { std::mem::transmute(Self::enumerate_physical_device_queue_family_performance_query_counters_khr as vk::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR)},
            },
            VulkanCommand {
                name: "vkEnumeratePhysicalDevices",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: true,
                proc: unsafe { std::mem::transmute(Self::enumerate_physical_devices as vk::PFN_vkEnumeratePhysicalDevices)},
            },
            VulkanCommand {
                name: "vkGetDisplayModeProperties2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetDisplayProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetDisplayModeProperties2Khr),
                proc: unsafe { std::mem::transmute(Self::get_display_mode_properties2_khr as vk::PFN_vkGetDisplayModeProperties2KHR)},
            },
            VulkanCommand {
                name: "vkGetDisplayModePropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetDisplayModePropertiesKhr),
                proc: unsafe { std::mem::transmute(Self::get_display_mode_properties_khr as vk::PFN_vkGetDisplayModePropertiesKHR)},
            },
            VulkanCommand {
                name: "vkGetDisplayPlaneCapabilities2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetDisplayProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetDisplayPlaneCapabilities2Khr),
                proc: unsafe { std::mem::transmute(Self::get_display_plane_capabilities2_khr as vk::PFN_vkGetDisplayPlaneCapabilities2KHR)},
            },
            VulkanCommand {
                name: "vkGetDisplayPlaneCapabilitiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetDisplayPlaneCapabilitiesKhr),
                proc: unsafe { std::mem::transmute(Self::get_display_plane_capabilities_khr as vk::PFN_vkGetDisplayPlaneCapabilitiesKHR)},
            },
            VulkanCommand {
                name: "vkGetDisplayPlaneSupportedDisplaysKHR",
                features: smallvec![Feature::Extension(Extension::KHRDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetDisplayPlaneSupportedDisplaysKhr),
                proc: unsafe { std::mem::transmute(Self::get_display_plane_supported_displays_khr as vk::PFN_vkGetDisplayPlaneSupportedDisplaysKHR)},
            },
            VulkanCommand {
                name: "vkGetDrmDisplayEXT",
                features: smallvec![Feature::Extension(Extension::EXTAcquireDrmDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetDrmDisplayExt),
                proc: unsafe { std::mem::transmute(Self::get_drm_display_ext as vk::PFN_vkGetDrmDisplayEXT)},
            },
            VulkanCommand {
                name: "vkGetInstanceProcAddr",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: true,
                proc: unsafe { std::mem::transmute(Self::get_instance_proc_addr as vk::PFN_vkGetInstanceProcAddr)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT",
                features: smallvec![Feature::Extension(Extension::EXTCalibratedTimestamps)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceCalibrateableTimeDomainsExt),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_calibrateable_time_domains_ext as vk::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV",
                features: smallvec![Feature::Extension(Extension::NVCooperativeMatrix)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceCooperativeMatrixPropertiesNv),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_cooperative_matrix_properties_nv as vk::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceDirectFBPresentationSupportEXT",
                features: smallvec![Feature::Extension(Extension::EXTDirectfbSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceDirectFbPresentationSupportExt),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_direct_fb_presentation_support_ext as vk::PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceDisplayPlaneProperties2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetDisplayProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceDisplayPlaneProperties2Khr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_display_plane_properties2_khr as vk::PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceDisplayPlanePropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceDisplayPlanePropertiesKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_display_plane_properties_khr as vk::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceDisplayProperties2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetDisplayProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceDisplayProperties2Khr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_display_properties2_khr as vk::PFN_vkGetPhysicalDeviceDisplayProperties2KHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceDisplayPropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceDisplayPropertiesKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_display_properties_khr as vk::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceExternalBufferProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceExternalBufferProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_external_buffer_properties as vk::PFN_vkGetPhysicalDeviceExternalBufferProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceExternalBufferPropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalMemoryCapabilities)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceExternalBufferProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_external_buffer_properties as vk::PFN_vkGetPhysicalDeviceExternalBufferProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceExternalFenceProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceExternalFenceProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_external_fence_properties as vk::PFN_vkGetPhysicalDeviceExternalFenceProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceExternalFencePropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalFenceCapabilities)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceExternalFenceProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_external_fence_properties as vk::PFN_vkGetPhysicalDeviceExternalFenceProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceExternalImageFormatPropertiesNV",
                features: smallvec![Feature::Extension(Extension::NVExternalMemoryCapabilities)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceExternalImageFormatPropertiesNv),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_external_image_format_properties_nv as vk::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceExternalSemaphoreProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceExternalSemaphoreProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_external_semaphore_properties as vk::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRExternalSemaphoreCapabilities)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceExternalSemaphoreProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_external_semaphore_properties as vk::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceFeatures",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceFeatures),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_features as vk::PFN_vkGetPhysicalDeviceFeatures)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceFeatures2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceFeatures2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_features2 as vk::PFN_vkGetPhysicalDeviceFeatures2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceFeatures2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetPhysicalDeviceProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceFeatures2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_features2 as vk::PFN_vkGetPhysicalDeviceFeatures2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceFormatProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceFormatProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_format_properties as vk::PFN_vkGetPhysicalDeviceFormatProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceFormatProperties2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceFormatProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_format_properties2 as vk::PFN_vkGetPhysicalDeviceFormatProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceFormatProperties2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetPhysicalDeviceProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceFormatProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_format_properties2 as vk::PFN_vkGetPhysicalDeviceFormatProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceFragmentShadingRatesKHR",
                features: smallvec![Feature::Extension(Extension::KHRFragmentShadingRate)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceFragmentShadingRatesKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_fragment_shading_rates_khr as vk::PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceImageFormatProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceImageFormatProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_image_format_properties as vk::PFN_vkGetPhysicalDeviceImageFormatProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceImageFormatProperties2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceImageFormatProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_image_format_properties2 as vk::PFN_vkGetPhysicalDeviceImageFormatProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceImageFormatProperties2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetPhysicalDeviceProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceImageFormatProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_image_format_properties2 as vk::PFN_vkGetPhysicalDeviceImageFormatProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceMemoryProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceMemoryProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_memory_properties as vk::PFN_vkGetPhysicalDeviceMemoryProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceMemoryProperties2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceMemoryProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_memory_properties2 as vk::PFN_vkGetPhysicalDeviceMemoryProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceMemoryProperties2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetPhysicalDeviceProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceMemoryProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_memory_properties2 as vk::PFN_vkGetPhysicalDeviceMemoryProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceMultisamplePropertiesEXT",
                features: smallvec![Feature::Extension(Extension::EXTSampleLocations)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceMultisamplePropertiesExt),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_multisample_properties_ext as vk::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceOpticalFlowImageFormatsNV",
                features: smallvec![Feature::Extension(Extension::NVOpticalFlow)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceOpticalFlowImageFormatsNv),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_optical_flow_image_formats_nv as vk::PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDevicePresentRectanglesKHR",
                features: smallvec![Feature::Extension(Extension::KHRSwapchain), Feature::Extension(Extension::KHRDeviceGroup)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDevicePresentRectanglesKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_present_rectangles_khr as vk::PFN_vkGetPhysicalDevicePresentRectanglesKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_properties as vk::PFN_vkGetPhysicalDeviceProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceProperties2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_properties2 as vk::PFN_vkGetPhysicalDeviceProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceProperties2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetPhysicalDeviceProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_properties2 as vk::PFN_vkGetPhysicalDeviceProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR",
                features: smallvec![Feature::Extension(Extension::KHRPerformanceQuery)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceQueueFamilyPerformanceQueryPassesKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_queue_family_performance_query_passes_khr as vk::PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceQueueFamilyProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceQueueFamilyProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_queue_family_properties as vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceQueueFamilyProperties2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceQueueFamilyProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_queue_family_properties2 as vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceQueueFamilyProperties2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetPhysicalDeviceProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceQueueFamilyProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_queue_family_properties2 as vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceScreenPresentationSupportQNX",
                features: smallvec![Feature::Extension(Extension::QNXScreenSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceScreenPresentationSupportQnx),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_screen_presentation_support_qnx as vk::PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSparseImageFormatProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 0})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSparseImageFormatProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_sparse_image_format_properties as vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSparseImageFormatProperties2",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 1})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSparseImageFormatProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_sparse_image_format_properties2 as vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSparseImageFormatProperties2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetPhysicalDeviceProperties2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSparseImageFormatProperties2),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_sparse_image_format_properties2 as vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV",
                features: smallvec![Feature::Extension(Extension::NVCoverageReductionMode)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNv),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_supported_framebuffer_mixed_samples_combinations_nv as vk::PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSurfaceCapabilities2EXT",
                features: smallvec![Feature::Extension(Extension::EXTDisplaySurfaceCounter)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSurfaceCapabilities2Ext),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_surface_capabilities2_ext as vk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSurfaceCapabilities2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetSurfaceCapabilities2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSurfaceCapabilities2Khr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_surface_capabilities2_khr as vk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSurfaceCapabilitiesKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_surface_capabilities_khr as vk::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSurfaceFormats2KHR",
                features: smallvec![Feature::Extension(Extension::KHRGetSurfaceCapabilities2)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSurfaceFormats2Khr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_surface_formats2_khr as vk::PFN_vkGetPhysicalDeviceSurfaceFormats2KHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSurfaceFormatsKHR",
                features: smallvec![Feature::Extension(Extension::KHRSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSurfaceFormatsKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_surface_formats_khr as vk::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSurfacePresentModes2EXT",
                features: smallvec![Feature::Extension(Extension::EXTFullScreenExclusive)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSurfacePresentModes2Ext),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_surface_present_modes2_ext as vk::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSurfacePresentModesKHR",
                features: smallvec![Feature::Extension(Extension::KHRSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSurfacePresentModesKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_surface_present_modes_khr as vk::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceSurfaceSupportKHR",
                features: smallvec![Feature::Extension(Extension::KHRSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceSurfaceSupportKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_surface_support_khr as vk::PFN_vkGetPhysicalDeviceSurfaceSupportKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceToolProperties",
                features: smallvec![Feature::Core(ApiVersion { major: 1, minor: 3})],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceToolProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_tool_properties as vk::PFN_vkGetPhysicalDeviceToolProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceToolPropertiesEXT",
                features: smallvec![Feature::Extension(Extension::EXTToolingInfo)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceToolProperties),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_tool_properties as vk::PFN_vkGetPhysicalDeviceToolProperties)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceVideoCapabilitiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceVideoCapabilitiesKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_video_capabilities_khr as vk::PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceVideoFormatPropertiesKHR",
                features: smallvec![Feature::Extension(Extension::KHRVideoQueue)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceVideoFormatPropertiesKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_video_format_properties_khr as vk::PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceWaylandPresentationSupportKHR",
                features: smallvec![Feature::Extension(Extension::KHRWaylandSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceWaylandPresentationSupportKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_wayland_presentation_support_khr as vk::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceWin32PresentationSupportKHR",
                features: smallvec![Feature::Extension(Extension::KHRWin32Surface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceWin32PresentationSupportKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_win32_presentation_support_khr as vk::PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceXcbPresentationSupportKHR",
                features: smallvec![Feature::Extension(Extension::KHRXcbSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceXcbPresentationSupportKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_xcb_presentation_support_khr as vk::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR)},
            },
            VulkanCommand {
                name: "vkGetPhysicalDeviceXlibPresentationSupportKHR",
                features: smallvec![Feature::Extension(Extension::KHRXlibSurface)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetPhysicalDeviceXlibPresentationSupportKhr),
                proc: unsafe { std::mem::transmute(Self::get_physical_device_xlib_presentation_support_khr as vk::PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR)},
            },
            VulkanCommand {
                name: "vkGetRandROutputDisplayEXT",
                features: smallvec![Feature::Extension(Extension::EXTAcquireXlibDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetRandROutputDisplayExt),
                proc: unsafe { std::mem::transmute(Self::get_rand_r_output_display_ext as vk::PFN_vkGetRandROutputDisplayEXT)},
            },
            VulkanCommand {
                name: "vkGetWinrtDisplayNV",
                features: smallvec![Feature::Extension(Extension::NVAcquireWinrtDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::GetWinrtDisplayNv),
                proc: unsafe { std::mem::transmute(Self::get_winrt_display_nv as vk::PFN_vkGetWinrtDisplayNV)},
            },
            VulkanCommand {
                name: "vkReleaseDisplayEXT",
                features: smallvec![Feature::Extension(Extension::EXTDirectModeDisplay)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::ReleaseDisplayExt),
                proc: unsafe { std::mem::transmute(Self::release_display_ext as vk::PFN_vkReleaseDisplayEXT)},
            },
            VulkanCommand {
                name: "vkSubmitDebugUtilsMessageEXT",
                features: smallvec![Feature::Extension(Extension::EXTDebugUtils)],
                hooked: hooked_commands.contains(&LayerVulkanCommand::SubmitDebugUtilsMessageExt),
                proc: unsafe { std::mem::transmute(Self::submit_debug_utils_message_ext as vk::PFN_vkSubmitDebugUtilsMessageEXT)},
            },
        ])
    }
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
            .borrow()
            .hooks()
            .get_physical_device_features(physical_device, unsafe { p_features.as_mut() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_format_properties(
                physical_device,
                format,
                unsafe { p_format_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_image_format_properties(
                physical_device,
                format,
                _type,
                tiling,
                usage,
                flags,
                unsafe { p_image_format_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
            .get_physical_device_properties(
                physical_device,
                unsafe { p_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_queue_family_properties(
                physical_device,
                if p_queue_family_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_queue_family_properties,
                            *unsafe { p_queue_family_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_memory_properties(
                physical_device,
                unsafe { p_memory_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_sparse_image_format_properties(
                physical_device,
                format,
                _type,
                samples,
                usage,
                tiling,
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_features2(
                physical_device,
                unsafe { p_features.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_properties2(
                physical_device,
                unsafe { p_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_format_properties2(
                physical_device,
                format,
                unsafe { p_format_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_image_format_properties2(
                physical_device,
                unsafe { p_image_format_info.as_ref() }.unwrap(),
                unsafe { p_image_format_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_queue_family_properties2(
                physical_device,
                if p_queue_family_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_queue_family_properties,
                            *unsafe { p_queue_family_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_memory_properties2(
                physical_device,
                unsafe { p_memory_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_sparse_image_format_properties2(
                physical_device,
                unsafe { p_format_info.as_ref() }.unwrap(),
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_external_buffer_properties(
                physical_device,
                unsafe { p_external_buffer_info.as_ref() }.unwrap(),
                unsafe { p_external_buffer_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_external_fence_properties(
                physical_device,
                unsafe { p_external_fence_info.as_ref() }.unwrap(),
                unsafe { p_external_fence_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_physical_device_external_semaphore_properties(
                physical_device,
                unsafe { p_external_semaphore_info.as_ref() }.unwrap(),
                unsafe { p_external_semaphore_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_tool_properties(
                physical_device,
                if p_tool_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_tool_properties,
                            *unsafe { p_tool_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_physical_device_surface_capabilities_khr(
                physical_device,
                surface,
                unsafe { p_surface_capabilities.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_surface_formats_khr(
                physical_device,
                surface,
                if p_surface_formats.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_surface_formats,
                            *unsafe { p_surface_format_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_present_rectangles_khr(
                physical_device,
                surface,
                if p_rects.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_rects,
                            *unsafe { p_rect_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_display_properties_khr(
                physical_device,
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_display_plane_properties_khr(
                physical_device,
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_display_mode_properties_khr(
                physical_device,
                display,
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .create_display_mode_khr(
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
            .borrow()
            .hooks()
            .get_display_plane_capabilities_khr(
                physical_device,
                mode,
                plane_index,
                unsafe { p_capabilities.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_physical_device_video_capabilities_khr(
                physical_device,
                unsafe { p_video_profile.as_ref() }.unwrap(),
                unsafe { p_capabilities.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_video_format_properties_khr(
                physical_device,
                unsafe { p_video_format_info.as_ref() }.unwrap(),
                if p_video_format_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_video_format_properties,
                            *unsafe { p_video_format_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
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
                if p_counter_descriptions.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_counter_descriptions,
                            *unsafe { p_counter_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_physical_device_surface_capabilities2_khr(
                physical_device,
                unsafe { p_surface_info.as_ref() }.unwrap(),
                unsafe { p_surface_capabilities.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_surface_formats2_khr(
                physical_device,
                unsafe { p_surface_info.as_ref() }.unwrap(),
                if p_surface_formats.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_surface_formats,
                            *unsafe { p_surface_format_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_display_properties2_khr(
                physical_device,
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_display_plane_properties2_khr(
                physical_device,
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_display_mode_properties2_khr(
                physical_device,
                display,
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
            .get_display_plane_capabilities2_khr(
                physical_device,
                unsafe { p_display_plane_info.as_ref() }.unwrap(),
                unsafe { p_capabilities.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_fragment_shading_rates_khr(
                physical_device,
                if p_fragment_shading_rates.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_fragment_shading_rates,
                            *unsafe { p_fragment_shading_rate_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .debug_report_message_ext(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_physical_device_external_image_format_properties_nv(
                physical_device,
                format,
                _type,
                tiling,
                usage,
                flags,
                external_handle_type,
                unsafe { p_external_image_format_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .acquire_xlib_display_ext(physical_device, unsafe { dpy.as_mut() }.unwrap(), display);
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
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_rand_r_output_display_ext(
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
            .borrow()
            .hooks()
            .get_physical_device_surface_capabilities2_ext(
                physical_device,
                surface,
                unsafe { p_surface_capabilities.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_physical_device_multisample_properties_ext(
                physical_device,
                samples,
                unsafe { p_multisample_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_cooperative_matrix_properties_nv(
                physical_device,
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_property_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
                physical_device,
                if p_combinations.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_combinations,
                            *unsafe { p_combination_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
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
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_drm_display_ext(physical_device, drm_fd, connector_id);
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = instance_info
            .customized_info
            .borrow()
            .hooks()
            .get_physical_device_optical_flow_image_formats_nv(
                physical_device,
                unsafe { p_optical_flow_image_format_info.as_ref() }.unwrap(),
                if p_image_format_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_image_format_properties,
                            *unsafe { p_format_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
        let layer_result = device_info.customized_info.borrow().hooks().queue_submit(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .queue_wait_idle(queue);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .device_wait_idle();
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .unmap_memory(memory);
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_buffer_memory_requirements(
                buffer,
                unsafe { p_memory_requirements.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_image_memory_requirements(
                image,
                unsafe { p_memory_requirements.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_image_sparse_memory_requirements(
                image,
                if p_sparse_memory_requirements.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_sparse_memory_requirements,
                            *unsafe { p_sparse_memory_requirement_count.as_ref() }.unwrap()
                                as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .queue_bind_sparse(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_fence_status(fence);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .wait_for_fences(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_event_status(event);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .set_event(event);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .reset_event(event);
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_query_pool_results(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_image_subresource_layout(
                image,
                unsafe { p_subresource.as_ref() }.unwrap(),
                unsafe { p_layout.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .create_graphics_pipelines(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .create_compute_pipelines(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .update_descriptor_sets(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_render_area_granularity(render_pass, unsafe { p_granularity.as_mut() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_bind_pipeline(command_buffer, pipeline_bind_point, pipeline);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_depth_bias(
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_depth_bounds(command_buffer, min_depth_bounds, max_depth_bounds);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_stencil_compare_mask(command_buffer, face_mask, compare_mask);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_stencil_write_mask(command_buffer, face_mask, write_mask);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_stencil_reference(command_buffer, face_mask, reference);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_bind_descriptor_sets(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                unsafe {
                    std::slice::from_raw_parts(p_descriptor_sets, descriptor_set_count as usize)
                },
                unsafe {
                    std::slice::from_raw_parts(p_dynamic_offsets, dynamic_offset_count as usize)
                },
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_bind_index_buffer(command_buffer, buffer, offset, index_type);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_bind_vertex_buffers(
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
        let layer_result = device_info.customized_info.borrow().hooks().cmd_draw(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_draw_indexed(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_draw_indirect(command_buffer, buffer, offset, draw_count, stride);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_draw_indexed_indirect(command_buffer, buffer, offset, draw_count, stride);
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
        let layer_result = device_info.customized_info.borrow().hooks().cmd_dispatch(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_copy_buffer(command_buffer, src_buffer, dst_buffer, unsafe {
                std::slice::from_raw_parts(p_regions, region_count as usize)
            });
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
        let layer_result = device_info.customized_info.borrow().hooks().cmd_copy_image(
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
        let layer_result = device_info.customized_info.borrow().hooks().cmd_blit_image(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_copy_buffer_to_image(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_copy_image_to_buffer(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_update_buffer(command_buffer, dst_buffer, dst_offset, unsafe {
                std::slice::from_raw_parts(p_data as *const u8, data_size as usize)
            });
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_fill_buffer(command_buffer, dst_buffer, dst_offset, size, data);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_clear_color_image(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_clear_depth_stencil_image(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_clear_attachments(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_resolve_image(
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
        let layer_result = device_info.customized_info.borrow().hooks().cmd_set_event(
            command_buffer,
            event,
            stage_mask,
        );
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_wait_events(
                command_buffer,
                unsafe { std::slice::from_raw_parts(p_events, event_count as usize) },
                src_stage_mask,
                dst_stage_mask,
                unsafe {
                    std::slice::from_raw_parts(p_memory_barriers, memory_barrier_count as usize)
                },
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_pipeline_barrier(
                command_buffer,
                src_stage_mask,
                dst_stage_mask,
                dependency_flags,
                unsafe {
                    std::slice::from_raw_parts(p_memory_barriers, memory_barrier_count as usize)
                },
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
        let layer_result = device_info.customized_info.borrow().hooks().cmd_end_query(
            command_buffer,
            query_pool,
            query,
        );
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_reset_query_pool(command_buffer, query_pool, first_query, query_count);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_write_timestamp(command_buffer, pipeline_stage, query_pool, query);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_copy_query_pool_results(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_push_constants(command_buffer, layout, stage_flags, offset, unsafe {
                std::slice::from_raw_parts(p_values as *const u8, size as usize)
            });
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_begin_render_pass(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .bind_buffer_memory2(unsafe {
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .bind_image_memory2(unsafe {
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_dispatch_base(
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
            .borrow()
            .hooks()
            .get_image_memory_requirements2(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_memory_requirements.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_buffer_memory_requirements2(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_memory_requirements.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_image_sparse_memory_requirements2(
                unsafe { p_info.as_ref() }.unwrap(),
                if p_sparse_memory_requirements.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_sparse_memory_requirements,
                            *unsafe { p_sparse_memory_requirement_count.as_ref() }.unwrap()
                                as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_descriptor_set_layout_support(
                unsafe { p_create_info.as_ref() }.unwrap(),
                unsafe { p_support.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_draw_indirect_count(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_draw_indexed_indirect_count(
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_begin_render_pass2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_next_subpass2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_end_render_pass2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .set_private_data(object_type, object_handle, private_data_slot, data);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_private_data(object_type, object_handle, private_data_slot);
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
        let layer_result = device_info.customized_info.borrow().hooks().cmd_set_event2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_wait_events2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_pipeline_barrier2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_write_timestamp2(command_buffer, stage, query_pool, query);
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
        let layer_result = device_info.customized_info.borrow().hooks().queue_submit2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_copy_buffer2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_copy_image2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_copy_buffer_to_image2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_copy_image_to_buffer2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_blit_image2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_resolve_image2(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_begin_rendering(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_bind_vertex_buffers2(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_stencil_op(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_device_buffer_memory_requirements(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_memory_requirements.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_device_image_memory_requirements(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_memory_requirements.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_device_image_sparse_memory_requirements(
                unsafe { p_info.as_ref() }.unwrap(),
                if p_sparse_memory_requirements.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_sparse_memory_requirements,
                            *unsafe { p_sparse_memory_requirement_count.as_ref() }.unwrap()
                                as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_device_group_present_capabilities_khr(
                unsafe { p_device_group_present_capabilities.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .create_shared_swapchains_khr(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_video_session_memory_requirements_khr(
                video_session,
                if p_memory_requirements.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_memory_requirements,
                            *unsafe { p_memory_requirements_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_end_video_coding_khr(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_control_video_coding_khr(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_memory_win32_handle_properties_khr(
                handle_type,
                handle,
                unsafe { p_memory_win32_handle_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_memory_fd_properties_khr(
                handle_type,
                fd,
                unsafe { p_memory_fd_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_push_descriptor_set_khr(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .import_fence_win32_handle_khr(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .release_profiling_lock_khr();
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_pipeline_executable_properties_khr(
                unsafe { p_pipeline_info.as_ref() }.unwrap(),
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_executable_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_pipeline_executable_statistics_khr(
                unsafe { p_executable_info.as_ref() }.unwrap(),
                if p_statistics.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_statistics,
                            *unsafe { p_statistic_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_pipeline_executable_internal_representations_khr(
                unsafe { p_executable_info.as_ref() }.unwrap(),
                if p_internal_representations.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_internal_representations,
                            *unsafe { p_internal_representation_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_write_buffer_marker2_amd(command_buffer, stage, dst_buffer, dst_offset, marker);
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_queue_checkpoint_data2_nv(
                queue,
                if p_checkpoint_data.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_checkpoint_data,
                            *unsafe { p_checkpoint_data_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
    extern "system" fn get_swapchain_gralloc_usage_android(
        device: vk::Device,
        format: vk::Format,
        image_usage: vk::ImageUsageFlags,
        gralloc_usage: *mut c_int,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetSwapchainGrallocUsageANDROID
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.android_native_buffer;
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_swapchain_gralloc_usage_android(format, image_usage);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { gralloc_usage.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_swapchain_gralloc_usage_android)(
                    device,
                    format,
                    image_usage,
                    gralloc_usage,
                )
            },
        }
    }
    extern "system" fn acquire_image_android(
        device: vk::Device,
        image: vk::Image,
        native_fence_fd: c_int,
        semaphore: vk::Semaphore,
        fence: vk::Fence,
    ) -> vk::Result {
        let global = Self::instance();
        // vkAcquireImageANDROID
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.android_native_buffer;
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .acquire_image_android(image, native_fence_fd, semaphore, fence);
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.acquire_image_android)(
                    device,
                    image,
                    native_fence_fd,
                    semaphore,
                    fence,
                )
            },
        }
    }
    extern "system" fn queue_signal_release_image_android(
        queue: vk::Queue,
        wait_semaphore_count: u32,
        p_wait_semaphores: *const vk::Semaphore,
        image: vk::Image,
        p_native_fence_fd: *mut c_int,
    ) -> vk::Result {
        let global = Self::instance();
        // vkQueueSignalReleaseImageANDROID
        let device_info = global.get_device_info(queue).unwrap();
        let dispatch_table = &device_info.dispatch_table.android_native_buffer;
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .queue_signal_release_image_android(
                queue,
                unsafe {
                    std::slice::from_raw_parts(p_wait_semaphores, wait_semaphore_count as usize)
                },
                image,
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { p_native_fence_fd.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.queue_signal_release_image_android)(
                    queue,
                    wait_semaphore_count,
                    p_wait_semaphores,
                    image,
                    p_native_fence_fd,
                )
            },
        }
    }
    extern "system" fn get_swapchain_gralloc_usage2_android(
        device: vk::Device,
        format: vk::Format,
        image_usage: vk::ImageUsageFlags,
        swapchain_image_usage: vk::SwapchainImageUsageFlagsANDROID,
        gralloc_consumer_usage: *mut u64,
        gralloc_producer_usage: *mut u64,
    ) -> vk::Result {
        let global = Self::instance();
        // vkGetSwapchainGrallocUsage2ANDROID
        let device_info = global.get_device_info(device).unwrap();
        let dispatch_table = &device_info.dispatch_table.android_native_buffer;
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_swapchain_gralloc_usage2_android(
                format,
                image_usage,
                swapchain_image_usage,
                unsafe { gralloc_consumer_usage.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(res) => {
                    *unsafe { gralloc_producer_usage.as_mut() }.unwrap() = res;
                    vk::Result::SUCCESS
                }
                Err(e) => e,
            },
            LayerResult::Unhandled => unsafe {
                (dispatch_table.get_swapchain_gralloc_usage2_android)(
                    device,
                    format,
                    image_usage,
                    swapchain_image_usage,
                    gralloc_consumer_usage,
                    gralloc_producer_usage,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_debug_marker_insert_ext(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_end_transform_feedback_ext(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_begin_query_indexed_ext(command_buffer, query_pool, query, flags, index);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_end_query_indexed_ext(command_buffer, query_pool, query, index);
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_image_view_address_nvx(image_view, unsafe { p_properties.as_mut() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_viewport_w_scaling_nv(command_buffer, first_viewport, unsafe {
                std::slice::from_raw_parts(p_viewport_w_scalings, viewport_count as usize)
            });
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .register_display_event_ext(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_refresh_cycle_duration_google(
                swapchain,
                unsafe { p_display_timing_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_past_presentation_timing_google(
                swapchain,
                if p_presentation_timings.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_presentation_timings,
                            *unsafe { p_presentation_timing_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_discard_rectangle_ext(command_buffer, first_discard_rectangle, unsafe {
                std::slice::from_raw_parts(p_discard_rectangles, discard_rectangle_count as usize)
            });
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .set_hdr_metadata_ext(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_begin_debug_utils_label_ext(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_android_hardware_buffer_properties_android(
                unsafe { buffer.as_ref() }.unwrap(),
                unsafe { p_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_sample_locations_ext(
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
            .borrow()
            .hooks()
            .get_image_drm_format_modifier_properties_ext(
                image,
                unsafe { p_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_bind_shading_rate_image_nv(command_buffer, image_view, image_layout);
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_coarse_sample_order_nv(command_buffer, sample_order_type, unsafe {
                std::slice::from_raw_parts(
                    p_custom_sample_orders,
                    custom_sample_order_count as usize,
                )
            });
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_acceleration_structure_memory_requirements_nv(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_memory_requirements.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_trace_rays_nv(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .create_ray_tracing_pipelines_nv(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_memory_host_pointer_properties_ext(
                handle_type,
                unsafe { p_host_pointer.as_ref() }.unwrap(),
                unsafe { p_memory_host_pointer_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_write_buffer_marker_amd(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_calibrated_timestamps_ext(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_draw_mesh_tasks_nv(command_buffer, task_count, first_task);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_draw_mesh_tasks_indirect_nv(command_buffer, buffer, offset, draw_count, stride);
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_exclusive_scissor_nv(command_buffer, first_exclusive_scissor, unsafe {
                std::slice::from_raw_parts(p_exclusive_scissors, exclusive_scissor_count as usize)
            });
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_checkpoint_nv(
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_queue_checkpoint_data_nv(
                queue,
                if p_checkpoint_data.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_checkpoint_data,
                            *unsafe { p_checkpoint_data_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_performance_parameter_intel(parameter, unsafe { p_value.as_mut() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_line_stipple_ext(command_buffer, line_stipple_factor, line_stipple_pattern);
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_generated_commands_memory_requirements_nv(
                unsafe { p_info.as_ref() }.unwrap(),
                unsafe { p_memory_requirements.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .export_metal_objects_ext(unsafe { p_metal_objects_info.as_mut() }.unwrap());
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_image_subresource_layout2_ext(
                image,
                unsafe { p_subresource.as_ref() }.unwrap(),
                unsafe { p_layout.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_vertex_input_ext(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_memory_zircon_handle_properties_fuchsia(
                handle_type,
                zircon_handle,
                unsafe { p_memory_zircon_handle_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_buffer_collection_properties_fuchsia(
                collection,
                unsafe { p_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
            .get_device_subpass_shading_max_workgroup_size_huawei(
                renderpass,
                unsafe { p_max_workgroup_size.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_bind_invocation_mask_huawei(command_buffer, image_view, image_layout);
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_memory_remote_address_nv(
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
            .borrow()
            .hooks()
            .get_pipeline_properties_ext(
                unsafe { p_pipeline_info.as_ref() }.unwrap(),
                unsafe { p_pipeline_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_color_write_enable_ext(
                command_buffer,
                unsafe {
                    std::slice::from_raw_parts(p_color_write_enables, attachment_count as usize)
                }
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_draw_multi_ext(
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_draw_multi_indexed_ext(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .write_micromaps_properties_ext(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_micromap_build_sizes_ext(
                build_type,
                unsafe { p_build_info.as_ref() }.unwrap(),
                unsafe { p_size_info.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_descriptor_set_layout_host_mapping_info_valve(
                unsafe { p_binding_reference.as_ref() }.unwrap(),
                unsafe { p_host_mapping.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_copy_memory_indirect_nv(command_buffer, copy_buffer_address, copy_count, stride);
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_sample_mask_ext(command_buffer, samples, unsafe {
                std::slice::from_raw_parts(p_sample_mask, ((samples.as_raw() + 31) / 32) as usize)
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_color_blend_enable_ext(
                command_buffer,
                first_attachment,
                unsafe {
                    std::slice::from_raw_parts(p_color_blend_enables, attachment_count as usize)
                }
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_color_write_mask_ext(command_buffer, first_attachment, unsafe {
                std::slice::from_raw_parts(p_color_write_masks, attachment_count as usize)
            });
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_set_viewport_swizzle_nv(command_buffer, first_viewport, unsafe {
                std::slice::from_raw_parts(p_viewport_swizzles, viewport_count as usize)
            });
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
            .get_shader_module_identifier_ext(
                shader_module,
                unsafe { p_identifier.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
            .get_shader_module_create_info_identifier_ext(
                unsafe { p_create_info.as_ref() }.unwrap(),
                unsafe { p_identifier.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_optical_flow_execute_nv(
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
        #[allow(clippy::unnecessary_cast)]
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .get_framebuffer_tile_properties_qcom(
                framebuffer,
                if p_properties.is_null() {
                    None
                } else {
                    Some(unsafe {
                        std::slice::from_raw_parts_mut(
                            p_properties,
                            *unsafe { p_properties_count.as_ref() }.unwrap() as usize,
                        )
                    })
                },
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
            .get_dynamic_rendering_tile_properties_qcom(
                unsafe { p_rendering_info.as_ref() }.unwrap(),
                unsafe { p_properties.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => match res {
                Ok(()) => vk::Result::SUCCESS,
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .copy_acceleration_structure_khr(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
                unsafe { p_size_info.as_mut() }.unwrap(),
            );
        match layer_result {
            LayerResult::Handled(res) => res,
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_trace_rays_khr(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_trace_rays_indirect_khr(
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
        let layer_result = device_info
            .customized_info
            .borrow()
            .hooks()
            .cmd_draw_mesh_tasks_ext(command_buffer, group_countx, group_county, group_countz);
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
            .borrow()
            .hooks()
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
            .borrow()
            .hooks()
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
