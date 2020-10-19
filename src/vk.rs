// Oliver Berzs
// https://github.com/oberzs/draw-it

// Vulkan Rust bindings generated from vk.xml

#![macro_use]
#![allow(non_camel_case_types, unused, improper_ctypes)]

use std::os::raw::c_char;
use std::os::raw::c_void;

// Basetypes
pub type SampleMask = u32;
pub type Bool32 = u32;
pub type Flags = u32;
pub type DeviceSize = u64;
pub type DeviceAddress = u64;

// Handles
pub type Instance = usize;
pub type PhysicalDevice = usize;
pub type Device = usize;
pub type Queue = usize;
pub type CommandBuffer = usize;
pub type DeviceMemory = u64;
pub type CommandPool = u64;
pub type Buffer = u64;
pub type BufferView = u64;
pub type Image = u64;
pub type ImageView = u64;
pub type ShaderModule = u64;
pub type Pipeline = u64;
pub type PipelineLayout = u64;
pub type Sampler = u64;
pub type DescriptorSet = u64;
pub type DescriptorSetLayout = u64;
pub type DescriptorPool = u64;
pub type Fence = u64;
pub type Semaphore = u64;
pub type Event = u64;
pub type QueryPool = u64;
pub type Framebuffer = u64;
pub type RenderPass = u64;
pub type PipelineCache = u64;
pub type IndirectCommandsLayoutNV = u64;
pub type DescriptorUpdateTemplate = u64;
pub type SamplerYcbcrConversion = u64;
pub type ValidationCacheEXT = u64;
pub type AccelerationStructureKHR = u64;
pub type PerformanceConfigurationINTEL = u64;
pub type DeferredOperationKHR = u64;
pub type PrivateDataSlotEXT = u64;
pub type DisplayKHR = u64;
pub type DisplayModeKHR = u64;
pub type SurfaceKHR = u64;
pub type SwapchainKHR = u64;
pub type DebugReportCallbackEXT = u64;
pub type DebugUtilsMessengerEXT = u64;

// Bitmasks
pub type FramebufferCreateFlags = FramebufferCreateFlagBits;
pub type QueryPoolCreateFlags = Flags;
pub type RenderPassCreateFlags = RenderPassCreateFlagBits;
pub type SamplerCreateFlags = SamplerCreateFlagBits;
pub type PipelineLayoutCreateFlags = Flags;
pub type PipelineCacheCreateFlags = PipelineCacheCreateFlagBits;
pub type PipelineDepthStencilStateCreateFlags = Flags;
pub type PipelineDynamicStateCreateFlags = Flags;
pub type PipelineColorBlendStateCreateFlags = Flags;
pub type PipelineMultisampleStateCreateFlags = Flags;
pub type PipelineRasterizationStateCreateFlags = Flags;
pub type PipelineViewportStateCreateFlags = Flags;
pub type PipelineTessellationStateCreateFlags = Flags;
pub type PipelineInputAssemblyStateCreateFlags = Flags;
pub type PipelineVertexInputStateCreateFlags = Flags;
pub type PipelineShaderStageCreateFlags = PipelineShaderStageCreateFlagBits;
pub type DescriptorSetLayoutCreateFlags = DescriptorSetLayoutCreateFlagBits;
pub type BufferViewCreateFlags = Flags;
pub type InstanceCreateFlags = Flags;
pub type DeviceCreateFlags = Flags;
pub type DeviceQueueCreateFlags = DeviceQueueCreateFlagBits;
pub type QueueFlags = QueueFlagBits;
pub type MemoryPropertyFlags = MemoryPropertyFlagBits;
pub type MemoryHeapFlags = MemoryHeapFlagBits;
pub type AccessFlags = AccessFlagBits;
pub type BufferUsageFlags = BufferUsageFlagBits;
pub type BufferCreateFlags = BufferCreateFlagBits;
pub type ShaderStageFlags = ShaderStageFlagBits;
pub type ImageUsageFlags = ImageUsageFlagBits;
pub type ImageCreateFlags = ImageCreateFlagBits;
pub type ImageViewCreateFlags = ImageViewCreateFlagBits;
pub type PipelineCreateFlags = PipelineCreateFlagBits;
pub type ColorComponentFlags = ColorComponentFlagBits;
pub type FenceCreateFlags = FenceCreateFlagBits;
pub type SemaphoreCreateFlags = Flags;
pub type FormatFeatureFlags = FormatFeatureFlagBits;
pub type QueryControlFlags = QueryControlFlagBits;
pub type QueryResultFlags = QueryResultFlagBits;
pub type ShaderModuleCreateFlags = ShaderModuleCreateFlagBits;
pub type EventCreateFlags = Flags;
pub type CommandPoolCreateFlags = CommandPoolCreateFlagBits;
pub type CommandPoolResetFlags = CommandPoolResetFlagBits;
pub type CommandBufferResetFlags = CommandBufferResetFlagBits;
pub type CommandBufferUsageFlags = CommandBufferUsageFlagBits;
pub type QueryPipelineStatisticFlags = QueryPipelineStatisticFlagBits;
pub type MemoryMapFlags = Flags;
pub type ImageAspectFlags = ImageAspectFlagBits;
pub type SparseMemoryBindFlags = SparseMemoryBindFlagBits;
pub type SparseImageFormatFlags = SparseImageFormatFlagBits;
pub type SubpassDescriptionFlags = SubpassDescriptionFlagBits;
pub type PipelineStageFlags = PipelineStageFlagBits;
pub type SampleCountFlags = SampleCountFlagBits;
pub type AttachmentDescriptionFlags = AttachmentDescriptionFlagBits;
pub type StencilFaceFlags = StencilFaceFlagBits;
pub type CullModeFlags = CullModeFlagBits;
pub type DescriptorPoolCreateFlags = DescriptorPoolCreateFlagBits;
pub type DescriptorPoolResetFlags = Flags;
pub type DependencyFlags = DependencyFlagBits;
pub type SubgroupFeatureFlags = SubgroupFeatureFlagBits;
pub type IndirectCommandsLayoutUsageFlagsNV = IndirectCommandsLayoutUsageFlagBitsNV;
pub type IndirectStateFlagsNV = IndirectStateFlagBitsNV;
pub type GeometryFlagsKHR = GeometryFlagBitsKHR;
pub type GeometryInstanceFlagsKHR = GeometryInstanceFlagBitsKHR;
pub type BuildAccelerationStructureFlagsKHR = BuildAccelerationStructureFlagBitsKHR;
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlagBitsEXT;
pub type DescriptorUpdateTemplateCreateFlags = Flags;
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlagBitsEXT;
pub type PerformanceCounterDescriptionFlagsKHR = PerformanceCounterDescriptionFlagBitsKHR;
pub type AcquireProfilingLockFlagsKHR = AcquireProfilingLockFlagBitsKHR;
pub type SemaphoreWaitFlags = SemaphoreWaitFlagBits;
pub type PipelineCompilerControlFlagsAMD = PipelineCompilerControlFlagBitsAMD;
pub type ShaderCorePropertiesFlagsAMD = ShaderCorePropertiesFlagBitsAMD;
pub type DeviceDiagnosticsConfigFlagsNV = DeviceDiagnosticsConfigFlagBitsNV;
pub type CompositeAlphaFlagsKHR = CompositeAlphaFlagBitsKHR;
pub type DisplayPlaneAlphaFlagsKHR = DisplayPlaneAlphaFlagBitsKHR;
pub type SurfaceTransformFlagsKHR = SurfaceTransformFlagBitsKHR;
pub type SwapchainCreateFlagsKHR = SwapchainCreateFlagBitsKHR;
pub type DisplayModeCreateFlagsKHR = Flags;
pub type DisplaySurfaceCreateFlagsKHR = Flags;
pub type AndroidSurfaceCreateFlagsKHR = Flags;
pub type ViSurfaceCreateFlagsNN = Flags;
pub type WaylandSurfaceCreateFlagsKHR = Flags;
pub type Win32SurfaceCreateFlagsKHR = Flags;
pub type XlibSurfaceCreateFlagsKHR = Flags;
pub type XcbSurfaceCreateFlagsKHR = Flags;
pub type DirectFBSurfaceCreateFlagsEXT = Flags;
pub type IOSSurfaceCreateFlagsMVK = Flags;
pub type MacOSSurfaceCreateFlagsMVK = Flags;
pub type MetalSurfaceCreateFlagsEXT = Flags;
pub type ImagePipeSurfaceCreateFlagsFUCHSIA = Flags;
pub type StreamDescriptorSurfaceCreateFlagsGGP = Flags;
pub type HeadlessSurfaceCreateFlagsEXT = Flags;
pub type PeerMemoryFeatureFlags = PeerMemoryFeatureFlagBits;
pub type MemoryAllocateFlags = MemoryAllocateFlagBits;
pub type DeviceGroupPresentModeFlagsKHR = DeviceGroupPresentModeFlagBitsKHR;
pub type DebugReportFlagsEXT = DebugReportFlagBitsEXT;
pub type CommandPoolTrimFlags = Flags;
pub type ExternalMemoryHandleTypeFlagsNV = ExternalMemoryHandleTypeFlagBitsNV;
pub type ExternalMemoryFeatureFlagsNV = ExternalMemoryFeatureFlagBitsNV;
pub type ExternalMemoryHandleTypeFlags = ExternalMemoryHandleTypeFlagBits;
pub type ExternalMemoryFeatureFlags = ExternalMemoryFeatureFlagBits;
pub type ExternalSemaphoreHandleTypeFlags = ExternalSemaphoreHandleTypeFlagBits;
pub type ExternalSemaphoreFeatureFlags = ExternalSemaphoreFeatureFlagBits;
pub type SemaphoreImportFlags = SemaphoreImportFlagBits;
pub type ExternalFenceHandleTypeFlags = ExternalFenceHandleTypeFlagBits;
pub type ExternalFenceFeatureFlags = ExternalFenceFeatureFlagBits;
pub type FenceImportFlags = FenceImportFlagBits;
pub type SurfaceCounterFlagsEXT = SurfaceCounterFlagBitsEXT;
pub type PipelineViewportSwizzleStateCreateFlagsNV = Flags;
pub type PipelineDiscardRectangleStateCreateFlagsEXT = Flags;
pub type PipelineCoverageToColorStateCreateFlagsNV = Flags;
pub type PipelineCoverageModulationStateCreateFlagsNV = Flags;
pub type PipelineCoverageReductionStateCreateFlagsNV = Flags;
pub type ValidationCacheCreateFlagsEXT = Flags;
pub type DebugUtilsMessageSeverityFlagsEXT = DebugUtilsMessageSeverityFlagBitsEXT;
pub type DebugUtilsMessageTypeFlagsEXT = DebugUtilsMessageTypeFlagBitsEXT;
pub type DebugUtilsMessengerCreateFlagsEXT = Flags;
pub type DebugUtilsMessengerCallbackDataFlagsEXT = Flags;
pub type PipelineRasterizationConservativeStateCreateFlagsEXT = Flags;
pub type DescriptorBindingFlags = DescriptorBindingFlagBits;
pub type ConditionalRenderingFlagsEXT = ConditionalRenderingFlagBitsEXT;
pub type ResolveModeFlags = ResolveModeFlagBits;
pub type PipelineRasterizationStateStreamCreateFlagsEXT = Flags;
pub type PipelineRasterizationDepthClipStateCreateFlagsEXT = Flags;
pub type SwapchainImageUsageFlagsANDROID = SwapchainImageUsageFlagBitsANDROID;
pub type ToolPurposeFlagsEXT = ToolPurposeFlagBitsEXT;

// Unions
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearColorValue {
    pub float32: [f32; 4 as usize],
    pub int32: [i32; 4 as usize],
    pub uint32: [u32; 4 as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}

#[repr(C)]
pub union PerformanceCounterResultKHR {
    pub int32: i32,
    pub int64: i64,
    pub uint32: u32,
    pub uint64: u64,
    pub float32: f32,
    pub float64: f64,
}

#[repr(C)]
pub union PerformanceValueDataINTEL {
    pub value32: u32,
    pub value64: u64,
    pub value_float: f32,
    pub value_bool: Bool32,
    pub value_string: *const c_char,
}

#[repr(C)]
pub union PipelineExecutableStatisticValueKHR {
    pub b32: Bool32,
    pub i64: i64,
    pub u64: u64,
    pub f64: f64,
}

#[repr(C)]
pub union DeviceOrHostAddressKHR {
    pub device_address: DeviceAddress,
    pub host_address: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DeviceOrHostAddressConstKHR {
    pub device_address: DeviceAddress,
    pub host_address: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union AccelerationStructureGeometryDataKHR {
    pub triangles: AccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: AccelerationStructureGeometryAabbsDataKHR,
    pub instances: AccelerationStructureGeometryInstancesDataKHR,
}

// Structs
#[repr(C)]
pub struct BaseOutStructure {
    pub s_type: StructureType,
    pub p_next: *const BaseOutStructure,
}

#[repr(C)]
pub struct BaseInStructure {
    pub s_type: StructureType,
    pub p_next: *const BaseInStructure,
}

#[repr(C)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[repr(C)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

#[repr(C)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}

#[repr(C)]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: [c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pub pipeline_cache_uuid: [u8; UUID_SIZE as usize],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct ExtensionProperties {
    pub extension_name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub spec_version: u32,
}

#[repr(C)]
pub struct LayerProperties {
    pub layer_name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
}

#[repr(C)]
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: u32,
    pub p_engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
}

#[repr(C)]
pub struct AllocationCallbacks {
    pub p_user_data: *const c_void,
    pub pfn_allocation: AllocationFunction,
    pub pfn_reallocation: ReallocationFunction,
    pub pfn_free: FreeFunction,
    pub pfn_internal_allocation: InternalAllocationNotification,
    pub pfn_internal_free: InternalFreeNotification,
}

#[repr(C)]
pub struct DeviceQueueCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
}

#[repr(C)]
pub struct DeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
}

#[repr(C)]
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
}

#[repr(C)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; MAX_MEMORY_TYPES as usize],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS as usize],
}

#[repr(C)]
pub struct MemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_index: u32,
}

#[repr(C)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: u32,
}

#[repr(C)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: ImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}

#[repr(C)]
pub struct SparseImageMemoryRequirements {
    pub format_properties: SparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: DeviceSize,
    pub image_mip_tail_offset: DeviceSize,
    pub image_mip_tail_stride: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}

#[repr(C)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}

#[repr(C)]
pub struct MappedMemoryRange {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags,
}

#[repr(C)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize,
}

#[repr(C)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}

#[repr(C)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}

#[repr(C)]
pub struct WriteDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub p_image_info: *const DescriptorImageInfo,
    pub p_buffer_info: *const DescriptorBufferInfo,
    pub p_texel_buffer_view: *const BufferView,
}

#[repr(C)]
pub struct CopyDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}

#[repr(C)]
pub struct BufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}

#[repr(C)]
pub struct BufferViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}

#[repr(C)]
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}

#[repr(C)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
pub struct MemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}

#[repr(C)]
pub struct BufferMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
pub struct ImageMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}

#[repr(C)]
pub struct ImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlagBits,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: ImageLayout,
}

#[repr(C)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub row_pitch: DeviceSize,
    pub array_pitch: DeviceSize,
    pub depth_pitch: DeviceSize,
}

#[repr(C)]
pub struct ImageViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}

#[repr(C)]
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
pub struct SparseMemoryBind {
    pub resource_offset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}

#[repr(C)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}

#[repr(C)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}

#[repr(C)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}

#[repr(C)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseImageMemoryBind,
}

#[repr(C)]
pub struct BindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub buffer_bind_count: u32,
    pub p_buffer_binds: *const SparseBufferMemoryBindInfo,
    pub image_opaque_bind_count: u32,
    pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
    pub image_bind_count: u32,
    pub p_image_binds: *const SparseImageMemoryBindInfo,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}

#[repr(C)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

#[repr(C)]
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2 as usize],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2 as usize],
}

#[repr(C)]
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}

#[repr(C)]
pub struct ImageResolve {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

#[repr(C)]
pub struct ShaderModuleCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: usize,
    pub p_code: *const u32,
}

#[repr(C)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub p_immutable_samplers: *const Sampler,
}

#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub p_bindings: *const DescriptorSetLayoutBinding,
}

#[repr(C)]
pub struct DescriptorPoolSize {
    pub vk_type: DescriptorType,
    pub descriptor_count: u32,
}

#[repr(C)]
pub struct DescriptorPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub p_pool_sizes: *const DescriptorPoolSize,
}

#[repr(C)]
pub struct DescriptorSetAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
}

#[repr(C)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}

#[repr(C)]
pub struct SpecializationInfo {
    pub map_entry_count: u32,
    pub p_map_entries: *const SpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const c_void,
}

#[repr(C)]
pub struct PipelineShaderStageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlagBits,
    pub module: ShaderModule,
    pub p_name: *const c_char,
    pub p_specialization_info: *const SpecializationInfo,
}

#[repr(C)]
pub struct ComputePipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

#[repr(C)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}

#[repr(C)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}

#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: Bool32,
}

#[repr(C)]
pub struct PipelineTessellationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}

#[repr(C)]
pub struct PipelineViewportStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub p_viewports: *const Viewport,
    pub scissor_count: u32,
    pub p_scissors: *const Rect2D,
}

#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: Bool32,
    pub rasterizer_discard_enable: Bool32,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: Bool32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}

#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlagBits,
    pub sample_shading_enable: Bool32,
    pub min_sample_shading: f32,
    pub p_sample_mask: *const SampleMask,
    pub alpha_to_coverage_enable: Bool32,
    pub alpha_to_one_enable: Bool32,
}

#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: Bool32,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}

#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: Bool32,
    pub logic_op: LogicOp,
    pub attachment_count: u32,
    pub p_attachments: *const PipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4 as usize],
}

#[repr(C)]
pub struct PipelineDynamicStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub p_dynamic_states: *const DynamicState,
}

#[repr(C)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: Bool32,
    pub depth_write_enable: Bool32,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: Bool32,
    pub stencil_test_enable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}

#[repr(C)]
pub struct GraphicsPipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
    pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
    pub p_viewport_state: *const PipelineViewportStateCreateInfo,
    pub p_rasterization_state: *const PipelineRasterizationStateCreateInfo,
    pub p_multisample_state: *const PipelineMultisampleStateCreateInfo,
    pub p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
    pub p_color_blend_state: *const PipelineColorBlendStateCreateInfo,
    pub p_dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

#[repr(C)]
pub struct PipelineCacheCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data_size: usize,
    pub p_initial_data: *const c_void,
}

#[repr(C)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
pub struct PipelineLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
}

#[repr(C)]
pub struct SamplerCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: f32,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
}

#[repr(C)]
pub struct CommandPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}

#[repr(C)]
pub struct CommandBufferAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}

#[repr(C)]
pub struct CommandBufferInheritanceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct CommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandBufferUsageFlags,
    pub p_inheritance_info: *const CommandBufferInheritanceInfo,
}

#[repr(C)]
pub struct RenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_value_count: u32,
    pub p_clear_values: *const ClearValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: ClearValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}

#[repr(C)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference,
    pub p_resolve_attachments: *const AttachmentReference,
    pub p_depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}

#[repr(C)]
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}

#[repr(C)]
pub struct RenderPassCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency,
}

#[repr(C)]
pub struct EventCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: EventCreateFlags,
}

#[repr(C)]
pub struct FenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FenceCreateFlags,
}

#[repr(C)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: Bool32,
    pub full_draw_index_uint32: Bool32,
    pub image_cube_array: Bool32,
    pub independent_blend: Bool32,
    pub geometry_shader: Bool32,
    pub tessellation_shader: Bool32,
    pub sample_rate_shading: Bool32,
    pub dual_src_blend: Bool32,
    pub logic_op: Bool32,
    pub multi_draw_indirect: Bool32,
    pub draw_indirect_first_instance: Bool32,
    pub depth_clamp: Bool32,
    pub depth_bias_clamp: Bool32,
    pub fill_mode_non_solid: Bool32,
    pub depth_bounds: Bool32,
    pub wide_lines: Bool32,
    pub large_points: Bool32,
    pub alpha_to_one: Bool32,
    pub multi_viewport: Bool32,
    pub sampler_anisotropy: Bool32,
    pub texture_compression_etc2: Bool32,
    pub texture_compression_astc_ldr: Bool32,
    pub texture_compression_bc: Bool32,
    pub occlusion_query_precise: Bool32,
    pub pipeline_statistics_query: Bool32,
    pub vertex_pipeline_stores_and_atomics: Bool32,
    pub fragment_stores_and_atomics: Bool32,
    pub shader_tessellation_and_geometry_point_size: Bool32,
    pub shader_image_gather_extended: Bool32,
    pub shader_storage_image_extended_formats: Bool32,
    pub shader_storage_image_multisample: Bool32,
    pub shader_storage_image_read_without_format: Bool32,
    pub shader_storage_image_write_without_format: Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    pub shader_clip_distance: Bool32,
    pub shader_cull_distance: Bool32,
    pub shader_float64: Bool32,
    pub shader_int64: Bool32,
    pub shader_int16: Bool32,
    pub shader_resource_residency: Bool32,
    pub shader_resource_min_lod: Bool32,
    pub sparse_binding: Bool32,
    pub sparse_residency_buffer: Bool32,
    pub sparse_residency_image2_d: Bool32,
    pub sparse_residency_image3_d: Bool32,
    pub sparse_residency2_samples: Bool32,
    pub sparse_residency4_samples: Bool32,
    pub sparse_residency8_samples: Bool32,
    pub sparse_residency16_samples: Bool32,
    pub sparse_residency_aliased: Bool32,
    pub variable_multisample_rate: Bool32,
    pub inherited_queries: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard2_dblock_shape: Bool32,
    pub residency_standard2_dmultisample_block_shape: Bool32,
    pub residency_standard3_dblock_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension1_d: u32,
    pub max_image_dimension2_d: u32,
    pub max_image_dimension3_d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: DeviceSize,
    pub sparse_address_space_size: DeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3 as usize],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3 as usize],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2 as usize],
    pub viewport_bounds_range: [f32; 2 as usize],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: DeviceSize,
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    pub min_storage_buffer_offset_alignment: DeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: Bool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2 as usize],
    pub line_width_range: [f32; 2 as usize],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: Bool32,
    pub standard_sample_locations: Bool32,
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    pub non_coherent_atom_size: DeviceSize,
}

#[repr(C)]
pub struct SemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreCreateFlags,
}

#[repr(C)]
pub struct QueryPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct FramebufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[repr(C)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

#[repr(C)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

#[repr(C)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[repr(C)]
pub struct SubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub p_wait_dst_stage_mask: *const PipelineStageFlags,
    pub command_buffer_count: u32,
    pub p_command_buffers: *const CommandBuffer,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}

#[repr(C)]
pub struct DisplayPropertiesKHR {
    pub display: DisplayKHR,
    pub display_name: *const c_char,
    pub physical_dimensions: Extent2D,
    pub physical_resolution: Extent2D,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub plane_reorder_possible: Bool32,
    pub persistent_content: Bool32,
}

#[repr(C)]
pub struct DisplayPlanePropertiesKHR {
    pub current_display: DisplayKHR,
    pub current_stack_index: u32,
}

#[repr(C)]
pub struct DisplayModeParametersKHR {
    pub visible_region: Extent2D,
    pub refresh_rate: u32,
}

#[repr(C)]
pub struct DisplayModePropertiesKHR {
    pub display_mode: DisplayModeKHR,
    pub parameters: DisplayModeParametersKHR,
}

#[repr(C)]
pub struct DisplayModeCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplayModeCreateFlagsKHR,
    pub parameters: DisplayModeParametersKHR,
}

#[repr(C)]
pub struct DisplayPlaneCapabilitiesKHR {
    pub supported_alpha: DisplayPlaneAlphaFlagsKHR,
    pub min_src_position: Offset2D,
    pub max_src_position: Offset2D,
    pub min_src_extent: Extent2D,
    pub max_src_extent: Extent2D,
    pub min_dst_position: Offset2D,
    pub max_dst_position: Offset2D,
    pub min_dst_extent: Extent2D,
    pub max_dst_extent: Extent2D,
}

#[repr(C)]
pub struct DisplaySurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplaySurfaceCreateFlagsKHR,
    pub display_mode: DisplayModeKHR,
    pub plane_index: u32,
    pub plane_stack_index: u32,
    pub transform: SurfaceTransformFlagBitsKHR,
    pub global_alpha: f32,
    pub alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
    pub image_extent: Extent2D,
}

#[repr(C)]
pub struct DisplayPresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_rect: Rect2D,
    pub dst_rect: Rect2D,
    pub persistent: Bool32,
}

#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlags,
}

#[repr(C)]
pub struct AndroidSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AndroidSurfaceCreateFlagsKHR,
    pub window: *const ANativeWindow,
}

#[repr(C)]
pub struct ViSurfaceCreateInfoNN {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ViSurfaceCreateFlagsNN,
    pub window: *const c_void,
}

#[repr(C)]
pub struct WaylandSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: WaylandSurfaceCreateFlagsKHR,
    pub display: *const wl_display,
    pub surface: *const wl_surface,
}

#[repr(C)]
pub struct Win32SurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: Win32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}

#[repr(C)]
pub struct XlibSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XlibSurfaceCreateFlagsKHR,
    pub dpy: *const Display,
    pub window: Window,
}

#[repr(C)]
pub struct XcbSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XcbSurfaceCreateFlagsKHR,
    pub connection: *const xcb_connection_t,
    pub window: xcb_window_t,
}

#[repr(C)]
pub struct DirectFBSurfaceCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DirectFBSurfaceCreateFlagsEXT,
    pub dfb: *const IDirectFB,
    pub surface: *const IDirectFBSurface,
}

#[repr(C)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
    pub image_pipe_handle: zx_handle_t,
}

#[repr(C)]
pub struct StreamDescriptorSurfaceCreateInfoGGP {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: StreamDescriptorSurfaceCreateFlagsGGP,
    pub stream_descriptor: GgpStreamDescriptor,
}

#[repr(C)]
pub struct SurfaceFormatKHR {
    pub format: Format,
    pub color_space: ColorSpaceKHR,
}

#[repr(C)]
pub struct SwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: SurfaceKHR,
    pub min_image_count: u32,
    pub image_format: Format,
    pub image_color_space: ColorSpaceKHR,
    pub image_extent: Extent2D,
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlags,
    pub image_sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub pre_transform: SurfaceTransformFlagBitsKHR,
    pub composite_alpha: CompositeAlphaFlagBitsKHR,
    pub present_mode: PresentModeKHR,
    pub clipped: Bool32,
    pub old_swapchain: SwapchainKHR,
}

#[repr(C)]
pub struct PresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub swapchain_count: u32,
    pub p_swapchains: *const SwapchainKHR,
    pub p_image_indices: *const u32,
    pub p_results: *const Result,
}

#[repr(C)]
pub struct DebugReportCallbackCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugReportFlagsEXT,
    pub pfn_callback: DebugReportCallbackEXT,
    pub p_user_data: *const c_void,
}

#[repr(C)]
pub struct ValidationFlagsEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disabled_validation_check_count: u32,
    pub p_disabled_validation_checks: *const ValidationCheckEXT,
}

#[repr(C)]
pub struct ValidationFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub enabled_validation_feature_count: u32,
    pub p_enabled_validation_features: *const ValidationFeatureEnableEXT,
    pub disabled_validation_feature_count: u32,
    pub p_disabled_validation_features: *const ValidationFeatureDisableEXT,
}

#[repr(C)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rasterization_order: RasterizationOrderAMD,
}

#[repr(C)]
pub struct DebugMarkerObjectNameInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: DebugReportObjectTypeEXT,
    pub object: u64,
    pub p_object_name: *const c_char,
}

#[repr(C)]
pub struct DebugMarkerObjectTagInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: DebugReportObjectTypeEXT,
    pub object: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const c_void,
}

#[repr(C)]
pub struct DebugMarkerMarkerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_marker_name: *const c_char,
    pub color: [f32; 4 as usize],
}

#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
}

#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
}

#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}

#[repr(C)]
pub struct ExternalImageFormatPropertiesNV {
    pub image_format_properties: ImageFormatProperties,
    pub external_memory_features: ExternalMemoryFeatureFlagsNV,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct ExportMemoryAllocateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct ImportMemoryWin32HandleInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagsNV,
    pub handle: HANDLE,
}

#[repr(C)]
pub struct ExportMemoryWin32HandleInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
}

#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_count: u32,
    pub p_acquire_syncs: *const DeviceMemory,
    pub p_acquire_keys: *const u64,
    pub p_acquire_timeout_milliseconds: *const u32,
    pub release_count: u32,
    pub p_release_syncs: *const DeviceMemory,
    pub p_release_keys: *const u64,
}

#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_generated_commands: Bool32,
}

#[repr(C)]
pub struct DevicePrivateDataCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub private_data_slot_request_count: u32,
}

#[repr(C)]
pub struct PrivateDataSlotCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PrivateDataSlotCreateFlagsEXT,
}

#[repr(C)]
pub struct PhysicalDevicePrivateDataFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub private_data: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_graphics_shader_group_count: u32,
    pub max_indirect_sequence_count: u32,
    pub max_indirect_commands_token_count: u32,
    pub max_indirect_commands_stream_count: u32,
    pub max_indirect_commands_token_offset: u32,
    pub max_indirect_commands_stream_stride: u32,
    pub min_sequences_count_buffer_offset_alignment: u32,
    pub min_sequences_index_buffer_offset_alignment: u32,
    pub min_indirect_commands_buffer_offset_alignment: u32,
}

#[repr(C)]
pub struct GraphicsShaderGroupCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
}

#[repr(C)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub group_count: u32,
    pub p_groups: *const GraphicsShaderGroupCreateInfoNV,
    pub pipeline_count: u32,
    pub p_pipelines: *const Pipeline,
}

#[repr(C)]
pub struct BindShaderGroupIndirectCommandNV {
    pub group_index: u32,
}

#[repr(C)]
pub struct BindIndexBufferIndirectCommandNV {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub index_type: IndexType,
}

#[repr(C)]
pub struct BindVertexBufferIndirectCommandNV {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub stride: u32,
}

#[repr(C)]
pub struct SetStateFlagsIndirectCommandNV {
    pub data: u32,
}

#[repr(C)]
pub struct IndirectCommandsStreamNV {
    pub buffer: Buffer,
    pub offset: DeviceSize,
}

#[repr(C)]
pub struct IndirectCommandsLayoutTokenNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub token_type: IndirectCommandsTokenTypeNV,
    pub stream: u32,
    pub offset: u32,
    pub vertex_binding_unit: u32,
    pub vertex_dynamic_stride: Bool32,
    pub pushconstant_pipeline_layout: PipelineLayout,
    pub pushconstant_shader_stage_flags: ShaderStageFlags,
    pub pushconstant_offset: u32,
    pub pushconstant_size: u32,
    pub indirect_state_flags: IndirectStateFlagsNV,
    pub index_type_count: u32,
    pub p_index_types: *const IndexType,
    pub p_index_type_values: *const u32,
}

