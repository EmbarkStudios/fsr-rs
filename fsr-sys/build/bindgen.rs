use std::{env, path::PathBuf};

use bindgen::{callbacks::ItemKind, Builder};
use regex::Regex;

#[derive(Debug)]
struct Renamer {
    prefix: Regex,
    suffix: Regex,
}

impl Renamer {
    fn new() -> Self {
        Self {
            prefix: Regex::new(r"(?i)^ffx_?(fsr2)?_?").unwrap(),
            suffix: Regex::new("(DX12|VK)$").unwrap(),
        }
    }
}
impl bindgen::callbacks::ParseCallbacks for Renamer {
    fn item_name(&self, name: &str) -> Option<String> {
        // Remove ffx/ffxfsr2 prefixes.
        let name = self.prefix.replace_all(name, "").to_string();

        Some(name)
    }

    // Remove enum prefixes
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        Some(
            self.prefix
                .replace_all(original_variant_name, "")
                .to_string(),
        )
    }

    fn generated_name_override(
        &self,
        item_info: bindgen::callbacks::ItemInfo<'_>,
    ) -> Option<String> {
        if let ItemKind::Function = item_info.kind {
            if self.prefix.is_match(item_info.name) {
                let name = self.prefix.replace_all(item_info.name, "");
                let mut name = self.suffix.replace_all(&name, "").to_string();
                if name.len() > 1 {
                    name[..1].make_ascii_lowercase();
                }
                return Some(name);
            }
        }
        None
    }
}

fn builder() -> Builder {
    Builder::default()
        .generate_comments(false)
        .derive_default(true)
        .prepend_enum_name(false)
        .clang_arg("-xc++")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(Renamer::new()))
}

pub fn generate_bindings(api_dir: &str) {
    let wrapper = format!("{}/ffx_fsr2.h", api_dir);

    let mut bindings = builder()
        .header(wrapper)
        .allowlist_file(r".*ffx-fsr2-api(/|\\).*\.h");

    if cfg!(not(target_os = "windows")) {
        bindings = bindings.clang_args(["-DFFX_GCC"]);
    }

    let bindings = bindings.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(&format!("{}/src/", env!("CARGO_MANIFEST_DIR")));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

pub fn generate_vk_bindings(api_dir: &str, vk_include_dir: &str) {
    let wrapper = format!("{}/ffx_fsr2.h", api_dir);
    let wrapper_api = format!("{}/vk/ffx_fsr2_vk.h", api_dir);

    let mut bindings = builder()
        .header(wrapper)
        .header(wrapper_api)
        .clang_arg(format!("-I{}", vk_include_dir))
        .allowlist_recursively(false)
        .allowlist_file(r".*vk(/|\\).*\.h");

    if cfg!(not(target_os = "windows")) {
        bindings = bindings.clang_args(["-DFFX_GCC"]).clang_arg("-std=c++2a");
    }

    let bindings = bindings.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(&format!("{}/src/", env!("CARGO_MANIFEST_DIR")));
    bindings
        .write_to_file(out_path.join("vk_bindings.rs"))
        .expect("Couldn't write bindings!");
}

pub fn generate_d3d12_bindings(api_dir: &str) {
    let wrapper = format!("{}/ffx_fsr2.h", api_dir);
    let wrapper_api = format!("{}/dx12/ffx_fsr2_dx12.h", api_dir);

    let bindings = builder()
        .header(wrapper)
        .header(wrapper_api)
        .allowlist_recursively(false)
        .allowlist_file(r".*dx12(/|\\).*\.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(&format!("{}/src/", env!("CARGO_MANIFEST_DIR")));
    bindings
        .write_to_file(out_path.join("d3d12_bindings.rs"))
        .expect("Couldn't write bindings!");
}
