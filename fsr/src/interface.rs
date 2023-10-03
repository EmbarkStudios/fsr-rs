use std::alloc::{Layout, LayoutError};

pub struct ScratchBuffer {
    ptr: *mut u8,
    layout: Layout,
}

impl ScratchBuffer {
    pub fn new(size: usize) -> Result<Self, LayoutError> {
        if size == 0 {
            panic!("non zero size required");
        }
        let layout = Layout::from_size_align(size, 8)?;
        // SAFETY: we ensured size is nonzero above
        let ptr = unsafe { std::alloc::alloc(layout) };
        Ok(Self { ptr, layout })
    }

    /// # Safety
    ///
    /// You must ensure that nothing is using the pointer when you drop this scratch buffer, as well
    /// as ensure standard aliasing guarantees when used within Rust. (mostly you shouldn't need to
    /// use this as a user, the library will do this internaly)
    pub unsafe fn ptr(&mut self) -> *mut u8 {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.layout.size()
    }
}

unsafe impl Send for ScratchBuffer {}
unsafe impl Sync for ScratchBuffer {}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        // SAFETY: it's up to anyone who used `ptr` to guarantee nothing is using our data at this point.
        // we know we allocated ptr with the layout.
        unsafe {
            std::alloc::dealloc(self.ptr, self.layout);
        }
    }
}

pub struct Interface {
    // field order matters, we drop in reverse field order so want to drop the interface before the
    // scratch buffer which the interface is using.
    pub interface: fsr_sys::Interface,
    pub scratch_buffer: ScratchBuffer,
}

unsafe impl Send for Interface {}
unsafe impl Sync for Interface {}