#[repr(C)]
pub struct IndirectCommandsLayoutCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IndirectCommandsLayoutUsageFlagsNV,
    pub pipeline_bind_point: PipelineBindPoint,
    pub token_count: u32,
    pub p_tokens: *const IndirectCommandsLayoutTokenNV,
    pub stream_count: u32,
    pub p_stream_strides: *const u32,
}

#[repr(C)]
pub struct GeneratedCommandsInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    pub stream_count: u32,
    pub p_streams: *const IndirectCommandsStreamNV,
    pub sequences_count: u32,
    pub preprocess_buffer: Buffer,
    pub preprocess_offset: DeviceSize,
    pub preprocess_size: DeviceSize,
    pub sequences_count_buffer: Buffer,
    pub sequences_count_offset: DeviceSize,
    pub sequences_index_buffer: Buffer,
    pub sequences_index_offset: DeviceSize,
}

#[repr(C)]
pub struct GeneratedCommandsMemoryRequirementsInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    pub max_sequences_count: u32,
}

#[repr(C)]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub features: PhysicalDeviceFeatures,
}

#[repr(C)]
pub struct PhysicalDeviceFeatures2KHR {}

#[repr(C)]
pub struct PhysicalDeviceProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub properties: PhysicalDeviceProperties,
}

#[repr(C)]
pub struct PhysicalDeviceProperties2KHR {}

#[repr(C)]
pub struct FormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format_properties: FormatProperties,
}

#[repr(C)]
pub struct FormatProperties2KHR {}

#[repr(C)]
pub struct ImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_format_properties: ImageFormatProperties,
}

#[repr(C)]
pub struct ImageFormatProperties2KHR {}

#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub vk_type: ImageType,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub flags: ImageCreateFlags,
}

#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2KHR {}

#[repr(C)]
pub struct QueueFamilyProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_properties: QueueFamilyProperties,
}

#[repr(C)]
pub struct QueueFamilyProperties2KHR {}

#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_properties: PhysicalDeviceMemoryProperties,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2KHR {}

#[repr(C)]
pub struct SparseImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub properties: SparseImageFormatProperties,
}

#[repr(C)]
pub struct SparseImageFormatProperties2KHR {}

#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub vk_type: ImageType,
    pub samples: SampleCountFlagBits,
    pub usage: ImageUsageFlags,
    pub tiling: ImageTiling,
}

#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2KHR {}

#[repr(C)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_push_descriptors: u32,
}

#[repr(C)]
pub struct ConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}

#[repr(C)]
pub struct ConformanceVersionKHR {}

#[repr(C)]
pub struct PhysicalDeviceDriverProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub driver_id: DriverId,
    pub driver_name: [c_char; MAX_DRIVER_NAME_SIZE as usize],
    pub driver_info: [c_char; MAX_DRIVER_INFO_SIZE as usize],
    pub conformance_version: ConformanceVersion,
}

#[repr(C)]
pub struct PhysicalDeviceDriverPropertiesKHR {}

#[repr(C)]
pub struct PresentRegionsKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_regions: *const PresentRegionKHR,
}

#[repr(C)]
pub struct PresentRegionKHR {
    pub rectangle_count: u32,
    pub p_rectangles: *const RectLayerKHR,
}

#[repr(C)]
pub struct RectLayerKHR {
    pub offset: Offset2D,
    pub extent: Extent2D,
    pub layer: u32,
}

#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceVariablePointerFeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceVariablePointerFeatures {}

#[repr(C)]
pub struct ExternalMemoryProperties {
    pub external_memory_features: ExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct ExternalMemoryPropertiesKHR {}

#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfoKHR {}

#[repr(C)]
pub struct ExternalImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}

#[repr(C)]
pub struct ExternalImageFormatPropertiesKHR {}

#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub usage: BufferUsageFlags,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfoKHR {}

#[repr(C)]
pub struct ExternalBufferProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}

#[repr(C)]
pub struct ExternalBufferPropertiesKHR {}

#[repr(C)]
pub struct PhysicalDeviceIDProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_uuid: [u8; UUID_SIZE as usize],
    pub driver_uuid: [u8; UUID_SIZE as usize],
    pub device_luid: [u8; LUID_SIZE as usize],
    pub device_node_mask: u32,
    pub device_luidvalid: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceIDPropertiesKHR {}

#[repr(C)]
pub struct ExternalMemoryImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct ExternalMemoryImageCreateInfoKHR {}

#[repr(C)]
pub struct ExternalMemoryBufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct ExternalMemoryBufferCreateInfoKHR {}

#[repr(C)]
pub struct ExportMemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct ExportMemoryAllocateInfoKHR {}

#[repr(C)]
pub struct ImportMemoryWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}

#[repr(C)]
pub struct ExportMemoryWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}

#[repr(C)]
pub struct MemoryWin32HandlePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_type_bits: u32,
}

#[repr(C)]
pub struct MemoryGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct ImportMemoryFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub fd: i32,
}

#[repr(C)]
pub struct MemoryFdPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_type_bits: u32,
}

#[repr(C)]
pub struct MemoryGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_count: u32,
    pub p_acquire_syncs: *const DeviceMemory,
    pub p_acquire_keys: *const u64,
    pub p_acquire_timeouts: *const u32,
    pub release_count: u32,
    pub p_release_syncs: *const DeviceMemory,
    pub p_release_keys: *const u64,
}

#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfoKHR {}

#[repr(C)]
pub struct ExternalSemaphoreProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
}

#[repr(C)]
pub struct ExternalSemaphorePropertiesKHR {}

#[repr(C)]
pub struct ExportSemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
}

#[repr(C)]
pub struct ExportSemaphoreCreateInfoKHR {}

#[repr(C)]
pub struct ImportSemaphoreWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}

#[repr(C)]
pub struct ExportSemaphoreWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}

#[repr(C)]
pub struct D3D12FenceSubmitInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_values_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_values_count: u32,
    pub p_signal_semaphore_values: *const u64,
}

#[repr(C)]
pub struct SemaphoreGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct ImportSemaphoreFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub fd: i32,
}

#[repr(C)]
pub struct SemaphoreGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfoKHR {}

#[repr(C)]
pub struct ExternalFenceProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    pub external_fence_features: ExternalFenceFeatureFlags,
}

#[repr(C)]
pub struct ExternalFencePropertiesKHR {}

#[repr(C)]
pub struct ExportFenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalFenceHandleTypeFlags,
}

#[repr(C)]
pub struct ExportFenceCreateInfoKHR {}

#[repr(C)]
pub struct ImportFenceWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}

#[repr(C)]
pub struct ExportFenceWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}

#[repr(C)]
pub struct FenceGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct ImportFenceFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub fd: i32,
}

#[repr(C)]
pub struct FenceGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct PhysicalDeviceMultiviewFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceMultiviewFeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceMultiviewProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
}

#[repr(C)]
pub struct PhysicalDeviceMultiviewPropertiesKHR {}

#[repr(C)]
pub struct RenderPassMultiviewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subpass_count: u32,
    pub p_view_masks: *const u32,
    pub dependency_count: u32,
    pub p_view_offsets: *const i32,
    pub correlation_mask_count: u32,
    pub p_correlation_masks: *const u32,
}

#[repr(C)]
pub struct RenderPassMultiviewCreateInfoKHR {}

#[repr(C)]
pub struct SurfaceCapabilities2EXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlags,
    pub supported_surface_counters: SurfaceCounterFlagsEXT,
}

#[repr(C)]
pub struct DisplayPowerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub power_state: DisplayPowerStateEXT,
}

#[repr(C)]
pub struct DeviceEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_event: DeviceEventTypeEXT,
}

#[repr(C)]
pub struct DisplayEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_event: DisplayEventTypeEXT,
}

#[repr(C)]
pub struct SwapchainCounterCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface_counters: SurfaceCounterFlagsEXT,
}

#[repr(C)]
pub struct PhysicalDeviceGroupProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub physical_device_count: u32,
    pub physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize],
    pub subset_allocation: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceGroupPropertiesKHR {}

#[repr(C)]
pub struct MemoryAllocateFlagsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryAllocateFlags,
    pub device_mask: u32,
}

#[repr(C)]
pub struct MemoryAllocateFlagsInfoKHR {}

#[repr(C)]
pub struct BindBufferMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}

#[repr(C)]
pub struct BindBufferMemoryInfoKHR {}

#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}

#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfoKHR {}

#[repr(C)]
pub struct BindImageMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}

#[repr(C)]
pub struct BindImageMemoryInfoKHR {}

#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub split_instance_bind_region_count: u32,
    pub p_split_instance_bind_regions: *const Rect2D,
}

#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfoKHR {}

#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
    pub device_render_area_count: u32,
    pub p_device_render_areas: *const Rect2D,
}

#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfoKHR {}

#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
}

#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfoKHR {}

#[repr(C)]
pub struct DeviceGroupSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphore_device_indices: *const u32,
    pub command_buffer_count: u32,
    pub p_command_buffer_device_masks: *const u32,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphore_device_indices: *const u32,
}

#[repr(C)]
pub struct DeviceGroupSubmitInfoKHR {}

#[repr(C)]
pub struct DeviceGroupBindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub resource_device_index: u32,
    pub memory_device_index: u32,
}

#[repr(C)]
pub struct DeviceGroupBindSparseInfoKHR {}

#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize],
    pub modes: DeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
pub struct ImageSwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
}

#[repr(C)]
pub struct BindImageMemorySwapchainInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub image_index: u32,
}

#[repr(C)]
pub struct AcquireNextImageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub timeout: u64,
    pub semaphore: Semaphore,
    pub fence: Fence,
    pub device_mask: u32,
}

#[repr(C)]
pub struct DeviceGroupPresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_device_masks: *const u32,
    pub mode: DeviceGroupPresentModeFlagBitsKHR,
}

#[repr(C)]
pub struct DeviceGroupDeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub physical_device_count: u32,
    pub p_physical_devices: *const PhysicalDevice,
}

#[repr(C)]
pub struct DeviceGroupDeviceCreateInfoKHR {}

#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub modes: DeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
pub struct DescriptorUpdateTemplateEntry {
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub offset: usize,
    pub stride: usize,
}

#[repr(C)]
pub struct DescriptorUpdateTemplateEntryKHR {}

#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entry_count: u32,
    pub p_descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
    pub template_type: DescriptorUpdateTemplateType,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline_layout: PipelineLayout,
    pub set: u32,
}

#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfoKHR {}

#[repr(C)]
pub struct XYColorEXT {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct HdrMetadataEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_primary_red: XYColorEXT,
    pub display_primary_green: XYColorEXT,
    pub display_primary_blue: XYColorEXT,
    pub white_point: XYColorEXT,
    pub max_luminance: f32,
    pub min_luminance: f32,
    pub max_content_light_level: f32,
    pub max_frame_average_light_level: f32,
}

#[repr(C)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub local_dimming_support: Bool32,
}

#[repr(C)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub local_dimming_enable: Bool32,
}

#[repr(C)]
pub struct RefreshCycleDurationGOOGLE {
    pub refresh_duration: u64,
}

#[repr(C)]
pub struct PastPresentationTimingGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
    pub actual_present_time: u64,
    pub earliest_present_time: u64,
    pub present_margin: u64,
}

#[repr(C)]
pub struct PresentTimesInfoGOOGLE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_times: *const PresentTimeGOOGLE,
}

#[repr(C)]
pub struct PresentTimeGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
}

#[repr(C)]
pub struct IOSSurfaceCreateInfoMVK {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IOSSurfaceCreateFlagsMVK,
    pub p_view: *const c_void,
}

#[repr(C)]
pub struct MacOSSurfaceCreateInfoMVK {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MacOSSurfaceCreateFlagsMVK,
    pub p_view: *const c_void,
}

#[repr(C)]
pub struct MetalSurfaceCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MetalSurfaceCreateFlagsEXT,
    pub p_layer: *const CAMetalLayer,
}

#[repr(C)]
pub struct ViewportWScalingNV {
    pub xcoeff: f32,
    pub ycoeff: f32,
}

#[repr(C)]
pub struct PipelineViewportWScalingStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub viewport_wscaling_enable: Bool32,
    pub viewport_count: u32,
    pub p_viewport_wscalings: *const ViewportWScalingNV,
}

#[repr(C)]
pub struct ViewportSwizzleNV {
    pub x: ViewportCoordinateSwizzleNV,
    pub y: ViewportCoordinateSwizzleNV,
    pub z: ViewportCoordinateSwizzleNV,
    pub w: ViewportCoordinateSwizzleNV,
}

#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
    pub viewport_count: u32,
    pub p_viewport_swizzles: *const ViewportSwizzleNV,
}

#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_discard_rectangles: u32,
}

#[repr(C)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    pub discard_rectangle_mode: DiscardRectangleModeEXT,
    pub discard_rectangle_count: u32,
    pub p_discard_rectangles: *const Rect2D,
}

#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub per_view_position_all_components: Bool32,
}

#[repr(C)]
pub struct InputAttachmentAspectReference {
    pub subpass: u32,
    pub input_attachment_index: u32,
    pub aspect_mask: ImageAspectFlags,
}

#[repr(C)]
pub struct InputAttachmentAspectReferenceKHR {}

#[repr(C)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aspect_reference_count: u32,
    pub p_aspect_references: *const InputAttachmentAspectReference,
}

#[repr(C)]
pub struct RenderPassInputAttachmentAspectCreateInfoKHR {}

#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface: SurfaceKHR,
}

#[repr(C)]
pub struct SurfaceCapabilities2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface_capabilities: SurfaceCapabilitiesKHR,
}

#[repr(C)]
pub struct SurfaceFormat2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface_format: SurfaceFormatKHR,
}

#[repr(C)]
pub struct DisplayProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_properties: DisplayPropertiesKHR,
}

#[repr(C)]
pub struct DisplayPlaneProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_plane_properties: DisplayPlanePropertiesKHR,
}

#[repr(C)]
pub struct DisplayModeProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_mode_properties: DisplayModePropertiesKHR,
}

#[repr(C)]
pub struct DisplayPlaneInfo2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: DisplayModeKHR,
    pub plane_index: u32,
}

#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub capabilities: DisplayPlaneCapabilitiesKHR,
}

#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shared_present_supported_usage_flags: ImageUsageFlags,
}

#[repr(C)]
pub struct PhysicalDevice16BitStorageFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub storage_buffer16_bit_access: Bool32,
    pub uniform_and_storage_buffer16_bit_access: Bool32,
    pub storage_push_constant16: Bool32,
    pub storage_input_output16: Bool32,
}

#[repr(C)]
pub struct PhysicalDevice16BitStorageFeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceSubgroupProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subgroup_size: u32,
    pub supported_stages: ShaderStageFlags,
    pub supported_operations: SubgroupFeatureFlags,
    pub quad_operations_in_all_stages: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_subgroup_extended_types: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR {}

#[repr(C)]
pub struct BufferMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}

#[repr(C)]
pub struct BufferMemoryRequirementsInfo2KHR {}

#[repr(C)]
pub struct ImageMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}

#[repr(C)]
pub struct ImageMemoryRequirementsInfo2KHR {}

#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}

#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2KHR {}

#[repr(C)]
pub struct MemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_requirements: MemoryRequirements,
}

#[repr(C)]
pub struct MemoryRequirements2KHR {}

#[repr(C)]
pub struct SparseImageMemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_requirements: SparseImageMemoryRequirements,
}

#[repr(C)]
pub struct SparseImageMemoryRequirements2KHR {}

#[repr(C)]
pub struct PhysicalDevicePointClippingProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub point_clipping_behavior: PointClippingBehavior,
}

#[repr(C)]
pub struct PhysicalDevicePointClippingPropertiesKHR {}

#[repr(C)]
pub struct MemoryDedicatedRequirements {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub prefers_dedicated_allocation: Bool32,
    pub requires_dedicated_allocation: Bool32,
}

#[repr(C)]
pub struct MemoryDedicatedRequirementsKHR {}

#[repr(C)]
pub struct MemoryDedicatedAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}

#[repr(C)]
pub struct MemoryDedicatedAllocateInfoKHR {}

#[repr(C)]
pub struct ImageViewUsageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: ImageUsageFlags,
}

#[repr(C)]
pub struct ImageViewUsageCreateInfoKHR {}

#[repr(C)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub domain_origin: TessellationDomainOrigin,
}

#[repr(C)]
pub struct PipelineTessellationDomainOriginStateCreateInfoKHR {}

#[repr(C)]
pub struct SamplerYcbcrConversionInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conversion: SamplerYcbcrConversion,
}

#[repr(C)]
pub struct SamplerYcbcrConversionInfoKHR {}

#[repr(C)]
pub struct SamplerYcbcrConversionCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ycbcr_model: SamplerYcbcrModelConversion,
    pub ycbcr_range: SamplerYcbcrRange,
    pub components: ComponentMapping,
    pub x_chroma_offset: ChromaLocation,
    pub y_chroma_offset: ChromaLocation,
    pub chroma_filter: Filter,
    pub force_explicit_reconstruction: Bool32,
}

#[repr(C)]
pub struct SamplerYcbcrConversionCreateInfoKHR {}

#[repr(C)]
pub struct BindImagePlaneMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlagBits,
}

#[repr(C)]
pub struct BindImagePlaneMemoryInfoKHR {}

#[repr(C)]
pub struct ImagePlaneMemoryRequirementsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlagBits,
}

#[repr(C)]
pub struct ImagePlaneMemoryRequirementsInfoKHR {}

#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sampler_ycbcr_conversion: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeaturesKHR {}

#[repr(C)]
pub struct SamplerYcbcrConversionImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub combined_image_sampler_descriptor_count: u32,
}

#[repr(C)]
pub struct SamplerYcbcrConversionImageFormatPropertiesKHR {}

#[repr(C)]
pub struct TextureLODGatherFormatPropertiesAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub supports_texture_gather_lodbias_amd: Bool32,
}

#[repr(C)]
pub struct ConditionalRenderingBeginInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub flags: ConditionalRenderingFlagsEXT,
}

#[repr(C)]
pub struct ProtectedSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_submit: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_memory: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_no_fault: Bool32,
}

#[repr(C)]
pub struct DeviceQueueInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
}

#[repr(C)]
pub struct PipelineCoverageToColorStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageToColorStateCreateFlagsNV,
    pub coverage_to_color_enable: Bool32,
    pub coverage_to_color_location: u32,
}

#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub filter_minmax_single_component_formats: Bool32,
    pub filter_minmax_image_component_mapping: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxPropertiesEXT {}

#[repr(C)]
pub struct SampleLocationEXT {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct SampleLocationsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_per_pixel: SampleCountFlagBits,
    pub sample_location_grid_size: Extent2D,
    pub sample_locations_count: u32,
    pub p_sample_locations: *const SampleLocationEXT,
}

#[repr(C)]
pub struct AttachmentSampleLocationsEXT {
    pub attachment_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}

#[repr(C)]
pub struct SubpassSampleLocationsEXT {
    pub subpass_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}

#[repr(C)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_initial_sample_locations_count: u32,
    pub p_attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT,
    pub post_subpass_sample_locations_count: u32,
    pub p_post_subpass_sample_locations: *const SubpassSampleLocationsEXT,
}

#[repr(C)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_enable: Bool32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}

#[repr(C)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_location_sample_counts: SampleCountFlags,
    pub max_sample_location_grid_size: Extent2D,
    pub sample_location_coordinate_range: [f32; 2 as usize],
    pub sample_location_sub_pixel_bits: u32,
    pub variable_sample_locations: Bool32,
}

#[repr(C)]
pub struct MultisamplePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_sample_location_grid_size: Extent2D,
}

#[repr(C)]
pub struct SamplerReductionModeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub reduction_mode: SamplerReductionMode,
}

#[repr(C)]
pub struct SamplerReductionModeCreateInfoEXT {}

#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub advanced_blend_coherent_operations: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub advanced_blend_max_color_attachments: u32,
    pub advanced_blend_independent_blend: Bool32,
    pub advanced_blend_non_premultiplied_src_color: Bool32,
    pub advanced_blend_non_premultiplied_dst_color: Bool32,
    pub advanced_blend_correlated_overlap: Bool32,
    pub advanced_blend_all_operations: Bool32,
}

#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_premultiplied: Bool32,
    pub dst_premultiplied: Bool32,
    pub blend_overlap: BlendOverlapEXT,
}

#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub inline_uniform_block: Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_inline_uniform_block_size: u32,
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    pub max_descriptor_set_inline_uniform_blocks: u32,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}

#[repr(C)]
pub struct WriteDescriptorSetInlineUniformBlockEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_size: u32,
    pub p_data: *const c_void,
}

#[repr(C)]
pub struct DescriptorPoolInlineUniformBlockCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_inline_uniform_block_bindings: u32,
}

#[repr(C)]
pub struct PipelineCoverageModulationStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageModulationStateCreateFlagsNV,
    pub coverage_modulation_mode: CoverageModulationModeNV,
    pub coverage_modulation_table_enable: Bool32,
    pub coverage_modulation_table_count: u32,
    pub p_coverage_modulation_table: *const f32,
}

#[repr(C)]
pub struct ImageFormatListCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub view_format_count: u32,
    pub p_view_formats: *const Format,
}

#[repr(C)]
pub struct ImageFormatListCreateInfoKHR {}

#[repr(C)]
pub struct ValidationCacheCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ValidationCacheCreateFlagsEXT,
    pub initial_data_size: usize,
    pub p_initial_data: *const c_void,
}

#[repr(C)]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub validation_cache: ValidationCacheEXT,
}

#[repr(C)]
pub struct PhysicalDeviceMaintenance3Properties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceMaintenance3PropertiesKHR {}

#[repr(C)]
pub struct DescriptorSetLayoutSupport {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub supported: Bool32,
}

#[repr(C)]
pub struct DescriptorSetLayoutSupportKHR {}

#[repr(C)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_draw_parameters: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderDrawParameterFeatures {}

#[repr(C)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_float16: Bool32,
    pub shader_int8: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderFloat16Int8FeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceFloat16Int8FeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceFloatControlsProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float16: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float32: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float64: Bool32,
    pub shader_denorm_preserve_float16: Bool32,
    pub shader_denorm_preserve_float32: Bool32,
    pub shader_denorm_preserve_float64: Bool32,
    pub shader_denorm_flush_to_zero_float16: Bool32,
    pub shader_denorm_flush_to_zero_float32: Bool32,
    pub shader_denorm_flush_to_zero_float64: Bool32,
    pub shader_rounding_mode_rtefloat16: Bool32,
    pub shader_rounding_mode_rtefloat32: Bool32,
    pub shader_rounding_mode_rtefloat64: Bool32,
    pub shader_rounding_mode_rtzfloat16: Bool32,
    pub shader_rounding_mode_rtzfloat32: Bool32,
    pub shader_rounding_mode_rtzfloat64: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFloatControlsPropertiesKHR {}

#[repr(C)]
pub struct PhysicalDeviceHostQueryResetFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub host_query_reset: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceHostQueryResetFeaturesEXT {}

#[repr(C)]
pub struct NativeBufferUsage2ANDROID {
    pub consumer: u64,
    pub producer: u64,
}

#[repr(C)]
pub struct NativeBufferANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle: *const c_void,
    pub stride: i32,
    pub format: i32,
    pub usage: i32,
    pub usage2: NativeBufferUsage2ANDROID,
}

#[repr(C)]
pub struct SwapchainImageCreateInfoANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: SwapchainImageUsageFlagsANDROID,
}

#[repr(C)]
pub struct PhysicalDevicePresentationPropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shared_image: Bool32,
}

#[repr(C)]
pub struct ShaderResourceUsageAMD {
    pub num_used_vgprs: u32,
    pub num_used_sgprs: u32,
    pub lds_size_per_local_work_group: u32,
    pub lds_usage_size_in_bytes: usize,
    pub scratch_mem_usage_in_bytes: usize,
}

#[repr(C)]
pub struct ShaderStatisticsInfoAMD {
    pub shader_stage_mask: ShaderStageFlags,
    pub resource_usage: ShaderResourceUsageAMD,
    pub num_physical_vgprs: u32,
    pub num_physical_sgprs: u32,
    pub num_available_vgprs: u32,
    pub num_available_sgprs: u32,
    pub compute_work_group_size: [u32; 3 as usize],
}

#[repr(C)]
pub struct DeviceQueueGlobalPriorityCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub global_priority: QueueGlobalPriorityEXT,
}

#[repr(C)]
pub struct DebugUtilsObjectNameInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub p_object_name: *const c_char,
}

#[repr(C)]
pub struct DebugUtilsObjectTagInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const c_void,
}

#[repr(C)]
pub struct DebugUtilsLabelEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_label_name: *const c_char,
    pub color: [f32; 4 as usize],
}

#[repr(C)]
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCreateFlagsEXT,
    pub message_severity: DebugUtilsMessageSeverityFlagsEXT,
    pub message_type: DebugUtilsMessageTypeFlagsEXT,
    pub pfn_user_callback: DebugUtilsMessengerCallbackEXT,
    pub p_user_data: *const c_void,
}

#[repr(C)]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    pub p_message_id_name: *const c_char,
    pub message_id_number: i32,
    pub p_message: *const c_char,
    pub queue_label_count: u32,
    pub p_queue_labels: *const DebugUtilsLabelEXT,
    pub cmd_buf_label_count: u32,
    pub p_cmd_buf_labels: *const DebugUtilsLabelEXT,
    pub object_count: u32,
    pub p_objects: *const DebugUtilsObjectNameInfoEXT,
}

#[repr(C)]
pub struct ImportMemoryHostPointerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub p_host_pointer: *const c_void,
}

#[repr(C)]
pub struct MemoryHostPointerPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_type_bits: u32,
}

#[repr(C)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_imported_host_pointer_alignment: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub primitive_overestimation_size: f32,
    pub max_extra_primitive_overestimation_size: f32,
    pub extra_primitive_overestimation_size_granularity: f32,
    pub primitive_underestimation: Bool32,
    pub conservative_point_and_line_rasterization: Bool32,
    pub degenerate_triangles_rasterized: Bool32,
    pub degenerate_lines_rasterized: Bool32,
    pub fully_covered_fragment_shader_input_variable: Bool32,
    pub conservative_rasterization_post_depth_coverage: Bool32,
}

#[repr(C)]
pub struct CalibratedTimestampInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub time_domain: TimeDomainEXT,
}

#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_engine_count: u32,
    pub shader_arrays_per_engine_count: u32,
    pub compute_units_per_shader_array: u32,
    pub simd_per_compute_unit: u32,
    pub wavefronts_per_simd: u32,
    pub wavefront_size: u32,
    pub sgprs_per_simd: u32,
    pub min_sgpr_allocation: u32,
    pub max_sgpr_allocation: u32,
    pub sgpr_allocation_granularity: u32,
    pub vgprs_per_simd: u32,
    pub min_vgpr_allocation: u32,
    pub max_vgpr_allocation: u32,
    pub vgpr_allocation_granularity: u32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderCoreProperties2AMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_core_features: ShaderCorePropertiesFlagsAMD,
    pub active_compute_unit_count: u32,
}

#[repr(C)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
    pub conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    pub extra_primitive_overestimation_size: f32,
}

#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_update_unused_while_pending: Bool32,
    pub descriptor_binding_partially_bound: Bool32,
    pub descriptor_binding_variable_descriptor_count: Bool32,
    pub runtime_descriptor_array: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingFeaturesEXT {}

#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    pub robust_buffer_access_update_after_bind: Bool32,
    pub quad_divergent_implicit_lod: Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
}

#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingPropertiesEXT {}

#[repr(C)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binding_count: u32,
    pub p_binding_flags: *const DescriptorBindingFlags,
}

#[repr(C)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfoEXT {}

#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set_count: u32,
    pub p_descriptor_counts: *const u32,
}

#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfoEXT {}

#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_variable_descriptor_count: u32,
}

#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupportEXT {}

#[repr(C)]
pub struct AttachmentDescription2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}

#[repr(C)]
pub struct AttachmentDescription2KHR {}

#[repr(C)]
pub struct AttachmentReference2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment: u32,
    pub layout: ImageLayout,
    pub aspect_mask: ImageAspectFlags,
}

#[repr(C)]
pub struct AttachmentReference2KHR {}

#[repr(C)]
pub struct SubpassDescription2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub view_mask: u32,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference2,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference2,
    pub p_resolve_attachments: *const AttachmentReference2,
    pub p_depth_stencil_attachment: *const AttachmentReference2,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}

#[repr(C)]
pub struct SubpassDescription2KHR {}

#[repr(C)]
pub struct SubpassDependency2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
    pub view_offset: i32,
}

#[repr(C)]
pub struct SubpassDependency2KHR {}

#[repr(C)]
pub struct RenderPassCreateInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription2,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription2,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency2,
    pub correlated_view_mask_count: u32,
    pub p_correlated_view_masks: *const u32,
}

#[repr(C)]
pub struct RenderPassCreateInfo2KHR {}

#[repr(C)]
pub struct SubpassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub contents: SubpassContents,
}

#[repr(C)]
pub struct SubpassBeginInfoKHR {}

#[repr(C)]
pub struct SubpassEndInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}

#[repr(C)]
pub struct SubpassEndInfoKHR {}

#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub timeline_semaphore: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreFeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_timeline_semaphore_value_difference: u64,
}

#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphorePropertiesKHR {}

#[repr(C)]
pub struct SemaphoreTypeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore_type: SemaphoreType,
    pub initial_value: u64,
}

#[repr(C)]
pub struct SemaphoreTypeCreateInfoKHR {}

#[repr(C)]
pub struct TimelineSemaphoreSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_value_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_value_count: u32,
    pub p_signal_semaphore_values: *const u64,
}

#[repr(C)]
pub struct TimelineSemaphoreSubmitInfoKHR {}

#[repr(C)]
pub struct SemaphoreWaitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreWaitFlags,
    pub semaphore_count: u32,
    pub p_semaphores: *const Semaphore,
    pub p_values: *const u64,
}

#[repr(C)]
pub struct SemaphoreWaitInfoKHR {}

#[repr(C)]
pub struct SemaphoreSignalInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub value: u64,
}

#[repr(C)]
pub struct SemaphoreSignalInfoKHR {}

#[repr(C)]
pub struct VertexInputBindingDivisorDescriptionEXT {
    pub binding: u32,
    pub divisor: u32,
}

