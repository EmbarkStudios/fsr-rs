
#[cfg(not(target_os = "macos"))]
mod fsr;

#[cfg(feature = "generate-bindings")]
mod bindgen;

fn main() {
    #[cfg(not(target_os = "macos"))]
    fsr::build("./FidelityFX-FSR2/src/ffx-fsr2-api/");

    #[cfg(feature = "generate-bindings")]
    {
        let api_dir = "./FidelityFX-FSR2/src/ffx-fsr2-api/";
        let vk_include_dir = "./Vulkan-Headers/include/";
        bindgen::generate_bindings(api_dir);
        bindgen::generate_vk_bindings(api_dir, vk_include_dir);
        bindgen::generate_d3d12_bindings(api_dir);
    }
}
