use std::ptr;

#[test]
fn test_link() {
    if std::env::var("DUMMY_CONDITION").is_ok() {
        // Should not be able to compile without linking to the FSR library
        unsafe {
            fsr_sys::ContextCreate(ptr::null_mut(), ptr::null());
            fsr_sys::vk::GetVkImageLayout(ptr::null_mut(), 0);
            fsr_sys::d3d12::GetDeviceDX12(ptr::null_mut());
        }
    }
}
