//! Unsafe Rust bindings for [FidelityFX Super Resolution 2](https://github.com/GPUOpen-Effects/FidelityFX-FSR2)
//!
//!
//! # Vulkan psuedo code
//! ```no_run
//! // Create the FSR interface
//! // The context created below should not out live the scratch buffer.
//! let mut scratch_buffer =
//!     vec![0u8; fsr2::vk::get_scratch_memory_size(&vk_instance, vk_physical_device)];
//! let interface = fsr2::vk::get_interface(
//!     &vk_entry,
//!     &vk_instance,
//!     vk_physical_device,
//!     &mut scratch_buffer,
//! ).unwrap();
//!
//! // Create the FSR context
//! let context_desc = fsr2::ContextDescription {
//!     interface: &fsr_interface,
//!     device: &fsr2::vk::get_device(vk_device),
//!     display_size: [1920, 1080],
//!     max_render_size: [1280, 720],
//!     flags: fsr2::InitializationFlagBits::ENABLE_HIGH_DYNAMIC_RANGE
//!     message_callback: None,
//! };
//! let context = fsr2::Context::new(context_desc).unwrap();
//!
//! // Dispatch gpu work
//! let desc = fsr2::DispatchDescription::new(
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

pub use fsr2_sys::Device;
pub use fsr2_sys::Interface;
pub use fsr2_sys::MsgType;
pub use fsr2_sys::Resource;
pub use fsr2_sys::ResourceStates;

/// A typedef representing a command list or command buffer.
pub struct CommandList(fsr2_sys::CommandList);
/// A structure encapsulating the FidelityFX Super Resolution 2 context.
pub struct Context(fsr2_sys::Context);

#[repr(i32)]
#[derive(Debug)]
pub enum Error {
    InvalidPointer = fsr2_sys::FFX_ERROR_INVALID_POINTER,
    InvalidAlignment = fsr2_sys::FFX_ERROR_INVALID_ALIGNMENT,
    InvalidSize = fsr2_sys::FFX_ERROR_INVALID_SIZE,
    Eof = fsr2_sys::FFX_ERROR_EOF,
    InvalidPath = fsr2_sys::FFX_ERROR_INVALID_PATH,
    MalfmoredData = fsr2_sys::FFX_ERROR_MALFORMED_DATA,
    OutOfMemory = fsr2_sys::FFX_ERROR_OUT_OF_MEMORY,
    IncompleteInterface = fsr2_sys::FFX_ERROR_INCOMPLETE_INTERFACE,
    InvalidEnum = fsr2_sys::FFX_ERROR_INVALID_ENUM,
    InvalidArgument = fsr2_sys::FFX_ERROR_INVALID_ARGUMENT,
    OutOfRange = fsr2_sys::FFX_ERROR_OUT_OF_RANGE,
    NullDevice = fsr2_sys::FFX_ERROR_NULL_DEVICE,
    BackendApiError = fsr2_sys::FFX_ERROR_BACKEND_API_ERROR,
    InsufficientMemory = fsr2_sys::FFX_ERROR_INSUFFICIENT_MEMORY,
    Unknown = 0,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl std::error::Error for Error {}

impl Error {
    fn from_error_code(value: fsr2_sys::ErrorCode) -> Self {
        match value {
            fsr2_sys::FFX_ERROR_INVALID_POINTER => Self::InvalidPointer,
            fsr2_sys::FFX_ERROR_INVALID_ALIGNMENT => Self::InvalidAlignment,
            fsr2_sys::FFX_ERROR_INVALID_SIZE => Self::InvalidSize,
            fsr2_sys::FFX_ERROR_EOF => Self::Eof,
            fsr2_sys::FFX_ERROR_INVALID_PATH => Self::InvalidPath,
            fsr2_sys::FFX_ERROR_MALFORMED_DATA => Self::MalfmoredData,
            fsr2_sys::FFX_ERROR_OUT_OF_MEMORY => Self::OutOfMemory,
            fsr2_sys::FFX_ERROR_INCOMPLETE_INTERFACE => Self::IncompleteInterface,
            fsr2_sys::FFX_ERROR_INVALID_ENUM => Self::InvalidEnum,
            fsr2_sys::FFX_ERROR_INVALID_ARGUMENT => Self::InvalidArgument,
            fsr2_sys::FFX_ERROR_OUT_OF_RANGE => Self::OutOfRange,
            fsr2_sys::FFX_ERROR_NULL_DEVICE => Self::NullDevice,
            fsr2_sys::FFX_ERROR_BACKEND_API_ERROR => Self::BackendApiError,
            fsr2_sys::FFX_ERROR_INSUFFICIENT_MEMORY => Self::InsufficientMemory,
            _ => Self::Unknown,
        }
    }
}

pub struct ContextDescription<'a> {
    pub interface: &'a Interface,
    pub flags: InitializationFlagBits,
    pub max_render_size: [u32; 2],
    pub display_size: [u32; 2],
    pub device: &'a Device,
    pub message_callback: Option<unsafe extern "C" fn(i32, *const u32)>,
}

