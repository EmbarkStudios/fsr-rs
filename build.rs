use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct Renamer;
impl bindgen::callbacks::ParseCallbacks for Renamer {
    fn item_name(&self, name: &str) -> Option<String> {
        // Remove VK suffix. we put vulkan function in their own
        let name = if name.ends_with("VK") {
            name.replace("VK", "")
        } else {
            name.to_owned()
        };

        // Remove ffx/ffxfsr2 prefixes. we use namespaces.
        let name = name
            .replace("ffxFsr2", "")
            .replace("ffx", "")
            .replace("FfxFsr2", "")
            .replace("Ffx", "");

        Some(name)
    }

    // Remove enum prefixes
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        Some(original_variant_name.replace("FFX_RESOURCE_STATE_", ""))
    }
}

fn main() {
    let fsr2_dir = "FidelityFX-FSR2";
    let fsr2_binary_dir = format!("{}/bin/ffx_fsr2_api/", fsr2_dir);
    let fsr2_binary_dir = std::path::Path::new(&fsr2_binary_dir)
        .canonicalize()
        .unwrap();

    println!(
        "cargo:rustc-link-search=native={}",
        fsr2_binary_dir.as_os_str().to_str().unwrap()
    );

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

    // Build fsr2 and link
    let _ = cmake::Config::new(fsr2_dir)
        .define("GFX_API", "VK")
        .build_target("ffx_fsr2_api_x64")
        .uses_cxx11()
        .build();
    let _ = cmake::Config::new(fsr2_dir)
        .define("GFX_API", "VK")
        .build_target("ffx_fsr2_api_vk_x64")
        .uses_cxx11()
        .build();
    println!("cargo:rustc-link-lib=ffx_fsr2_api_x64");
    println!("cargo:rustc-link-lib=ffx_fsr2_api_vk_x64");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    let wrapper = "./FidelityFX-FSR2/src/ffx-fsr2-api/ffx_fsr2.h";
    let wrapper_vk = "./FidelityFX-FSR2/src/ffx-fsr2-api/vk/ffx_fsr2_vk.h";

    println!("cargo:rerun-if-changed={}", wrapper);
    println!("cargo:rerun-if-changed={}", wrapper_vk);

    let vulkan_inc_dir = format!("-I{}/Include", env::var("VULKAN_SDK").unwrap_or_default());

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .layout_tests(false)
        .derive_default(true)
        .prepend_enum_name(false)
        .header("header.h") // get weird ignore list issues if i include fsr2 headers directly.
        .clang_arg(vulkan_inc_dir)
        .clang_arg("-xc++")
        .allowlist_file(wrapper)
        .allowlist_file(wrapper_vk)
        .blocklist_item("VkPhysicalDevice")
        .blocklist_item("VkPhysicalDevice_T")
        .blocklist_item("VkDevice")
        .blocklist_item("VkDevice_T")
        .blocklist_item("VkCommandBuffer")
        .blocklist_item("VkCommandBuffer_T")
        .blocklist_item("VkImage")
        .blocklist_item("VkImage_T")
        .blocklist_item("VkBuffer")
        .blocklist_item("VkBuffer_T")
        .blocklist_item("VkImageView")
        .blocklist_item("VkImageView_T")
        .blocklist_item("VkFormat")
        .blocklist_item("VkImageLayout")
        .blocklist_item("PFN_vkGetDeviceProcAddr")
        .new_type_alias("CommandList")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(Renamer))
        .rustified_enum("FfxResourceStates")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
