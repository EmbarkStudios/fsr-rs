<div align="center">

# 🌉 fsr

**Rust bindings for [FidelityFX Super Resolution 2](https://github.com/GPUOpen-Effects/FidelityFX-FSR2)**


[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/fsr.svg)](https://crates.io/crates/fsr)
[![Docs](https://docs.rs/fsr/badge.svg)](https://docs.rs/fsr)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/fsr-rs/status.svg)](https://deps.rs/repo/github/EmbarkStudios/fsr-rs)
[![Build status](https://github.com/EmbarkStudios/fsr-rs/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/fsr-rs/actions)
</div>

Unsafe rust bindings for [FidelityFX Super Resolution 2](https://github.com/GPUOpen-Effects/FidelityFX-FSR2).

API | Feature Flag | Crate
--|--|--
Vulkan | `vulkan` | [ash](https://crates.io/crates/ash)
DirectX12 | `d3d12`| [windows](https://crates.io/crates/windows)

`fsr-sys` contains opaque types for dx12/vulkan which can be used with any api bindings.

Currently Vulkan is expected to be linked dynamically. See [`ash`](https://docs.rs/ash/latest/ash/) for more information.

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
