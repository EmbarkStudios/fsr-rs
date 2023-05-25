#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::excessive_precision)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "vulkan")]
pub mod vk {
    use crate::*;

    type VkPhysicalDevice = u64;
    type VkDevice = u64;
    type VkCommandBuffer = u64;
    type PFN_vkGetDeviceProcAddr = *const std::ffi::c_void;
    type VkBuffer = u64;
    type VkImage = u64;
    type VkImageView = u64;
    type VkFormat = i32;
    type VkImageLayout = i32;

    include!(concat!(env!("OUT_DIR"), "/vk_bindings.rs"));
}

#[cfg(feature = "d3d12")]
pub mod d3d12 {

    use crate::*;
    type ID3D12CommandList = std::ffi::c_void;
    type ID3D12Device = std::ffi::c_void;
    type ID3D12Resource = std::ffi::c_void;

    type UINT = u32;

    include!(concat!(env!("OUT_DIR"), "/d3d12_bindings.rs"));
}