#[repr(C)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_binding_divisor_count: u32,
    pub p_vertex_binding_divisors: *const VertexInputBindingDivisorDescriptionEXT,
}

#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_vertex_attrib_divisor: u32,
}

#[repr(C)]
pub struct PhysicalDevicePCIBusInfoPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pci_domain: u32,
    pub pci_bus: u32,
    pub pci_device: u32,
    pub pci_function: u32,
}

#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *const AHardwareBuffer,
}

#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub android_hardware_buffer_usage: u64,
}

#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
}

#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}

#[repr(C)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub external_format: u64,
    pub format_features: FormatFeatureFlags,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_xchroma_offset: ChromaLocation,
    pub suggested_ychroma_offset: ChromaLocation,
}

#[repr(C)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conditional_rendering_enable: Bool32,
}

#[repr(C)]
pub struct ExternalFormatANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub external_format: u64,
}

#[repr(C)]
pub struct PhysicalDevice8BitStorageFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub storage_buffer8_bit_access: Bool32,
    pub uniform_and_storage_buffer8_bit_access: Bool32,
    pub storage_push_constant8: Bool32,
}

#[repr(C)]
pub struct PhysicalDevice8BitStorageFeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conditional_rendering: Bool32,
    pub inherited_conditional_rendering: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVulkanMemoryModelFeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_buffer_int64_atomics: Bool32,
    pub shader_shared_int64_atomics: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderAtomicInt64FeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_buffer_float32_atomics: Bool32,
    pub shader_buffer_float32_atomic_add: Bool32,
    pub shader_buffer_float64_atomics: Bool32,
    pub shader_buffer_float64_atomic_add: Bool32,
    pub shader_shared_float32_atomics: Bool32,
    pub shader_shared_float32_atomic_add: Bool32,
    pub shader_shared_float64_atomics: Bool32,
    pub shader_shared_float64_atomic_add: Bool32,
    pub shader_image_float32_atomics: Bool32,
    pub shader_image_float32_atomic_add: Bool32,
    pub sparse_image_float32_atomics: Bool32,
    pub sparse_image_float32_atomic_add: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_attribute_instance_rate_divisor: Bool32,
    pub vertex_attribute_instance_rate_zero_divisor: Bool32,
}

#[repr(C)]
pub struct QueueFamilyCheckpointPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub checkpoint_execution_stage_mask: PipelineStageFlags,
}

#[repr(C)]
pub struct CheckpointDataNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage: PipelineStageFlagBits,
    pub p_checkpoint_marker: *const c_void,
}

#[repr(C)]
pub struct PhysicalDeviceDepthStencilResolveProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub supported_depth_resolve_modes: ResolveModeFlags,
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    pub independent_resolve_none: Bool32,
    pub independent_resolve: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceDepthStencilResolvePropertiesKHR {}

#[repr(C)]
pub struct SubpassDescriptionDepthStencilResolve {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_resolve_mode: ResolveModeFlagBits,
    pub stencil_resolve_mode: ResolveModeFlagBits,
    pub p_depth_stencil_resolve_attachment: *const AttachmentReference2,
}

#[repr(C)]
pub struct SubpassDescriptionDepthStencilResolveKHR {}

#[repr(C)]
pub struct ImageViewASTCDecodeModeEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub decode_mode: Format,
}

#[repr(C)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub decode_mode_shared_exponent: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform_feedback: Bool32,
    pub geometry_streams: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_transform_feedback_streams: u32,
    pub max_transform_feedback_buffers: u32,
    pub max_transform_feedback_buffer_size: DeviceSize,
    pub max_transform_feedback_stream_data_size: u32,
    pub max_transform_feedback_buffer_data_size: u32,
    pub max_transform_feedback_buffer_data_stride: u32,
    pub transform_feedback_queries: Bool32,
    pub transform_feedback_streams_lines_triangles: Bool32,
    pub transform_feedback_rasterization_stream_select: Bool32,
    pub transform_feedback_draw: Bool32,
}

#[repr(C)]
pub struct PipelineRasterizationStateStreamCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationStateStreamCreateFlagsEXT,
    pub rasterization_stream: u32,
}

#[repr(C)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub representative_fragment_test: Bool32,
}

#[repr(C)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub representative_fragment_test_enable: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub exclusive_scissor: Bool32,
}

#[repr(C)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub exclusive_scissor_count: u32,
    pub p_exclusive_scissors: *const Rect2D,
}

#[repr(C)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub corner_sampled_image: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compute_derivative_group_quads: Bool32,
    pub compute_derivative_group_linear: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_shader_barycentric: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_footprint: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation_image_aliasing: Bool32,
}

#[repr(C)]
pub struct ShadingRatePaletteNV {
    pub shading_rate_palette_entry_count: u32,
    pub p_shading_rate_palette_entries: *const ShadingRatePaletteEntryNV,
}

#[repr(C)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shading_rate_image_enable: Bool32,
    pub viewport_count: u32,
    pub p_shading_rate_palettes: *const ShadingRatePaletteNV,
}

#[repr(C)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shading_rate_image: Bool32,
    pub shading_rate_coarse_sample_order: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShadingRateImagePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shading_rate_texel_size: Extent2D,
    pub shading_rate_palette_size: u32,
    pub shading_rate_max_coarse_samples: u32,
}

#[repr(C)]
pub struct CoarseSampleLocationNV {
    pub pixel_x: u32,
    pub pixel_y: u32,
    pub sample: u32,
}

#[repr(C)]
pub struct CoarseSampleOrderCustomNV {
    pub shading_rate: ShadingRatePaletteEntryNV,
    pub sample_count: u32,
    pub sample_location_count: u32,
    pub p_sample_locations: *const CoarseSampleLocationNV,
}

#[repr(C)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_order_type: CoarseSampleOrderTypeNV,
    pub custom_sample_order_count: u32,
    pub p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
}

#[repr(C)]
pub struct PhysicalDeviceMeshShaderFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub task_shader: Bool32,
    pub mesh_shader: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceMeshShaderPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_draw_mesh_tasks_count: u32,
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: [u32; 3 as usize],
    pub max_task_total_memory_size: u32,
    pub max_task_output_count: u32,
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: [u32; 3 as usize],
    pub max_mesh_total_memory_size: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
}

#[repr(C)]
pub struct DrawMeshTasksIndirectCommandNV {
    pub task_count: u32,
    pub first_task: u32,
}

#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vk_type: RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
}

#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vk_type: RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
    pub p_shader_group_capture_replay_handle: *const c_void,
}

#[repr(C)]
pub struct RayTracingPipelineCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub group_count: u32,
    pub p_groups: *const RayTracingShaderGroupCreateInfoNV,
    pub max_recursion_depth: u32,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

#[repr(C)]
pub struct RayTracingPipelineCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub group_count: u32,
    pub p_groups: *const RayTracingShaderGroupCreateInfoKHR,
    pub max_recursion_depth: u32,
    pub libraries: PipelineLibraryCreateInfoKHR,
    pub p_library_interface: *const RayTracingPipelineInterfaceCreateInfoKHR,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

#[repr(C)]
pub struct GeometryTrianglesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_data: Buffer,
    pub vertex_offset: DeviceSize,
    pub vertex_count: u32,
    pub vertex_stride: DeviceSize,
    pub vertex_format: Format,
    pub index_data: Buffer,
    pub index_offset: DeviceSize,
    pub index_count: u32,
    pub index_type: IndexType,
    pub transform_data: Buffer,
    pub transform_offset: DeviceSize,
}

#[repr(C)]
pub struct GeometryAABBNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aabb_data: Buffer,
    pub num_aabbs: u32,
    pub stride: u32,
    pub offset: DeviceSize,
}

#[repr(C)]
pub struct GeometryDataNV {
    pub triangles: GeometryTrianglesNV,
    pub aabbs: GeometryAABBNV,
}

#[repr(C)]
pub struct GeometryNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub geometry_type: GeometryTypeKHR,
    pub geometry: GeometryDataNV,
    pub flags: GeometryFlagsKHR,
}

#[repr(C)]
pub struct AccelerationStructureInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vk_type: AccelerationStructureTypeNV,
    pub flags: BuildAccelerationStructureFlagsNV,
    pub instance_count: u32,
    pub geometry_count: u32,
    pub p_geometries: *const GeometryNV,
}

#[repr(C)]
pub struct AccelerationStructureCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compacted_size: DeviceSize,
    pub info: AccelerationStructureInfoNV,
}

#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure: AccelerationStructureKHR,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}

#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoNV {}

#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const AccelerationStructureKHR,
}

#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureNV {}

#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vk_type: AccelerationStructureMemoryRequirementsTypeKHR,
    pub build_type: AccelerationStructureBuildTypeKHR,
    pub acceleration_structure: AccelerationStructureKHR,
}

#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vk_type: AccelerationStructureMemoryRequirementsTypeNV,
    pub acceleration_structure: AccelerationStructureNV,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ray_tracing: Bool32,
    pub ray_tracing_shader_group_handle_capture_replay: Bool32,
    pub ray_tracing_shader_group_handle_capture_replay_mixed: Bool32,
    pub ray_tracing_acceleration_structure_capture_replay: Bool32,
    pub ray_tracing_indirect_trace_rays: Bool32,
    pub ray_tracing_indirect_acceleration_structure_build: Bool32,
    pub ray_tracing_host_acceleration_structure_commands: Bool32,
    pub ray_query: Bool32,
    pub ray_tracing_primitive_culling: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_group_handle_size: u32,
    pub max_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_primitive_count: u64,
    pub max_descriptor_set_acceleration_structures: u32,
    pub shader_group_handle_capture_replay_size: u32,
}

#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_group_handle_size: u32,
    pub max_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_triangle_count: u64,
    pub max_descriptor_set_acceleration_structures: u32,
}

#[repr(C)]
pub struct StridedBufferRegionKHR {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub stride: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
pub struct TraceRaysIndirectCommandKHR {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
pub struct DrmFormatModifierPropertiesListEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties: *const DrmFormatModifierPropertiesEXT,
}

#[repr(C)]
pub struct DrmFormatModifierPropertiesEXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: FormatFeatureFlags,
}

#[repr(C)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier: u64,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}

#[repr(C)]
pub struct ImageDrmFormatModifierListCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifiers: *const u64,
}

#[repr(C)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub p_plane_layouts: *const SubresourceLayout,
}

#[repr(C)]
pub struct ImageDrmFormatModifierPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier: u64,
}

#[repr(C)]
pub struct ImageStencilUsageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stencil_usage: ImageUsageFlags,
}

#[repr(C)]
pub struct ImageStencilUsageCreateInfoEXT {}

#[repr(C)]
pub struct DeviceMemoryOverallocationCreateInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub overallocation_behavior: MemoryOverallocationBehaviorAMD,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_density_map: Bool32,
    pub fragment_density_map_dynamic: Bool32,
    pub fragment_density_map_non_subsampled_images: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_density_map_deferred: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_fragment_density_texel_size: Extent2D,
    pub max_fragment_density_texel_size: Extent2D,
    pub fragment_density_invocations: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subsampled_loads: Bool32,
    pub subsampled_coarse_reconstruction_early_access: Bool32,
    pub max_subsampled_array_layers: u32,
    pub max_descriptor_set_subsampled_samplers: u32,
}

#[repr(C)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_density_map_attachment: AttachmentReference,
}

#[repr(C)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub scalar_block_layout: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceScalarBlockLayoutFeaturesEXT {}

#[repr(C)]
pub struct SurfaceProtectedCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub supports_protected: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub uniform_buffer_standard_layout: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_clip_enable: Bool32,
}

#[repr(C)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
    pub depth_clip_enable: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub heap_budget: [DeviceSize; MAX_MEMORY_HEAPS as usize],
    pub heap_usage: [DeviceSize; MAX_MEMORY_HEAPS as usize],
}

#[repr(C)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_priority: Bool32,
}

#[repr(C)]
pub struct MemoryPriorityAllocateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub priority: f32,
}

#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesKHR {}

#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceBufferAddressFeaturesEXT {}

#[repr(C)]
pub struct BufferDeviceAddressInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}

#[repr(C)]
pub struct BufferDeviceAddressInfoKHR {}

#[repr(C)]
pub struct BufferDeviceAddressInfoEXT {}

#[repr(C)]
pub struct BufferOpaqueCaptureAddressCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_address: u64,
}

#[repr(C)]
pub struct BufferOpaqueCaptureAddressCreateInfoKHR {}

#[repr(C)]
pub struct BufferDeviceAddressCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_address: DeviceAddress,
}

#[repr(C)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view_type: ImageViewType,
}

#[repr(C)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub filter_cubic: Bool32,
    pub filter_cubic_minmax: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub imageless_framebuffer: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceImagelessFramebufferFeaturesKHR {}

#[repr(C)]
pub struct FramebufferAttachmentsCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_image_info_count: u32,
    pub p_attachment_image_infos: *const FramebufferAttachmentImageInfo,
}

#[repr(C)]
pub struct FramebufferAttachmentsCreateInfoKHR {}

#[repr(C)]
pub struct FramebufferAttachmentImageInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub usage: ImageUsageFlags,
    pub width: u32,
    pub height: u32,
    pub layer_count: u32,
    pub view_format_count: u32,
    pub p_view_formats: *const Format,
}

#[repr(C)]
pub struct FramebufferAttachmentImageInfoKHR {}

#[repr(C)]
pub struct RenderPassAttachmentBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
}

#[repr(C)]
pub struct RenderPassAttachmentBeginInfoKHR {}

#[repr(C)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub texture_compression_astc_hdr: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub cooperative_matrix: Bool32,
    pub cooperative_matrix_robust_buffer_access: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub cooperative_matrix_supported_stages: ShaderStageFlags,
}

#[repr(C)]
pub struct CooperativeMatrixPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub m_size: u32,
    pub n_size: u32,
    pub k_size: u32,
    pub a_type: ComponentTypeNV,
    pub b_type: ComponentTypeNV,
    pub c_type: ComponentTypeNV,
    pub d_type: ComponentTypeNV,
    pub scope: ScopeNV,
}

#[repr(C)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ycbcr_image_arrays: Bool32,
}

#[repr(C)]
pub struct ImageViewHandleInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub descriptor_type: DescriptorType,
    pub sampler: Sampler,
}

#[repr(C)]
pub struct ImageViewAddressPropertiesNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_address: DeviceAddress,
    pub size: DeviceSize,
}

#[repr(C)]
pub struct PresentFrameTokenGGP {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub frame_token: GgpFrameToken,
}

#[repr(C)]
pub struct PipelineCreationFeedbackEXT {
    pub flags: PipelineCreationFeedbackFlagsEXT,
    pub duration: u64,
}

#[repr(C)]
pub struct PipelineCreationFeedbackCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_pipeline_creation_feedback: *const PipelineCreationFeedbackEXT,
    pub pipeline_stage_creation_feedback_count: u32,
    pub p_pipeline_stage_creation_feedbacks: *const PipelineCreationFeedbackEXT,
}

#[repr(C)]
pub struct SurfaceFullScreenExclusiveInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub full_screen_exclusive: FullScreenExclusiveEXT,
}

#[repr(C)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub hmonitor: HMONITOR,
}

#[repr(C)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub full_screen_exclusive_supported: Bool32,
}

#[repr(C)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub performance_counter_query_pools: Bool32,
    pub performance_counter_multiple_query_pools: Bool32,
}

#[repr(C)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allow_command_buffer_query_copies: Bool32,
}

#[repr(C)]
pub struct PerformanceCounterKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub unit: PerformanceCounterUnitKHR,
    pub scope: PerformanceCounterScopeKHR,
    pub storage: PerformanceCounterStorageKHR,
    pub uuid: [u8; UUID_SIZE as usize],
}

#[repr(C)]
pub struct PerformanceCounterDescriptionKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PerformanceCounterDescriptionFlagsKHR,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub category: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
}

#[repr(C)]
pub struct QueryPoolPerformanceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_index: u32,
    pub counter_index_count: u32,
    pub p_counter_indices: *const u32,
}

#[repr(C)]
pub struct AcquireProfilingLockInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AcquireProfilingLockFlagsKHR,
    pub timeout: u64,
}

#[repr(C)]
pub struct PerformanceQuerySubmitInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub counter_pass_index: u32,
}

#[repr(C)]
pub struct HeadlessSurfaceCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: HeadlessSurfaceCreateFlagsEXT,
}

#[repr(C)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub coverage_reduction_mode: Bool32,
}

#[repr(C)]
pub struct PipelineCoverageReductionStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageReductionStateCreateFlagsNV,
    pub coverage_reduction_mode: CoverageReductionModeNV,
}

#[repr(C)]
pub struct FramebufferMixedSamplesCombinationNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub coverage_reduction_mode: CoverageReductionModeNV,
    pub rasterization_samples: SampleCountFlagBits,
    pub depth_stencil_samples: SampleCountFlags,
    pub color_samples: SampleCountFlags,
}

#[repr(C)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_integer_functions2: Bool32,
}

#[repr(C)]
pub struct PerformanceValueINTEL {
    pub vk_type: PerformanceValueTypeINTEL,
    pub data: PerformanceValueDataINTEL,
}

#[repr(C)]
pub struct InitializePerformanceApiInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_user_data: *const c_void,
}

#[repr(C)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub performance_counters_sampling: QueryPoolSamplingModeINTEL,
}

#[repr(C)]
pub struct QueryPoolCreateInfoINTEL {}

#[repr(C)]
pub struct PerformanceMarkerInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub marker: u64,
}

#[repr(C)]
pub struct PerformanceStreamMarkerInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub marker: u32,
}

#[repr(C)]
pub struct PerformanceOverrideInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vk_type: PerformanceOverrideTypeINTEL,
    pub enable: Bool32,
    pub parameter: u64,
}

#[repr(C)]
pub struct PerformanceConfigurationAcquireInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vk_type: PerformanceConfigurationTypeINTEL,
}

#[repr(C)]
pub struct PhysicalDeviceShaderClockFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_subgroup_clock: Bool32,
    pub shader_device_clock: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index_type_uint8: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_smcount: u32,
    pub shader_warps_per_sm: u32,
}

#[repr(C)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_smbuiltins: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_shader_sample_interlock: Bool32,
    pub fragment_shader_pixel_interlock: Bool32,
    pub fragment_shader_shading_rate_interlock: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub separate_depth_stencil_layouts: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR {}

#[repr(C)]
pub struct AttachmentReferenceStencilLayout {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stencil_layout: ImageLayout,
}

#[repr(C)]
pub struct AttachmentReferenceStencilLayoutKHR {}

#[repr(C)]
pub struct AttachmentDescriptionStencilLayout {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stencil_initial_layout: ImageLayout,
    pub stencil_final_layout: ImageLayout,
}

#[repr(C)]
pub struct AttachmentDescriptionStencilLayoutKHR {}

#[repr(C)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_executable_info: Bool32,
}

#[repr(C)]
pub struct PipelineInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
}

#[repr(C)]
pub struct PipelineExecutablePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stages: ShaderStageFlags,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub subgroup_size: u32,
}

#[repr(C)]
pub struct PipelineExecutableInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
    pub executable_index: u32,
}

#[repr(C)]
pub struct PipelineExecutableStatisticKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub format: PipelineExecutableStatisticFormatKHR,
    pub value: PipelineExecutableStatisticValueKHR,
}

#[repr(C)]
pub struct PipelineExecutableInternalRepresentationKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub is_text: Bool32,
    pub data_size: usize,
    pub p_data: *const c_void,
}

#[repr(C)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_demote_to_helper_invocation: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub texel_buffer_alignment: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
    pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subgroup_size_control: Bool32,
    pub compute_full_subgroups: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_compute_workgroup_subgroups: u32,
    pub required_subgroup_size_stages: ShaderStageFlags,
}

#[repr(C)]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub required_subgroup_size: u32,
}

#[repr(C)]
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_address: u64,
}

#[repr(C)]
pub struct MemoryOpaqueCaptureAddressAllocateInfoKHR {}

#[repr(C)]
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}

#[repr(C)]
pub struct DeviceMemoryOpaqueCaptureAddressInfoKHR {}

#[repr(C)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rectangular_lines: Bool32,
    pub bresenham_lines: Bool32,
    pub smooth_lines: Bool32,
    pub stippled_rectangular_lines: Bool32,
    pub stippled_bresenham_lines: Bool32,
    pub stippled_smooth_lines: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceLineRasterizationPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub line_sub_pixel_precision_bits: u32,
}

#[repr(C)]
pub struct PipelineRasterizationLineStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub line_rasterization_mode: LineRasterizationModeEXT,
    pub stippled_line_enable: Bool32,
    pub line_stipple_factor: u32,
    pub line_stipple_pattern: u16,
}

#[repr(C)]
pub struct PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_creation_cache_control: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVulkan11Features {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub storage_buffer16_bit_access: Bool32,
    pub uniform_and_storage_buffer16_bit_access: Bool32,
    pub storage_push_constant16: Bool32,
    pub storage_input_output16: Bool32,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
    pub protected_memory: Bool32,
    pub sampler_ycbcr_conversion: Bool32,
    pub shader_draw_parameters: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVulkan11Properties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_uuid: [u8; UUID_SIZE as usize],
    pub driver_uuid: [u8; UUID_SIZE as usize],
    pub device_luid: [u8; LUID_SIZE as usize],
    pub device_node_mask: u32,
    pub device_luidvalid: Bool32,
    pub subgroup_size: u32,
    pub subgroup_supported_stages: ShaderStageFlags,
    pub subgroup_supported_operations: SubgroupFeatureFlags,
    pub subgroup_quad_operations_in_all_stages: Bool32,
    pub point_clipping_behavior: PointClippingBehavior,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
    pub protected_no_fault: Bool32,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceVulkan12Features {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sampler_mirror_clamp_to_edge: Bool32,
    pub draw_indirect_count: Bool32,
    pub storage_buffer8_bit_access: Bool32,
    pub uniform_and_storage_buffer8_bit_access: Bool32,
    pub storage_push_constant8: Bool32,
    pub shader_buffer_int64_atomics: Bool32,
    pub shader_shared_int64_atomics: Bool32,
    pub shader_float16: Bool32,
    pub shader_int8: Bool32,
    pub descriptor_indexing: Bool32,
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_update_unused_while_pending: Bool32,
    pub descriptor_binding_partially_bound: Bool32,
    pub descriptor_binding_variable_descriptor_count: Bool32,
    pub runtime_descriptor_array: Bool32,
    pub sampler_filter_minmax: Bool32,
    pub scalar_block_layout: Bool32,
    pub imageless_framebuffer: Bool32,
    pub uniform_buffer_standard_layout: Bool32,
    pub shader_subgroup_extended_types: Bool32,
    pub separate_depth_stencil_layouts: Bool32,
    pub host_query_reset: Bool32,
    pub timeline_semaphore: Bool32,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
    pub shader_output_viewport_index: Bool32,
    pub shader_output_layer: Bool32,
    pub subgroup_broadcast_dynamic_id: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceVulkan12Properties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub driver_id: DriverId,
    pub driver_name: [c_char; MAX_DRIVER_NAME_SIZE as usize],
    pub driver_info: [c_char; MAX_DRIVER_INFO_SIZE as usize],
    pub conformance_version: ConformanceVersion,
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float16: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float32: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float64: Bool32,
    pub shader_denorm_preserve_float16: Bool32,
    pub shader_denorm_preserve_float32: Bool32,
    pub shader_denorm_preserve_float64: Bool32,
    pub shader_denorm_flush_to_zero_float16: Bool32,
    pub shader_denorm_flush_to_zero_float32: Bool32,
    pub shader_denorm_flush_to_zero_float64: Bool32,
    pub shader_rounding_mode_rtefloat16: Bool32,
    pub shader_rounding_mode_rtefloat32: Bool32,
    pub shader_rounding_mode_rtefloat64: Bool32,
    pub shader_rounding_mode_rtzfloat16: Bool32,
    pub shader_rounding_mode_rtzfloat32: Bool32,
    pub shader_rounding_mode_rtzfloat64: Bool32,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    pub robust_buffer_access_update_after_bind: Bool32,
    pub quad_divergent_implicit_lod: Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
    pub supported_depth_resolve_modes: ResolveModeFlags,
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    pub independent_resolve_none: Bool32,
    pub independent_resolve: Bool32,
    pub filter_minmax_single_component_formats: Bool32,
    pub filter_minmax_image_component_mapping: Bool32,
    pub max_timeline_semaphore_value_difference: u64,
    pub framebuffer_integer_color_sample_counts: SampleCountFlags,
}

#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compiler_control_flags: PipelineCompilerControlFlagsAMD,
}

#[repr(C)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_coherent_memory: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceToolPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub version: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub purposes: ToolPurposeFlagsEXT,
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub layer: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
}

#[repr(C)]
pub struct SamplerCustomBorderColorCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub custom_border_color: ClearColorValue,
    pub format: Format,
}

#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_custom_border_color_samplers: u32,
}

#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub custom_border_colors: Bool32,
    pub custom_border_color_without_format: Bool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryTrianglesDataKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_format: Format,
    pub vertex_data: DeviceOrHostAddressConstKHR,
    pub vertex_stride: DeviceSize,
    pub index_type: IndexType,
    pub index_data: DeviceOrHostAddressConstKHR,
    pub transform_data: DeviceOrHostAddressConstKHR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryAabbsDataKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data: DeviceOrHostAddressConstKHR,
    pub stride: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryInstancesDataKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub array_of_pointers: Bool32,
    pub data: DeviceOrHostAddressConstKHR,
}

#[repr(C)]
pub struct AccelerationStructureGeometryKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub geometry_type: GeometryTypeKHR,
    pub geometry: AccelerationStructureGeometryDataKHR,
    pub flags: GeometryFlagsKHR,
}

#[repr(C)]
pub struct AccelerationStructureBuildGeometryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vk_type: AccelerationStructureTypeKHR,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub update: Bool32,
    pub src_acceleration_structure: AccelerationStructureKHR,
    pub dst_acceleration_structure: AccelerationStructureKHR,
    pub geometry_array_of_pointers: Bool32,
    pub geometry_count: u32,
    pub pp_geometries: *const *const AccelerationStructureGeometryKHR,
    pub scratch_data: DeviceOrHostAddressKHR,
}

#[repr(C)]
pub struct AccelerationStructureBuildOffsetInfoKHR {
    pub primitive_count: u32,
    pub primitive_offset: u32,
    pub first_vertex: u32,
    pub transform_offset: u32,
}

#[repr(C)]
pub struct AccelerationStructureCreateGeometryTypeInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub geometry_type: GeometryTypeKHR,
    pub max_primitive_count: u32,
    pub index_type: IndexType,
    pub max_vertex_count: u32,
    pub vertex_format: Format,
    pub allows_transforms: Bool32,
}

#[repr(C)]
pub struct AccelerationStructureCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compacted_size: DeviceSize,
    pub vk_type: AccelerationStructureTypeKHR,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub max_geometry_count: u32,
    pub p_geometry_infos: *const AccelerationStructureCreateGeometryTypeInfoKHR,
    pub device_address: DeviceAddress,
}

#[repr(C)]
pub struct AabbPositionsKHR {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

#[repr(C)]
pub struct AabbPositionsNV {}

#[repr(C)]
pub struct TransformMatrixKHR {
    pub matrix: [f32; 3 as usize],
}

#[repr(C)]
pub struct TransformMatrixNV {}

#[repr(C)]
pub struct AccelerationStructureInstanceKHR {
    pub transform: TransformMatrixKHR,
    pub instance_custom_index: u32,
    pub mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}

#[repr(C)]
pub struct AccelerationStructureInstanceNV {}

#[repr(C)]
pub struct AccelerationStructureDeviceAddressInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure: AccelerationStructureKHR,
}

#[repr(C)]
pub struct AccelerationStructureVersionKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub version_data: *const u8,
}

#[repr(C)]
pub struct CopyAccelerationStructureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: AccelerationStructureKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct CopyAccelerationStructureToMemoryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: AccelerationStructureKHR,
    pub dst: DeviceOrHostAddressKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct CopyMemoryToAccelerationStructureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: DeviceOrHostAddressConstKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct RayTracingPipelineInterfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_payload_size: u32,
    pub max_attribute_size: u32,
    pub max_callable_size: u32,
}

#[repr(C)]
pub struct DeferredOperationInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub operation_handle: DeferredOperationKHR,
}

#[repr(C)]
pub struct PipelineLibraryCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub library_count: u32,
    pub p_libraries: *const Pipeline,
}

#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub extended_dynamic_state: Bool32,
}

#[repr(C)]
pub struct RenderPassTransformBeginInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform: SurfaceTransformFlagBitsKHR,
}

#[repr(C)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform: SurfaceTransformFlagBitsKHR,
    pub render_area: Rect2D,
}

#[repr(C)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub diagnostics_config: Bool32,
}

#[repr(C)]
pub struct DeviceDiagnosticsConfigCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceDiagnosticsConfigFlagsNV,
}

#[repr(C)]
pub struct PhysicalDeviceRobustness2FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub robust_buffer_access2: Bool32,
    pub robust_image_access2: Bool32,
    pub null_descriptor: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceRobustness2PropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub robust_storage_buffer_access_size_alignment: DeviceSize,
    pub robust_uniform_buffer_access_size_alignment: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceImageRobustnessFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub robust_image_access: Bool32,
}

// Vulkan constants
pub const NULL_HANDLE: u64 = 0;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const UUID_SIZE: u32 = 16;
pub const LUID_SIZE: u32 = 8;
pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const MAX_DESCRIPTION_SIZE: u32 = 256;
pub const MAX_MEMORY_TYPES: u32 = 32;
pub const MAX_MEMORY_HEAPS: u32 = 16;
pub const LOD_CLAMP_NONE: f32 = 1000.0;
pub const REMAINING_MIP_LEVELS: u32 = 0xffffffff;
pub const REMAINING_ARRAY_LAYERS: u32 = 0xffffffff;
pub const WHOLE_SIZE: u64 = 0xffffffffffffffff;
pub const ATTACHMENT_UNUSED: u32 = 0xffffffff;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const QUEUE_FAMILY_IGNORED: u32 = 0xffffffff;
pub const QUEUE_FAMILY_EXTERNAL: u32 = 0xffffffff;
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = 0xffffffff;
pub const SUBPASS_EXTERNAL: u32 = 0xffffffff;
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
pub const SHADER_UNUSED_KHR: u32 = 0xffffffff;

