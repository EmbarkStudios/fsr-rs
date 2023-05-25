use ash::vk::Handle;

pub struct CommandList(fsr2_sys::CommandList);
pub struct Device(fsr2_sys::Device);
pub struct Interface(fsr2_sys::Interface);
pub struct Context(fsr2_sys::Context);
pub struct Resource(fsr2_sys::Resource);

pub mod vk;

pub use fsr2_sys::ResourceStates;

pub struct ContextDescription<'a> {
    interface: &'a Interface,
    flags: InitializationFlagBits,
    max_render_size: [u32; 2],
    display_size: [u32; 2],
    device: &'a Device,
    message_callback: Option<unsafe extern "C" fn(i32, *const u16)>,
}

impl Into<fsr2_sys::ContextDescription> for ContextDescription<'_> {
    fn into(self) -> fsr2_sys::ContextDescription {
        fsr2_sys::ContextDescription {
            callbacks: self.interface.0,
            flags: self.flags.bits,
            maxRenderSize: fsr2_sys::Dimensions2D {
                width: self.max_render_size[0],
                height: self.max_render_size[1],
            },
            displaySize: fsr2_sys::Dimensions2D {
                width: self.display_size[0],
                height: self.display_size[1],
            },
            device: self.device.0,
            fpMessage: self.message_callback,
        }
    }
}

pub fn create_context(desc: ContextDescription) -> Context {
    let mut context = fsr2_sys::Context::default();
    unsafe {
        fsr2_sys::ContextCreate(&mut context, &desc.into());
    }
    Context(context)
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

struct DispatchDescription<'a> {
    cmd_list: &'a mut CommandList,

    color: Option<&'a Resource>,
    color_opaque_only: Option<&'a Resource>,
    depth: Option<&'a Resource>,
    exposure: Option<&'a Resource>,
    motion_vectors: Option<&'a Resource>,
    reactive: Option<&'a Resource>,
    transparency_and_composition: Option<&'a Resource>,
    output: Option<&'a mut Resource>,

    enable_auto_reactive: bool,
    enable_sharpening: bool,

    auto_reactive_max: f32,
    auto_tc_scale: f32,
    auto_reactive_scale: f32,
    auto_tc_threshold: f32,

    motion_vector_scale: [f32; 2],

    pre_exposure: f32,
    frame_time_delta: f32,

    jitter_offfset: [f32; 2],

    render_size: [u32; 2],

    camera_near: f32,
    camera_far: f32,
    camera_fov_y: f32,

    sharpness: f32,

    view_space_to_meters_factor: f32,

    reset: bool,
}

impl From<DispatchDescription<'_>> for fsr2_sys::DispatchDescription {
    fn from(val: DispatchDescription) -> Self {
        fsr2_sys::DispatchDescription {
            commandList: val.cmd_list.0,
            output: val.output.map(|r| r.0).unwrap_or_default(),
            color: val.color.map(|r| r.0).unwrap_or_default(),
            transparencyAndComposition: val
                .transparency_and_composition
                .map(|r| r.0)
                .unwrap_or_default(),
            colorOpaqueOnly: val.color_opaque_only.map(|r| r.0).unwrap_or_default(),
            depth: val.depth.map(|r| r.0).unwrap_or_default(),
            exposure: val.exposure.map(|r| r.0).unwrap_or_default(),
            reactive: val.reactive.map(|r| r.0).unwrap_or_default(),
            motionVectors: val.motion_vectors.map(|r| r.0).unwrap_or_default(),
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