impl From<ContextDescription<'_>> for fsr2_sys::ContextDescription {
    fn from(val: ContextDescription<'_>) -> Self {
        fsr2_sys::ContextDescription {
            callbacks: *val.interface,
            flags: val.flags.bits,
            maxRenderSize: fsr2_sys::Dimensions2D {
                width: val.max_render_size[0],
                height: val.max_render_size[1],
            },
            displaySize: fsr2_sys::Dimensions2D {
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
        const ENABLE_DEBUG_CHECKING = fsr2_sys::FFX_FSR2_ENABLE_DEBUG_CHECKING as u32;
        const ENABLE_DEPTH_INFINITE = fsr2_sys::FFX_FSR2_ENABLE_DEPTH_INFINITE as u32;
        const ENABLE_DEPTH_INVERTED = fsr2_sys::FFX_FSR2_ENABLE_DEPTH_INVERTED as u32;
        const ENABLE_DISPLAY_RESOLUTION_MOTION_VECTORS = fsr2_sys::FFX_FSR2_ENABLE_DISPLAY_RESOLUTION_MOTION_VECTORS as u32;
        const ENABLE_DYNAMIC_RESOLUTION = fsr2_sys::FFX_FSR2_ENABLE_DYNAMIC_RESOLUTION as u32;
        const ENABLE_HIGH_DYNAMIC_RANGE = fsr2_sys::FFX_FSR2_ENABLE_HIGH_DYNAMIC_RANGE as u32;
        const ENABLE_MOTION_VECTORS_JITTER_CANCELLATION = fsr2_sys::FFX_FSR2_ENABLE_MOTION_VECTORS_JITTER_CANCELLATION as u32;
        const ENABLE_TEXTURE1D_USAGE = fsr2_sys::FFX_FSR2_ENABLE_TEXTURE1D_USAGE as u32;
        const ENABLE_AUTO_EXPOSURE = fsr2_sys::FFX_FSR2_ENABLE_AUTO_EXPOSURE as u32;
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

    pub jitter_offfset: [f32; 2],

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
            jitter_offfset: [0.0, 0.0],
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

    pub fn jitter_offfset(mut self, value: [f32; 2]) -> DispatchDescription {
        self.jitter_offfset = value;
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

impl From<DispatchDescription> for fsr2_sys::DispatchDescription {
    fn from(val: DispatchDescription) -> Self {
        fsr2_sys::DispatchDescription {
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
            renderSize: fsr2_sys::Dimensions2D {
                width: val.render_size[0],
                height: val.render_size[1],
            },
            frameTimeDelta: val.frame_time_delta,
            motionVectorScale: fsr2_sys::FloatCoords2D {
                x: val.motion_vector_scale[0],
                y: val.motion_vector_scale[1],
            },
            preExposure: val.pre_exposure,
            jitterOffset: fsr2_sys::FloatCoords2D {
                x: val.jitter_offfset[0],
                y: val.jitter_offfset[1],
            },
            cameraFovAngleVertical: val.camera_fov_y,
            sharpness: val.sharpness,
            reset: val.reset,
        }
    }
}

impl Context {
    pub fn new(desc: ContextDescription<'_>) -> Result<Self, Error> {
        let mut context = fsr2_sys::Context::default();
        unsafe {
            let error = fsr2_sys::ContextCreate(&mut context, &desc.into());
            if error != fsr2_sys::FFX_OK {
                return Err(Error::from_error_code(error));
            }
        }
        Ok(Context(context))
    }

    pub unsafe fn dispatch(&mut self, desc: DispatchDescription) -> Result<(), Error> {
        let error = unsafe { fsr2_sys::ContextDispatch(&mut self.0, &desc.into()) };
        if error != fsr2_sys::FFX_OK {
            return Err(Error::from_error_code(error));
        }
        Ok(())
    }

    pub unsafe fn destroy(&mut self) -> Result<(), Error> {
        let error = unsafe { fsr2_sys::ContextDestroy(&mut self.0) };
        if error != fsr2_sys::FFX_OK {
            return Err(Error::from_error_code(error));
        }
        Ok(())
    }
}
