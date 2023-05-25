use crate::*;
use ash::vk::Handle;
use widestring::U16String;

impl From<ash::vk::CommandBuffer> for CommandList {
    fn from(value: ash::vk::CommandBuffer) -> Self {
        unsafe { CommandList(fsr2_sys::vk::GetCommandListVK(value.as_raw())) }
    }
}

pub unsafe fn get_scratch_memory_size(physical_device: ash::vk::PhysicalDevice) -> usize {
    fsr2_sys::vk::GetScratchMemorySizeVK(physical_device.as_raw())
}

pub unsafe fn get_interface(
    instance: ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
    scratch_buffer: &mut Vec<u8>,
) -> Interface {
    let mut interface = fsr2_sys::Interface::default();
    fsr2_sys::vk::GetInterfaceVK(
        &mut interface,
        scratch_buffer.as_mut_ptr().cast::<std::ffi::c_void>(),
        scratch_buffer.len(),
        physical_device.as_raw(),
        std::mem::transmute(Some(instance.fp_v1_0().get_device_proc_addr)),
    );
    interface
}

pub unsafe fn get_device(device: ash::vk::Device) -> Device {
    fsr2_sys::vk::GetDeviceVK(device.as_raw())
}

pub unsafe fn get_texture_resource(
    context: &mut Context,
    image: ash::vk::Image,
    image_view: ash::vk::ImageView,
    format: ash::vk::Format,
    size: [u32; 2],
    state: ResourceStates,
    name: &str,
) {
    fsr2_sys::vk::GetTextureResourceVK(
        &mut context.0,
        image.as_raw(),
        image_view.as_raw(),
        size[0],
        size[1],
        format.as_raw(),
        U16String::from_str(name).as_ptr(),
        state,
    );
}
