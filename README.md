# Vulkan Layer for Rust

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

- [ ] Handle the case where the underlying driver returns a null pointer to function. Currently we still return a valid function pointer, however the Android loader may test the returned function pointer to decide if such function is supported. Basically, allowing interception of `vkGet*ProcAddr` will fix this issue.
- [ ] Set up `Android.bp` to build in an Android tree.
- [x] Auto-generate the binding from [`vk_layer.h`](https://github.com/KhronosGroup/Vulkan-Headers/blob/9e61870ecbd32514113b467e0a0c46f60ed222c7/include/vulkan/vk_layer.h).
- [x] Auto-generate the `global_simple_intercept.rs` from `vk.xml`.
- [x] Auto-generate the `layer_trait.rs` file from `vk.xml`.
- [ ] Use a attribute macro to track which function is implemented in the `LayerTrait`, and don't inject all other functions for performance.
- [ ] Do not use const items for the function name to pointer array, it is not guaranteed to have only one address. Replace it with a runtime variable.
- [ ] Release global resources when the dynamic library is unloaded. Use platform specific way.
- [ ] Use procedure macro to generate the export functions in `lib.rs` file for the layer user.
- [ ] Use the xtask workflow to generate the layer json file.
- [ ] Support the latest layer interface version. Currently 2 is the latest version. e.g. correctly handle the `vk_layerGetPhysicalDeviceProcAddr` case.
- [ ] Allow intercepting [pre-instance functions](https://github.com/KhronosGroup/Vulkan-Loader/blob/0c63db1aeda6916690b863688fa6cdf2ac1f790b/docs/LoaderLayerInterface.md#pre-instance-functions).
- [ ] Add docstring to generated `layer_trait.rs` file.
