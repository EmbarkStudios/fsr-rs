mod bindgen;

fn build_fsr(api_dir: &str, _vk_include_dir: &str) {
    let sources = glob::glob(&format!("{}/**/*.cpp", api_dir)).expect("Failed to find sources");

    // Compile d3d12 / vulkan  backend into the lib
    #[cfg(not(feature = "d3d12"))]
    let sources = sources.filter(|p| !p.as_ref().unwrap().to_str().unwrap().contains("dx12"));
    #[cfg(not(feature = "vulkan"))]
    let sources = sources.filter(|p| !p.as_ref().unwrap().to_str().unwrap().contains("vk"));

    let sources: Vec<_> = sources.map(|p| p.unwrap()).collect();

    let mut build = cc::Build::new();
    build.files(sources.iter()).cpp(true);
    #[cfg(feature = "vulkan")]
    build
        .include(&format!("{}/../../shader_permutations/vk", api_dir))
        .include(_vk_include_dir);
    #[cfg(feature = "d3d12")]
    build.include(&format!("{}/../../shader_permutations/dx12", api_dir));
    build.compile("ffx_fsr2_api");

    // Link compiled lib
    println!("cargo:rustc-link-lib=ffx_fsr2_api");
}

fn main() {
    let api_dir = "../FidelityFX-FSR2/src/ffx-fsr2-api";
    let vk_include_dir = "../Vulkan-Headers/include/";

    // link vulkan, stolen from ash
    /*{
        let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
        let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();

        println!("cargo:rerun-if-env-changed=VULKAN_SDK");
        if let Ok(var) = env::var("VULKAN_SDK") {
            let suffix = match (&*target_family, &*target_pointer_width) {
                ("windows", "32") => "Lib32",
                ("windows", "64") => "Lib",
                _ => "lib",
            };
            println!("cargo:rustc-link-search={}/{}", var, suffix);
        }
        let lib = match &*target_family {
            "windows" => "vulkan-1",
            _ => "vulkan",
        };
        println!("cargo:rustc-link-lib={}", lib);
    }*/

    build_fsr(api_dir, vk_include_dir);
    bindgen::generate_bindings(api_dir);

    #[cfg(feature = "vulkan")]
    bindgen::generate_vk_bindings(api_dir, vk_include_dir);
    #[cfg(feature = "d3d12")]
    bindgen::generate_d3d12_bindings(api_dir);
}
