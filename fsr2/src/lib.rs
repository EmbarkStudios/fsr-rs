#[cfg(feature = "d3d12")]
pub mod d3d12;
#[cfg(feature = "vulkan")]
pub mod vk;

pub use fsr2_sys::Device;
pub use fsr2_sys::Interface;
pub use fsr2_sys::Resource;
pub use fsr2_sys::ResourceStates;
pub struct CommandList(fsr2_sys::CommandList);
pub struct Context(fsr2_sys::Context);

pub struct ContextDescription<'a> {
    interface: &'a Interface,
    flags: InitializationFlagBits,
    max_render_size: [u32; 2],
    display_size: [u32; 2],
    device: &'a Device,
    message_callback: Option<unsafe extern "C" fn(i32, *const u16)>,
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

pub struct DispatchDescription<'a> {
    pub cmd_list: &'a mut CommandList,
    pub output: &'a mut Resource,

    pub color: &'a Resource,
    pub depth: &'a Resource,
    pub motion_vectors: &'a Resource,
    pub color_opaque_only: Option<&'a Resource>,
    pub exposure: Option<&'a Resource>,
    pub reactive: Option<&'a Resource>,
    pub transparency_and_composition: Option<&'a Resource>,

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

impl<'a> DispatchDescription<'a> {
    pub fn new(
        cmd_list: &'a mut CommandList,
        color: &'a Resource,
        depth: &'a Resource,
        motion_vectors: &'a Resource,
        output: &'a mut Resource,
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

    pub fn camera(mut self, near: f32, far: f32, fov_y: f32) -> DispatchDescription<'a> {
        self.camera_near = near;
        self.camera_far = far;
        self.camera_fov_y = fov_y;
        self
    }

    pub fn pre_exposure(mut self, value: f32) -> DispatchDescription<'a> {
        self.pre_exposure = value;
        self
    }

    pub fn view_space_to_meters_factor(mut self, value: f32) -> DispatchDescription<'a> {
        self.view_space_to_meters_factor = value;
        self
    }

    pub fn exposure(mut self, resource: &'a Resource) -> DispatchDescription<'a> {
        self.exposure = Some(resource);
        self
    }

    pub fn reactive(mut self, resource: &'a Resource) -> DispatchDescription<'a> {
        self.reactive = Some(resource);
        self
    }

    pub fn motion_vector_scale(mut self, value: [f32; 2]) -> DispatchDescription<'a> {
        self.motion_vector_scale = value;
        self
    }

    pub fn jitter_offfset(mut self, value: [f32; 2]) -> DispatchDescription<'a> {
        self.jitter_offfset = value;
        self
    }

    pub fn sharpness(mut self, sharpness: f32) -> DispatchDescription<'a> {
        self.enable_sharpening = true;
        self.sharpness = sharpness;
        self
    }

    pub fn auto_reactive(
        mut self,
        color_opaque_only: &'a Resource,
        transparency_and_composition: &'a Resource,
        auto_reactive_max: f32,
        auto_tc_scale: f32,
        auto_reactive_scale: f32,
        auto_tc_threshold: f32,
    ) -> DispatchDescription<'a> {
        self.color_opaque_only = Some(color_opaque_only);
        self.transparency_and_composition = Some(transparency_and_composition);
        self.enable_auto_reactive = true;
        self.auto_reactive_max = auto_reactive_max;
        self.auto_tc_scale = auto_tc_scale;
        self.auto_reactive_scale = auto_reactive_scale;
        self.auto_tc_threshold = auto_tc_threshold;
        self
    }

    pub fn reset(mut self, value: bool) -> DispatchDescription<'a> {
        self.reset = value;
        self
    }
}

impl From<DispatchDescription<'_>> for fsr2_sys::DispatchDescription {
    fn from(val: DispatchDescription) -> Self {
        fsr2_sys::DispatchDescription {
            commandList: val.cmd_list.0,
            output: *val.output,
            color: *val.color,
            transparencyAndComposition: val
                .transparency_and_composition
                .copied()
                .unwrap_or_default(),
            colorOpaqueOnly: val.color_opaque_only.copied().unwrap_or_default(),
            depth: *val.depth,
            exposure: val.exposure.copied().unwrap_or_default(),
            reactive: val.reactive.copied().unwrap_or_default(),
            motionVectors: *val.motion_vectors,
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
    pub fn dispatch(&mut self, desc: DispatchDescription) {
        unsafe {
            fsr2_sys::ContextDispatch(&mut self.0, &desc.into());
        }
    }
}
