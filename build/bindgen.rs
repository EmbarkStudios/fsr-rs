use std::{env, path::PathBuf};

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

pub fn generate_bindings(api_dir: &str, vk_include_dir: &str) {
    let wrapper = format!("{}/ffx_fsr2.h", api_dir);
    let wrapper_vk = format!("{}/vk/ffx_fsr2_vk.h", api_dir);

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .layout_tests(false)
        .derive_default(true)
        .prepend_enum_name(false)
        .header("header.h") // get ignore list issues if I include fsr2 headers directly.
        .clang_arg(format!("-I{}", vk_include_dir))
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
