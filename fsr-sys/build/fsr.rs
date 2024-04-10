use std::{env, path::Path};

use fs_extra::{copy_items, dir::CopyOptions};

fn copy_contents(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    let mut options = CopyOptions::new(); // Initialize the options
    options.overwrite = true; // Overwrite files if they already exist

    // Get a list of all entries in 'from' directory
    let entries = std::fs::read_dir(from).unwrap();

    let mut paths = Vec::new();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() || path.is_file() {
            paths.push(path);
        }
    }

    // Perform the copy operation using the paths collected
    copy_items(&paths, to, &options).unwrap();
}

pub fn build(api_dir: &str) {
    let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
    if target_pointer_width != "64" {
        panic!("fsr-sys only supports building on 64-bit platforms currently");
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let cmake_build_dir = format!("{}/build", out_dir);
    std::fs::create_dir(&cmake_build_dir).ok();
    copy_contents(&api_dir, &cmake_build_dir);
    let mut cmake_config = cmake::Config::new(api_dir);
    cmake_config.no_build_target(true);

    #[cfg(not(feature = "vulkan"))]
    {
        cmake_config.define("FFX_FSR2_API_VK", "OFF");
    }
    #[cfg(not(feature = "d3d12"))]
    {
        cmake_config.define("FFX_FSR2_API_DX12", "OFF");
    }

    let dst = cmake_config.build().join("build/bin/ffx_fsr2_api");
    println!("cargo:rustc-link-search={}", dst.display());
    let suffix = match cmake_config.get_profile() {
        "Debug" => "d",
        _ => "",
    };

    #[cfg(feature = "vulkan")]
    {
        // https://github.com/ash-rs/ash/blob/master/ash/build.rs

        let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
        let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();

        println!("cargo:rerun-if-env-changed=VULKAN_SDK");
        if let Ok(var) = env::var("VULKAN_SDK") {
            let suffix = match (&*target_family, &*target_pointer_width) {
                ("windows", "32") => "Lib32",
                ("windows", "64") => "Lib",
                _ => "lib",
            };
            println!("cargo:rustc-link-search={var}/{suffix}");
        }
        let lib = match &*target_family {
            "windows" => "vulkan-1",
            _ => "vulkan",
        };
        println!("cargo:rustc-link-lib={lib}");
        println!("cargo:rustc-link-lib=ffx_fsr2_api_vk_x64{}", suffix);
    }
    #[cfg(feature = "d3d12")]
    {
        println!("cargo:rustc-link-lib=ffx_fsr2_api_dx12_x64{}", suffix);
    }
    println!("cargo:rustc-link-lib=ffx_fsr2_api_x64{}", suffix);
}
