//! Unsafe Rust bindings for [FidelityFX Super Resolution 2](https://github.com/GPUOpen-Effects/FidelityFX-FSR2)
//!
//!
//! # Vulkan psuedo code
//! ```no_run
//! // Create the FSR interface
//! // The context created below should not out live the scratch buffer.
//! let mut scratch_buffer =
//!     vec![0u8; fsr::vk::get_scratch_memory_size(&vk_instance, vk_physical_device)];
//! let interface = fsr::vk::get_interface(
//!     &vk_entry,
//!     &vk_instance,
//!     vk_physical_device,
//!     &mut scratch_buffer,
//! ).unwrap();
//!
//! // Create the FSR context
//! let context_desc = fsr::ContextDescription {
//!     interface: fsr_interface,
//!     device: &fsr::vk::get_device(vk_device),
//!     display_size: [1920, 1080],
//!     max_render_size: [1280, 720],
//!     flags: fsr::InitializationFlagBits::ENABLE_HIGH_DYNAMIC_RANGE
//!     message_callback: None,
//! };
//! let context = fsr::Context::new(context_desc).unwrap();
//!
//! // Dispatch gpu work
//! let desc = fsr::DispatchDescription::new(
//!     vk_command_list.into(),
//!     color,
//!     depth,
//!     velocity,
//!     output,
//!     delta_time_s,
//!     [1280, 720],
//! );
//! fsr_context.dispatch(desc).expect("Failed to dispatch fsr");
//! ```

#[cfg(feature = "d3d12")]
pub mod d3d12;
#[cfg(feature = "vulkan")]
pub mod vk;

pub mod interface;
pub use crate::interface::Interface;

pub use fsr_sys::Device;
pub use fsr_sys::MsgType;
pub use fsr_sys::Resource;
pub use fsr_sys::ResourceStates;

/// A typedef representing a command list or command buffer.
pub struct CommandList(fsr_sys::CommandList);
/// A structure encapsulating the FidelityFX Super Resolution 2 context.
pub struct Context {
    pub(crate) context: Box<fsr_sys::Context>, // `fsr_sys::Context` is rather large to live on the stack.
    _interface: Interface,
}

#[repr(i32)]
#[derive(Debug)]
pub enum FsrError {
    InvalidPointer = fsr_sys::ERROR_INVALID_POINTER,
    InvalidAlignment = fsr_sys::ERROR_INVALID_ALIGNMENT,
    InvalidSize = fsr_sys::ERROR_INVALID_SIZE,
    Eof = fsr_sys::ERROR_EOF,
    InvalidPath = fsr_sys::ERROR_INVALID_PATH,
    MalfmoredData = fsr_sys::ERROR_MALFORMED_DATA,
    OutOfMemory = fsr_sys::ERROR_OUT_OF_MEMORY,
    IncompleteInterface = fsr_sys::ERROR_INCOMPLETE_INTERFACE,
    InvalidEnum = fsr_sys::ERROR_INVALID_ENUM,
    InvalidArgument = fsr_sys::ERROR_INVALID_ARGUMENT,
    OutOfRange = fsr_sys::ERROR_OUT_OF_RANGE,
    NullDevice = fsr_sys::ERROR_NULL_DEVICE,
    BackendApiError = fsr_sys::ERROR_BACKEND_API_ERROR,
    InsufficientMemory = fsr_sys::ERROR_INSUFFICIENT_MEMORY,
    Unknown = 0,
}

impl std::fmt::Display for FsrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl std::error::Error for FsrError {}

