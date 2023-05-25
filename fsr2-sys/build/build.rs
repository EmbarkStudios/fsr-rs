use std::env;
use std::path::PathBuf;

mod bindgen;

fn build_fsr(api_dir: &str, vk_include_dir: &str) {
    let sources: Vec<PathBuf> = glob::glob(&format!("{}/**/*.cpp", api_dir))
        .expect("Failed to find sources")
        .into_iter()
        .filter(|p| !p.as_ref().unwrap().to_str().unwrap().contains("dx12")) // filter dx12 shaders, currently only include vulkan.
        .map(|p| p.unwrap().to_path_buf())
        .collect();

    cc::Build::new()
        .files(sources.iter())
        .cpp(true)
        .define("FOO", Some("bar"))
        .include(&format!("{}/../../shader_permutations/vk", api_dir))
        .include(vk_include_dir)
        .compile("ffx_fsr2_api");

    // Link compiled lib
    println!("cargo:rustc-link-lib=ffx_fsr2_api");
}

fn main() {
    let api_dir = "../FidelityFX-FSR2/src/ffx-fsr2-api";

    // link vulkan, stolen from ash
    {
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
    }

    let vk_include_dir = "../Vulkan-Headers/include/";

    build_fsr(&api_dir, &vk_include_dir);
    bindgen::generate_bindings(&api_dir, &vk_include_dir);
}
