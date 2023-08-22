# Vulkan Layer for Rust
[![postsubmit](https://github.com/google/vk-layer-for-rust/actions/workflows/postsubmit.yml/badge.svg)](https://github.com/google/vk-layer-for-rust/actions/workflows/postsubmit.yml)
[![postsubmit miri](https://github.com/google/vk-layer-for-rust/actions/workflows/postsubmit-miri.yml/badge.svg?branch=main)](https://github.com/google/vk-layer-for-rust/actions/workflows/postsubmit-miri.yml)
[![Linux coverage](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fgoogle%2Fvk-layer-for-rust%2Fstatic_resource%2Fcoverage-Linux%2Fcoverage_badge.json)](https://google.github.io/vk-layer-for-rust/coverage-Linux/llvm-cov/html/index.html)
[![Windows coverage](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fgoogle%2Fvk-layer-for-rust%2Fstatic_resource%2Fcoverage-Windows%2Fcoverage_badge.json)](https://google.github.io/vk-layer-for-rust/coverage-Windows/llvm-cov/html/index.html)
[![Linux doc](https://img.shields.io/badge/doc-Linux-blue)](https://google.github.io/vk-layer-for-rust/doc-Linux/vulkan_layer/index.html)
[![Windows doc](https://img.shields.io/badge/doc-Windows-blue)](https://google.github.io/vk-layer-for-rust/doc-Windows/vulkan_layer/index.html)

This project provides a way to use safe Rust to write a [Vulkan layer](https://github.com/KhronosGroup/Vulkan-Loader/blob/121c1f42025a82dca7922a503ca77df51c37b394/docs/LoaderInterfaceArchitecture.md#layers).

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for details.

## License

Apache 2.0; see [`LICENSE`](LICENSE) for details.

## Disclaimer

This project is not an official Google project. It is not supported by
Google and Google specifically disclaims all warranties as to its quality,
merchantability, or fitness for a particular purpose.

## Build

### cross-compile from Windows to Android

1. Install the Rust Android toolchain

   ```bash
   rustup target add x86_64-linux-android
   ```

2. Set up the linker to use. Add the following lines to the [`config.toml` file](https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure). I will just use `%CARGO_HOME%/config.toml`.

   ```plaintext
   [target.x86_64-linux-android]
   linker = "%NDK_HOME%\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\x86_64-linux-android33-clang"
   ```

   Note that `%NDK_HOME%` must be expanded to an explicit absolute path, which is usually `%LOCALAPPDATA%/Android/Sdk/ndk/<version-number>`.

3. If the NDK version is greater than 22, follow [this workaround](https://stackoverflow.com/a/74041320).

4. `cargo build --target x86_64-linux-android`.

### Windows natively

```bash
cargo build
```

### Soong in an Android tree

TODO

## TODO

- [x] Allow interception of `vkGet*ProcAddr` will fix this issue. Handle the case where the underlying driver returns a null pointer to function. Currently we still return a valid function pointer, however the Android loader may test the returned function pointer to decide if such function is supported.
- [x] Set up `Android.bp` to build in an Android tree.
  - [ ] Upgrade ash in aosp, and remove vulkano, so that we can build from aosp/master.
- [x] Auto-generate the binding from [`vk_layer.h`](https://github.com/KhronosGroup/Vulkan-Headers/blob/9e61870ecbd32514113b467e0a0c46f60ed222c7/include/vulkan/vk_layer.h).
- [x] Auto-generate the `global_simple_intercept.rs` from `vk.xml`.
- [x] Auto-generate the `layer_trait.rs` file from `vk.xml`.
- [x] Use a attribute macro to track which function is implemented in the `LayerTrait`, and don't inject all other functions for performance.
- [ ] Make global instance trivially destructible after all instances are destroyed. We can't rely on the destructor of DLL to perform clean up. We need to require the user to declare the global static, and declare one for each layer. User's `Layer` global instance will also be created before the first `vkCreateInstance` is returned and will be destroyed after the last `vkDestroyInstance` is called.
- [ ] Use procedure macro to generate the export functions in `lib.rs` file for the layer user.
- [ ] Use the xtask workflow to generate the layer json file.
- [ ] Support the latest layer interface version. Currently 2 is the latest version. e.g. correctly handle the `vk_layerGetPhysicalDeviceProcAddr` case.
- [ ] Allow intercepting [pre-instance functions](https://github.com/KhronosGroup/Vulkan-Loader/blob/0c63db1aeda6916690b863688fa6cdf2ac1f790b/docs/LoaderLayerInterface.md#pre-instance-functions).
- [ ] Add docstring to generated `layer_trait.rs` file.
- [ ] Testing
  - [ ] e2e test: the test boundary is the Vulkan layer ABI. The Vulkan loader and the mock ICD will be used for testing. Write a `cdylib` crate named `tests`, with a layer that is allowed to customize the behavior through a function exported as a dynamic library symbol. We run different tests in different processes. For different tests, we customize the layer differently, and asserts either inside the customization point or after it returns, e.g. to test initialization and destruction on DLL loading and unloading time, we can customize the ctor and the drop implementation for the layer, then load and unload the Vulkan library then verify if the ctor is called the same time as the drop. We also need to create a `e2e` task to build the DLL, generate the json, set the environement variables, and spawn tests in different processes(cargo-nextest can be used here since it runs tests in their own process).
  - [x] `vulkan-layer` level integration test
- [ ] catch unwind at the FFI boundary to allow the library to be compiled with `panic="unwind"`.
- [ ] Imporve Miri test
  - Check why fp comparison in miri fails in test_should_move_layer_device_link_forward, test_should_move_layer_device_link_forward, and test_should_return_fp_for_get_instance_proc_addr.