impl FsrError {
    fn from_error_code(value: fsr_sys::ErrorCode) -> Self {
        match value {
            fsr_sys::ERROR_INVALID_POINTER => Self::InvalidPointer,
            fsr_sys::ERROR_INVALID_ALIGNMENT => Self::InvalidAlignment,
            fsr_sys::ERROR_INVALID_SIZE => Self::InvalidSize,
            fsr_sys::ERROR_EOF => Self::Eof,
            fsr_sys::ERROR_INVALID_PATH => Self::InvalidPath,
            fsr_sys::ERROR_MALFORMED_DATA => Self::MalfmoredData,
            fsr_sys::ERROR_OUT_OF_MEMORY => Self::OutOfMemory,
            fsr_sys::ERROR_INCOMPLETE_INTERFACE => Self::IncompleteInterface,
            fsr_sys::ERROR_INVALID_ENUM => Self::InvalidEnum,
            fsr_sys::ERROR_INVALID_ARGUMENT => Self::InvalidArgument,
            fsr_sys::ERROR_OUT_OF_RANGE => Self::OutOfRange,
            fsr_sys::ERROR_NULL_DEVICE => Self::NullDevice,
            fsr_sys::ERROR_BACKEND_API_ERROR => Self::BackendApiError,
            fsr_sys::ERROR_INSUFFICIENT_MEMORY => Self::InsufficientMemory,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to allocate scratch buffer")]
    ScratchBuffer(#[source] std::alloc::LayoutError),

    #[error("Failed compiling module dependency")] // TODO: show which
    Fsr(#[source] FsrError),
}

pub struct ContextDescription<'a> {
    pub interface: Interface,
    pub flags: InitializationFlagBits,
    pub max_render_size: [u32; 2],
    pub display_size: [u32; 2],
    pub device: &'a Device,
    pub message_callback: Option<unsafe extern "C" fn(MsgType, *const widestring::WideChar)>,
}

impl From<&ContextDescription<'_>> for fsr_sys::ContextDescription {
    fn from(val: &ContextDescription<'_>) -> Self {
        fsr_sys::ContextDescription {
            callbacks: val.interface.interface,
            flags: val.flags.bits(),
            maxRenderSize: fsr_sys::Dimensions2D {
                width: val.max_render_size[0],
                height: val.max_render_size[1],
            },
            displaySize: fsr_sys::Dimensions2D {
                width: val.display_size[0],
                height: val.display_size[1],
            },
            device: *val.device,
            fpMessage: val.message_callback,
        }
    }
}

bitflags::bitflags! {
    pub struct InitializationFlagBits: u32 {
        const ENABLE_DEBUG_CHECKING = fsr_sys::ENABLE_DEBUG_CHECKING as u32;
        const ENABLE_DEPTH_INFINITE = fsr_sys::ENABLE_DEPTH_INFINITE as u32;
        const ENABLE_DEPTH_INVERTED = fsr_sys::ENABLE_DEPTH_INVERTED as u32;
        const ENABLE_DISPLAY_RESOLUTION_MOTION_VECTORS = fsr_sys::ENABLE_DISPLAY_RESOLUTION_MOTION_VECTORS as u32;
        const ENABLE_DYNAMIC_RESOLUTION = fsr_sys::ENABLE_DYNAMIC_RESOLUTION as u32;
        const ENABLE_HIGH_DYNAMIC_RANGE = fsr_sys::ENABLE_HIGH_DYNAMIC_RANGE as u32;
        const ENABLE_MOTION_VECTORS_JITTER_CANCELLATION = fsr_sys::ENABLE_MOTION_VECTORS_JITTER_CANCELLATION as u32;
        const ENABLE_TEXTURE1D_USAGE = fsr_sys::ENABLE_TEXTURE1D_USAGE as u32;
        const ENABLE_AUTO_EXPOSURE = fsr_sys::ENABLE_AUTO_EXPOSURE as u32;
    }
}

pub struct DispatchDescription {
    pub cmd_list: CommandList,
    pub output: Resource,

    pub color: Resource,
    pub depth: Resource,
    pub motion_vectors: Resource,
    pub color_opaque_only: Option<Resource>,
    pub exposure: Option<Resource>,
    pub reactive: Option<Resource>,
    pub transparency_and_composition: Option<Resource>,

    pub enable_auto_reactive: bool,
    pub enable_sharpening: bool,

    pub auto_reactive_max: f32,
    pub auto_tc_scale: f32,
    pub auto_reactive_scale: f32,
    pub auto_tc_threshold: f32,

    pub motion_vector_scale: [f32; 2],

    pub pre_exposure: f32,
    pub frame_time_delta: f32,

    pub jitter_offset: [f32; 2],

    pub render_size: [u32; 2],

    pub camera_near: f32,
    pub camera_far: f32,
    pub camera_fov_y: f32,

    pub sharpness: f32,

    pub view_space_to_meters_factor: f32,

    pub reset: bool,
}

impl DispatchDescription {
    pub fn new(
        cmd_list: CommandList,
        color: Resource,
        depth: Resource,
        motion_vectors: Resource,
        output: Resource,
        frame_time_delta: f32,
        render_size: [u32; 2],
    ) -> Self {
        Self {
            cmd_list,
            color,
            color_opaque_only: None,
            depth,
            exposure: None,
            motion_vectors,
            reactive: None,
            transparency_and_composition: None,
            output,
            enable_auto_reactive: false,
            enable_sharpening: false,
            auto_reactive_max: 0.0,
            auto_tc_scale: 1.0,
            auto_reactive_scale: 1.0,
            auto_tc_threshold: 0.0,
            motion_vector_scale: [1.0, 1.0],
            pre_exposure: 1.0,
            frame_time_delta,
            jitter_offset: [0.0, 0.0],
            render_size,
            camera_near: 0.01,
            camera_far: 1000.0,
            camera_fov_y: 1.0,
            sharpness: 0.0,
            view_space_to_meters_factor: 1.0,
            reset: false,
        }
    }

    pub fn camera(mut self, near: f32, far: f32, fov_y: f32) -> DispatchDescription {
        self.camera_near = near;
        self.camera_far = far;
        self.camera_fov_y = fov_y;
        self
    }

    pub fn pre_exposure(mut self, value: f32) -> DispatchDescription {
        self.pre_exposure = value;
        self
    }

    pub fn view_space_to_meters_factor(mut self, value: f32) -> DispatchDescription {
        self.view_space_to_meters_factor = value;
        self
    }

    pub fn exposure(mut self, resource: Resource) -> DispatchDescription {
        self.exposure = Some(resource);
        self
    }

    pub fn reactive(mut self, resource: Resource) -> DispatchDescription {
        self.reactive = Some(resource);
        self
    }

    pub fn motion_vector_scale(mut self, value: [f32; 2]) -> DispatchDescription {
        self.motion_vector_scale = value;
        self
    }

    pub fn jitter_offset(mut self, value: [f32; 2]) -> DispatchDescription {
        self.jitter_offset = value;
        self
    }

    pub fn sharpness(mut self, sharpness: f32) -> DispatchDescription {
        self.enable_sharpening = true;
        self.sharpness = sharpness;
        self
    }

    pub fn auto_reactive(
        mut self,
        color_opaque_only: Resource,
        transparency_and_composition: Resource,
        auto_reactive_max: f32,
        auto_tc_scale: f32,
        auto_reactive_scale: f32,
        auto_tc_threshold: f32,
    ) -> DispatchDescription {
        self.color_opaque_only = Some(color_opaque_only);
        self.transparency_and_composition = Some(transparency_and_composition);
        self.enable_auto_reactive = true;
        self.auto_reactive_max = auto_reactive_max;
        self.auto_tc_scale = auto_tc_scale;
        self.auto_reactive_scale = auto_reactive_scale;
        self.auto_tc_threshold = auto_tc_threshold;
        self
    }

    pub fn reset(mut self, value: bool) -> DispatchDescription {
        self.reset = value;
        self
    }
}

impl From<DispatchDescription> for fsr_sys::DispatchDescription {
    fn from(val: DispatchDescription) -> Self {
        fsr_sys::DispatchDescription {
            commandList: val.cmd_list.0,
            output: val.output,
            color: val.color,
            transparencyAndComposition: val.transparency_and_composition.unwrap_or_default(),
            colorOpaqueOnly: val.color_opaque_only.unwrap_or_default(),
            depth: val.depth,
            exposure: val.exposure.unwrap_or_default(),
            reactive: val.reactive.unwrap_or_default(),
            motionVectors: val.motion_vectors,
            autoReactiveMax: val.auto_reactive_max,
            autoTcScale: val.auto_tc_scale,
            enableSharpening: val.enable_sharpening,
            enableAutoReactive: val.enable_auto_reactive,
            autoReactiveScale: val.auto_reactive_scale,
            autoTcThreshold: val.auto_tc_threshold,
            cameraNear: val.camera_near,
            cameraFar: val.camera_far,
            viewSpaceToMetersFactor: val.view_space_to_meters_factor,
            renderSize: fsr_sys::Dimensions2D {
                width: val.render_size[0],
                height: val.render_size[1],
            },
            frameTimeDelta: val.frame_time_delta,
            motionVectorScale: fsr_sys::FloatCoords2D {
                x: val.motion_vector_scale[0],
                y: val.motion_vector_scale[1],
            },
            preExposure: val.pre_exposure,
            jitterOffset: fsr_sys::FloatCoords2D {
                x: val.jitter_offset[0],
                y: val.jitter_offset[1],
            },
            cameraFovAngleVertical: val.camera_fov_y,
            sharpness: val.sharpness,
            reset: val.reset,
        }
    }
}

impl Context {
    pub unsafe fn new(desc: ContextDescription<'_>) -> Result<Self, Error> {
        let mut context = Box::<fsr_sys::Context>::default();
        unsafe {
            let error = fsr_sys::ContextCreate(context.as_mut(), &(&desc).into());
            if error != fsr_sys::OK {
                return Err(Error::Fsr(FsrError::from_error_code(error)));
            }
        }
        Ok(Context {
            context,
            _interface: desc.interface,
        })
    }

    pub unsafe fn dispatch(&mut self, desc: DispatchDescription) -> Result<(), Error> {
        let error = unsafe { fsr_sys::ContextDispatch(self.context.as_mut(), &desc.into()) };
        if error != fsr_sys::OK {
            return Err(Error::Fsr(FsrError::from_error_code(error)));
        }
        Ok(())
    }

    pub unsafe fn destroy(&mut self) -> Result<(), Error> {
        let error = unsafe { fsr_sys::ContextDestroy(self.context.as_mut()) };
        if error != fsr_sys::OK {
            return Err(Error::Fsr(FsrError::from_error_code(error)));
        }
        Ok(())
    }
}