// Vulkan enum ImageLayout
pub type ImageLayout = u32;
pub const IMAGE_LAYOUT_UNDEFINED: u32 = 0;
pub const IMAGE_LAYOUT_GENERAL: u32 = 1;
pub const IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: u32 = 2;
pub const IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: u32 = 3;
pub const IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: u32 = 4;
pub const IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: u32 = 5;
pub const IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: u32 = 6;
pub const IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: u32 = 7;
pub const IMAGE_LAYOUT_PREINITIALIZED: u32 = 8;

// Vulkan enum AttachmentLoadOp
pub type AttachmentLoadOp = u32;
pub const ATTACHMENT_LOAD_OP_LOAD: u32 = 0;
pub const ATTACHMENT_LOAD_OP_CLEAR: u32 = 1;
pub const ATTACHMENT_LOAD_OP_DONT_CARE: u32 = 2;

// Vulkan enum AttachmentStoreOp
pub type AttachmentStoreOp = u32;
pub const ATTACHMENT_STORE_OP_STORE: u32 = 0;
pub const ATTACHMENT_STORE_OP_DONT_CARE: u32 = 1;

// Vulkan enum ImageType
pub type ImageType = u32;
pub const IMAGE_TYPE_1D: u32 = 0;
pub const IMAGE_TYPE_2D: u32 = 1;
pub const IMAGE_TYPE_3D: u32 = 2;

// Vulkan enum ImageTiling
pub type ImageTiling = u32;
pub const IMAGE_TILING_OPTIMAL: u32 = 0;
pub const IMAGE_TILING_LINEAR: u32 = 1;

// Vulkan enum ImageViewType
pub type ImageViewType = u32;
pub const IMAGE_VIEW_TYPE_1D: u32 = 0;
pub const IMAGE_VIEW_TYPE_2D: u32 = 1;
pub const IMAGE_VIEW_TYPE_3D: u32 = 2;
pub const IMAGE_VIEW_TYPE_CUBE: u32 = 3;
pub const IMAGE_VIEW_TYPE_1D_ARRAY: u32 = 4;
pub const IMAGE_VIEW_TYPE_2D_ARRAY: u32 = 5;
pub const IMAGE_VIEW_TYPE_CUBE_ARRAY: u32 = 6;

// Vulkan enum CommandBufferLevel
pub type CommandBufferLevel = u32;
pub const COMMAND_BUFFER_LEVEL_PRIMARY: u32 = 0;
pub const COMMAND_BUFFER_LEVEL_SECONDARY: u32 = 1;

// Vulkan enum ComponentSwizzle
pub type ComponentSwizzle = u32;
pub const COMPONENT_SWIZZLE_IDENTITY: u32 = 0;
pub const COMPONENT_SWIZZLE_ZERO: u32 = 1;
pub const COMPONENT_SWIZZLE_ONE: u32 = 2;
pub const COMPONENT_SWIZZLE_R: u32 = 3;
pub const COMPONENT_SWIZZLE_G: u32 = 4;
pub const COMPONENT_SWIZZLE_B: u32 = 5;
pub const COMPONENT_SWIZZLE_A: u32 = 6;

// Vulkan enum DescriptorType
pub type DescriptorType = u32;
pub const DESCRIPTOR_TYPE_SAMPLER: u32 = 0;
pub const DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: u32 = 1;
pub const DESCRIPTOR_TYPE_SAMPLED_IMAGE: u32 = 2;
pub const DESCRIPTOR_TYPE_STORAGE_IMAGE: u32 = 3;
pub const DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: u32 = 4;
pub const DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: u32 = 5;
pub const DESCRIPTOR_TYPE_UNIFORM_BUFFER: u32 = 6;
pub const DESCRIPTOR_TYPE_STORAGE_BUFFER: u32 = 7;
pub const DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: u32 = 8;
pub const DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: u32 = 9;
pub const DESCRIPTOR_TYPE_INPUT_ATTACHMENT: u32 = 10;

// Vulkan enum QueryType
pub type QueryType = u32;
pub const QUERY_TYPE_OCCLUSION: u32 = 0;
pub const QUERY_TYPE_PIPELINE_STATISTICS: u32 = 1;
pub const QUERY_TYPE_TIMESTAMP: u32 = 2;

// Vulkan enum BorderColor
pub type BorderColor = u32;
pub const BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: u32 = 0;
pub const BORDER_COLOR_INT_TRANSPARENT_BLACK: u32 = 1;
pub const BORDER_COLOR_FLOAT_OPAQUE_BLACK: u32 = 2;
pub const BORDER_COLOR_INT_OPAQUE_BLACK: u32 = 3;
pub const BORDER_COLOR_FLOAT_OPAQUE_WHITE: u32 = 4;
pub const BORDER_COLOR_INT_OPAQUE_WHITE: u32 = 5;

// Vulkan enum PipelineBindPoint
pub type PipelineBindPoint = u32;
pub const PIPELINE_BIND_POINT_GRAPHICS: u32 = 0;
pub const PIPELINE_BIND_POINT_COMPUTE: u32 = 1;

// Vulkan enum PipelineCacheHeaderVersion
pub type PipelineCacheHeaderVersion = u32;
pub const PIPELINE_CACHE_HEADER_VERSION_ONE: u32 = 1;

// Vulkan enum PipelineCacheCreateFlagBits
pub type PipelineCacheCreateFlagBits = u32;

// Vulkan enum PrimitiveTopology
pub type PrimitiveTopology = u32;
pub const PRIMITIVE_TOPOLOGY_POINT_LIST: u32 = 0;
pub const PRIMITIVE_TOPOLOGY_LINE_LIST: u32 = 1;
pub const PRIMITIVE_TOPOLOGY_LINE_STRIP: u32 = 2;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: u32 = 3;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: u32 = 4;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: u32 = 5;
pub const PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: u32 = 6;
pub const PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: u32 = 7;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: u32 = 8;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: u32 = 9;
pub const PRIMITIVE_TOPOLOGY_PATCH_LIST: u32 = 10;

// Vulkan enum SharingMode
pub type SharingMode = u32;
pub const SHARING_MODE_EXCLUSIVE: u32 = 0;
pub const SHARING_MODE_CONCURRENT: u32 = 1;

// Vulkan enum IndexType
pub type IndexType = u32;
pub const INDEX_TYPE_UINT16: u32 = 0;
pub const INDEX_TYPE_UINT32: u32 = 1;

// Vulkan enum Filter
pub type Filter = u32;
pub const FILTER_NEAREST: u32 = 0;
pub const FILTER_LINEAR: u32 = 1;

// Vulkan enum SamplerMipmapMode
pub type SamplerMipmapMode = u32;
pub const SAMPLER_MIPMAP_MODE_NEAREST: u32 = 0;
pub const SAMPLER_MIPMAP_MODE_LINEAR: u32 = 1;

// Vulkan enum SamplerAddressMode
pub type SamplerAddressMode = u32;
pub const SAMPLER_ADDRESS_MODE_REPEAT: u32 = 0;
pub const SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: u32 = 1;
pub const SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: u32 = 2;
pub const SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: u32 = 3;

// Vulkan enum CompareOp
pub type CompareOp = u32;
pub const COMPARE_OP_NEVER: u32 = 0;
pub const COMPARE_OP_LESS: u32 = 1;
pub const COMPARE_OP_EQUAL: u32 = 2;
pub const COMPARE_OP_LESS_OR_EQUAL: u32 = 3;
pub const COMPARE_OP_GREATER: u32 = 4;
pub const COMPARE_OP_NOT_EQUAL: u32 = 5;
pub const COMPARE_OP_GREATER_OR_EQUAL: u32 = 6;
pub const COMPARE_OP_ALWAYS: u32 = 7;

// Vulkan enum PolygonMode
pub type PolygonMode = u32;
pub const POLYGON_MODE_FILL: u32 = 0;
pub const POLYGON_MODE_LINE: u32 = 1;
pub const POLYGON_MODE_POINT: u32 = 2;

// Vulkan enum CullModeFlagBits
pub type CullModeFlagBits = u32;
pub const CULL_MODE_NONE: u32 = 0;
pub const CULL_MODE_FRONT_BIT: u32 = 0x1;
pub const CULL_MODE_BACK_BIT: u32 = 0x2;
pub const CULL_MODE_FRONT_AND_BACK: u32 = 0x00000003;

// Vulkan enum FrontFace
pub type FrontFace = u32;
pub const FRONT_FACE_COUNTER_CLOCKWISE: u32 = 0;
pub const FRONT_FACE_CLOCKWISE: u32 = 1;

// Vulkan enum BlendFactor
pub type BlendFactor = u32;
pub const BLEND_FACTOR_ZERO: u32 = 0;
pub const BLEND_FACTOR_ONE: u32 = 1;
pub const BLEND_FACTOR_SRC_COLOR: u32 = 2;
pub const BLEND_FACTOR_ONE_MINUS_SRC_COLOR: u32 = 3;
pub const BLEND_FACTOR_DST_COLOR: u32 = 4;
pub const BLEND_FACTOR_ONE_MINUS_DST_COLOR: u32 = 5;
pub const BLEND_FACTOR_SRC_ALPHA: u32 = 6;
pub const BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: u32 = 7;
pub const BLEND_FACTOR_DST_ALPHA: u32 = 8;
pub const BLEND_FACTOR_ONE_MINUS_DST_ALPHA: u32 = 9;
pub const BLEND_FACTOR_CONSTANT_COLOR: u32 = 10;
pub const BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: u32 = 11;
pub const BLEND_FACTOR_CONSTANT_ALPHA: u32 = 12;
pub const BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: u32 = 13;
pub const BLEND_FACTOR_SRC_ALPHA_SATURATE: u32 = 14;
pub const BLEND_FACTOR_SRC1_COLOR: u32 = 15;
pub const BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: u32 = 16;
pub const BLEND_FACTOR_SRC1_ALPHA: u32 = 17;
pub const BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: u32 = 18;

// Vulkan enum BlendOp
pub type BlendOp = u32;
pub const BLEND_OP_ADD: u32 = 0;
pub const BLEND_OP_SUBTRACT: u32 = 1;
pub const BLEND_OP_REVERSE_SUBTRACT: u32 = 2;
pub const BLEND_OP_MIN: u32 = 3;
pub const BLEND_OP_MAX: u32 = 4;

// Vulkan enum StencilOp
pub type StencilOp = u32;
pub const STENCIL_OP_KEEP: u32 = 0;
pub const STENCIL_OP_ZERO: u32 = 1;
pub const STENCIL_OP_REPLACE: u32 = 2;
pub const STENCIL_OP_INCREMENT_AND_CLAMP: u32 = 3;
pub const STENCIL_OP_DECREMENT_AND_CLAMP: u32 = 4;
pub const STENCIL_OP_INVERT: u32 = 5;
pub const STENCIL_OP_INCREMENT_AND_WRAP: u32 = 6;
pub const STENCIL_OP_DECREMENT_AND_WRAP: u32 = 7;

// Vulkan enum LogicOp
pub type LogicOp = u32;
pub const LOGIC_OP_CLEAR: u32 = 0;
pub const LOGIC_OP_AND: u32 = 1;
pub const LOGIC_OP_AND_REVERSE: u32 = 2;
pub const LOGIC_OP_COPY: u32 = 3;
pub const LOGIC_OP_AND_INVERTED: u32 = 4;
pub const LOGIC_OP_NO_OP: u32 = 5;
pub const LOGIC_OP_XOR: u32 = 6;
pub const LOGIC_OP_OR: u32 = 7;
pub const LOGIC_OP_NOR: u32 = 8;
pub const LOGIC_OP_EQUIVALENT: u32 = 9;
pub const LOGIC_OP_INVERT: u32 = 10;
pub const LOGIC_OP_OR_REVERSE: u32 = 11;
pub const LOGIC_OP_COPY_INVERTED: u32 = 12;
pub const LOGIC_OP_OR_INVERTED: u32 = 13;
pub const LOGIC_OP_NAND: u32 = 14;
pub const LOGIC_OP_SET: u32 = 15;

// Vulkan enum InternalAllocationType
pub type InternalAllocationType = u32;
pub const INTERNAL_ALLOCATION_TYPE_EXECUTABLE: u32 = 0;

// Vulkan enum SystemAllocationScope
pub type SystemAllocationScope = u32;
pub const SYSTEM_ALLOCATION_SCOPE_COMMAND: u32 = 0;
pub const SYSTEM_ALLOCATION_SCOPE_OBJECT: u32 = 1;
pub const SYSTEM_ALLOCATION_SCOPE_CACHE: u32 = 2;
pub const SYSTEM_ALLOCATION_SCOPE_DEVICE: u32 = 3;
pub const SYSTEM_ALLOCATION_SCOPE_INSTANCE: u32 = 4;

// Vulkan enum PhysicalDeviceType
pub type PhysicalDeviceType = u32;
pub const PHYSICAL_DEVICE_TYPE_OTHER: u32 = 0;
pub const PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: u32 = 1;
pub const PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: u32 = 2;
pub const PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: u32 = 3;
pub const PHYSICAL_DEVICE_TYPE_CPU: u32 = 4;

// Vulkan enum VertexInputRate
pub type VertexInputRate = u32;
pub const VERTEX_INPUT_RATE_VERTEX: u32 = 0;
pub const VERTEX_INPUT_RATE_INSTANCE: u32 = 1;

// Vulkan enum Format
pub type Format = u32;
pub const FORMAT_UNDEFINED: u32 = 0;
pub const FORMAT_R4G4_UNORM_PACK8: u32 = 1;
pub const FORMAT_R4G4B4A4_UNORM_PACK16: u32 = 2;
pub const FORMAT_B4G4R4A4_UNORM_PACK16: u32 = 3;
pub const FORMAT_R5G6B5_UNORM_PACK16: u32 = 4;
pub const FORMAT_B5G6R5_UNORM_PACK16: u32 = 5;
pub const FORMAT_R5G5B5A1_UNORM_PACK16: u32 = 6;
pub const FORMAT_B5G5R5A1_UNORM_PACK16: u32 = 7;
pub const FORMAT_A1R5G5B5_UNORM_PACK16: u32 = 8;
pub const FORMAT_R8_UNORM: u32 = 9;
pub const FORMAT_R8_SNORM: u32 = 10;
pub const FORMAT_R8_USCALED: u32 = 11;
pub const FORMAT_R8_SSCALED: u32 = 12;
pub const FORMAT_R8_UINT: u32 = 13;
pub const FORMAT_R8_SINT: u32 = 14;
pub const FORMAT_R8_SRGB: u32 = 15;
pub const FORMAT_R8G8_UNORM: u32 = 16;
pub const FORMAT_R8G8_SNORM: u32 = 17;
pub const FORMAT_R8G8_USCALED: u32 = 18;
pub const FORMAT_R8G8_SSCALED: u32 = 19;
pub const FORMAT_R8G8_UINT: u32 = 20;
pub const FORMAT_R8G8_SINT: u32 = 21;
pub const FORMAT_R8G8_SRGB: u32 = 22;
pub const FORMAT_R8G8B8_UNORM: u32 = 23;
pub const FORMAT_R8G8B8_SNORM: u32 = 24;
pub const FORMAT_R8G8B8_USCALED: u32 = 25;
pub const FORMAT_R8G8B8_SSCALED: u32 = 26;
pub const FORMAT_R8G8B8_UINT: u32 = 27;
pub const FORMAT_R8G8B8_SINT: u32 = 28;
pub const FORMAT_R8G8B8_SRGB: u32 = 29;
pub const FORMAT_B8G8R8_UNORM: u32 = 30;
pub const FORMAT_B8G8R8_SNORM: u32 = 31;
pub const FORMAT_B8G8R8_USCALED: u32 = 32;
pub const FORMAT_B8G8R8_SSCALED: u32 = 33;
pub const FORMAT_B8G8R8_UINT: u32 = 34;
pub const FORMAT_B8G8R8_SINT: u32 = 35;
pub const FORMAT_B8G8R8_SRGB: u32 = 36;
pub const FORMAT_R8G8B8A8_UNORM: u32 = 37;
pub const FORMAT_R8G8B8A8_SNORM: u32 = 38;
pub const FORMAT_R8G8B8A8_USCALED: u32 = 39;
pub const FORMAT_R8G8B8A8_SSCALED: u32 = 40;
pub const FORMAT_R8G8B8A8_UINT: u32 = 41;
pub const FORMAT_R8G8B8A8_SINT: u32 = 42;
pub const FORMAT_R8G8B8A8_SRGB: u32 = 43;
pub const FORMAT_B8G8R8A8_UNORM: u32 = 44;
pub const FORMAT_B8G8R8A8_SNORM: u32 = 45;
pub const FORMAT_B8G8R8A8_USCALED: u32 = 46;
pub const FORMAT_B8G8R8A8_SSCALED: u32 = 47;
pub const FORMAT_B8G8R8A8_UINT: u32 = 48;
pub const FORMAT_B8G8R8A8_SINT: u32 = 49;
pub const FORMAT_B8G8R8A8_SRGB: u32 = 50;
pub const FORMAT_A8B8G8R8_UNORM_PACK32: u32 = 51;
pub const FORMAT_A8B8G8R8_SNORM_PACK32: u32 = 52;
pub const FORMAT_A8B8G8R8_USCALED_PACK32: u32 = 53;
pub const FORMAT_A8B8G8R8_SSCALED_PACK32: u32 = 54;
pub const FORMAT_A8B8G8R8_UINT_PACK32: u32 = 55;
pub const FORMAT_A8B8G8R8_SINT_PACK32: u32 = 56;
pub const FORMAT_A8B8G8R8_SRGB_PACK32: u32 = 57;
pub const FORMAT_A2R10G10B10_UNORM_PACK32: u32 = 58;
pub const FORMAT_A2R10G10B10_SNORM_PACK32: u32 = 59;
pub const FORMAT_A2R10G10B10_USCALED_PACK32: u32 = 60;
pub const FORMAT_A2R10G10B10_SSCALED_PACK32: u32 = 61;
pub const FORMAT_A2R10G10B10_UINT_PACK32: u32 = 62;
pub const FORMAT_A2R10G10B10_SINT_PACK32: u32 = 63;
pub const FORMAT_A2B10G10R10_UNORM_PACK32: u32 = 64;
pub const FORMAT_A2B10G10R10_SNORM_PACK32: u32 = 65;
pub const FORMAT_A2B10G10R10_USCALED_PACK32: u32 = 66;
pub const FORMAT_A2B10G10R10_SSCALED_PACK32: u32 = 67;
pub const FORMAT_A2B10G10R10_UINT_PACK32: u32 = 68;
pub const FORMAT_A2B10G10R10_SINT_PACK32: u32 = 69;
pub const FORMAT_R16_UNORM: u32 = 70;
pub const FORMAT_R16_SNORM: u32 = 71;
pub const FORMAT_R16_USCALED: u32 = 72;
pub const FORMAT_R16_SSCALED: u32 = 73;
pub const FORMAT_R16_UINT: u32 = 74;
pub const FORMAT_R16_SINT: u32 = 75;
pub const FORMAT_R16_SFLOAT: u32 = 76;
pub const FORMAT_R16G16_UNORM: u32 = 77;
pub const FORMAT_R16G16_SNORM: u32 = 78;
pub const FORMAT_R16G16_USCALED: u32 = 79;
pub const FORMAT_R16G16_SSCALED: u32 = 80;
pub const FORMAT_R16G16_UINT: u32 = 81;
pub const FORMAT_R16G16_SINT: u32 = 82;
pub const FORMAT_R16G16_SFLOAT: u32 = 83;
pub const FORMAT_R16G16B16_UNORM: u32 = 84;
pub const FORMAT_R16G16B16_SNORM: u32 = 85;
pub const FORMAT_R16G16B16_USCALED: u32 = 86;
pub const FORMAT_R16G16B16_SSCALED: u32 = 87;
pub const FORMAT_R16G16B16_UINT: u32 = 88;
pub const FORMAT_R16G16B16_SINT: u32 = 89;
pub const FORMAT_R16G16B16_SFLOAT: u32 = 90;
pub const FORMAT_R16G16B16A16_UNORM: u32 = 91;
pub const FORMAT_R16G16B16A16_SNORM: u32 = 92;
pub const FORMAT_R16G16B16A16_USCALED: u32 = 93;
pub const FORMAT_R16G16B16A16_SSCALED: u32 = 94;
pub const FORMAT_R16G16B16A16_UINT: u32 = 95;
pub const FORMAT_R16G16B16A16_SINT: u32 = 96;
pub const FORMAT_R16G16B16A16_SFLOAT: u32 = 97;
pub const FORMAT_R32_UINT: u32 = 98;
pub const FORMAT_R32_SINT: u32 = 99;
pub const FORMAT_R32_SFLOAT: u32 = 100;
pub const FORMAT_R32G32_UINT: u32 = 101;
pub const FORMAT_R32G32_SINT: u32 = 102;
pub const FORMAT_R32G32_SFLOAT: u32 = 103;
pub const FORMAT_R32G32B32_UINT: u32 = 104;
pub const FORMAT_R32G32B32_SINT: u32 = 105;
pub const FORMAT_R32G32B32_SFLOAT: u32 = 106;
pub const FORMAT_R32G32B32A32_UINT: u32 = 107;
pub const FORMAT_R32G32B32A32_SINT: u32 = 108;
pub const FORMAT_R32G32B32A32_SFLOAT: u32 = 109;
pub const FORMAT_R64_UINT: u32 = 110;
pub const FORMAT_R64_SINT: u32 = 111;
pub const FORMAT_R64_SFLOAT: u32 = 112;
pub const FORMAT_R64G64_UINT: u32 = 113;
pub const FORMAT_R64G64_SINT: u32 = 114;
pub const FORMAT_R64G64_SFLOAT: u32 = 115;
pub const FORMAT_R64G64B64_UINT: u32 = 116;
pub const FORMAT_R64G64B64_SINT: u32 = 117;
pub const FORMAT_R64G64B64_SFLOAT: u32 = 118;
pub const FORMAT_R64G64B64A64_UINT: u32 = 119;
pub const FORMAT_R64G64B64A64_SINT: u32 = 120;
pub const FORMAT_R64G64B64A64_SFLOAT: u32 = 121;
pub const FORMAT_B10G11R11_UFLOAT_PACK32: u32 = 122;
pub const FORMAT_E5B9G9R9_UFLOAT_PACK32: u32 = 123;
pub const FORMAT_D16_UNORM: u32 = 124;
pub const FORMAT_X8_D24_UNORM_PACK32: u32 = 125;
pub const FORMAT_D32_SFLOAT: u32 = 126;
pub const FORMAT_S8_UINT: u32 = 127;
pub const FORMAT_D16_UNORM_S8_UINT: u32 = 128;
pub const FORMAT_D24_UNORM_S8_UINT: u32 = 129;
pub const FORMAT_D32_SFLOAT_S8_UINT: u32 = 130;
pub const FORMAT_BC1_RGB_UNORM_BLOCK: u32 = 131;
pub const FORMAT_BC1_RGB_SRGB_BLOCK: u32 = 132;
pub const FORMAT_BC1_RGBA_UNORM_BLOCK: u32 = 133;
pub const FORMAT_BC1_RGBA_SRGB_BLOCK: u32 = 134;
pub const FORMAT_BC2_UNORM_BLOCK: u32 = 135;
pub const FORMAT_BC2_SRGB_BLOCK: u32 = 136;
pub const FORMAT_BC3_UNORM_BLOCK: u32 = 137;
pub const FORMAT_BC3_SRGB_BLOCK: u32 = 138;
pub const FORMAT_BC4_UNORM_BLOCK: u32 = 139;
pub const FORMAT_BC4_SNORM_BLOCK: u32 = 140;
pub const FORMAT_BC5_UNORM_BLOCK: u32 = 141;
pub const FORMAT_BC5_SNORM_BLOCK: u32 = 142;
pub const FORMAT_BC6H_UFLOAT_BLOCK: u32 = 143;
pub const FORMAT_BC6H_SFLOAT_BLOCK: u32 = 144;
pub const FORMAT_BC7_UNORM_BLOCK: u32 = 145;
pub const FORMAT_BC7_SRGB_BLOCK: u32 = 146;
pub const FORMAT_ETC2_R8G8B8_UNORM_BLOCK: u32 = 147;
pub const FORMAT_ETC2_R8G8B8_SRGB_BLOCK: u32 = 148;
pub const FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: u32 = 149;
pub const FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: u32 = 150;
pub const FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: u32 = 151;
pub const FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: u32 = 152;
pub const FORMAT_EAC_R11_UNORM_BLOCK: u32 = 153;
pub const FORMAT_EAC_R11_SNORM_BLOCK: u32 = 154;
pub const FORMAT_EAC_R11G11_UNORM_BLOCK: u32 = 155;
pub const FORMAT_EAC_R11G11_SNORM_BLOCK: u32 = 156;
pub const FORMAT_ASTC_4X4_UNORM_BLOCK: u32 = 157;
pub const FORMAT_ASTC_4X4_SRGB_BLOCK: u32 = 158;
pub const FORMAT_ASTC_5X4_UNORM_BLOCK: u32 = 159;
pub const FORMAT_ASTC_5X4_SRGB_BLOCK: u32 = 160;
pub const FORMAT_ASTC_5X5_UNORM_BLOCK: u32 = 161;
pub const FORMAT_ASTC_5X5_SRGB_BLOCK: u32 = 162;
pub const FORMAT_ASTC_6X5_UNORM_BLOCK: u32 = 163;
pub const FORMAT_ASTC_6X5_SRGB_BLOCK: u32 = 164;
pub const FORMAT_ASTC_6X6_UNORM_BLOCK: u32 = 165;
pub const FORMAT_ASTC_6X6_SRGB_BLOCK: u32 = 166;
pub const FORMAT_ASTC_8X5_UNORM_BLOCK: u32 = 167;
pub const FORMAT_ASTC_8X5_SRGB_BLOCK: u32 = 168;
pub const FORMAT_ASTC_8X6_UNORM_BLOCK: u32 = 169;
pub const FORMAT_ASTC_8X6_SRGB_BLOCK: u32 = 170;
pub const FORMAT_ASTC_8X8_UNORM_BLOCK: u32 = 171;
pub const FORMAT_ASTC_8X8_SRGB_BLOCK: u32 = 172;
pub const FORMAT_ASTC_10X5_UNORM_BLOCK: u32 = 173;
pub const FORMAT_ASTC_10X5_SRGB_BLOCK: u32 = 174;
pub const FORMAT_ASTC_10X6_UNORM_BLOCK: u32 = 175;
pub const FORMAT_ASTC_10X6_SRGB_BLOCK: u32 = 176;
pub const FORMAT_ASTC_10X8_UNORM_BLOCK: u32 = 177;
pub const FORMAT_ASTC_10X8_SRGB_BLOCK: u32 = 178;
pub const FORMAT_ASTC_10X10_UNORM_BLOCK: u32 = 179;
pub const FORMAT_ASTC_10X10_SRGB_BLOCK: u32 = 180;
pub const FORMAT_ASTC_12X10_UNORM_BLOCK: u32 = 181;
pub const FORMAT_ASTC_12X10_SRGB_BLOCK: u32 = 182;
pub const FORMAT_ASTC_12X12_UNORM_BLOCK: u32 = 183;
pub const FORMAT_ASTC_12X12_SRGB_BLOCK: u32 = 184;

// Vulkan enum StructureType
pub type StructureType = u32;
pub const STRUCTURE_TYPE_APPLICATION_INFO: u32 = 0;
pub const STRUCTURE_TYPE_INSTANCE_CREATE_INFO: u32 = 1;
pub const STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: u32 = 2;
pub const STRUCTURE_TYPE_DEVICE_CREATE_INFO: u32 = 3;
pub const STRUCTURE_TYPE_SUBMIT_INFO: u32 = 4;
pub const STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: u32 = 5;
pub const STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: u32 = 6;
pub const STRUCTURE_TYPE_BIND_SPARSE_INFO: u32 = 7;
pub const STRUCTURE_TYPE_FENCE_CREATE_INFO: u32 = 8;
pub const STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: u32 = 9;
pub const STRUCTURE_TYPE_EVENT_CREATE_INFO: u32 = 10;
pub const STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: u32 = 11;
pub const STRUCTURE_TYPE_BUFFER_CREATE_INFO: u32 = 12;
pub const STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: u32 = 13;
pub const STRUCTURE_TYPE_IMAGE_CREATE_INFO: u32 = 14;
pub const STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: u32 = 15;
pub const STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: u32 = 16;
pub const STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: u32 = 17;
pub const STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: u32 = 18;
pub const STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: u32 = 19;
pub const STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: u32 = 20;
pub const STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: u32 = 21;
pub const STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: u32 = 22;
pub const STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: u32 = 23;
pub const STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: u32 = 24;
pub const STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: u32 = 25;
pub const STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: u32 = 26;
pub const STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: u32 = 27;
pub const STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: u32 = 28;
pub const STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: u32 = 29;
pub const STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: u32 = 30;
pub const STRUCTURE_TYPE_SAMPLER_CREATE_INFO: u32 = 31;
pub const STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: u32 = 32;
pub const STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: u32 = 33;
pub const STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: u32 = 34;
pub const STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: u32 = 35;
pub const STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: u32 = 36;
pub const STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: u32 = 37;
pub const STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: u32 = 38;
pub const STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: u32 = 39;
pub const STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: u32 = 40;
pub const STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: u32 = 41;
pub const STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: u32 = 42;
pub const STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: u32 = 43;
pub const STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: u32 = 44;
pub const STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: u32 = 45;
pub const STRUCTURE_TYPE_MEMORY_BARRIER: u32 = 46;
pub const STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: u32 = 47;
pub const STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: u32 = 48;

// Vulkan enum SubpassContents
pub type SubpassContents = u32;
pub const SUBPASS_CONTENTS_INLINE: u32 = 0;
pub const SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: u32 = 1;

// Vulkan enum Result
pub type Result = i32;
pub const SUCCESS: i32 = 0;
pub const NOT_READY: i32 = 1;
pub const TIMEOUT: i32 = 2;
pub const EVENT_SET: i32 = 3;
pub const EVENT_RESET: i32 = 4;
pub const INCOMPLETE: i32 = 5;
pub const ERROR_OUT_OF_HOST_MEMORY: i32 = -1;
pub const ERROR_OUT_OF_DEVICE_MEMORY: i32 = -2;
pub const ERROR_INITIALIZATION_FAILED: i32 = -3;
pub const ERROR_DEVICE_LOST: i32 = -4;
pub const ERROR_MEMORY_MAP_FAILED: i32 = -5;
pub const ERROR_LAYER_NOT_PRESENT: i32 = -6;
pub const ERROR_EXTENSION_NOT_PRESENT: i32 = -7;
pub const ERROR_FEATURE_NOT_PRESENT: i32 = -8;
pub const ERROR_INCOMPATIBLE_DRIVER: i32 = -9;
pub const ERROR_TOO_MANY_OBJECTS: i32 = -10;
pub const ERROR_FORMAT_NOT_SUPPORTED: i32 = -11;
pub const ERROR_FRAGMENTED_POOL: i32 = -12;
pub const ERROR_UNKNOWN: i32 = -13;

