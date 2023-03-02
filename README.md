<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 no-emphasis-as-heading -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

<!--- FIXME: Pick an emoji and name your project! --->
# `⚖ fsr2`

<!--- FIXME: Write short catchy description/tagline of project --->
**Rust bindings for [FidelityFX Super Resolution 2](https://github.com/GPUOpen-Effects/FidelityFX-FSR2)**

<!--- FIXME: Update crate, repo and CI workflow names here! Remove any that are not relevant --->

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/fsr2.svg)](https://crates.io/crates/fsr2)
[![Docs](https://docs.rs/fsr2/badge.svg)](https://docs.rs/rust-gpu)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/fsr2/status.svg)](https://deps.rs/repo/github/EmbarkStudios/fsr2)
[![Build status](https://github.com/EmbarkStudios/fsr2/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/fsr2/actions)
</div>

Unsafe rust bindings for [FidelityFX Super Resolution 2](https://github.com/GPUOpen-Effects/FidelityFX-FSR2). Currently only Vulkan is supported and enabled by default.

API | Supported
--|--
Vulkan | ✅
DirectX12 | 🛠 (todo)

# Updaing FSR2

If you want to update the FSR2 version you need to do the following steps:

* Replace all files in `FidelityFX-FSR2/src/ffx-fsr2-api` and remove the cmake files.
* Build FSR and copy the permutation shaders to `shader_permutations/`
* Potentially fix up some includes in the FSR source code to build on all platforms.

## Contributing

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in an Embark Studios project, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

### License

This contribution is dual licensed under EITHER OF

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to Embark or any other licensee/user of the contribution.
