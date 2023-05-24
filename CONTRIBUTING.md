# How to Contribute

We would love to accept your patches and contributions to this project.

## Before you begin

### Sign our Contributor License Agreement

Contributions to this project must be accompanied by a
[Contributor License Agreement](https://cla.developers.google.com/about) (CLA).
You (or your employer) retain the copyright to your contribution; this simply
gives us permission to use and redistribute your contributions as part of the
project.

If you or your current employer have already signed the Google CLA (even if it
was for a different project), you probably don't need to do it again.

Visit <https://cla.developers.google.com/> to see your current agreements or to
sign a new one.

### Review our Community Guidelines

This project follows [Google's Open Source Community
Guidelines](https://opensource.google/conduct/).

## Contribution process

### Code Reviews

All submissions, including submissions by project members, require review. We
use [GitHub pull requests](https://docs.github.com/articles/about-pull-requests)
for this purpose.

### Format the code

Nightly features in `rustfmt` is used, so run `cargo +nightly fmt` to format the entire project.

### Regenerate code

1. Install LLVM and set up the environment variable required by `bindgen`: [link](https://rust-lang.github.io/rust-bindgen/requirements.html), because `bindgen` is used in the codegen process.

2. Make sure python newer than 3.9 is installed.

3. Run the following command:

```bash
cargo xtask codegen
```

The codegen scripts will automatically format the generated file, so no need to manually format the project after running codegen.

[cargo xtask pattern](https://github.com/matklad/cargo-xtask) is used to automate the code generation process.

Files are generated in different mechanism:

* `vulkan-layer/src/bindings/generated/vk_layer_bindings.rs`

  This file is generated from `vk_layer.h` in the `Vulkan-Headers` dependency by using `bindgen`.

* `vulkan-layer/src/generated/{layer_trait,global_simple_intercept}.rs`

  These 2 files are generated through the `scritps/vulkan_layer_genvk.py`. This script utilizes the `OutputGenerator` class defined in the `generator.py` from the Vulkan registry repo.