// Vulkan enum DynamicState
pub type DynamicState = u32;
pub const DYNAMIC_STATE_VIEWPORT: u32 = 0;
pub const DYNAMIC_STATE_SCISSOR: u32 = 1;
pub const DYNAMIC_STATE_LINE_WIDTH: u32 = 2;
pub const DYNAMIC_STATE_DEPTH_BIAS: u32 = 3;
pub const DYNAMIC_STATE_BLEND_CONSTANTS: u32 = 4;
pub const DYNAMIC_STATE_DEPTH_BOUNDS: u32 = 5;
pub const DYNAMIC_STATE_STENCIL_COMPARE_MASK: u32 = 6;
pub const DYNAMIC_STATE_STENCIL_WRITE_MASK: u32 = 7;
pub const DYNAMIC_STATE_STENCIL_REFERENCE: u32 = 8;

// Vulkan enum DescriptorUpdateTemplateType
pub type DescriptorUpdateTemplateType = u32;
pub const DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET: u32 = 0;

// Vulkan enum ObjectType
pub type ObjectType = u32;
pub const OBJECT_TYPE_UNKNOWN: u32 = 0;
pub const OBJECT_TYPE_INSTANCE: u32 = 1;
pub const OBJECT_TYPE_PHYSICAL_DEVICE: u32 = 2;
pub const OBJECT_TYPE_DEVICE: u32 = 3;
pub const OBJECT_TYPE_QUEUE: u32 = 4;
pub const OBJECT_TYPE_SEMAPHORE: u32 = 5;
pub const OBJECT_TYPE_COMMAND_BUFFER: u32 = 6;
pub const OBJECT_TYPE_FENCE: u32 = 7;
pub const OBJECT_TYPE_DEVICE_MEMORY: u32 = 8;
pub const OBJECT_TYPE_BUFFER: u32 = 9;
pub const OBJECT_TYPE_IMAGE: u32 = 10;
pub const OBJECT_TYPE_EVENT: u32 = 11;
pub const OBJECT_TYPE_QUERY_POOL: u32 = 12;
pub const OBJECT_TYPE_BUFFER_VIEW: u32 = 13;
pub const OBJECT_TYPE_IMAGE_VIEW: u32 = 14;
pub const OBJECT_TYPE_SHADER_MODULE: u32 = 15;
pub const OBJECT_TYPE_PIPELINE_CACHE: u32 = 16;
pub const OBJECT_TYPE_PIPELINE_LAYOUT: u32 = 17;
pub const OBJECT_TYPE_RENDER_PASS: u32 = 18;
pub const OBJECT_TYPE_PIPELINE: u32 = 19;
pub const OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: u32 = 20;
pub const OBJECT_TYPE_SAMPLER: u32 = 21;
pub const OBJECT_TYPE_DESCRIPTOR_POOL: u32 = 22;
pub const OBJECT_TYPE_DESCRIPTOR_SET: u32 = 23;
pub const OBJECT_TYPE_FRAMEBUFFER: u32 = 24;
pub const OBJECT_TYPE_COMMAND_POOL: u32 = 25;

// Vulkan enum QueueFlagBits
pub type QueueFlagBits = u32;
pub const QUEUE_GRAPHICS_BIT: u32 = 0x1;
pub const QUEUE_COMPUTE_BIT: u32 = 0x2;
pub const QUEUE_TRANSFER_BIT: u32 = 0x4;
pub const QUEUE_SPARSE_BINDING_BIT: u32 = 0x8;

// Vulkan enum RenderPassCreateFlagBits
pub type RenderPassCreateFlagBits = u32;

// Vulkan enum DeviceQueueCreateFlagBits
pub type DeviceQueueCreateFlagBits = u32;

// Vulkan enum MemoryPropertyFlagBits
pub type MemoryPropertyFlagBits = u32;
pub const MEMORY_PROPERTY_DEVICE_LOCAL_BIT: u32 = 0x1;
pub const MEMORY_PROPERTY_HOST_VISIBLE_BIT: u32 = 0x2;
pub const MEMORY_PROPERTY_HOST_COHERENT_BIT: u32 = 0x4;
pub const MEMORY_PROPERTY_HOST_CACHED_BIT: u32 = 0x8;
pub const MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: u32 = 0x10;

// Vulkan enum MemoryHeapFlagBits
pub type MemoryHeapFlagBits = u32;
pub const MEMORY_HEAP_DEVICE_LOCAL_BIT: u32 = 0x1;

// Vulkan enum AccessFlagBits
pub type AccessFlagBits = u32;
pub const ACCESS_INDIRECT_COMMAND_READ_BIT: u32 = 0x1;
pub const ACCESS_INDEX_READ_BIT: u32 = 0x2;
pub const ACCESS_VERTEX_ATTRIBUTE_READ_BIT: u32 = 0x4;
pub const ACCESS_UNIFORM_READ_BIT: u32 = 0x8;
pub const ACCESS_INPUT_ATTACHMENT_READ_BIT: u32 = 0x10;
pub const ACCESS_SHADER_READ_BIT: u32 = 0x20;
pub const ACCESS_SHADER_WRITE_BIT: u32 = 0x40;
pub const ACCESS_COLOR_ATTACHMENT_READ_BIT: u32 = 0x80;
pub const ACCESS_COLOR_ATTACHMENT_WRITE_BIT: u32 = 0x100;
pub const ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: u32 = 0x200;
pub const ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: u32 = 0x400;
pub const ACCESS_TRANSFER_READ_BIT: u32 = 0x800;
pub const ACCESS_TRANSFER_WRITE_BIT: u32 = 0x1000;
pub const ACCESS_HOST_READ_BIT: u32 = 0x2000;
pub const ACCESS_HOST_WRITE_BIT: u32 = 0x4000;
pub const ACCESS_MEMORY_READ_BIT: u32 = 0x8000;
pub const ACCESS_MEMORY_WRITE_BIT: u32 = 0x10000;

// Vulkan enum BufferUsageFlagBits
pub type BufferUsageFlagBits = u32;
pub const BUFFER_USAGE_TRANSFER_SRC_BIT: u32 = 0x1;
pub const BUFFER_USAGE_TRANSFER_DST_BIT: u32 = 0x2;
pub const BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: u32 = 0x4;
pub const BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: u32 = 0x8;
pub const BUFFER_USAGE_UNIFORM_BUFFER_BIT: u32 = 0x10;
pub const BUFFER_USAGE_STORAGE_BUFFER_BIT: u32 = 0x20;
pub const BUFFER_USAGE_INDEX_BUFFER_BIT: u32 = 0x40;
pub const BUFFER_USAGE_VERTEX_BUFFER_BIT: u32 = 0x80;
pub const BUFFER_USAGE_INDIRECT_BUFFER_BIT: u32 = 0x100;

// Vulkan enum BufferCreateFlagBits
pub type BufferCreateFlagBits = u32;
pub const BUFFER_CREATE_SPARSE_BINDING_BIT: u32 = 0x1;
pub const BUFFER_CREATE_SPARSE_RESIDENCY_BIT: u32 = 0x2;
pub const BUFFER_CREATE_SPARSE_ALIASED_BIT: u32 = 0x4;

// Vulkan enum ShaderStageFlagBits
pub type ShaderStageFlagBits = u32;
pub const SHADER_STAGE_VERTEX_BIT: u32 = 0x1;
pub const SHADER_STAGE_TESSELLATION_CONTROL_BIT: u32 = 0x2;
pub const SHADER_STAGE_TESSELLATION_EVALUATION_BIT: u32 = 0x4;
pub const SHADER_STAGE_GEOMETRY_BIT: u32 = 0x8;
pub const SHADER_STAGE_FRAGMENT_BIT: u32 = 0x10;
pub const SHADER_STAGE_COMPUTE_BIT: u32 = 0x20;
pub const SHADER_STAGE_ALL_GRAPHICS: u32 = 0x0000001F;
pub const SHADER_STAGE_ALL: u32 = 0x7FFFFFFF;

// Vulkan enum ImageUsageFlagBits
pub type ImageUsageFlagBits = u32;
pub const IMAGE_USAGE_TRANSFER_SRC_BIT: u32 = 0x1;
pub const IMAGE_USAGE_TRANSFER_DST_BIT: u32 = 0x2;
pub const IMAGE_USAGE_SAMPLED_BIT: u32 = 0x4;
pub const IMAGE_USAGE_STORAGE_BIT: u32 = 0x8;
pub const IMAGE_USAGE_COLOR_ATTACHMENT_BIT: u32 = 0x10;
pub const IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: u32 = 0x20;
pub const IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: u32 = 0x40;
pub const IMAGE_USAGE_INPUT_ATTACHMENT_BIT: u32 = 0x80;

// Vulkan enum ImageCreateFlagBits
pub type ImageCreateFlagBits = u32;
pub const IMAGE_CREATE_SPARSE_BINDING_BIT: u32 = 0x1;
pub const IMAGE_CREATE_SPARSE_RESIDENCY_BIT: u32 = 0x2;
pub const IMAGE_CREATE_SPARSE_ALIASED_BIT: u32 = 0x4;
pub const IMAGE_CREATE_MUTABLE_FORMAT_BIT: u32 = 0x8;
pub const IMAGE_CREATE_CUBE_COMPATIBLE_BIT: u32 = 0x10;

// Vulkan enum ImageViewCreateFlagBits
pub type ImageViewCreateFlagBits = u32;

// Vulkan enum SamplerCreateFlagBits
pub type SamplerCreateFlagBits = u32;

// Vulkan enum PipelineCreateFlagBits
pub type PipelineCreateFlagBits = u32;
pub const PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: u32 = 0x1;
pub const PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: u32 = 0x2;
pub const PIPELINE_CREATE_DERIVATIVE_BIT: u32 = 0x4;

// Vulkan enum PipelineShaderStageCreateFlagBits
pub type PipelineShaderStageCreateFlagBits = u32;

// Vulkan enum ColorComponentFlagBits
pub type ColorComponentFlagBits = u32;
pub const COLOR_COMPONENT_R_BIT: u32 = 0x1;
pub const COLOR_COMPONENT_G_BIT: u32 = 0x2;
pub const COLOR_COMPONENT_B_BIT: u32 = 0x4;
pub const COLOR_COMPONENT_A_BIT: u32 = 0x8;

// Vulkan enum FenceCreateFlagBits
pub type FenceCreateFlagBits = u32;
pub const FENCE_CREATE_SIGNALED_BIT: u32 = 0x1;

// Vulkan enum SemaphoreCreateFlagBits
pub type SemaphoreCreateFlagBits = u32;

// Vulkan enum FormatFeatureFlagBits
pub type FormatFeatureFlagBits = u32;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_BIT: u32 = 0x1;
pub const FORMAT_FEATURE_STORAGE_IMAGE_BIT: u32 = 0x2;
pub const FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: u32 = 0x4;
pub const FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: u32 = 0x8;
pub const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: u32 = 0x10;
pub const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: u32 = 0x20;
pub const FORMAT_FEATURE_VERTEX_BUFFER_BIT: u32 = 0x40;
pub const FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: u32 = 0x80;
pub const FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: u32 = 0x100;
pub const FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: u32 = 0x200;
pub const FORMAT_FEATURE_BLIT_SRC_BIT: u32 = 0x400;
pub const FORMAT_FEATURE_BLIT_DST_BIT: u32 = 0x800;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: u32 = 0x1000;

// Vulkan enum QueryControlFlagBits
pub type QueryControlFlagBits = u32;
pub const QUERY_CONTROL_PRECISE_BIT: u32 = 0x1;

// Vulkan enum QueryResultFlagBits
pub type QueryResultFlagBits = u32;
pub const QUERY_RESULT_64_BIT: u32 = 0x1;
pub const QUERY_RESULT_WAIT_BIT: u32 = 0x2;
pub const QUERY_RESULT_WITH_AVAILABILITY_BIT: u32 = 0x4;
pub const QUERY_RESULT_PARTIAL_BIT: u32 = 0x8;

// Vulkan enum CommandBufferUsageFlagBits
pub type CommandBufferUsageFlagBits = u32;
pub const COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: u32 = 0x1;
pub const COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: u32 = 0x2;
pub const COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: u32 = 0x4;

// Vulkan enum QueryPipelineStatisticFlagBits
pub type QueryPipelineStatisticFlagBits = u32;
pub const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: u32 = 0x1;
pub const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: u32 = 0x2;
pub const QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: u32 = 0x4;
pub const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: u32 = 0x8;
pub const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: u32 = 0x10;
pub const QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: u32 = 0x20;
pub const QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: u32 = 0x40;
pub const QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: u32 = 0x80;
pub const QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: u32 = 0x100;
pub const QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: u32 = 0x200;
pub const QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: u32 = 0x400;

// Vulkan enum ImageAspectFlagBits
pub type ImageAspectFlagBits = u32;
pub const IMAGE_ASPECT_COLOR_BIT: u32 = 0x1;
pub const IMAGE_ASPECT_DEPTH_BIT: u32 = 0x2;
pub const IMAGE_ASPECT_STENCIL_BIT: u32 = 0x4;
pub const IMAGE_ASPECT_METADATA_BIT: u32 = 0x8;

// Vulkan enum SparseImageFormatFlagBits
pub type SparseImageFormatFlagBits = u32;
pub const SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: u32 = 0x1;
pub const SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: u32 = 0x2;
pub const SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: u32 = 0x4;

// Vulkan enum SparseMemoryBindFlagBits
pub type SparseMemoryBindFlagBits = u32;
pub const SPARSE_MEMORY_BIND_METADATA_BIT: u32 = 0x1;

// Vulkan enum PipelineStageFlagBits
pub type PipelineStageFlagBits = u32;
pub const PIPELINE_STAGE_TOP_OF_PIPE_BIT: u32 = 0x1;
pub const PIPELINE_STAGE_DRAW_INDIRECT_BIT: u32 = 0x2;
pub const PIPELINE_STAGE_VERTEX_INPUT_BIT: u32 = 0x4;
pub const PIPELINE_STAGE_VERTEX_SHADER_BIT: u32 = 0x8;
pub const PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: u32 = 0x10;
pub const PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: u32 = 0x20;
pub const PIPELINE_STAGE_GEOMETRY_SHADER_BIT: u32 = 0x40;
pub const PIPELINE_STAGE_FRAGMENT_SHADER_BIT: u32 = 0x80;
pub const PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: u32 = 0x100;
pub const PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: u32 = 0x200;
pub const PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: u32 = 0x400;
pub const PIPELINE_STAGE_COMPUTE_SHADER_BIT: u32 = 0x800;
pub const PIPELINE_STAGE_TRANSFER_BIT: u32 = 0x1000;
pub const PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: u32 = 0x2000;
pub const PIPELINE_STAGE_HOST_BIT: u32 = 0x4000;
pub const PIPELINE_STAGE_ALL_GRAPHICS_BIT: u32 = 0x8000;
pub const PIPELINE_STAGE_ALL_COMMANDS_BIT: u32 = 0x10000;

// Vulkan enum CommandPoolCreateFlagBits
pub type CommandPoolCreateFlagBits = u32;
pub const COMMAND_POOL_CREATE_TRANSIENT_BIT: u32 = 0x1;
pub const COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: u32 = 0x2;

// Vulkan enum CommandPoolResetFlagBits
pub type CommandPoolResetFlagBits = u32;
pub const COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: u32 = 0x1;

// Vulkan enum CommandBufferResetFlagBits
pub type CommandBufferResetFlagBits = u32;
pub const COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: u32 = 0x1;

// Vulkan enum SampleCountFlagBits
pub type SampleCountFlagBits = u32;
pub const SAMPLE_COUNT_1_BIT: u32 = 0x1;
pub const SAMPLE_COUNT_2_BIT: u32 = 0x2;
pub const SAMPLE_COUNT_4_BIT: u32 = 0x4;
pub const SAMPLE_COUNT_8_BIT: u32 = 0x8;
pub const SAMPLE_COUNT_16_BIT: u32 = 0x10;
pub const SAMPLE_COUNT_32_BIT: u32 = 0x20;
pub const SAMPLE_COUNT_64_BIT: u32 = 0x40;

// Vulkan enum AttachmentDescriptionFlagBits
pub type AttachmentDescriptionFlagBits = u32;
pub const ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: u32 = 0x1;

// Vulkan enum StencilFaceFlagBits
pub type StencilFaceFlagBits = u32;
pub const STENCIL_FACE_FRONT_BIT: u32 = 0x1;
pub const STENCIL_FACE_BACK_BIT: u32 = 0x2;
pub const STENCIL_FACE_FRONT_AND_BACK: u32 = 0x00000003;

// Vulkan enum DescriptorPoolCreateFlagBits
pub type DescriptorPoolCreateFlagBits = u32;
pub const DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: u32 = 0x1;

// Vulkan enum DependencyFlagBits
pub type DependencyFlagBits = u32;
pub const DEPENDENCY_BY_REGION_BIT: u32 = 0x1;

// Vulkan enum SemaphoreType
pub type SemaphoreType = u32;
pub const SEMAPHORE_TYPE_BINARY: u32 = 0;
pub const SEMAPHORE_TYPE_TIMELINE: u32 = 1;

// Vulkan enum SemaphoreWaitFlagBits
pub type SemaphoreWaitFlagBits = u32;
pub const SEMAPHORE_WAIT_ANY_BIT: u32 = 0x1;

// Vulkan enum PresentModeKHR
pub type PresentModeKHR = u32;
pub const PRESENT_MODE_IMMEDIATE_KHR: u32 = 0;
pub const PRESENT_MODE_MAILBOX_KHR: u32 = 1;
pub const PRESENT_MODE_FIFO_KHR: u32 = 2;
pub const PRESENT_MODE_FIFO_RELAXED_KHR: u32 = 3;

// Vulkan enum ColorSpaceKHR
pub type ColorSpaceKHR = u32;
pub const COLOR_SPACE_SRGB_NONLINEAR_KHR: u32 = 0;

// Vulkan enum DisplayPlaneAlphaFlagBitsKHR
pub type DisplayPlaneAlphaFlagBitsKHR = u32;
pub const DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: u32 = 0x1;
pub const DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: u32 = 0x2;
pub const DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: u32 = 0x4;
pub const DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR: u32 = 0x8;

// Vulkan enum CompositeAlphaFlagBitsKHR
pub type CompositeAlphaFlagBitsKHR = u32;
pub const COMPOSITE_ALPHA_OPAQUE_BIT_KHR: u32 = 0x1;
pub const COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: u32 = 0x2;
pub const COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: u32 = 0x4;
pub const COMPOSITE_ALPHA_INHERIT_BIT_KHR: u32 = 0x8;

// Vulkan enum SurfaceTransformFlagBitsKHR
pub type SurfaceTransformFlagBitsKHR = u32;
pub const SURFACE_TRANSFORM_IDENTITY_BIT_KHR: u32 = 0x1;
pub const SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: u32 = 0x2;
pub const SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: u32 = 0x4;
pub const SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: u32 = 0x8;
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: u32 = 0x10;
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: u32 = 0x20;
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: u32 = 0x40;
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: u32 = 0x80;
pub const SURFACE_TRANSFORM_INHERIT_BIT_KHR: u32 = 0x100;

// Vulkan enum SwapchainImageUsageFlagBitsANDROID
pub type SwapchainImageUsageFlagBitsANDROID = u32;
pub const SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_ANDROID: u32 = 0x1;

// Vulkan enum TimeDomainEXT
pub type TimeDomainEXT = u32;
pub const TIME_DOMAIN_DEVICE_EXT: u32 = 0;
pub const TIME_DOMAIN_CLOCK_MONOTONIC_EXT: u32 = 1;
pub const TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT: u32 = 2;
pub const TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT: u32 = 3;

// Vulkan enum DebugReportFlagBitsEXT
pub type DebugReportFlagBitsEXT = u32;
pub const DEBUG_REPORT_INFORMATION_BIT_EXT: u32 = 0x1;
pub const DEBUG_REPORT_WARNING_BIT_EXT: u32 = 0x2;
pub const DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: u32 = 0x4;
pub const DEBUG_REPORT_ERROR_BIT_EXT: u32 = 0x8;
pub const DEBUG_REPORT_DEBUG_BIT_EXT: u32 = 0x10;

// Vulkan enum DebugReportObjectTypeEXT
pub type DebugReportObjectTypeEXT = u32;
pub const DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: u32 = 0;
pub const DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: u32 = 1;
pub const DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: u32 = 2;
pub const DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: u32 = 3;
pub const DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: u32 = 4;
pub const DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: u32 = 5;
pub const DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: u32 = 6;
pub const DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: u32 = 7;
pub const DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: u32 = 8;
pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: u32 = 9;
pub const DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: u32 = 10;
pub const DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: u32 = 11;
pub const DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: u32 = 12;
pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: u32 = 13;
pub const DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: u32 = 14;
pub const DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: u32 = 15;
pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: u32 = 16;
pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: u32 = 17;
pub const DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: u32 = 18;
pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: u32 = 19;
pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT: u32 = 20;
pub const DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: u32 = 21;
pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: u32 = 22;
pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: u32 = 23;
pub const DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: u32 = 24;
pub const DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: u32 = 25;
pub const DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: u32 = 26;
pub const DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: u32 = 27;
pub const DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: u32 = 28;
pub const DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: u32 = 29;
pub const DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: u32 = 30;
pub const DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT: u32 = 33;

// Vulkan enum RasterizationOrderAMD
pub type RasterizationOrderAMD = u32;
pub const RASTERIZATION_ORDER_STRICT_AMD: u32 = 0;
pub const RASTERIZATION_ORDER_RELAXED_AMD: u32 = 1;

// Vulkan enum ExternalMemoryHandleTypeFlagBitsNV
pub type ExternalMemoryHandleTypeFlagBitsNV = u32;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV: u32 = 0x1;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV: u32 = 0x2;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV: u32 = 0x4;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV: u32 = 0x8;

// Vulkan enum ExternalMemoryFeatureFlagBitsNV
pub type ExternalMemoryFeatureFlagBitsNV = u32;
pub const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV: u32 = 0x1;
pub const EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV: u32 = 0x2;
pub const EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV: u32 = 0x4;

// Vulkan enum ValidationCheckEXT
pub type ValidationCheckEXT = u32;
pub const VALIDATION_CHECK_ALL_EXT: u32 = 0;
pub const VALIDATION_CHECK_SHADERS_EXT: u32 = 1;

// Vulkan enum ValidationFeatureEnableEXT
pub type ValidationFeatureEnableEXT = u32;
pub const VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT: u32 = 0;
pub const VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: u32 = 1;
pub const VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT: u32 = 2;
pub const VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT: u32 = 3;

// Vulkan enum ValidationFeatureDisableEXT
pub type ValidationFeatureDisableEXT = u32;
pub const VALIDATION_FEATURE_DISABLE_ALL_EXT: u32 = 0;
pub const VALIDATION_FEATURE_DISABLE_SHADERS_EXT: u32 = 1;
pub const VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT: u32 = 2;
pub const VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT: u32 = 3;
pub const VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT: u32 = 4;
pub const VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT: u32 = 5;
pub const VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT: u32 = 6;

// Vulkan enum SubgroupFeatureFlagBits
pub type SubgroupFeatureFlagBits = u32;
pub const SUBGROUP_FEATURE_BASIC_BIT: u32 = 0x1;
pub const SUBGROUP_FEATURE_VOTE_BIT: u32 = 0x2;
pub const SUBGROUP_FEATURE_ARITHMETIC_BIT: u32 = 0x4;
pub const SUBGROUP_FEATURE_BALLOT_BIT: u32 = 0x8;
pub const SUBGROUP_FEATURE_SHUFFLE_BIT: u32 = 0x10;
pub const SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT: u32 = 0x20;
pub const SUBGROUP_FEATURE_CLUSTERED_BIT: u32 = 0x40;
pub const SUBGROUP_FEATURE_QUAD_BIT: u32 = 0x80;

// Vulkan enum IndirectCommandsLayoutUsageFlagBitsNV
pub type IndirectCommandsLayoutUsageFlagBitsNV = u32;
pub const INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV: u32 = 0x1;
pub const INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV: u32 = 0x2;
pub const INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV: u32 = 0x4;

// Vulkan enum IndirectStateFlagBitsNV
pub type IndirectStateFlagBitsNV = u32;
pub const INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV: u32 = 0x1;

// Vulkan enum IndirectCommandsTokenTypeNV
pub type IndirectCommandsTokenTypeNV = u32;
pub const INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV: u32 = 0;
pub const INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV: u32 = 1;
pub const INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV: u32 = 2;
pub const INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV: u32 = 3;
pub const INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV: u32 = 4;
pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV: u32 = 5;
pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV: u32 = 6;
pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV: u32 = 7;

// Vulkan enum PrivateDataSlotCreateFlagBitsEXT
pub type PrivateDataSlotCreateFlagBitsEXT = u32;

// Vulkan enum DescriptorSetLayoutCreateFlagBits
pub type DescriptorSetLayoutCreateFlagBits = u32;

// Vulkan enum ExternalMemoryHandleTypeFlagBits
pub type ExternalMemoryHandleTypeFlagBits = u32;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT: u32 = 0x1;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT: u32 = 0x2;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: u32 = 0x4;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT: u32 = 0x8;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT: u32 = 0x10;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT: u32 = 0x20;
pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT: u32 = 0x40;

// Vulkan enum ExternalMemoryFeatureFlagBits
pub type ExternalMemoryFeatureFlagBits = u32;
pub const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT: u32 = 0x1;
pub const EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT: u32 = 0x2;
pub const EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT: u32 = 0x4;

// Vulkan enum ExternalSemaphoreHandleTypeFlagBits
pub type ExternalSemaphoreHandleTypeFlagBits = u32;
pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT: u32 = 0x1;
pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT: u32 = 0x2;
pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: u32 = 0x4;
pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT: u32 = 0x8;
pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT: u32 = 0x10;

// Vulkan enum ExternalSemaphoreFeatureFlagBits
pub type ExternalSemaphoreFeatureFlagBits = u32;
pub const EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT: u32 = 0x1;
pub const EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT: u32 = 0x2;

// Vulkan enum SemaphoreImportFlagBits
pub type SemaphoreImportFlagBits = u32;
pub const SEMAPHORE_IMPORT_TEMPORARY_BIT: u32 = 0x1;

// Vulkan enum ExternalFenceHandleTypeFlagBits
pub type ExternalFenceHandleTypeFlagBits = u32;
pub const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT: u32 = 0x1;
pub const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT: u32 = 0x2;
pub const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: u32 = 0x4;
pub const EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT: u32 = 0x8;

// Vulkan enum ExternalFenceFeatureFlagBits
pub type ExternalFenceFeatureFlagBits = u32;
pub const EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT: u32 = 0x1;
pub const EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT: u32 = 0x2;

// Vulkan enum FenceImportFlagBits
pub type FenceImportFlagBits = u32;
pub const FENCE_IMPORT_TEMPORARY_BIT: u32 = 0x1;

// Vulkan enum SurfaceCounterFlagBitsEXT
pub type SurfaceCounterFlagBitsEXT = u32;
pub const SURFACE_COUNTER_VBLANK_EXT: u32 = 0x1;

// Vulkan enum DisplayPowerStateEXT
pub type DisplayPowerStateEXT = u32;
pub const DISPLAY_POWER_STATE_OFF_EXT: u32 = 0;
pub const DISPLAY_POWER_STATE_SUSPEND_EXT: u32 = 1;
pub const DISPLAY_POWER_STATE_ON_EXT: u32 = 2;

// Vulkan enum DeviceEventTypeEXT
pub type DeviceEventTypeEXT = u32;
pub const DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: u32 = 0;

// Vulkan enum DisplayEventTypeEXT
pub type DisplayEventTypeEXT = u32;
pub const DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: u32 = 0;

// Vulkan enum PeerMemoryFeatureFlagBits
pub type PeerMemoryFeatureFlagBits = u32;
pub const PEER_MEMORY_FEATURE_COPY_SRC_BIT: u32 = 0x1;
pub const PEER_MEMORY_FEATURE_COPY_DST_BIT: u32 = 0x2;
pub const PEER_MEMORY_FEATURE_GENERIC_SRC_BIT: u32 = 0x4;
pub const PEER_MEMORY_FEATURE_GENERIC_DST_BIT: u32 = 0x8;

// Vulkan enum MemoryAllocateFlagBits
pub type MemoryAllocateFlagBits = u32;
pub const MEMORY_ALLOCATE_DEVICE_MASK_BIT: u32 = 0x1;

// Vulkan enum DeviceGroupPresentModeFlagBitsKHR
pub type DeviceGroupPresentModeFlagBitsKHR = u32;
pub const DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR: u32 = 0x1;
pub const DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR: u32 = 0x2;
pub const DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR: u32 = 0x4;
pub const DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR: u32 = 0x8;

// Vulkan enum SwapchainCreateFlagBitsKHR
pub type SwapchainCreateFlagBitsKHR = u32;

// Vulkan enum ViewportCoordinateSwizzleNV
pub type ViewportCoordinateSwizzleNV = u32;
pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: u32 = 0;
pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: u32 = 1;
pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: u32 = 2;
pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: u32 = 3;
pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: u32 = 4;
pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: u32 = 5;
pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: u32 = 6;
pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: u32 = 7;

// Vulkan enum DiscardRectangleModeEXT
pub type DiscardRectangleModeEXT = u32;
pub const DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: u32 = 0;
pub const DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: u32 = 1;

// Vulkan enum SubpassDescriptionFlagBits
pub type SubpassDescriptionFlagBits = u32;

// Vulkan enum PointClippingBehavior
pub type PointClippingBehavior = u32;
pub const POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: u32 = 0;
pub const POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY: u32 = 1;

// Vulkan enum SamplerReductionMode
pub type SamplerReductionMode = u32;
pub const SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE: u32 = 0;
pub const SAMPLER_REDUCTION_MODE_MIN: u32 = 1;
pub const SAMPLER_REDUCTION_MODE_MAX: u32 = 2;

// Vulkan enum TessellationDomainOrigin
pub type TessellationDomainOrigin = u32;
pub const TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT: u32 = 0;
pub const TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT: u32 = 1;

// Vulkan enum SamplerYcbcrModelConversion
pub type SamplerYcbcrModelConversion = u32;
pub const SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: u32 = 0;
pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY: u32 = 1;
pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709: u32 = 2;
pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601: u32 = 3;
pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020: u32 = 4;

// Vulkan enum SamplerYcbcrRange
pub type SamplerYcbcrRange = u32;
pub const SAMPLER_YCBCR_RANGE_ITU_FULL: u32 = 0;
pub const SAMPLER_YCBCR_RANGE_ITU_NARROW: u32 = 1;

