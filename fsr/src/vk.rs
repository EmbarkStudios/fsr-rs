use crate::{interface::ScratchBuffer, *};
use ash::vk::Handle;
use widestring::WideString;

impl From<ash::vk::CommandBuffer> for CommandList {
    fn from(value: ash::vk::CommandBuffer) -> Self {
        unsafe { CommandList(fsr_sys::vk::GetCommandListVK(value.as_raw())) }
    }
}

pub unsafe fn get_interface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
) -> Result<Interface, Error> {
    let scratch_buffer_size = unsafe {
        fsr_sys::vk::GetScratchMemorySizeVK(
            physical_device.as_raw(),
            std::mem::transmute(Some(
                instance.fp_v1_0().enumerate_device_extension_properties,
            )),
        )
    };

    let mut retval = Interface {
        scratch_buffer: ScratchBuffer::new(scratch_buffer_size)
            .map_err(|e| Error::ScratchBuffer(e))?,
        interface: Default::default(),
    };

    // Create the actual fsr interface
    let error = unsafe {
        fsr_sys::vk::GetInterfaceVK(
            &mut retval.interface,
            retval.scratch_buffer.ptr().cast::<std::ffi::c_void>(),
            retval.scratch_buffer.len(),
            instance.handle().as_raw(),
            physical_device.as_raw(),
            std::mem::transmute(Some(entry.static_fn().get_instance_proc_addr)),
            std::mem::transmute(Some(instance.fp_v1_0().get_device_proc_addr)),
        )
    };
    if error != fsr_sys::FFX_OK {
        return Err(Error::Fsr(FsrError::from_error_code(error)));
    }

    Ok(retval)
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
            &mut context.context,
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
