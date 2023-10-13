#[cfg(feature = "generate-bindings")]
mod bindgen;

fn build_fsr(api_dir: &str, _vk_include_dir: &str) {
    let sources = [
        "ffx_assert.cpp",
        "dx12/shaders/ffx_fsr2_shaders_dx12.cpp",
        "dx12/ffx_fsr2_dx12.cpp",
        "ffx_fsr2.cpp",
        "vk/shaders/ffx_fsr2_shaders_vk.cpp",
        "vk/ffx_fsr2_vk.cpp",
    ]
    .into_iter()
    .map(|p| format!("{api_dir}/{p}"))
    .collect::<Vec<_>>();

    // Compile d3d12 / vulkan  backend into the lib
    #[cfg(not(feature = "d3d12"))]
    let sources = sources.into_iter().filter(|p| !p.contains("dx12"));
    #[cfg(not(feature = "vulkan"))]
    let sources = sources.into_iter().filter(|p| !p.contains("vk"));

    let mut build = cc::Build::new();
    build
        .files(sources)
        .cpp(true)
        .define("DYNAMIC_LINK_VULKAN", "1");

    #[cfg(not(target_os = "windows"))]
    build.define("FFX_GCC", "1").std("c++2a");

    #[cfg(feature = "vulkan")]
    build
        .include(&format!("{}/../../shader_permutations/vk", api_dir))
        .include(_vk_include_dir);

    #[cfg(feature = "d3d12")]
    build.include(&format!("{}/../../shader_permutations/dx12", api_dir));

    build.compile("ffx_fsr2_api");
}

fn main() {
    let api_dir = "./FidelityFX-FSR2/src/ffx-fsr2-api";
    let vk_include_dir = "./Vulkan-Headers/include/";

    build_fsr(api_dir, vk_include_dir);

    #[cfg(feature = "generate-bindings")]
    {
        bindgen::generate_bindings(api_dir);
        bindgen::generate_vk_bindings(api_dir, vk_include_dir);
        bindgen::generate_d3d12_bindings(api_dir);
    }
}