// Vulkan enum ChromaLocation
pub type ChromaLocation = u32;
pub const CHROMA_LOCATION_COSITED_EVEN: u32 = 0;
pub const CHROMA_LOCATION_MIDPOINT: u32 = 1;

// Vulkan enum BlendOverlapEXT
pub type BlendOverlapEXT = u32;
pub const BLEND_OVERLAP_UNCORRELATED_EXT: u32 = 0;
pub const BLEND_OVERLAP_DISJOINT_EXT: u32 = 1;
pub const BLEND_OVERLAP_CONJOINT_EXT: u32 = 2;

// Vulkan enum CoverageModulationModeNV
pub type CoverageModulationModeNV = u32;
pub const COVERAGE_MODULATION_MODE_NONE_NV: u32 = 0;
pub const COVERAGE_MODULATION_MODE_RGB_NV: u32 = 1;
pub const COVERAGE_MODULATION_MODE_ALPHA_NV: u32 = 2;
pub const COVERAGE_MODULATION_MODE_RGBA_NV: u32 = 3;

// Vulkan enum CoverageReductionModeNV
pub type CoverageReductionModeNV = u32;
pub const COVERAGE_REDUCTION_MODE_MERGE_NV: u32 = 0;
pub const COVERAGE_REDUCTION_MODE_TRUNCATE_NV: u32 = 1;

// Vulkan enum ValidationCacheHeaderVersionEXT
pub type ValidationCacheHeaderVersionEXT = u32;
pub const VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: u32 = 1;

// Vulkan enum ShaderInfoTypeAMD
pub type ShaderInfoTypeAMD = u32;
pub const SHADER_INFO_TYPE_STATISTICS_AMD: u32 = 0;
pub const SHADER_INFO_TYPE_BINARY_AMD: u32 = 1;
pub const SHADER_INFO_TYPE_DISASSEMBLY_AMD: u32 = 2;

// Vulkan enum QueueGlobalPriorityEXT
pub type QueueGlobalPriorityEXT = u32;
pub const QUEUE_GLOBAL_PRIORITY_LOW_EXT: u32 = 128;
pub const QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT: u32 = 256;
pub const QUEUE_GLOBAL_PRIORITY_HIGH_EXT: u32 = 512;
pub const QUEUE_GLOBAL_PRIORITY_REALTIME_EXT: u32 = 1024;

// Vulkan enum DebugUtilsMessageSeverityFlagBitsEXT
pub type DebugUtilsMessageSeverityFlagBitsEXT = u32;
pub const DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT: u32 = 0x1;
pub const DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT: u32 = 0x10;
pub const DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT: u32 = 0x100;
pub const DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT: u32 = 0x1000;

// Vulkan enum DebugUtilsMessageTypeFlagBitsEXT
pub type DebugUtilsMessageTypeFlagBitsEXT = u32;
pub const DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT: u32 = 0x1;
pub const DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT: u32 = 0x2;
pub const DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT: u32 = 0x4;

// Vulkan enum ConservativeRasterizationModeEXT
pub type ConservativeRasterizationModeEXT = u32;
pub const CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT: u32 = 0;
pub const CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT: u32 = 1;
pub const CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT: u32 = 2;

// Vulkan enum DescriptorBindingFlagBits
pub type DescriptorBindingFlagBits = u32;
pub const DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT: u32 = 0x1;
pub const DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT: u32 = 0x2;
pub const DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT: u32 = 0x4;
pub const DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT: u32 = 0x8;

// Vulkan enum VendorId
pub type VendorId = u32;
pub const VENDOR_ID_VIV: u32 = 0x10001;
pub const VENDOR_ID_VSI: u32 = 0x10002;
pub const VENDOR_ID_KAZAN: u32 = 0x10003;
pub const VENDOR_ID_CODEPLAY: u32 = 0x10004;
pub const VENDOR_ID_MESA: u32 = 0x10005;

// Vulkan enum DriverId
pub type DriverId = u32;
pub const DRIVER_ID_AMD_PROPRIETARY: u32 = 1;
pub const DRIVER_ID_AMD_OPEN_SOURCE: u32 = 2;
pub const DRIVER_ID_MESA_RADV: u32 = 3;
pub const DRIVER_ID_NVIDIA_PROPRIETARY: u32 = 4;
pub const DRIVER_ID_INTEL_PROPRIETARY_WINDOWS: u32 = 5;
pub const DRIVER_ID_INTEL_OPEN_SOURCE_MESA: u32 = 6;
pub const DRIVER_ID_IMAGINATION_PROPRIETARY: u32 = 7;
pub const DRIVER_ID_QUALCOMM_PROPRIETARY: u32 = 8;
pub const DRIVER_ID_ARM_PROPRIETARY: u32 = 9;
pub const DRIVER_ID_GOOGLE_SWIFTSHADER: u32 = 10;
pub const DRIVER_ID_GGP_PROPRIETARY: u32 = 11;
pub const DRIVER_ID_BROADCOM_PROPRIETARY: u32 = 12;
pub const DRIVER_ID_MESA_LLVMPIPE: u32 = 13;
pub const DRIVER_ID_MOLTENVK: u32 = 14;

// Vulkan enum ConditionalRenderingFlagBitsEXT
pub type ConditionalRenderingFlagBitsEXT = u32;
pub const CONDITIONAL_RENDERING_INVERTED_BIT_EXT: u32 = 0x1;

// Vulkan enum ResolveModeFlagBits
pub type ResolveModeFlagBits = u32;
pub const RESOLVE_MODE_NONE: u32 = 0;
pub const RESOLVE_MODE_SAMPLE_ZERO_BIT: u32 = 0x1;
pub const RESOLVE_MODE_AVERAGE_BIT: u32 = 0x2;
pub const RESOLVE_MODE_MIN_BIT: u32 = 0x4;
pub const RESOLVE_MODE_MAX_BIT: u32 = 0x8;

// Vulkan enum ShadingRatePaletteEntryNV
pub type ShadingRatePaletteEntryNV = u32;
pub const SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV: u32 = 0;
pub const SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV: u32 = 1;
pub const SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV: u32 = 2;
pub const SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV: u32 = 3;
pub const SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV: u32 = 4;
pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV: u32 = 5;
pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV: u32 = 6;
pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV: u32 = 7;
pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV: u32 = 8;
pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV: u32 = 9;
pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV: u32 = 10;
pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV: u32 = 11;

// Vulkan enum CoarseSampleOrderTypeNV
pub type CoarseSampleOrderTypeNV = u32;
pub const COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV: u32 = 0;
pub const COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV: u32 = 1;
pub const COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV: u32 = 2;
pub const COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV: u32 = 3;

// Vulkan enum GeometryInstanceFlagBitsKHR
pub type GeometryInstanceFlagBitsKHR = u32;
pub const GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR: u32 = 0x1;
pub const GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR: u32 = 0x2;
pub const GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR: u32 = 0x4;
pub const GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR: u32 = 0x8;

// Vulkan enum GeometryFlagBitsKHR
pub type GeometryFlagBitsKHR = u32;
pub const GEOMETRY_OPAQUE_BIT_KHR: u32 = 0x1;
pub const GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR: u32 = 0x2;

// Vulkan enum BuildAccelerationStructureFlagBitsKHR
pub type BuildAccelerationStructureFlagBitsKHR = u32;
pub const BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR: u32 = 0x1;
pub const BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR: u32 = 0x2;
pub const BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR: u32 = 0x4;
pub const BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR: u32 = 0x8;
pub const BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR: u32 = 0x10;

// Vulkan enum CopyAccelerationStructureModeKHR
pub type CopyAccelerationStructureModeKHR = u32;
pub const COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR: u32 = 0;
pub const COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR: u32 = 1;
pub const COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR: u32 = 2;
pub const COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR: u32 = 3;

// Vulkan enum AccelerationStructureTypeKHR
pub type AccelerationStructureTypeKHR = u32;
pub const ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR: u32 = 0;
pub const ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR: u32 = 1;

// Vulkan enum GeometryTypeKHR
pub type GeometryTypeKHR = u32;
pub const GEOMETRY_TYPE_TRIANGLES_KHR: u32 = 0;
pub const GEOMETRY_TYPE_AABBS_KHR: u32 = 1;

// Vulkan enum AccelerationStructureMemoryRequirementsTypeKHR
pub type AccelerationStructureMemoryRequirementsTypeKHR = u32;
pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_KHR: u32 = 0;
pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_KHR: u32 = 1;
pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_KHR: u32 = 2;

// Vulkan enum AccelerationStructureBuildTypeKHR
pub type AccelerationStructureBuildTypeKHR = u32;
pub const ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR: u32 = 0;
pub const ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR: u32 = 1;
pub const ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR: u32 = 2;

// Vulkan enum RayTracingShaderGroupTypeKHR
pub type RayTracingShaderGroupTypeKHR = u32;
pub const RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR: u32 = 0;
pub const RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR: u32 = 1;
pub const RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR: u32 = 2;

// Vulkan enum MemoryOverallocationBehaviorAMD
pub type MemoryOverallocationBehaviorAMD = u32;
pub const MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD: u32 = 0;
pub const MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD: u32 = 1;
pub const MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD: u32 = 2;

// Vulkan enum FramebufferCreateFlagBits
pub type FramebufferCreateFlagBits = u32;

// Vulkan enum ScopeNV
pub type ScopeNV = u32;
pub const SCOPE_DEVICE_NV: u32 = 1;
pub const SCOPE_WORKGROUP_NV: u32 = 2;
pub const SCOPE_SUBGROUP_NV: u32 = 3;
pub const SCOPE_QUEUE_FAMILY_NV: u32 = 5;

// Vulkan enum ComponentTypeNV
pub type ComponentTypeNV = u32;
pub const COMPONENT_TYPE_FLOAT16_NV: u32 = 0;
pub const COMPONENT_TYPE_FLOAT32_NV: u32 = 1;
pub const COMPONENT_TYPE_FLOAT64_NV: u32 = 2;
pub const COMPONENT_TYPE_SINT8_NV: u32 = 3;
pub const COMPONENT_TYPE_SINT16_NV: u32 = 4;
pub const COMPONENT_TYPE_SINT32_NV: u32 = 5;
pub const COMPONENT_TYPE_SINT64_NV: u32 = 6;
pub const COMPONENT_TYPE_UINT8_NV: u32 = 7;
pub const COMPONENT_TYPE_UINT16_NV: u32 = 8;
pub const COMPONENT_TYPE_UINT32_NV: u32 = 9;
pub const COMPONENT_TYPE_UINT64_NV: u32 = 10;

// Vulkan enum DeviceDiagnosticsConfigFlagBitsNV
pub type DeviceDiagnosticsConfigFlagBitsNV = u32;
pub const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV: u32 = 0x1;
pub const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV: u32 = 0x2;
pub const DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV: u32 = 0x4;

// Vulkan enum PipelineCreationFeedbackFlagBitsEXT
pub type PipelineCreationFeedbackFlagBitsEXT = u32;
pub const PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT: u32 = 0x1;
pub const PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT: u32 = 0x2;
pub const PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT: u32 = 0x4;

// Vulkan enum FullScreenExclusiveEXT
pub type FullScreenExclusiveEXT = u32;
pub const FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: u32 = 0;
pub const FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: u32 = 1;
pub const FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: u32 = 2;
pub const FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: u32 = 3;

// Vulkan enum PerformanceCounterScopeKHR
pub type PerformanceCounterScopeKHR = u32;
pub const PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR: u32 = 0;
pub const PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR: u32 = 1;
pub const PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR: u32 = 2;

// Vulkan enum PerformanceCounterUnitKHR
pub type PerformanceCounterUnitKHR = u32;
pub const PERFORMANCE_COUNTER_UNIT_GENERIC_KHR: u32 = 0;
pub const PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR: u32 = 1;
pub const PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR: u32 = 2;
pub const PERFORMANCE_COUNTER_UNIT_BYTES_KHR: u32 = 3;
pub const PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR: u32 = 4;
pub const PERFORMANCE_COUNTER_UNIT_KELVIN_KHR: u32 = 5;
pub const PERFORMANCE_COUNTER_UNIT_WATTS_KHR: u32 = 6;
pub const PERFORMANCE_COUNTER_UNIT_VOLTS_KHR: u32 = 7;
pub const PERFORMANCE_COUNTER_UNIT_AMPS_KHR: u32 = 8;
pub const PERFORMANCE_COUNTER_UNIT_HERTZ_KHR: u32 = 9;
pub const PERFORMANCE_COUNTER_UNIT_CYCLES_KHR: u32 = 10;

// Vulkan enum PerformanceCounterStorageKHR
pub type PerformanceCounterStorageKHR = u32;
pub const PERFORMANCE_COUNTER_STORAGE_INT32_KHR: u32 = 0;
pub const PERFORMANCE_COUNTER_STORAGE_INT64_KHR: u32 = 1;
pub const PERFORMANCE_COUNTER_STORAGE_UINT32_KHR: u32 = 2;
pub const PERFORMANCE_COUNTER_STORAGE_UINT64_KHR: u32 = 3;
pub const PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR: u32 = 4;
pub const PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR: u32 = 5;

// Vulkan enum PerformanceCounterDescriptionFlagBitsKHR
pub type PerformanceCounterDescriptionFlagBitsKHR = u32;
pub const PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR: u32 = 0x1;
pub const PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR: u32 = 0x2;

// Vulkan enum AcquireProfilingLockFlagBitsKHR
pub type AcquireProfilingLockFlagBitsKHR = u32;

// Vulkan enum ShaderCorePropertiesFlagBitsAMD
pub type ShaderCorePropertiesFlagBitsAMD = u32;

// Vulkan enum PerformanceConfigurationTypeINTEL
pub type PerformanceConfigurationTypeINTEL = u32;
pub const PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: u32 = 0;

// Vulkan enum QueryPoolSamplingModeINTEL
pub type QueryPoolSamplingModeINTEL = u32;
pub const QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL: u32 = 0;

// Vulkan enum PerformanceOverrideTypeINTEL
pub type PerformanceOverrideTypeINTEL = u32;
pub const PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL: u32 = 0;
pub const PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL: u32 = 1;

// Vulkan enum PerformanceParameterTypeINTEL
pub type PerformanceParameterTypeINTEL = u32;
pub const PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL: u32 = 0;
pub const PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL: u32 = 1;

// Vulkan enum PerformanceValueTypeINTEL
pub type PerformanceValueTypeINTEL = u32;
pub const PERFORMANCE_VALUE_TYPE_UINT32_INTEL: u32 = 0;
pub const PERFORMANCE_VALUE_TYPE_UINT64_INTEL: u32 = 1;
pub const PERFORMANCE_VALUE_TYPE_FLOAT_INTEL: u32 = 2;
pub const PERFORMANCE_VALUE_TYPE_BOOL_INTEL: u32 = 3;
pub const PERFORMANCE_VALUE_TYPE_STRING_INTEL: u32 = 4;

// Vulkan enum ShaderFloatControlsIndependence
pub type ShaderFloatControlsIndependence = u32;
pub const SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY: u32 = 0;
pub const SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL: u32 = 1;
pub const SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE: u32 = 2;

// Vulkan enum PipelineExecutableStatisticFormatKHR
pub type PipelineExecutableStatisticFormatKHR = u32;
pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR: u32 = 0;
pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR: u32 = 1;
pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR: u32 = 2;
pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR: u32 = 3;

// Vulkan enum LineRasterizationModeEXT
pub type LineRasterizationModeEXT = u32;
pub const LINE_RASTERIZATION_MODE_DEFAULT_EXT: u32 = 0;
pub const LINE_RASTERIZATION_MODE_RECTANGULAR_EXT: u32 = 1;
pub const LINE_RASTERIZATION_MODE_BRESENHAM_EXT: u32 = 2;
pub const LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT: u32 = 3;

// Vulkan enum ShaderModuleCreateFlagBits
pub type ShaderModuleCreateFlagBits = u32;

// Vulkan enum PipelineCompilerControlFlagBitsAMD
pub type PipelineCompilerControlFlagBitsAMD = u32;

// Vulkan enum ToolPurposeFlagBitsEXT
pub type ToolPurposeFlagBitsEXT = u32;
pub const TOOL_PURPOSE_VALIDATION_BIT_EXT: u32 = 0x1;
pub const TOOL_PURPOSE_PROFILING_BIT_EXT: u32 = 0x2;
pub const TOOL_PURPOSE_TRACING_BIT_EXT: u32 = 0x4;
pub const TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT: u32 = 0x8;
pub const TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT: u32 = 0x10;

