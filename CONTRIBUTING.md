# How to Contribute

We would love to accept your patches and contributions to this project.

## Before you begin

### Sign our Contributor License Agreement

Contributions to this project must be accompanied by a
[Contributor License Agreement](https://cla.developers.google.com/about) (CLA). You (or your
employer) retain the copyright to your contribution; this simply gives us permission to use and
redistribute your contributions as part of the project.

If you or your current employer have already signed the Google CLA (even if it was for a different
project), you probably don't need to do it again.

Visit <https://cla.developers.google.com/> to see your current agreements or to sign a new one.

### Review our Community Guidelines

This project follows
[Google's Open Source Community Guidelines](https://opensource.google/conduct/).

## Contribution process

### Prerequisites

- Install [cargo-make](https://sagiegurari.github.io/cargo-make/#installation).
- Install [pipenv](https://pipenv.pypa.io/en/latest/#install-pipenv-today).
- Install Python dependencies: `pipenv install`.

### Code Reviews

All submissions, including submissions by project members, require review. We use
[GitHub pull requests](https://docs.github.com/articles/about-pull-requests) for this purpose.

### Format the code

Presubmit will fail if the code is not properly format. To run all lints and formatters, run
`cargo make lint`.

#### Rust

Nightly features in `rustfmt` is used, so run `cargo +nightly fmt` to format the entire project.

To run both rustfmt and clippy, run `cargo make rust-lint`.

#### Python

Python scripts are used to generate some Rust source files. Use
[black](https://black.readthedocs.io/en/stable) to format python code:

```bash
$ pipenv run black .
All done! ✨ 🍰 ✨
4 files left unchanged.
```

Use [ruff](https://github.com/astral-sh/ruff) to lint the python scripts:

```bash
pipenv run ruff check . --fix
```

To run all python formatters and lints, run `cargo make python-lint`.

#### TOML

[taplo](https://taplo.tamasfe.dev/) is used to format all TOML files.

Use the `toml-fmt` cargo-make target to run it, and it will also handle the end of lines for
different platforms.

```bash
cargo make toml-fmt
```

We can also just use `taplo`. One must take care of end of lines on Windows.

```bash
taplo fmt
```

on Windows

```bash
taplo fmt --option clrf=true
```

### Regenerate code

Files named `generated.rs` are generated files. One should avoid manually editing the files.
Presubmit will fail if the generated files are not sync with the source.

1. Install LLVM and set up the environment variable required by `bindgen`:
   [link](https://rust-lang.github.io/rust-bindgen/requirements.html), because `bindgen` is used in
   the codegen process.

1. Make sure python newer than 3.9 is installed.

1. Run the following command:

```bash
cargo make codegen
```

The codegen scripts will automatically format the generated file, so no need to manually format the
project after running codegen.

A separate Rust binary `scripts/codegen.rs` is used to automate the code generation process.

Files are generated in different mechanism:

- `vulkan-layer/src/bindings/vk_layer/generated.rs`

  This file is generated from `vk_layer.h` in the `Vulkan-Headers` dependency by using `bindgen`.

- `vulkan-layer/src/{layer_trait,global_simple_intercept}/generated.rs`

  These 2 files are generated through the `scritps/vulkan_layer_genvk.py`. This script utilizes the
  `OutputGenerator` class defined in the `generator.py` from the Vulkan registry repo.

### Test

All tests mentioned are run in CI. Miri tests are run as a post submit because of the slowness while
others are presubmit.

#### Unit tests and integration tests

`cargo-nextest` is the recommended way to run the test. Run `cargo nextest run` to run all tests.
This is how CI runs the tests. All tests are supposed to only run in a separate process.

Vanilla `cargo test` is not supported, because that will run tests in the same binary in the same
process. However, `cargo +nightly test -Z panic-abort-tests` is Ok because it will
[allow subprocess testing](https://github.com/rust-lang/rust/issues/67650). In addition, the library
is supposed to be built with `panic="abort"`.

#### Documentation tests

```bash
cargo +nightly test --doc --all-features --workspace -- --show-output
```

#### Coverage

Code coverage is also part of the CI. Changes shouldn't cause the coverage to drop. Newly added
functions or modules are expected high test coverage.

[`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov)
[with `cargo-nextest`](https://nexte.st/book/test-coverage.html#llvm-cov) is used to generate the
code coverage info. To generate the `lcov` coverage file locally under `target/lcov.info`, run
`cargo llvm-cov nextest --lcov --output-path target/lcov.info`.

#### Miri tests

When writing unsafe code or changing the "trivially-destructible" behavior of the global resources,
Miri tests should be used to catch undefined behaviors and resource leak. It is recommended to run
Miri tests on Linux. Follow [these steps](https://github.com/rust-lang/miri#using-miri) to install
Miri. Use the following command to run the Miri test on Linux:

```bash
MIRIFLAGS="-Zmiri-tree-borrows" cargo +nightly miri nextest run --all-targets --all-features -j8 --no-fail-fast
```

Miri tests are usually very slow, use the
[test filter](https://nexte.st/book/filter-expressions.html) to run a specific test.

```bash
MIRIFLAGS="-Zmiri-tree-borrows" cargo +nightly miri nextest run --all-targets --all-features --no-fail-fast -E'test(test_should_return_fp_when_called_with_get_instance_proc_addr_name)'
```

### Document

All public intefaces must have documents. Otherwise the presubmit will fail. After a new PR is
merged, the document will be automatically regenerated and published. Use the badge in
[`README.md`](README.md) to browse the document. To generate the document locally, run

```bash
cargo +nightly doc --workspace --all-features --no-deps --open
```
