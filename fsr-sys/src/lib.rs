#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::excessive_precision)]

include!("bindings.rs");

#[cfg(feature = "vulkan")]
pub mod vk {
    use super::*;
    use ash::vk::{
        Buffer as VkBuffer, CommandBuffer as VkCommandBuffer, Device as VkDevice,
        Format as VkFormat, Image as VkImage, ImageLayout as VkImageLayout,
        ImageView as VkImageView, PFN_vkGetDeviceProcAddr, PhysicalDevice as VkPhysicalDevice,
    };

    include!("vk_bindings.rs");
}

#[cfg(feature = "d3d12")]
pub mod d3d12 {
    use super::*;

    type ID3D12CommandList = std::ffi::c_void;
    type ID3D12Device = std::ffi::c_void;
    type ID3D12Resource = std::ffi::c_void;

    type UINT = u32;

    include!("d3d12_bindings.rs");
}
