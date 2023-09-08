use crate::*;
use widestring::U16String;
use windows::Win32::Graphics::Direct3D12::{ID3D12CommandList, ID3D12Device, ID3D12Resource};

impl From<&mut ID3D12CommandList> for CommandList {
    fn from(value: &mut ID3D12CommandList) -> Self {
        unsafe { CommandList(fsr_sys::d3d12::GetCommandListDX12(value as *mut _ as _)) }
    }
}

pub unsafe fn get_scratch_memory_size() -> usize {
    fsr_sys::d3d12::GetScratchMemorySizeDX12()
}

pub unsafe fn get_interface(device: &mut ID3D12Device, scratch_buffer: &mut Vec<u8>) -> Interface {
    let mut interface = fsr_sys::Interface::default();
    fsr_sys::d3d12::GetInterfaceDX12(
        &mut interface,
        device as *mut _ as _,
        scratch_buffer.as_mut_ptr().cast::<std::ffi::c_void>(),
        scratch_buffer.len(),
    );
    interface
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
        &mut context.0,
        resource as *mut _ as _,
        U16String::from_str(name).as_ptr(),
        state,
        shader_component_mapping,
    );
}
