use crate::{interface::ScratchBuffer, *};
use widestring::U16String;
use windows::Win32::Graphics::Direct3D12::{ID3D12CommandList, ID3D12Device, ID3D12Resource};

impl From<&mut ID3D12CommandList> for CommandList {
    fn from(value: &mut ID3D12CommandList) -> Self {
        unsafe { CommandList(fsr_sys::d3d12::GetCommandListDX12(value as *mut _ as _)) }
    }
}

unsafe fn get_scratch_memory_size() -> usize {
    fsr_sys::d3d12::GetScratchMemorySizeDX12()
}

pub unsafe fn get_interface(device: &mut ID3D12Device) -> Result<Interface, Error> {
    let scratch_buffer =
        ScratchBuffer::new(get_scratch_memory_size()).map_err(|e| Error::ScratchBuffer(e))?;

    let mut retval = Interface {
        interface: fsr_sys::Interface::default(),
        scratch_buffer,
    };

    fsr_sys::d3d12::GetInterfaceDX12(
        &mut retval.interface,
        device as *mut _ as _,
        retval.scratch_buffer.ptr().cast::<std::ffi::c_void>(),
        retval.scratch_buffer.len(),
    );

    Ok(retval)
}

pub unsafe fn get_device(device: &mut ID3D12Device) -> Device {
    fsr_sys::d3d12::GetDeviceDX12(device as *mut _ as _)
}

pub unsafe fn get_texture_resource(
    context: &mut Context,
    resource: &mut ID3D12Resource,
    state: ResourceStates,
    name: &str,
    shader_component_mapping: u32,
) {
    fsr_sys::d3d12::GetResourceDX12(
        &mut context.context,
        resource as *mut _ as _,
        U16String::from_str(name).as_ptr(),
        state,
        shader_component_mapping,
    );
}
