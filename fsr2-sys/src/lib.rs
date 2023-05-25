#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(unused)]

// Replace vulkan type's with ash.
mod root {
    type VkPhysicalDevice = u64;
    type VkDevice = u64;
    type VkCommandBuffer = u64;
    type PFN_vkGetDeviceProcAddr = Option<ash::vk::PFN_vkGetDeviceProcAddr>;
    type VkBuffer = u64;
    type VkImage = u64;
    type VkImageView = u64;
    type VkFormat = i32;
    type VkImageLayout = i32;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use root::*;