// Commands
extern "C" {
    #[link_name = "vkCreateInstance"]
    #[must_use]
    pub fn create_instance(
        p_create_info: *const InstanceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_instance: *mut Instance,
    ) -> Result;

    #[link_name = "vkDestroyInstance"]
    pub fn destroy_instance(instance: Instance, p_allocator: *const AllocationCallbacks);

    #[link_name = "vkEnumeratePhysicalDevices"]
    #[must_use]
    pub fn enumerate_physical_devices(
        instance: Instance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut PhysicalDevice,
    ) -> Result;

    #[link_name = "vkGetDeviceProcAddr"]
    #[must_use]
    pub fn get_device_proc_addr(device: Device, p_name: *const c_char) -> VoidFunction;

    #[link_name = "vkGetInstanceProcAddr"]
    #[must_use]
    pub fn get_instance_proc_addr(instance: Instance, p_name: *const c_char) -> VoidFunction;

    #[link_name = "vkGetPhysicalDeviceProperties"]
    pub fn get_physical_device_properties(
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties,
    );

    #[link_name = "vkGetPhysicalDeviceQueueFamilyProperties"]
    pub fn get_physical_device_queue_family_properties(
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties,
    );

    #[link_name = "vkGetPhysicalDeviceMemoryProperties"]
    pub fn get_physical_device_memory_properties(
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties,
    );

    #[link_name = "vkGetPhysicalDeviceFeatures"]
    pub fn get_physical_device_features(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures,
    );

    #[link_name = "vkGetPhysicalDeviceFormatProperties"]
    pub fn get_physical_device_format_properties(
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties,
    );

    #[link_name = "vkGetPhysicalDeviceImageFormatProperties"]
    #[must_use]
    pub fn get_physical_device_image_format_properties(
        physical_device: PhysicalDevice,
        format: Format,
        vk_type: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        p_image_format_properties: *mut ImageFormatProperties,
    ) -> Result;

    #[link_name = "vkCreateDevice"]
    #[must_use]
    pub fn create_device(
        physical_device: PhysicalDevice,
        p_create_info: *const DeviceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_device: *mut Device,
    ) -> Result;

    #[link_name = "vkDestroyDevice"]
    pub fn destroy_device(device: Device, p_allocator: *const AllocationCallbacks);

    #[link_name = "vkEnumerateInstanceVersion"]
    #[must_use]
    pub fn enumerate_instance_version(p_api_version: *mut u32) -> Result;

    #[link_name = "vkEnumerateInstanceLayerProperties"]
    #[must_use]
    pub fn enumerate_instance_layer_properties(
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> Result;

    #[link_name = "vkEnumerateInstanceExtensionProperties"]
    #[must_use]
    pub fn enumerate_instance_extension_properties(
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
    ) -> Result;

    #[link_name = "vkEnumerateDeviceLayerProperties"]
    #[must_use]
    pub fn enumerate_device_layer_properties(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> Result;

    #[link_name = "vkEnumerateDeviceExtensionProperties"]
    #[must_use]
    pub fn enumerate_device_extension_properties(
        physical_device: PhysicalDevice,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
    ) -> Result;

    #[link_name = "vkGetDeviceQueue"]
    pub fn get_device_queue(
        device: Device,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut Queue,
    );

    #[link_name = "vkQueueSubmit"]
    #[must_use]
    pub fn queue_submit(
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo,
        fence: Fence,
    ) -> Result;

    #[link_name = "vkQueueWaitIdle"]
    #[must_use]
    pub fn queue_wait_idle(queue: Queue) -> Result;

    #[link_name = "vkDeviceWaitIdle"]
    #[must_use]
    pub fn device_wait_idle(device: Device) -> Result;

    #[link_name = "vkAllocateMemory"]
    #[must_use]
    pub fn allocate_memory(
        device: Device,
        p_allocate_info: *const MemoryAllocateInfo,
        p_allocator: *const AllocationCallbacks,
        p_memory: *mut DeviceMemory,
    ) -> Result;

    #[link_name = "vkFreeMemory"]
    pub fn free_memory(
        device: Device,
        memory: DeviceMemory,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkMapMemory"]
    #[must_use]
    pub fn map_memory(
        device: Device,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> Result;

    #[link_name = "vkUnmapMemory"]
    pub fn unmap_memory(device: Device, memory: DeviceMemory);

    #[link_name = "vkFlushMappedMemoryRanges"]
    #[must_use]
    pub fn flush_mapped_memory_ranges(
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result;

    #[link_name = "vkInvalidateMappedMemoryRanges"]
    #[must_use]
    pub fn invalidate_mapped_memory_ranges(
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result;

    #[link_name = "vkGetDeviceMemoryCommitment"]
    pub fn get_device_memory_commitment(
        device: Device,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *mut DeviceSize,
    );

    #[link_name = "vkGetBufferMemoryRequirements"]
    pub fn get_buffer_memory_requirements(
        device: Device,
        buffer: Buffer,
        p_memory_requirements: *mut MemoryRequirements,
    );

    #[link_name = "vkBindBufferMemory"]
    #[must_use]
    pub fn bind_buffer_memory(
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result;

    #[link_name = "vkGetImageMemoryRequirements"]
    pub fn get_image_memory_requirements(
        device: Device,
        image: Image,
        p_memory_requirements: *mut MemoryRequirements,
    );

    #[link_name = "vkBindImageMemory"]
    #[must_use]
    pub fn bind_image_memory(
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result;

    #[link_name = "vkGetImageSparseMemoryRequirements"]
    pub fn get_image_sparse_memory_requirements(
        device: Device,
        image: Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
    );

    #[link_name = "vkGetPhysicalDeviceSparseImageFormatProperties"]
    pub fn get_physical_device_sparse_image_format_properties(
        physical_device: PhysicalDevice,
        format: Format,
        vk_type: ImageType,
        samples: SampleCountFlagBits,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties,
    );

    #[link_name = "vkQueueBindSparse"]
    #[must_use]
    pub fn queue_bind_sparse(
        queue: Queue,
        bind_info_count: u32,
        p_bind_info: *const BindSparseInfo,
        fence: Fence,
    ) -> Result;

    #[link_name = "vkCreateFence"]
    #[must_use]
    pub fn create_fence(
        device: Device,
        p_create_info: *const FenceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence,
    ) -> Result;

    #[link_name = "vkDestroyFence"]
    pub fn destroy_fence(device: Device, fence: Fence, p_allocator: *const AllocationCallbacks);

    #[link_name = "vkResetFences"]
    #[must_use]
    pub fn reset_fences(device: Device, fence_count: u32, p_fences: *const Fence) -> Result;

    #[link_name = "vkGetFenceStatus"]
    #[must_use]
    pub fn get_fence_status(device: Device, fence: Fence) -> Result;

    #[link_name = "vkWaitForFences"]
    #[must_use]
    pub fn wait_for_fences(
        device: Device,
        fence_count: u32,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> Result;

    #[link_name = "vkCreateSemaphore"]
    #[must_use]
    pub fn create_semaphore(
        device: Device,
        p_create_info: *const SemaphoreCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_semaphore: *mut Semaphore,
    ) -> Result;

    #[link_name = "vkDestroySemaphore"]
    pub fn destroy_semaphore(
        device: Device,
        semaphore: Semaphore,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkCreateEvent"]
    #[must_use]
    pub fn create_event(
        device: Device,
        p_create_info: *const EventCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_event: *mut Event,
    ) -> Result;

    #[link_name = "vkDestroyEvent"]
    pub fn destroy_event(device: Device, event: Event, p_allocator: *const AllocationCallbacks);

    #[link_name = "vkGetEventStatus"]
    #[must_use]
    pub fn get_event_status(device: Device, event: Event) -> Result;

    #[link_name = "vkSetEvent"]
    #[must_use]
    pub fn set_event(device: Device, event: Event) -> Result;

    #[link_name = "vkResetEvent"]
    #[must_use]
    pub fn reset_event(device: Device, event: Event) -> Result;

    #[link_name = "vkCreateQueryPool"]
    #[must_use]
    pub fn create_query_pool(
        device: Device,
        p_create_info: *const QueryPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_query_pool: *mut QueryPool,
    ) -> Result;

    #[link_name = "vkDestroyQueryPool"]
    pub fn destroy_query_pool(
        device: Device,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkGetQueryPoolResults"]
    #[must_use]
    pub fn get_query_pool_results(
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> Result;

    #[link_name = "vkResetQueryPool"]
    pub fn reset_query_pool(
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    );

    #[link_name = "vkCreateBuffer"]
    #[must_use]
    pub fn create_buffer(
        device: Device,
        p_create_info: *const BufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_buffer: *mut Buffer,
    ) -> Result;

    #[link_name = "vkDestroyBuffer"]
    pub fn destroy_buffer(device: Device, buffer: Buffer, p_allocator: *const AllocationCallbacks);

    #[link_name = "vkCreateBufferView"]
    #[must_use]
    pub fn create_buffer_view(
        device: Device,
        p_create_info: *const BufferViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut BufferView,
    ) -> Result;

    #[link_name = "vkDestroyBufferView"]
    pub fn destroy_buffer_view(
        device: Device,
        buffer_view: BufferView,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkCreateImage"]
    #[must_use]
    pub fn create_image(
        device: Device,
        p_create_info: *const ImageCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_image: *mut Image,
    ) -> Result;

    #[link_name = "vkDestroyImage"]
    pub fn destroy_image(device: Device, image: Image, p_allocator: *const AllocationCallbacks);

    #[link_name = "vkGetImageSubresourceLayout"]
    pub fn get_image_subresource_layout(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource,
        p_layout: *mut SubresourceLayout,
    );

    #[link_name = "vkCreateImageView"]
    #[must_use]
    pub fn create_image_view(
        device: Device,
        p_create_info: *const ImageViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut ImageView,
    ) -> Result;

    #[link_name = "vkDestroyImageView"]
    pub fn destroy_image_view(
        device: Device,
        image_view: ImageView,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkCreateShaderModule"]
    #[must_use]
    pub fn create_shader_module(
        device: Device,
        p_create_info: *const ShaderModuleCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_shader_module: *mut ShaderModule,
    ) -> Result;

    #[link_name = "vkDestroyShaderModule"]
    pub fn destroy_shader_module(
        device: Device,
        shader_module: ShaderModule,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkCreatePipelineCache"]
    #[must_use]
    pub fn create_pipeline_cache(
        device: Device,
        p_create_info: *const PipelineCacheCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_cache: *mut PipelineCache,
    ) -> Result;

    #[link_name = "vkDestroyPipelineCache"]
    pub fn destroy_pipeline_cache(
        device: Device,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkGetPipelineCacheData"]
    #[must_use]
    pub fn get_pipeline_cache_data(
        device: Device,
        pipeline_cache: PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result;

    #[link_name = "vkMergePipelineCaches"]
    #[must_use]
    pub fn merge_pipeline_caches(
        device: Device,
        dst_cache: PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const PipelineCache,
    ) -> Result;

    #[link_name = "vkCreateGraphicsPipelines"]
    #[must_use]
    pub fn create_graphics_pipelines(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const GraphicsPipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> Result;

    #[link_name = "vkCreateComputePipelines"]
    #[must_use]
    pub fn create_compute_pipelines(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ComputePipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> Result;

    #[link_name = "vkDestroyPipeline"]
    pub fn destroy_pipeline(
        device: Device,
        pipeline: Pipeline,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkCreatePipelineLayout"]
    #[must_use]
    pub fn create_pipeline_layout(
        device: Device,
        p_create_info: *const PipelineLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_layout: *mut PipelineLayout,
    ) -> Result;

    #[link_name = "vkDestroyPipelineLayout"]
    pub fn destroy_pipeline_layout(
        device: Device,
        pipeline_layout: PipelineLayout,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkCreateSampler"]
    #[must_use]
    pub fn create_sampler(
        device: Device,
        p_create_info: *const SamplerCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_sampler: *mut Sampler,
    ) -> Result;

    #[link_name = "vkDestroySampler"]
    pub fn destroy_sampler(
        device: Device,
        sampler: Sampler,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkCreateDescriptorSetLayout"]
    #[must_use]
    pub fn create_descriptor_set_layout(
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_set_layout: *mut DescriptorSetLayout,
    ) -> Result;

    #[link_name = "vkDestroyDescriptorSetLayout"]
    pub fn destroy_descriptor_set_layout(
        device: Device,
        descriptor_set_layout: DescriptorSetLayout,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkCreateDescriptorPool"]
    #[must_use]
    pub fn create_descriptor_pool(
        device: Device,
        p_create_info: *const DescriptorPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_pool: *mut DescriptorPool,
    ) -> Result;

    #[link_name = "vkDestroyDescriptorPool"]
    pub fn destroy_descriptor_pool(
        device: Device,
        descriptor_pool: DescriptorPool,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkResetDescriptorPool"]
    #[must_use]
    pub fn reset_descriptor_pool(
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> Result;

    #[link_name = "vkAllocateDescriptorSets"]
    #[must_use]
    pub fn allocate_descriptor_sets(
        device: Device,
        p_allocate_info: *const DescriptorSetAllocateInfo,
        p_descriptor_sets: *mut DescriptorSet,
    ) -> Result;

    #[link_name = "vkFreeDescriptorSets"]
    #[must_use]
    pub fn free_descriptor_sets(
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
    ) -> Result;

    #[link_name = "vkUpdateDescriptorSets"]
    pub fn update_descriptor_sets(
        device: Device,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const CopyDescriptorSet,
    );

    #[link_name = "vkCreateFramebuffer"]
    #[must_use]
    pub fn create_framebuffer(
        device: Device,
        p_create_info: *const FramebufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_framebuffer: *mut Framebuffer,
    ) -> Result;

    #[link_name = "vkDestroyFramebuffer"]
    pub fn destroy_framebuffer(
        device: Device,
        framebuffer: Framebuffer,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkCreateRenderPass"]
    #[must_use]
    pub fn create_render_pass(
        device: Device,
        p_create_info: *const RenderPassCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *mut RenderPass,
    ) -> Result;

    #[link_name = "vkDestroyRenderPass"]
    pub fn destroy_render_pass(
        device: Device,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkGetRenderAreaGranularity"]
    pub fn get_render_area_granularity(
        device: Device,
        render_pass: RenderPass,
        p_granularity: *mut Extent2D,
    );

    #[link_name = "vkCreateCommandPool"]
    #[must_use]
    pub fn create_command_pool(
        device: Device,
        p_create_info: *const CommandPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_command_pool: *mut CommandPool,
    ) -> Result;

    #[link_name = "vkDestroyCommandPool"]
    pub fn destroy_command_pool(
        device: Device,
        command_pool: CommandPool,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkResetCommandPool"]
    #[must_use]
    pub fn reset_command_pool(
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> Result;

    #[link_name = "vkAllocateCommandBuffers"]
    #[must_use]
    pub fn allocate_command_buffers(
        device: Device,
        p_allocate_info: *const CommandBufferAllocateInfo,
        p_command_buffers: *mut CommandBuffer,
    ) -> Result;

    #[link_name = "vkFreeCommandBuffers"]
    pub fn free_command_buffers(
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    );

    #[link_name = "vkBeginCommandBuffer"]
    #[must_use]
    pub fn begin_command_buffer(
        command_buffer: CommandBuffer,
        p_begin_info: *const CommandBufferBeginInfo,
    ) -> Result;

    #[link_name = "vkEndCommandBuffer"]
    #[must_use]
    pub fn end_command_buffer(command_buffer: CommandBuffer) -> Result;

    #[link_name = "vkResetCommandBuffer"]
    #[must_use]
    pub fn reset_command_buffer(
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> Result;

    #[link_name = "vkCmdBindPipeline"]
    pub fn cmd_bind_pipeline(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    );

    #[link_name = "vkCmdSetViewport"]
    pub fn cmd_set_viewport(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const Viewport,
    );

    #[link_name = "vkCmdSetScissor"]
    pub fn cmd_set_scissor(
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    );

    #[link_name = "vkCmdSetLineWidth"]
    pub fn cmd_set_line_width(command_buffer: CommandBuffer, line_width: f32);

    #[link_name = "vkCmdSetDepthBias"]
    pub fn cmd_set_depth_bias(
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    );

    #[link_name = "vkCmdSetBlendConstants"]
    pub fn cmd_set_blend_constants(command_buffer: CommandBuffer, blend_constants: f32);

    #[link_name = "vkCmdSetDepthBounds"]
    pub fn cmd_set_depth_bounds(
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    );

    #[link_name = "vkCmdSetStencilCompareMask"]
    pub fn cmd_set_stencil_compare_mask(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    );

    #[link_name = "vkCmdSetStencilWriteMask"]
    pub fn cmd_set_stencil_write_mask(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    );

    #[link_name = "vkCmdSetStencilReference"]
    pub fn cmd_set_stencil_reference(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    );

    #[link_name = "vkCmdBindDescriptorSets"]
    pub fn cmd_bind_descriptor_sets(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    );

    #[link_name = "vkCmdBindIndexBuffer"]
    pub fn cmd_bind_index_buffer(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    );

    #[link_name = "vkCmdBindVertexBuffers"]
    pub fn cmd_bind_vertex_buffers(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
    );

    #[link_name = "vkCmdDraw"]
    pub fn cmd_draw(
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );

    #[link_name = "vkCmdDrawIndexed"]
    pub fn cmd_draw_indexed(
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    );

    #[link_name = "vkCmdDrawIndirect"]
    pub fn cmd_draw_indirect(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    );

    #[link_name = "vkCmdDrawIndexedIndirect"]
    pub fn cmd_draw_indexed_indirect(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    );

    #[link_name = "vkCmdDispatch"]
    pub fn cmd_dispatch(
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    );

    #[link_name = "vkCmdDispatchIndirect"]
    pub fn cmd_dispatch_indirect(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);

    #[link_name = "vkCmdCopyBuffer"]
    pub fn cmd_copy_buffer(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferCopy,
    );

    #[link_name = "vkCmdCopyImage"]
    pub fn cmd_copy_image(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageCopy,
    );

    #[link_name = "vkCmdBlitImage"]
    pub fn cmd_blit_image(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageBlit,
        filter: Filter,
    );

    #[link_name = "vkCmdCopyBufferToImage"]
    pub fn cmd_copy_buffer_to_image(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    );

    #[link_name = "vkCmdCopyImageToBuffer"]
    pub fn cmd_copy_image_to_buffer(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    );

    #[link_name = "vkCmdUpdateBuffer"]
    pub fn cmd_update_buffer(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        p_data: *const c_void,
    );

    #[link_name = "vkCmdFillBuffer"]
    pub fn cmd_fill_buffer(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    );

    #[link_name = "vkCmdClearColorImage"]
    pub fn cmd_clear_color_image(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    );

    #[link_name = "vkCmdClearDepthStencilImage"]
    pub fn cmd_clear_depth_stencil_image(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_depth_stencil: *const ClearDepthStencilValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    );

    #[link_name = "vkCmdClearAttachments"]
    pub fn cmd_clear_attachments(
        command_buffer: CommandBuffer,
        attachment_count: u32,
        p_attachments: *const ClearAttachment,
        rect_count: u32,
        p_rects: *const ClearRect,
    );

    #[link_name = "vkCmdResolveImage"]
    pub fn cmd_resolve_image(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageResolve,
    );

    #[link_name = "vkCmdSetEvent"]
    pub fn cmd_set_event(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    );

    #[link_name = "vkCmdResetEvent"]
    pub fn cmd_reset_event(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    );

    #[link_name = "vkCmdWaitEvents"]
    pub fn cmd_wait_events(
        command_buffer: CommandBuffer,
        event_count: u32,
        p_events: *const Event,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    );

    #[link_name = "vkCmdPipelineBarrier"]
    pub fn cmd_pipeline_barrier(
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    );

    #[link_name = "vkCmdBeginQuery"]
    pub fn cmd_begin_query(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    );

    #[link_name = "vkCmdEndQuery"]
    pub fn cmd_end_query(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32);

    #[link_name = "vkCmdBeginConditionalRenderingEXT"]
    pub fn cmd_begin_conditional_rendering_ext(
        command_buffer: CommandBuffer,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
    );

    #[link_name = "vkCmdEndConditionalRenderingEXT"]
    pub fn cmd_end_conditional_rendering_ext(command_buffer: CommandBuffer);

    #[link_name = "vkCmdResetQueryPool"]
    pub fn cmd_reset_query_pool(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    );

    #[link_name = "vkCmdWriteTimestamp"]
    pub fn cmd_write_timestamp(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        query_pool: QueryPool,
        query: u32,
    );

    #[link_name = "vkCmdCopyQueryPoolResults"]
    pub fn cmd_copy_query_pool_results(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    );

    #[link_name = "vkCmdPushConstants"]
    pub fn cmd_push_constants(
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const c_void,
    );

    #[link_name = "vkCmdBeginRenderPass"]
    pub fn cmd_begin_render_pass(
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo,
        contents: SubpassContents,
    );

    #[link_name = "vkCmdNextSubpass"]
    pub fn cmd_next_subpass(command_buffer: CommandBuffer, contents: SubpassContents);

    #[link_name = "vkCmdEndRenderPass"]
    pub fn cmd_end_render_pass(command_buffer: CommandBuffer);

    #[link_name = "vkCmdExecuteCommands"]
    pub fn cmd_execute_commands(
        command_buffer: CommandBuffer,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    );

    #[link_name = "vkCreateAndroidSurfaceKHR"]
    #[must_use]
    pub fn create_android_surface_khr(
        instance: Instance,
        p_create_info: *const AndroidSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceDisplayPropertiesKHR"]
    #[must_use]
    pub fn get_physical_device_display_properties_khr(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPropertiesKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR"]
    #[must_use]
    pub fn get_physical_device_display_plane_properties_khr(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlanePropertiesKHR,
    ) -> Result;

    #[link_name = "vkGetDisplayPlaneSupportedDisplaysKHR"]
    #[must_use]
    pub fn get_display_plane_supported_displays_khr(
        physical_device: PhysicalDevice,
        plane_index: u32,
        p_display_count: *mut u32,
        p_displays: *mut DisplayKHR,
    ) -> Result;

    #[link_name = "vkGetDisplayModePropertiesKHR"]
    #[must_use]
    pub fn get_display_mode_properties_khr(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModePropertiesKHR,
    ) -> Result;

    #[link_name = "vkCreateDisplayModeKHR"]
    #[must_use]
    pub fn create_display_mode_khr(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_create_info: *const DisplayModeCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_mode: *mut DisplayModeKHR,
    ) -> Result;

    #[link_name = "vkGetDisplayPlaneCapabilitiesKHR"]
    #[must_use]
    pub fn get_display_plane_capabilities_khr(
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
        p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
    ) -> Result;

    #[link_name = "vkCreateDisplayPlaneSurfaceKHR"]
    #[must_use]
    pub fn create_display_plane_surface_khr(
        instance: Instance,
        p_create_info: *const DisplaySurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkCreateSharedSwapchainsKHR"]
    #[must_use]
    pub fn create_shared_swapchains_khr(
        device: Device,
        swapchain_count: u32,
        p_create_infos: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchains: *mut SwapchainKHR,
    ) -> Result;

    #[link_name = "vkDestroySurfaceKHR"]
    pub fn destroy_surface_khr(
        instance: Instance,
        surface: SurfaceKHR,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkGetPhysicalDeviceSurfaceSupportKHR"]
    #[must_use]
    pub fn get_physical_device_surface_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKHR,
        p_supported: *mut Bool32,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR"]
    #[must_use]
    pub fn get_physical_device_surface_capabilities_khr(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceSurfaceFormatsKHR"]
    #[must_use]
    pub fn get_physical_device_surface_formats_khr(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormatKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceSurfacePresentModesKHR"]
    #[must_use]
    pub fn get_physical_device_surface_present_modes_khr(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> Result;

    #[link_name = "vkCreateSwapchainKHR"]
    #[must_use]
    pub fn create_swapchain_khr(
        device: Device,
        p_create_info: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchain: *mut SwapchainKHR,
    ) -> Result;

    #[link_name = "vkDestroySwapchainKHR"]
    pub fn destroy_swapchain_khr(
        device: Device,
        swapchain: SwapchainKHR,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkGetSwapchainImagesKHR"]
    #[must_use]
    pub fn get_swapchain_images_khr(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut Image,
    ) -> Result;

    #[link_name = "vkAcquireNextImageKHR"]
    #[must_use]
    pub fn acquire_next_image_khr(
        device: Device,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
        p_image_index: *mut u32,
    ) -> Result;

    #[link_name = "vkQueuePresentKHR"]
    #[must_use]
    pub fn queue_present_khr(queue: Queue, p_present_info: *const PresentInfoKHR) -> Result;

    #[link_name = "vkCreateViSurfaceNN"]
    #[must_use]
    pub fn create_vi_surface_nn(
        instance: Instance,
        p_create_info: *const ViSurfaceCreateInfoNN,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkCreateWaylandSurfaceKHR"]
    #[must_use]
    pub fn create_wayland_surface_khr(
        instance: Instance,
        p_create_info: *const WaylandSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceWaylandPresentationSupportKHR"]
    #[must_use]
    pub fn get_physical_device_wayland_presentation_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32;

    #[link_name = "vkCreateWin32SurfaceKHR"]
    #[must_use]
    pub fn create_win32_surface_khr(
        instance: Instance,
        p_create_info: *const Win32SurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceWin32PresentationSupportKHR"]
    #[must_use]
    pub fn get_physical_device_win32_presentation_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Bool32;

    #[link_name = "vkCreateXlibSurfaceKHR"]
    #[must_use]
    pub fn create_xlib_surface_khr(
        instance: Instance,
        p_create_info: *const XlibSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceXlibPresentationSupportKHR"]
    #[must_use]
    pub fn get_physical_device_xlib_presentation_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32;

    #[link_name = "vkCreateXcbSurfaceKHR"]
    #[must_use]
    pub fn create_xcb_surface_khr(
        instance: Instance,
        p_create_info: *const XcbSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceXcbPresentationSupportKHR"]
    #[must_use]
    pub fn get_physical_device_xcb_presentation_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32;

    #[link_name = "vkCreateDirectFBSurfaceEXT"]
    #[must_use]
    pub fn create_direct_fbsurface_ext(
        instance: Instance,
        p_create_info: *const DirectFBSurfaceCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT"]
    #[must_use]
    pub fn get_physical_device_direct_fbpresentation_support_ext(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32;

    #[link_name = "vkCreateImagePipeSurfaceFUCHSIA"]
    #[must_use]
    pub fn create_image_pipe_surface_fuchsia(
        instance: Instance,
        p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkCreateStreamDescriptorSurfaceGGP"]
    #[must_use]
    pub fn create_stream_descriptor_surface_ggp(
        instance: Instance,
        p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkCreateDebugReportCallbackEXT"]
    #[must_use]
    pub fn create_debug_report_callback_ext(
        instance: Instance,
        p_create_info: *const DebugReportCallbackCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_callback: *mut DebugReportCallbackEXT,
    ) -> Result;

    #[link_name = "vkDestroyDebugReportCallbackEXT"]
    pub fn destroy_debug_report_callback_ext(
        instance: Instance,
        callback: DebugReportCallbackEXT,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkDebugReportMessageEXT"]
    pub fn debug_report_message_ext(
        instance: Instance,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
    );

    #[link_name = "vkDebugMarkerSetObjectNameEXT"]
    #[must_use]
    pub fn debug_marker_set_object_name_ext(
        device: Device,
        p_name_info: *const DebugMarkerObjectNameInfoEXT,
    ) -> Result;

    #[link_name = "vkDebugMarkerSetObjectTagEXT"]
    #[must_use]
    pub fn debug_marker_set_object_tag_ext(
        device: Device,
        p_tag_info: *const DebugMarkerObjectTagInfoEXT,
    ) -> Result;

    #[link_name = "vkCmdDebugMarkerBeginEXT"]
    pub fn cmd_debug_marker_begin_ext(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT,
    );

    #[link_name = "vkCmdDebugMarkerEndEXT"]
    pub fn cmd_debug_marker_end_ext(command_buffer: CommandBuffer);

    #[link_name = "vkCmdDebugMarkerInsertEXT"]
    pub fn cmd_debug_marker_insert_ext(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT,
    );

    #[link_name = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV"]
    #[must_use]
    pub fn get_physical_device_external_image_format_properties_nv(
        physical_device: PhysicalDevice,
        format: Format,
        vk_type: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
    ) -> Result;

    #[link_name = "vkGetMemoryWin32HandleNV"]
    #[must_use]
    pub fn get_memory_win32_handle_nv(
        device: Device,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_handle: *mut HANDLE,
    ) -> Result;

    #[link_name = "vkCmdExecuteGeneratedCommandsNV"]
    pub fn cmd_execute_generated_commands_nv(
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoNV,
    );

    #[link_name = "vkCmdPreprocessGeneratedCommandsNV"]
    pub fn cmd_preprocess_generated_commands_nv(
        command_buffer: CommandBuffer,
        p_generated_commands_info: *const GeneratedCommandsInfoNV,
    );

    #[link_name = "vkCmdBindPipelineShaderGroupNV"]
    pub fn cmd_bind_pipeline_shader_group_nv(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    );

    #[link_name = "vkGetGeneratedCommandsMemoryRequirementsNV"]
    pub fn get_generated_commands_memory_requirements_nv(
        device: Device,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoNV,
        p_memory_requirements: *mut MemoryRequirements2,
    );

    #[link_name = "vkCreateIndirectCommandsLayoutNV"]
    #[must_use]
    pub fn create_indirect_commands_layout_nv(
        device: Device,
        p_create_info: *const IndirectCommandsLayoutCreateInfoNV,
        p_allocator: *const AllocationCallbacks,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
    ) -> Result;

    #[link_name = "vkDestroyIndirectCommandsLayoutNV"]
    pub fn destroy_indirect_commands_layout_nv(
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkGetPhysicalDeviceFeatures2"]
    pub fn get_physical_device_features2(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures2,
    );

    #[link_name = "vkGetPhysicalDeviceProperties2"]
    pub fn get_physical_device_properties2(
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties2,
    );

    #[link_name = "vkGetPhysicalDeviceFormatProperties2"]
    pub fn get_physical_device_format_properties2(
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties2,
    );

    #[link_name = "vkGetPhysicalDeviceImageFormatProperties2"]
    #[must_use]
    pub fn get_physical_device_image_format_properties2(
        physical_device: PhysicalDevice,
        p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut ImageFormatProperties2,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceQueueFamilyProperties2"]
    pub fn get_physical_device_queue_family_properties2(
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2,
    );

    #[link_name = "vkGetPhysicalDeviceMemoryProperties2"]
    pub fn get_physical_device_memory_properties2(
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
    );

    #[link_name = "vkGetPhysicalDeviceSparseImageFormatProperties2"]
    pub fn get_physical_device_sparse_image_format_properties2(
        physical_device: PhysicalDevice,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2,
    );

    #[link_name = "vkCmdPushDescriptorSetKHR"]
    pub fn cmd_push_descriptor_set_khr(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet,
    );

    #[link_name = "vkTrimCommandPool"]
    pub fn trim_command_pool(
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    );

    #[link_name = "vkGetPhysicalDeviceExternalBufferProperties"]
    pub fn get_physical_device_external_buffer_properties(
        physical_device: PhysicalDevice,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *mut ExternalBufferProperties,
    );

    #[link_name = "vkGetMemoryWin32HandleKHR"]
    #[must_use]
    pub fn get_memory_win32_handle_khr(
        device: Device,
        p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
        p_handle: *mut HANDLE,
    ) -> Result;

    #[link_name = "vkGetMemoryWin32HandlePropertiesKHR"]
    #[must_use]
    pub fn get_memory_win32_handle_properties_khr(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        handle: HANDLE,
        p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
    ) -> Result;

    #[link_name = "vkGetMemoryFdKHR"]
    #[must_use]
    pub fn get_memory_fd_khr(
        device: Device,
        p_get_fd_info: *const MemoryGetFdInfoKHR,
        p_fd: *mut i32,
    ) -> Result;

    #[link_name = "vkGetMemoryFdPropertiesKHR"]
    #[must_use]
    pub fn get_memory_fd_properties_khr(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: i32,
        p_memory_fd_properties: *mut MemoryFdPropertiesKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceExternalSemaphoreProperties"]
    pub fn get_physical_device_external_semaphore_properties(
        physical_device: PhysicalDevice,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
    );

    #[link_name = "vkGetSemaphoreWin32HandleKHR"]
    #[must_use]
    pub fn get_semaphore_win32_handle_khr(
        device: Device,
        p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
        p_handle: *mut HANDLE,
    ) -> Result;

    #[link_name = "vkImportSemaphoreWin32HandleKHR"]
    #[must_use]
    pub fn import_semaphore_win32_handle_khr(
        device: Device,
        p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
    ) -> Result;

    #[link_name = "vkGetSemaphoreFdKHR"]
    #[must_use]
    pub fn get_semaphore_fd_khr(
        device: Device,
        p_get_fd_info: *const SemaphoreGetFdInfoKHR,
        p_fd: *mut i32,
    ) -> Result;

    #[link_name = "vkImportSemaphoreFdKHR"]
    #[must_use]
    pub fn import_semaphore_fd_khr(
        device: Device,
        p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceExternalFenceProperties"]
    pub fn get_physical_device_external_fence_properties(
        physical_device: PhysicalDevice,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *mut ExternalFenceProperties,
    );

    #[link_name = "vkGetFenceWin32HandleKHR"]
    #[must_use]
    pub fn get_fence_win32_handle_khr(
        device: Device,
        p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR,
        p_handle: *mut HANDLE,
    ) -> Result;

    #[link_name = "vkImportFenceWin32HandleKHR"]
    #[must_use]
    pub fn import_fence_win32_handle_khr(
        device: Device,
        p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR,
    ) -> Result;

    #[link_name = "vkGetFenceFdKHR"]
    #[must_use]
    pub fn get_fence_fd_khr(
        device: Device,
        p_get_fd_info: *const FenceGetFdInfoKHR,
        p_fd: *mut i32,
    ) -> Result;

    #[link_name = "vkImportFenceFdKHR"]
    #[must_use]
    pub fn import_fence_fd_khr(
        device: Device,
        p_import_fence_fd_info: *const ImportFenceFdInfoKHR,
    ) -> Result;

    #[link_name = "vkReleaseDisplayEXT"]
    #[must_use]
    pub fn release_display_ext(physical_device: PhysicalDevice, display: DisplayKHR) -> Result;

    #[link_name = "vkAcquireXlibDisplayEXT"]
    #[must_use]
    pub fn acquire_xlib_display_ext(
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        display: DisplayKHR,
    ) -> Result;

    #[link_name = "vkGetRandROutputDisplayEXT"]
    #[must_use]
    pub fn get_rand_routput_display_ext(
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        rr_output: RROutput,
        p_display: *mut DisplayKHR,
    ) -> Result;

    #[link_name = "vkDisplayPowerControlEXT"]
    #[must_use]
    pub fn display_power_control_ext(
        device: Device,
        display: DisplayKHR,
        p_display_power_info: *const DisplayPowerInfoEXT,
    ) -> Result;

    #[link_name = "vkRegisterDeviceEventEXT"]
    #[must_use]
    pub fn register_device_event_ext(
        device: Device,
        p_device_event_info: *const DeviceEventInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence,
    ) -> Result;

    #[link_name = "vkRegisterDisplayEventEXT"]
    #[must_use]
    pub fn register_display_event_ext(
        device: Device,
        display: DisplayKHR,
        p_display_event_info: *const DisplayEventInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence,
    ) -> Result;

    #[link_name = "vkGetSwapchainCounterEXT"]
    #[must_use]
    pub fn get_swapchain_counter_ext(
        device: Device,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagBitsEXT,
        p_counter_value: *mut u64,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceSurfaceCapabilities2EXT"]
    #[must_use]
    pub fn get_physical_device_surface_capabilities2_ext(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_capabilities: *mut SurfaceCapabilities2EXT,
    ) -> Result;

    #[link_name = "vkEnumeratePhysicalDeviceGroups"]
    #[must_use]
    pub fn enumerate_physical_device_groups(
        instance: Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
    ) -> Result;

    #[link_name = "vkGetDeviceGroupPeerMemoryFeatures"]
    pub fn get_device_group_peer_memory_features(
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
    );

    #[link_name = "vkBindBufferMemory2"]
    #[must_use]
    pub fn bind_buffer_memory2(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo,
    ) -> Result;

    #[link_name = "vkBindImageMemory2"]
    #[must_use]
    pub fn bind_image_memory2(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo,
    ) -> Result;

    #[link_name = "vkCmdSetDeviceMask"]
    pub fn cmd_set_device_mask(command_buffer: CommandBuffer, device_mask: u32);

    #[link_name = "vkGetDeviceGroupPresentCapabilitiesKHR"]
    #[must_use]
    pub fn get_device_group_present_capabilities_khr(
        device: Device,
        p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
    ) -> Result;

    #[link_name = "vkGetDeviceGroupSurfacePresentModesKHR"]
    #[must_use]
    pub fn get_device_group_surface_present_modes_khr(
        device: Device,
        surface: SurfaceKHR,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> Result;

    #[link_name = "vkAcquireNextImage2KHR"]
    #[must_use]
    pub fn acquire_next_image2_khr(
        device: Device,
        p_acquire_info: *const AcquireNextImageInfoKHR,
        p_image_index: *mut u32,
    ) -> Result;

    #[link_name = "vkCmdDispatchBase"]
    pub fn cmd_dispatch_base(
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    );

    #[link_name = "vkGetPhysicalDevicePresentRectanglesKHR"]
    #[must_use]
    pub fn get_physical_device_present_rectangles_khr(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut Rect2D,
    ) -> Result;

    #[link_name = "vkCreateDescriptorUpdateTemplate"]
    #[must_use]
    pub fn create_descriptor_update_template(
        device: Device,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> Result;

    #[link_name = "vkDestroyDescriptorUpdateTemplate"]
    pub fn destroy_descriptor_update_template(
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkUpdateDescriptorSetWithTemplate"]
    pub fn update_descriptor_set_with_template(
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    );

    #[link_name = "vkCmdPushDescriptorSetWithTemplateKHR"]
    pub fn cmd_push_descriptor_set_with_template_khr(
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        p_data: *const c_void,
    );

    #[link_name = "vkSetHdrMetadataEXT"]
    pub fn set_hdr_metadata_ext(
        device: Device,
        swapchain_count: u32,
        p_swapchains: *const SwapchainKHR,
        p_metadata: *const HdrMetadataEXT,
    );

    #[link_name = "vkGetSwapchainStatusKHR"]
    #[must_use]
    pub fn get_swapchain_status_khr(device: Device, swapchain: SwapchainKHR) -> Result;

    #[link_name = "vkGetRefreshCycleDurationGOOGLE"]
    #[must_use]
    pub fn get_refresh_cycle_duration_google(
        device: Device,
        swapchain: SwapchainKHR,
        p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
    ) -> Result;

    #[link_name = "vkGetPastPresentationTimingGOOGLE"]
    #[must_use]
    pub fn get_past_presentation_timing_google(
        device: Device,
        swapchain: SwapchainKHR,
        p_presentation_timing_count: *mut u32,
        p_presentation_timings: *mut PastPresentationTimingGOOGLE,
    ) -> Result;

    #[link_name = "vkCreateIOSSurfaceMVK"]
    #[must_use]
    pub fn create_iossurface_mvk(
        instance: Instance,
        p_create_info: *const IOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkCreateMacOSSurfaceMVK"]
    #[must_use]
    pub fn create_mac_ossurface_mvk(
        instance: Instance,
        p_create_info: *const MacOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkCreateMetalSurfaceEXT"]
    #[must_use]
    pub fn create_metal_surface_ext(
        instance: Instance,
        p_create_info: *const MetalSurfaceCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkCmdSetViewportWScalingNV"]
    pub fn cmd_set_viewport_wscaling_nv(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_wscalings: *const ViewportWScalingNV,
    );

    #[link_name = "vkCmdSetDiscardRectangleEXT"]
    pub fn cmd_set_discard_rectangle_ext(
        command_buffer: CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
        p_discard_rectangles: *const Rect2D,
    );

    #[link_name = "vkCmdSetSampleLocationsEXT"]
    pub fn cmd_set_sample_locations_ext(
        command_buffer: CommandBuffer,
        p_sample_locations_info: *const SampleLocationsInfoEXT,
    );

    #[link_name = "vkGetPhysicalDeviceMultisamplePropertiesEXT"]
    pub fn get_physical_device_multisample_properties_ext(
        physical_device: PhysicalDevice,
        samples: SampleCountFlagBits,
        p_multisample_properties: *mut MultisamplePropertiesEXT,
    );

    #[link_name = "vkGetPhysicalDeviceSurfaceCapabilities2KHR"]
    #[must_use]
    pub fn get_physical_device_surface_capabilities2_khr(
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        p_surface_capabilities: *mut SurfaceCapabilities2KHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceSurfaceFormats2KHR"]
    #[must_use]
    pub fn get_physical_device_surface_formats2_khr(
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormat2KHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceDisplayProperties2KHR"]
    #[must_use]
    pub fn get_physical_device_display_properties2_khr(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayProperties2KHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR"]
    #[must_use]
    pub fn get_physical_device_display_plane_properties2_khr(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlaneProperties2KHR,
    ) -> Result;

    #[link_name = "vkGetDisplayModeProperties2KHR"]
    #[must_use]
    pub fn get_display_mode_properties2_khr(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModeProperties2KHR,
    ) -> Result;

    #[link_name = "vkGetDisplayPlaneCapabilities2KHR"]
    #[must_use]
    pub fn get_display_plane_capabilities2_khr(
        physical_device: PhysicalDevice,
        p_display_plane_info: *const DisplayPlaneInfo2KHR,
        p_capabilities: *mut DisplayPlaneCapabilities2KHR,
    ) -> Result;

    #[link_name = "vkGetBufferMemoryRequirements2"]
    pub fn get_buffer_memory_requirements2(
        device: Device,
        p_info: *const BufferMemoryRequirementsInfo2,
        p_memory_requirements: *mut MemoryRequirements2,
    );

    #[link_name = "vkGetImageMemoryRequirements2"]
    pub fn get_image_memory_requirements2(
        device: Device,
        p_info: *const ImageMemoryRequirementsInfo2,
        p_memory_requirements: *mut MemoryRequirements2,
    );

    #[link_name = "vkGetImageSparseMemoryRequirements2"]
    pub fn get_image_sparse_memory_requirements2(
        device: Device,
        p_info: *const ImageSparseMemoryRequirementsInfo2,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
    );

    #[link_name = "vkCreateSamplerYcbcrConversion"]
    #[must_use]
    pub fn create_sampler_ycbcr_conversion(
        device: Device,
        p_create_info: *const SamplerYcbcrConversionCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> Result;

    #[link_name = "vkDestroySamplerYcbcrConversion"]
    pub fn destroy_sampler_ycbcr_conversion(
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkGetDeviceQueue2"]
    pub fn get_device_queue2(
        device: Device,
        p_queue_info: *const DeviceQueueInfo2,
        p_queue: *mut Queue,
    );

    #[link_name = "vkCreateValidationCacheEXT"]
    #[must_use]
    pub fn create_validation_cache_ext(
        device: Device,
        p_create_info: *const ValidationCacheCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_validation_cache: *mut ValidationCacheEXT,
    ) -> Result;

    #[link_name = "vkDestroyValidationCacheEXT"]
    pub fn destroy_validation_cache_ext(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkGetValidationCacheDataEXT"]
    #[must_use]
    pub fn get_validation_cache_data_ext(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result;

    #[link_name = "vkMergeValidationCachesEXT"]
    #[must_use]
    pub fn merge_validation_caches_ext(
        device: Device,
        dst_cache: ValidationCacheEXT,
        src_cache_count: u32,
        p_src_caches: *const ValidationCacheEXT,
    ) -> Result;

    #[link_name = "vkGetDescriptorSetLayoutSupport"]
    pub fn get_descriptor_set_layout_support(
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo,
        p_support: *mut DescriptorSetLayoutSupport,
    );

    #[link_name = "vkGetSwapchainGrallocUsageANDROID"]
    #[must_use]
    pub fn get_swapchain_gralloc_usage_android(
        device: Device,
        format: Format,
        image_usage: ImageUsageFlags,
        gralloc_usage: *mut i32,
    ) -> Result;

    #[link_name = "vkGetSwapchainGrallocUsage2ANDROID"]
    #[must_use]
    pub fn get_swapchain_gralloc_usage2_android(
        device: Device,
        format: Format,
        image_usage: ImageUsageFlags,
        swapchain_image_usage: SwapchainImageUsageFlagsANDROID,
        gralloc_consumer_usage: *mut u64,
        gralloc_producer_usage: *mut u64,
    ) -> Result;

    #[link_name = "vkAcquireImageANDROID"]
    #[must_use]
    pub fn acquire_image_android(
        device: Device,
        image: Image,
        native_fence_fd: i32,
        semaphore: Semaphore,
        fence: Fence,
    ) -> Result;

    #[link_name = "vkQueueSignalReleaseImageANDROID"]
    #[must_use]
    pub fn queue_signal_release_image_android(
        queue: Queue,
        wait_semaphore_count: u32,
        p_wait_semaphores: *const Semaphore,
        image: Image,
        p_native_fence_fd: *mut i32,
    ) -> Result;

    #[link_name = "vkGetShaderInfoAMD"]
    #[must_use]
    pub fn get_shader_info_amd(
        device: Device,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlagBits,
        info_type: ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
    ) -> Result;

    #[link_name = "vkSetLocalDimmingAMD"]
    pub fn set_local_dimming_amd(
        device: Device,
        swap_chain: SwapchainKHR,
        local_dimming_enable: Bool32,
    );

    #[link_name = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT"]
    #[must_use]
    pub fn get_physical_device_calibrateable_time_domains_ext(
        physical_device: PhysicalDevice,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut TimeDomainEXT,
    ) -> Result;

    #[link_name = "vkGetCalibratedTimestampsEXT"]
    #[must_use]
    pub fn get_calibrated_timestamps_ext(
        device: Device,
        timestamp_count: u32,
        p_timestamp_infos: *const CalibratedTimestampInfoEXT,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> Result;

    #[link_name = "vkSetDebugUtilsObjectNameEXT"]
    #[must_use]
    pub fn set_debug_utils_object_name_ext(
        device: Device,
        p_name_info: *const DebugUtilsObjectNameInfoEXT,
    ) -> Result;

    #[link_name = "vkSetDebugUtilsObjectTagEXT"]
    #[must_use]
    pub fn set_debug_utils_object_tag_ext(
        device: Device,
        p_tag_info: *const DebugUtilsObjectTagInfoEXT,
    ) -> Result;

    #[link_name = "vkQueueBeginDebugUtilsLabelEXT"]
    pub fn queue_begin_debug_utils_label_ext(queue: Queue, p_label_info: *const DebugUtilsLabelEXT);

    #[link_name = "vkQueueEndDebugUtilsLabelEXT"]
    pub fn queue_end_debug_utils_label_ext(queue: Queue);

    #[link_name = "vkQueueInsertDebugUtilsLabelEXT"]
    pub fn queue_insert_debug_utils_label_ext(
        queue: Queue,
        p_label_info: *const DebugUtilsLabelEXT,
    );

    #[link_name = "vkCmdBeginDebugUtilsLabelEXT"]
    pub fn cmd_begin_debug_utils_label_ext(
        command_buffer: CommandBuffer,
        p_label_info: *const DebugUtilsLabelEXT,
    );

    #[link_name = "vkCmdEndDebugUtilsLabelEXT"]
    pub fn cmd_end_debug_utils_label_ext(command_buffer: CommandBuffer);

    #[link_name = "vkCmdInsertDebugUtilsLabelEXT"]
    pub fn cmd_insert_debug_utils_label_ext(
        command_buffer: CommandBuffer,
        p_label_info: *const DebugUtilsLabelEXT,
    );

    #[link_name = "vkCreateDebugUtilsMessengerEXT"]
    #[must_use]
    pub fn create_debug_utils_messenger_ext(
        instance: Instance,
        p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_messenger: *mut DebugUtilsMessengerEXT,
    ) -> Result;

    #[link_name = "vkDestroyDebugUtilsMessengerEXT"]
    pub fn destroy_debug_utils_messenger_ext(
        instance: Instance,
        messenger: DebugUtilsMessengerEXT,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkSubmitDebugUtilsMessageEXT"]
    pub fn submit_debug_utils_message_ext(
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    );

    #[link_name = "vkGetMemoryHostPointerPropertiesEXT"]
    #[must_use]
    pub fn get_memory_host_pointer_properties_ext(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
    ) -> Result;

    #[link_name = "vkCmdWriteBufferMarkerAMD"]
    pub fn cmd_write_buffer_marker_amd(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    );

    #[link_name = "vkCreateRenderPass2"]
    #[must_use]
    pub fn create_render_pass2(
        device: Device,
        p_create_info: *const RenderPassCreateInfo2,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *mut RenderPass,
    ) -> Result;

    #[link_name = "vkCmdBeginRenderPass2"]
    pub fn cmd_begin_render_pass2(
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo,
        p_subpass_begin_info: *const SubpassBeginInfo,
    );

    #[link_name = "vkCmdNextSubpass2"]
    pub fn cmd_next_subpass2(
        command_buffer: CommandBuffer,
        p_subpass_begin_info: *const SubpassBeginInfo,
        p_subpass_end_info: *const SubpassEndInfo,
    );

    #[link_name = "vkCmdEndRenderPass2"]
    pub fn cmd_end_render_pass2(
        command_buffer: CommandBuffer,
        p_subpass_end_info: *const SubpassEndInfo,
    );

    #[link_name = "vkGetSemaphoreCounterValue"]
    #[must_use]
    pub fn get_semaphore_counter_value(
        device: Device,
        semaphore: Semaphore,
        p_value: *mut u64,
    ) -> Result;

    #[link_name = "vkWaitSemaphores"]
    #[must_use]
    pub fn wait_semaphores(
        device: Device,
        p_wait_info: *const SemaphoreWaitInfo,
        timeout: u64,
    ) -> Result;

    #[link_name = "vkSignalSemaphore"]
    #[must_use]
    pub fn signal_semaphore(device: Device, p_signal_info: *const SemaphoreSignalInfo) -> Result;

    #[link_name = "vkGetAndroidHardwareBufferPropertiesANDROID"]
    #[must_use]
    pub fn get_android_hardware_buffer_properties_android(
        device: Device,
        buffer: *const AHardwareBuffer,
        p_properties: *mut AndroidHardwareBufferPropertiesANDROID,
    ) -> Result;

    #[link_name = "vkGetMemoryAndroidHardwareBufferANDROID"]
    #[must_use]
    pub fn get_memory_android_hardware_buffer_android(
        device: Device,
        p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
        p_buffer: *mut *mut AHardwareBuffer,
    ) -> Result;

    #[link_name = "vkCmdDrawIndirectCount"]
    pub fn cmd_draw_indirect_count(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    );

    #[link_name = "vkCmdDrawIndexedIndirectCount"]
    pub fn cmd_draw_indexed_indirect_count(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    );

    #[link_name = "vkCmdSetCheckpointNV"]
    pub fn cmd_set_checkpoint_nv(command_buffer: CommandBuffer, p_checkpoint_marker: *const c_void);

    #[link_name = "vkGetQueueCheckpointDataNV"]
    pub fn get_queue_checkpoint_data_nv(
        queue: Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointDataNV,
    );

    #[link_name = "vkCmdBindTransformFeedbackBuffersEXT"]
    pub fn cmd_bind_transform_feedback_buffers_ext(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
    );

    #[link_name = "vkCmdBeginTransformFeedbackEXT"]
    pub fn cmd_begin_transform_feedback_ext(
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const DeviceSize,
    );

    #[link_name = "vkCmdEndTransformFeedbackEXT"]
    pub fn cmd_end_transform_feedback_ext(
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const DeviceSize,
    );

    #[link_name = "vkCmdBeginQueryIndexedEXT"]
    pub fn cmd_begin_query_indexed_ext(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    );

    #[link_name = "vkCmdEndQueryIndexedEXT"]
    pub fn cmd_end_query_indexed_ext(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        index: u32,
    );

    #[link_name = "vkCmdDrawIndirectByteCountEXT"]
    pub fn cmd_draw_indirect_byte_count_ext(
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    );

    #[link_name = "vkCmdSetExclusiveScissorNV"]
    pub fn cmd_set_exclusive_scissor_nv(
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissors: *const Rect2D,
    );

    #[link_name = "vkCmdBindShadingRateImageNV"]
    pub fn cmd_bind_shading_rate_image_nv(
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    );

    #[link_name = "vkCmdSetViewportShadingRatePaletteNV"]
    pub fn cmd_set_viewport_shading_rate_palette_nv(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_shading_rate_palettes: *const ShadingRatePaletteNV,
    );

    #[link_name = "vkCmdSetCoarseSampleOrderNV"]
    pub fn cmd_set_coarse_sample_order_nv(
        command_buffer: CommandBuffer,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_order_count: u32,
        p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
    );

    #[link_name = "vkCmdDrawMeshTasksNV"]
    pub fn cmd_draw_mesh_tasks_nv(command_buffer: CommandBuffer, task_count: u32, first_task: u32);

    #[link_name = "vkCmdDrawMeshTasksIndirectNV"]
    pub fn cmd_draw_mesh_tasks_indirect_nv(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    );

    #[link_name = "vkCmdDrawMeshTasksIndirectCountNV"]
    pub fn cmd_draw_mesh_tasks_indirect_count_nv(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    );

    #[link_name = "vkCompileDeferredNV"]
    #[must_use]
    pub fn compile_deferred_nv(device: Device, pipeline: Pipeline, shader: u32) -> Result;

    #[link_name = "vkCreateAccelerationStructureNV"]
    #[must_use]
    pub fn create_acceleration_structure_nv(
        device: Device,
        p_create_info: *const AccelerationStructureCreateInfoNV,
        p_allocator: *const AllocationCallbacks,
        p_acceleration_structure: *mut AccelerationStructureNV,
    ) -> Result;

    #[link_name = "vkDestroyAccelerationStructureKHR"]
    pub fn destroy_acceleration_structure_khr(
        device: Device,
        acceleration_structure: AccelerationStructureKHR,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkGetAccelerationStructureMemoryRequirementsKHR"]
    pub fn get_acceleration_structure_memory_requirements_khr(
        device: Device,
        p_info: *const AccelerationStructureMemoryRequirementsInfoKHR,
        p_memory_requirements: *mut MemoryRequirements2,
    );

    #[link_name = "vkGetAccelerationStructureMemoryRequirementsNV"]
    pub fn get_acceleration_structure_memory_requirements_nv(
        device: Device,
        p_info: *const AccelerationStructureMemoryRequirementsInfoNV,
        p_memory_requirements: *mut MemoryRequirements2KHR,
    );

    #[link_name = "vkBindAccelerationStructureMemoryKHR"]
    #[must_use]
    pub fn bind_acceleration_structure_memory_khr(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindAccelerationStructureMemoryInfoKHR,
    ) -> Result;

    #[link_name = "vkCmdCopyAccelerationStructureNV"]
    pub fn cmd_copy_acceleration_structure_nv(
        command_buffer: CommandBuffer,
        dst: AccelerationStructureKHR,
        src: AccelerationStructureKHR,
        mode: CopyAccelerationStructureModeKHR,
    );

    #[link_name = "vkCmdCopyAccelerationStructureKHR"]
    pub fn cmd_copy_acceleration_structure_khr(
        command_buffer: CommandBuffer,
        p_info: *const CopyAccelerationStructureInfoKHR,
    );

    #[link_name = "vkCopyAccelerationStructureKHR"]
    #[must_use]
    pub fn copy_acceleration_structure_khr(
        device: Device,
        p_info: *const CopyAccelerationStructureInfoKHR,
    ) -> Result;

    #[link_name = "vkCmdCopyAccelerationStructureToMemoryKHR"]
    pub fn cmd_copy_acceleration_structure_to_memory_khr(
        command_buffer: CommandBuffer,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
    );

    #[link_name = "vkCopyAccelerationStructureToMemoryKHR"]
    #[must_use]
    pub fn copy_acceleration_structure_to_memory_khr(
        device: Device,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
    ) -> Result;

    #[link_name = "vkCmdCopyMemoryToAccelerationStructureKHR"]
    pub fn cmd_copy_memory_to_acceleration_structure_khr(
        command_buffer: CommandBuffer,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
    );

    #[link_name = "vkCopyMemoryToAccelerationStructureKHR"]
    #[must_use]
    pub fn copy_memory_to_acceleration_structure_khr(
        device: Device,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
    ) -> Result;

    #[link_name = "vkCmdWriteAccelerationStructuresPropertiesKHR"]
    pub fn cmd_write_acceleration_structures_properties_khr(
        command_buffer: CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    );

    #[link_name = "vkCmdBuildAccelerationStructureNV"]
    pub fn cmd_build_acceleration_structure_nv(
        command_buffer: CommandBuffer,
        p_info: *const AccelerationStructureInfoNV,
        instance_data: Buffer,
        instance_offset: DeviceSize,
        update: Bool32,
        dst: AccelerationStructureKHR,
        src: AccelerationStructureKHR,
        scratch: Buffer,
        scratch_offset: DeviceSize,
    );

    #[link_name = "vkWriteAccelerationStructuresPropertiesKHR"]
    #[must_use]
    pub fn write_acceleration_structures_properties_khr(
        device: Device,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        data_size: usize,
        p_data: *mut c_void,
        stride: usize,
    ) -> Result;

    #[link_name = "vkCmdTraceRaysKHR"]
    pub fn cmd_trace_rays_khr(
        command_buffer: CommandBuffer,
        p_raygen_shader_binding_table: *const StridedBufferRegionKHR,
        p_miss_shader_binding_table: *const StridedBufferRegionKHR,
        p_hit_shader_binding_table: *const StridedBufferRegionKHR,
        p_callable_shader_binding_table: *const StridedBufferRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    );

    #[link_name = "vkCmdTraceRaysNV"]
    pub fn cmd_trace_rays_nv(
        command_buffer: CommandBuffer,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Buffer,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Buffer,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Buffer,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    );

    #[link_name = "vkGetRayTracingShaderGroupHandlesKHR"]
    #[must_use]
    pub fn get_ray_tracing_shader_group_handles_khr(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> Result;

    #[link_name = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR"]
    #[must_use]
    pub fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> Result;

    #[link_name = "vkGetAccelerationStructureHandleNV"]
    #[must_use]
    pub fn get_acceleration_structure_handle_nv(
        device: Device,
        acceleration_structure: AccelerationStructureKHR,
        data_size: usize,
        p_data: *mut c_void,
    ) -> Result;

    #[link_name = "vkCreateRayTracingPipelinesNV"]
    #[must_use]
    pub fn create_ray_tracing_pipelines_nv(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoNV,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> Result;

    #[link_name = "vkCreateRayTracingPipelinesKHR"]
    #[must_use]
    pub fn create_ray_tracing_pipelines_khr(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV"]
    #[must_use]
    pub fn get_physical_device_cooperative_matrix_properties_nv(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixPropertiesNV,
    ) -> Result;

    #[link_name = "vkCmdTraceRaysIndirectKHR"]
    pub fn cmd_trace_rays_indirect_khr(
        command_buffer: CommandBuffer,
        p_raygen_shader_binding_table: *const StridedBufferRegionKHR,
        p_miss_shader_binding_table: *const StridedBufferRegionKHR,
        p_hit_shader_binding_table: *const StridedBufferRegionKHR,
        p_callable_shader_binding_table: *const StridedBufferRegionKHR,
        buffer: Buffer,
        offset: DeviceSize,
    );

    #[link_name = "vkGetDeviceAccelerationStructureCompatibilityKHR"]
    #[must_use]
    pub fn get_device_acceleration_structure_compatibility_khr(
        device: Device,
        version: *const AccelerationStructureVersionKHR,
    ) -> Result;

    #[link_name = "vkGetImageViewHandleNVX"]
    #[must_use]
    pub fn get_image_view_handle_nvx(device: Device, p_info: *const ImageViewHandleInfoNVX) -> u32;

    #[link_name = "vkGetImageViewAddressNVX"]
    #[must_use]
    pub fn get_image_view_address_nvx(
        device: Device,
        image_view: ImageView,
        p_properties: *mut ImageViewAddressPropertiesNVX,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceSurfacePresentModes2EXT"]
    #[must_use]
    pub fn get_physical_device_surface_present_modes2_ext(
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> Result;

    #[link_name = "vkGetDeviceGroupSurfacePresentModes2EXT"]
    #[must_use]
    pub fn get_device_group_surface_present_modes2_ext(
        device: Device,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> Result;

    #[link_name = "vkAcquireFullScreenExclusiveModeEXT"]
    #[must_use]
    pub fn acquire_full_screen_exclusive_mode_ext(
        device: Device,
        swapchain: SwapchainKHR,
    ) -> Result;

    #[link_name = "vkReleaseFullScreenExclusiveModeEXT"]
    #[must_use]
    pub fn release_full_screen_exclusive_mode_ext(
        device: Device,
        swapchain: SwapchainKHR,
    ) -> Result;

    #[link_name = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR"]
    #[must_use]
    pub fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterKHR,
        p_counter_descriptions: *mut PerformanceCounterDescriptionKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR"]
    pub fn get_physical_device_queue_family_performance_query_passes_khr(
        physical_device: PhysicalDevice,
        p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
        p_num_passes: *mut u32,
    );

    #[link_name = "vkAcquireProfilingLockKHR"]
    #[must_use]
    pub fn acquire_profiling_lock_khr(
        device: Device,
        p_info: *const AcquireProfilingLockInfoKHR,
    ) -> Result;

    #[link_name = "vkReleaseProfilingLockKHR"]
    pub fn release_profiling_lock_khr(device: Device);

    #[link_name = "vkGetImageDrmFormatModifierPropertiesEXT"]
    #[must_use]
    pub fn get_image_drm_format_modifier_properties_ext(
        device: Device,
        image: Image,
        p_properties: *mut ImageDrmFormatModifierPropertiesEXT,
    ) -> Result;

    #[link_name = "vkGetBufferOpaqueCaptureAddress"]
    #[must_use]
    pub fn get_buffer_opaque_capture_address(
        device: Device,
        p_info: *const BufferDeviceAddressInfo,
    ) -> u64;

    #[link_name = "vkGetBufferDeviceAddress"]
    #[must_use]
    pub fn get_buffer_device_address(
        device: Device,
        p_info: *const BufferDeviceAddressInfo,
    ) -> DeviceAddress;

    #[link_name = "vkCreateHeadlessSurfaceEXT"]
    #[must_use]
    pub fn create_headless_surface_ext(
        instance: Instance,
        p_create_info: *const HeadlessSurfaceCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    #[link_name = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV"]
    #[must_use]
    pub fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        physical_device: PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV,
    ) -> Result;

    #[link_name = "vkInitializePerformanceApiINTEL"]
    #[must_use]
    pub fn initialize_performance_api_intel(
        device: Device,
        p_initialize_info: *const InitializePerformanceApiInfoINTEL,
    ) -> Result;

    #[link_name = "vkUninitializePerformanceApiINTEL"]
    pub fn uninitialize_performance_api_intel(device: Device);

    #[link_name = "vkCmdSetPerformanceMarkerINTEL"]
    #[must_use]
    pub fn cmd_set_performance_marker_intel(
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceMarkerInfoINTEL,
    ) -> Result;

    #[link_name = "vkCmdSetPerformanceStreamMarkerINTEL"]
    #[must_use]
    pub fn cmd_set_performance_stream_marker_intel(
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceStreamMarkerInfoINTEL,
    ) -> Result;

    #[link_name = "vkCmdSetPerformanceOverrideINTEL"]
    #[must_use]
    pub fn cmd_set_performance_override_intel(
        command_buffer: CommandBuffer,
        p_override_info: *const PerformanceOverrideInfoINTEL,
    ) -> Result;

    #[link_name = "vkAcquirePerformanceConfigurationINTEL"]
    #[must_use]
    pub fn acquire_performance_configuration_intel(
        device: Device,
        p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
        p_configuration: *mut PerformanceConfigurationINTEL,
    ) -> Result;

    #[link_name = "vkReleasePerformanceConfigurationINTEL"]
    #[must_use]
    pub fn release_performance_configuration_intel(
        device: Device,
        configuration: PerformanceConfigurationINTEL,
    ) -> Result;

    #[link_name = "vkQueueSetPerformanceConfigurationINTEL"]
    #[must_use]
    pub fn queue_set_performance_configuration_intel(
        queue: Queue,
        configuration: PerformanceConfigurationINTEL,
    ) -> Result;

    #[link_name = "vkGetPerformanceParameterINTEL"]
    #[must_use]
    pub fn get_performance_parameter_intel(
        device: Device,
        parameter: PerformanceParameterTypeINTEL,
        p_value: *mut PerformanceValueINTEL,
    ) -> Result;

    #[link_name = "vkGetDeviceMemoryOpaqueCaptureAddress"]
    #[must_use]
    pub fn get_device_memory_opaque_capture_address(
        device: Device,
        p_info: *const DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64;

    #[link_name = "vkGetPipelineExecutablePropertiesKHR"]
    #[must_use]
    pub fn get_pipeline_executable_properties_khr(
        device: Device,
        p_pipeline_info: *const PipelineInfoKHR,
        p_executable_count: *mut u32,
        p_properties: *mut PipelineExecutablePropertiesKHR,
    ) -> Result;

    #[link_name = "vkGetPipelineExecutableStatisticsKHR"]
    #[must_use]
    pub fn get_pipeline_executable_statistics_khr(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR,
        p_statistic_count: *mut u32,
        p_statistics: *mut PipelineExecutableStatisticKHR,
    ) -> Result;

    #[link_name = "vkGetPipelineExecutableInternalRepresentationsKHR"]
    #[must_use]
    pub fn get_pipeline_executable_internal_representations_khr(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
    ) -> Result;

    #[link_name = "vkCmdSetLineStippleEXT"]
    pub fn cmd_set_line_stipple_ext(
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    );

    #[link_name = "vkGetPhysicalDeviceToolPropertiesEXT"]
    #[must_use]
    pub fn get_physical_device_tool_properties_ext(
        physical_device: PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut PhysicalDeviceToolPropertiesEXT,
    ) -> Result;

    #[link_name = "vkCreateAccelerationStructureKHR"]
    #[must_use]
    pub fn create_acceleration_structure_khr(
        device: Device,
        p_create_info: *const AccelerationStructureCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_acceleration_structure: *mut AccelerationStructureKHR,
    ) -> Result;

    #[link_name = "vkCmdBuildAccelerationStructureKHR"]
    pub fn cmd_build_acceleration_structure_khr(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
        pp_offset_infos: *const *const AccelerationStructureBuildOffsetInfoKHR,
    );

    #[link_name = "vkCmdBuildAccelerationStructureIndirectKHR"]
    pub fn cmd_build_acceleration_structure_indirect_khr(
        command_buffer: CommandBuffer,
        p_info: *const AccelerationStructureBuildGeometryInfoKHR,
        indirect_buffer: Buffer,
        indirect_offset: DeviceSize,
        indirect_stride: u32,
    );

    #[link_name = "vkBuildAccelerationStructureKHR"]
    #[must_use]
    pub fn build_acceleration_structure_khr(
        device: Device,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
        pp_offset_infos: *const *const AccelerationStructureBuildOffsetInfoKHR,
    ) -> Result;

    #[link_name = "vkGetAccelerationStructureDeviceAddressKHR"]
    #[must_use]
    pub fn get_acceleration_structure_device_address_khr(
        device: Device,
        p_info: *const AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress;

    #[link_name = "vkCreateDeferredOperationKHR"]
    #[must_use]
    pub fn create_deferred_operation_khr(
        device: Device,
        p_allocator: *const AllocationCallbacks,
        p_deferred_operation: *mut DeferredOperationKHR,
    ) -> Result;

    #[link_name = "vkDestroyDeferredOperationKHR"]
    pub fn destroy_deferred_operation_khr(
        device: Device,
        operation: DeferredOperationKHR,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkGetDeferredOperationMaxConcurrencyKHR"]
    #[must_use]
    pub fn get_deferred_operation_max_concurrency_khr(
        device: Device,
        operation: DeferredOperationKHR,
    ) -> u32;

    #[link_name = "vkGetDeferredOperationResultKHR"]
    #[must_use]
    pub fn get_deferred_operation_result_khr(
        device: Device,
        operation: DeferredOperationKHR,
    ) -> Result;

    #[link_name = "vkDeferredOperationJoinKHR"]
    #[must_use]
    pub fn deferred_operation_join_khr(device: Device, operation: DeferredOperationKHR) -> Result;

    #[link_name = "vkCmdSetCullModeEXT"]
    pub fn cmd_set_cull_mode_ext(command_buffer: CommandBuffer, cull_mode: CullModeFlags);

    #[link_name = "vkCmdSetFrontFaceEXT"]
    pub fn cmd_set_front_face_ext(command_buffer: CommandBuffer, front_face: FrontFace);

    #[link_name = "vkCmdSetPrimitiveTopologyEXT"]
    pub fn cmd_set_primitive_topology_ext(
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    );

    #[link_name = "vkCmdSetViewportWithCountEXT"]
    pub fn cmd_set_viewport_with_count_ext(
        command_buffer: CommandBuffer,
        viewport_count: u32,
        p_viewports: *const Viewport,
    );

    #[link_name = "vkCmdSetScissorWithCountEXT"]
    pub fn cmd_set_scissor_with_count_ext(
        command_buffer: CommandBuffer,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    );

    #[link_name = "vkCmdBindVertexBuffers2EXT"]
    pub fn cmd_bind_vertex_buffers2_ext(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
        p_strides: *const DeviceSize,
    );

    #[link_name = "vkCmdSetDepthTestEnableEXT"]
    pub fn cmd_set_depth_test_enable_ext(command_buffer: CommandBuffer, depth_test_enable: Bool32);

    #[link_name = "vkCmdSetDepthWriteEnableEXT"]
    pub fn cmd_set_depth_write_enable_ext(
        command_buffer: CommandBuffer,
        depth_write_enable: Bool32,
    );

    #[link_name = "vkCmdSetDepthCompareOpEXT"]
    pub fn cmd_set_depth_compare_op_ext(command_buffer: CommandBuffer, depth_compare_op: CompareOp);

    #[link_name = "vkCmdSetDepthBoundsTestEnableEXT"]
    pub fn cmd_set_depth_bounds_test_enable_ext(
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: Bool32,
    );

    #[link_name = "vkCmdSetStencilTestEnableEXT"]
    pub fn cmd_set_stencil_test_enable_ext(
        command_buffer: CommandBuffer,
        stencil_test_enable: Bool32,
    );

    #[link_name = "vkCmdSetStencilOpEXT"]
    pub fn cmd_set_stencil_op_ext(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    );

    #[link_name = "vkCreatePrivateDataSlotEXT"]
    #[must_use]
    pub fn create_private_data_slot_ext(
        device: Device,
        p_create_info: *const PrivateDataSlotCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_private_data_slot: *mut PrivateDataSlotEXT,
    ) -> Result;

    #[link_name = "vkDestroyPrivateDataSlotEXT"]
    pub fn destroy_private_data_slot_ext(
        device: Device,
        private_data_slot: PrivateDataSlotEXT,
        p_allocator: *const AllocationCallbacks,
    );

    #[link_name = "vkSetPrivateDataEXT"]
    #[must_use]
    pub fn set_private_data_ext(
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlotEXT,
        data: u64,
    ) -> Result;

    #[link_name = "vkGetPrivateDataEXT"]
    pub fn get_private_data_ext(
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlotEXT,
        p_data: *mut u64,
    );

}

// Function pointers
pub type AllocationFunction =
    extern "system" fn(*mut c_void, usize, usize, SystemAllocationScope) -> *mut c_void;

pub type ReallocationFunction = extern "system" fn(
    *mut c_void,
    *mut c_void,
    usize,
    usize,
    SystemAllocationScope,
) -> *mut c_void;

pub type FreeFunction = extern "system" fn(*mut c_void, *mut c_void);

pub type InternalAllocationNotification = extern "system" fn(
    *mut c_void,
    usize,
    InternalAllocationType,
    SystemAllocationScope,
) -> *mut c_void;

pub type InternalFreeNotification = extern "system" fn(
    *mut c_void,
    usize,
    InternalAllocationType,
    SystemAllocationScope,
) -> *mut c_void;

pub type DebugUtilsMessengerCallbackEXT = extern "system" fn(
    DebugUtilsMessageSeverityFlagBitsEXT,
    DebugUtilsMessageTypeFlagsEXT,
    *const DebugUtilsMessengerCallbackDataEXT,
    *mut c_void,
) -> Bool32;

pub type VoidFunction = extern "system" fn() -> ();

// Unused types
pub type Display = c_void;
pub type VisualID = c_void;
pub type Window = u64;
pub type RROutput = c_void;
pub type wl_display = c_void;
pub type wl_surface = c_void;
pub type HINSTANCE = *const c_void;
pub type HWND = *const c_void;
pub type HMONITOR = c_void;
pub type HANDLE = c_void;
pub type SECURITY_ATTRIBUTES = c_void;
pub type DWORD = c_void;
pub type LPCWSTR = c_void;
pub type xcb_connection_t = c_void;
pub type xcb_visualid_t = c_void;
pub type xcb_window_t = c_void;
pub type IDirectFB = c_void;
pub type IDirectFBSurface = c_void;
pub type zx_handle_t = c_void;
pub type GgpStreamDescriptor = c_void;
pub type GgpFrameToken = c_void;
pub type ANativeWindow = c_void;
pub type AHardwareBuffer = c_void;
pub type CAMetalLayer = c_void;
pub type AccelerationStructureNV = c_void;
pub type BuildAccelerationStructureFlagsNV = c_void;
pub type AccelerationStructureTypeNV = c_void;
pub type AccelerationStructureMemoryRequirementsTypeNV = c_void;

// Manual structure types
pub const STRUCTURE_TYPE_PRESENT_INFO_KHR: u32 = 1000001001;
pub const STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: u32 = 1000001000;
pub const STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR: u32 = 1000004000;
pub const STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR: u32 = 1000009000;
pub const STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK: u32 = 1000123000;
pub const IMAGE_LAYOUT_PRESENT_SRC_KHR: u32 = 1000001002;

pub const ERROR_OUT_OF_DATE_KHR: i32 = -1000001004;

// Utils
pub fn check(result: Result) {
    match result {
        SUCCESS | NOT_READY | TIMEOUT | EVENT_SET | EVENT_RESET | INCOMPLETE => (),
        ERROR_OUT_OF_HOST_MEMORY => panic!("out of host memory"),
        ERROR_OUT_OF_DEVICE_MEMORY => panic!("out of device memory"),
        ERROR_INITIALIZATION_FAILED => panic!("initialization failed"),
        ERROR_DEVICE_LOST => panic!("device has been lost"),
        ERROR_MEMORY_MAP_FAILED => panic!("memory mapping failed"),
        ERROR_LAYER_NOT_PRESENT => panic!("layer not present"),
        ERROR_EXTENSION_NOT_PRESENT => panic!("extension not present"),
        ERROR_FEATURE_NOT_PRESENT => panic!("feature not present"),
        ERROR_INCOMPATIBLE_DRIVER => panic!("requested Vulkan version is not supported"),
        ERROR_TOO_MANY_OBJECTS => panic!("too many objects of type have been created"),
        ERROR_FORMAT_NOT_SUPPORTED => panic!("requested format is not supported"),
        ERROR_FRAGMENTED_POOL => panic!("pool allocation failed due to fragmentation"),
        n => panic!("unknowned error has occured: {}", n),
    }
}
