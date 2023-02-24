#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(unused)]

// Replace vulkan type's with ash.
mod root {
    type VkPhysicalDevice = ash::vk::PhysicalDevice;
    type VkDevice = ash::vk::Device;
    type VkCommandBuffer = ash::vk::CommandBuffer;
    type PFN_vkGetDeviceProcAddr = Option<ash::vk::PFN_vkGetDeviceProcAddr>;
    type VkBuffer = ash::vk::Buffer;
    type VkImage = ash::vk::Image;
    type VkImageView = ash::vk::ImageView;
    type VkFormat = ash::vk::Format;
    type VkImageLayout = ash::vk::ImageLayout;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// enums
pub use crate::root::ResourceStates;
// structs
pub use crate::root::Context;
pub use crate::root::ContextDescription;
pub use crate::root::Device;
pub use crate::root::Dimensions2D;
pub use crate::root::DispatchDescription;
pub use crate::root::FloatCoords2D;
pub use crate::root::Interface;
pub use crate::root::MsgType;
pub use crate::root::Resource;
// functions
pub use crate::root::ContextCreate;
pub use crate::root::ContextDestroy;
pub use crate::root::ContextDispatch;
pub use crate::root::GetJitterOffset;
pub use crate::root::GetJitterPhaseCount;

/// Vulkan specific fsr functions
pub mod vk {
    // structs
    pub use crate::root::Interface;
    // functions
    pub use crate::root::GetCommandList;
    pub use crate::root::GetDevice;
    pub use crate::root::GetInterface;
    pub use crate::root::GetScratchMemorySize;
    pub use crate::root::GetTextureResource;
}

impl From<ash::vk::CommandBuffer> for root::CommandList {
    fn from(value: ash::vk::CommandBuffer) -> Self {
        unsafe { vk::GetCommandList(value) }
    }
}

bitflags::bitflags! {
    pub struct InitializationFlagBits: u32 {
        const ENABLE_DEBUG_CHECKING = root::FFX_FSR2_ENABLE_DEBUG_CHECKING as u32;
        const ENABLE_DEPTH_INFINITE = root::FFX_FSR2_ENABLE_DEPTH_INFINITE as u32;
        const ENABLE_DEPTH_INVERTED = root::FFX_FSR2_ENABLE_DEPTH_INVERTED as u32;
        const ENABLE_DISPLAY_RESOLUTION_MOTION_VECTORS = root::FFX_FSR2_ENABLE_DISPLAY_RESOLUTION_MOTION_VECTORS as u32;
        const ENABLE_DYNAMIC_RESOLUTION = root::FFX_FSR2_ENABLE_DYNAMIC_RESOLUTION as u32;
        const ENABLE_HIGH_DYNAMIC_RANGE = root::FFX_FSR2_ENABLE_HIGH_DYNAMIC_RANGE as u32;
        const ENABLE_MOTION_VECTORS_JITTER_CANCELLATION = root::FFX_FSR2_ENABLE_MOTION_VECTORS_JITTER_CANCELLATION as u32;
        const ENABLE_TEXTURE1D_USAGE = root::FFX_FSR2_ENABLE_TEXTURE1D_USAGE as u32;
        const ENABLE_AUTO_EXPOSURE = root::FFX_FSR2_ENABLE_AUTO_EXPOSURE as u32;
    }
}
