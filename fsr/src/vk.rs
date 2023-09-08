use crate::*;
use ash::vk::Handle;
use widestring::WideString;

impl From<ash::vk::CommandBuffer> for CommandList {
    fn from(value: ash::vk::CommandBuffer) -> Self {
        unsafe { CommandList(fsr_sys::vk::GetCommandListVK(value.as_raw())) }
    }
}

pub unsafe fn get_scratch_memory_size(
    instance: &ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
) -> usize {
    unsafe {
        fsr_sys::vk::GetScratchMemorySizeVK(
            physical_device.as_raw(),
            std::mem::transmute(Some(
                instance.fp_v1_0().enumerate_device_extension_properties,
            )),
        )
    }
}

pub unsafe fn get_interface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
    scratch_buffer: &mut Vec<u8>,
) -> Result<Interface, Error> {
    let mut interface = fsr_sys::Interface::default();
    let error = unsafe {
        fsr_sys::vk::GetInterfaceVK(
            &mut interface,
            scratch_buffer.as_mut_ptr().cast::<std::ffi::c_void>(),
            scratch_buffer.len(),
            instance.handle().as_raw(),
            physical_device.as_raw(),
            std::mem::transmute(Some(entry.static_fn().get_instance_proc_addr)),
            std::mem::transmute(Some(instance.fp_v1_0().get_device_proc_addr)),
        )
    };
    if error != fsr_sys::FFX_OK {
        return Err(Error::from_error_code(error));
    }
    Ok(interface)
}

pub unsafe fn get_device(device: ash::Device) -> Device {
    unsafe { fsr_sys::vk::GetDeviceVK(device.handle().as_raw()) }
}

pub unsafe fn get_texture_resource(
    context: &mut Context,
    image: ash::vk::Image,
    image_view: ash::vk::ImageView,
    format: ash::vk::Format,
    size: [u32; 2],
    state: ResourceStates,
    name: &str,
) -> Resource {
    unsafe {
        fsr_sys::vk::GetTextureResourceVK(
            &mut context.0,
            image.as_raw(),
            image_view.as_raw(),
            size[0],
            size[1],
            format.as_raw(),
            WideString::from_str(name).as_ptr(),
            state,
        )
    }
}
