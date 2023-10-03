use std::{env, path::PathBuf};

#[derive(Debug)]
struct Renamer;
impl bindgen::callbacks::ParseCallbacks for Renamer {
    fn item_name(&self, name: &str) -> Option<String> {
        // Remove ffx/ffxfsr2 prefixes.
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

pub fn generate_bindings(api_dir: &str) {
    let wrapper = format!("{}/ffx_fsr2.h", api_dir);

    // Generate bindings
    let mut bindings = bindgen::Builder::default()
        .layout_tests(false)
        .derive_default(true)
        .prepend_enum_name(false)
        .header(wrapper)
        .clang_arg("-xc++")
        .trust_clang_mangling(false)
        .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
        .allowlist_file(r".*(/|\\)ffx-fsr2-api(/|\\)[^/\\]+\.h")
        .blocklist_type("widechar")
        .new_type_alias("CommandList")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(Renamer))
        .rustified_enum("FfxFsr2MsgType")
        .rustified_enum("FfxResourceStates");

    if cfg!(not(target_os = "windows")) {
        bindings = bindings.clang_args(["-DFFX_GCC"]);
    }

    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(&format!("{}/src/", env!("CARGO_MANIFEST_DIR")));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

pub fn generate_vk_bindings(api_dir: &str, vk_include_dir: &str) {
    let wrapper = format!("{}/ffx_fsr2.h", api_dir);
    let wrapper_api = format!("{}/vk/ffx_fsr2_vk.h", api_dir);

    // Generate bindings
    let mut bindings = bindgen::Builder::default()
        .layout_tests(false)
        .derive_default(true)
        .prepend_enum_name(false)
        .header(wrapper)
        .header(&wrapper_api)
        .clang_arg("-xc++")
        .trust_clang_mangling(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(Renamer))
        .clang_arg(format!("-I{}", vk_include_dir))
        .allowlist_recursively(false)
        .allowlist_file(&wrapper_api);

    if cfg!(not(target_os = "windows")) {
        bindings = bindings.clang_args(["-DFFX_GCC"]).clang_arg("-std=c++2a");
    }

    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(&format!("{}/src/", env!("CARGO_MANIFEST_DIR")));
    bindings
        .write_to_file(out_path.join("vk_bindings.rs"))
        .expect("Couldn't write bindings!");
}

pub fn generate_d3d12_bindings(api_dir: &str) {
    let wrapper = format!("{}/ffx_fsr2.h", api_dir);
    let wrapper_api = format!("{}/dx12/ffx_fsr2_dx12.h", api_dir);

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .layout_tests(false)
        .derive_default(true)
        .prepend_enum_name(false)
        .header(wrapper)
        .header(&wrapper_api)
        .clang_arg("-xc++")
        .trust_clang_mangling(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(Renamer))
        .allowlist_recursively(false)
        .allowlist_file(&wrapper_api)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(&format!("{}/src/", env!("CARGO_MANIFEST_DIR")));
    bindings
        .write_to_file(out_path.join("d3d12_bindings.rs"))
        .expect("Couldn't write bindings!");
}
