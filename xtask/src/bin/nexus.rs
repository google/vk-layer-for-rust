// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use log::info;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

use bindgen::EnumVariation;

// May not be precise, but should be enough for generating copyright comments.
fn current_year() -> u64 {
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    const SECONDS_PER_YEAR: u64 = 60 * 60 * 24 * 365;
    since_epoch.as_secs() / SECONDS_PER_YEAR + 1970
}

fn generate_android_native_binding(dst: &Path) {
    info!("Generating the binding for android headers...");

    let rustfmt_path = {
        let output = Command::new("rustup")
            .args(["which", "rustfmt", "--toolchain", "nightly"])
            .output()
            .expect("Could not spawn `rustup` command");
        assert!(
            output.status.success(),
            "Unsuccessful status code when running `rustup`: {:?}",
            output
        );
        String::from_utf8(output.stdout).expect("The `rustfmt` path is not valid `utf-8`")
    };
    let rustfmt_path = rustfmt_path.trim_end();

    let mut file = File::create(dst).unwrap();
    let license_comments = format!(
        "// Copyright {} Google LLC\n\
         //\n\
         // Licensed under the Apache License, Version 2.0 (the \"License\");\n\
         // you may not use this file except in compliance with the License.\n\
         // You may obtain a copy of the License at\n\
         //\n\
         //     http://www.apache.org/licenses/LICENSE-2.0\n\
         //\n\
         // Unless required by applicable law or agreed to in writing, software\n\
         // distributed under the License is distributed on an \"AS IS\" BASIS,\n\
         // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n\
         // See the License for the specific language governing permissions and\n\
         // limitations under the License.\n\n\
         #![allow(non_snake_case)]\n\
         #![allow(non_upper_case_globals)]\n\
         #![allow(non_camel_case_types)]\n\
         #![allow(dead_code)]\n\n",
        current_year()
    );
    file.write_all(license_comments.as_bytes()).unwrap();

    bindgen::Builder::default()
        .with_rustfmt(rustfmt_path)
        .header(r#"C:\src\vndk-sysroot\usr\include\vndk\hardware_buffer.h"#)
        .header(r#"C:\src\nexus-rust-vk-layer\nexus-vulkan-layer\ffi\cros_gralloc_handle.h"#)
        .clang_args(&[
            r#"--sysroot=C:\src\vndk-sysroot"#,
            "--target=x86_64-linux-android31",
            "-x",
            "c++",
        ])
        .default_enum_style(EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .derive_default(true)
        .bitfield_enum("AHardwareBuffer_UsageFlags")
        .generate()
        .unwrap()
        .write(Box::new(file))
        .unwrap();
    info!("The binding for vulkan_layer.h completes generation.");
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .filter_module("bindgen", log::LevelFilter::Error)
        .init();

    let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    assert!(cargo_manifest_dir.is_absolute());
    let mut project_root_dir = cargo_manifest_dir;
    assert!(project_root_dir.pop());

    let mut out_file = project_root_dir.clone();
    out_file.push("nexus-vulkan-layer");
    out_file.push("src");
    out_file.push("bindings.rs");

    generate_android_native_binding(&out_file);
}
