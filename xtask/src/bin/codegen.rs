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
    ffi::{OsStr, OsString},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    thread,
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

fn generate_vulkan_layer_header_binding(vulkan_headers_dir: &Path, dst: &Path) {
    info!("Generating the binding for vulkan_layer.h...");

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

    let mut vulkan_headers_include_dir = PathBuf::from(vulkan_headers_dir);
    vulkan_headers_include_dir.push("include");
    let mut vk_layer_h_path = vulkan_headers_include_dir.clone();
    vk_layer_h_path.push("vulkan");
    vk_layer_h_path.push("vk_layer.h");

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
         // limitations under the License.\n\n",
        current_year()
    );
    file.write_all(license_comments.as_bytes()).unwrap();

    bindgen::Builder::default()
        .with_rustfmt(rustfmt_path)
        .header(vk_layer_h_path.as_os_str().to_str().unwrap())
        .clang_args(&[
            "-I",
            vulkan_headers_include_dir.as_os_str().to_str().unwrap(),
        ])
        .default_enum_style(EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .allowlist_file("vk_layer.h")
        .allowlist_type("VkNegotiateLayerInterface")
        .allowlist_type("VkLayerDeviceCreateInfo")
        .allowlist_type("VkLayerFunction_?")
        .allowlist_type("VkLayerInstanceCreateInfo")
        .allowlist_type("VkNegotiateLayerStructType")
        .allowlist_type("PFN_GetPhysicalDeviceProcAddr")
        .allowlist_type("VkLayerInstanceLink_?")
        .allowlist_type("PFN_vkSetInstanceLoaderData")
        .allowlist_type("PFN_vkLayerCreateDevice")
        .allowlist_type("PFN_vkLayerDestroyDevice")
        .allowlist_type("VkLayerDeviceLink_?")
        .allowlist_type("PFN_vkSetDeviceLoaderData")
        // We expect ash to cover most type definitions for us.
        .allowlist_recursively(false)
        .generate()
        .unwrap()
        .write(Box::new(file))
        .unwrap();
    info!("The binding for vulkan_layer.h completes generation.");
}

fn guess_python_command() -> Option<&'static str> {
    ["python3", "python", "py"].into_iter().find(|cmd| {
        let output = match Command::new(cmd).arg("--version").output() {
            Ok(output) => output,
            Err(_) => return false,
        };
        output.status.success()
    })
}

struct GenvkArgs {
    script_path: PathBuf,
    target: OsString,
    registry: PathBuf,
    out_dir: PathBuf,
}

fn run_vulkan_layer_genvk(
    GenvkArgs {
        script_path,
        target,
        registry,
        out_dir,
    }: &GenvkArgs,
) {
    let mut output_file_path = out_dir.clone();
    output_file_path.push(target);
    info!("Generating {}...", output_file_path.display());
    let python_command = guess_python_command().expect("Failed to find installed python.");
    let status = Command::new(python_command)
        .args([
            script_path.as_os_str(),
            target,
            OsStr::new("-registry"),
            registry.as_os_str(),
            OsStr::new("-o"),
            out_dir.as_os_str(),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    assert!(status.success());

    let output_file = File::open(&output_file_path).unwrap();
    let formatted_content = Command::new("rustup")
        .args(["run", "nightly", "rustfmt", "--emit=stdout", "--quiet"])
        .stdin(output_file)
        .output()
        .unwrap()
        .stdout;
    let mut output_file = File::create(&output_file_path).unwrap();
    output_file.write_all(&formatted_content).unwrap();
    info!("{} completes generation.", output_file_path.display());
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

    let mut tasks: Vec<Box<dyn FnOnce() + Send + 'static>> = vec![];

    let mut vulkan_headers_dir = project_root_dir.clone();
    vulkan_headers_dir.push("third_party");
    vulkan_headers_dir.push("Vulkan-Headers");

    let mut vulkan_layer_dir = project_root_dir.clone();
    vulkan_layer_dir.push("vulkan-layer");
    let mut out_file = vulkan_layer_dir.clone();
    out_file.push("src");
    out_file.push("bindings");
    out_file.push("generated");
    out_file.push("vk_layer_bindings.rs");

    tasks.push({
        let vulkan_headers_dir = vulkan_headers_dir.clone();
        Box::new(move || generate_vulkan_layer_header_binding(&vulkan_headers_dir, &out_file))
    });

    let mut vulkan_layer_genvk_path = project_root_dir.clone();
    vulkan_layer_genvk_path.push("scripts");
    vulkan_layer_genvk_path.push("vulkan_layer_genvk.py");

    let mut vk_xml_path = vulkan_headers_dir.clone();
    vk_xml_path.push("registry");
    vk_xml_path.push("vk.xml");

    let mut vulkan_layer_generated_dir = project_root_dir.clone();
    vulkan_layer_generated_dir.push("vulkan-layer");
    vulkan_layer_generated_dir.push("src");
    vulkan_layer_generated_dir.push("generated");

    tasks.push({
        let genvk_args = GenvkArgs {
            script_path: vulkan_layer_genvk_path.clone(),
            target: OsString::from("layer_trait.rs"),
            registry: vk_xml_path.clone(),
            out_dir: vulkan_layer_generated_dir.clone(),
        };
        Box::new(move || run_vulkan_layer_genvk(&genvk_args))
    });
    tasks.push({
        let genvk_args = GenvkArgs {
            script_path: vulkan_layer_genvk_path.clone(),
            target: OsString::from("global_simple_intercept.rs"),
            registry: vk_xml_path.clone(),
            out_dir: vulkan_layer_generated_dir.clone(),
        };
        Box::new(move || run_vulkan_layer_genvk(&genvk_args))
    });
    let threads = tasks.into_iter().map(thread::spawn).collect::<Vec<_>>();
    for thread in threads {
        thread.join().unwrap();
    }
}
