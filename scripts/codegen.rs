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

use imara_diff::{intern::InternedInput, Sink as _, UnifiedDiffBuilder};
use log::{debug, info};
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
    sync::mpsc,
    thread,
    time::SystemTime,
};
use tempfile::TempDir;

use bindgen::EnumVariation;
use clap::Parser;

type Task = Box<dyn FnOnce() + Send + 'static>;

fn create_file_all(path: impl AsRef<Path>) -> io::Result<File> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)
}

// May not be precise, but should be enough for generating copyright comments.
fn current_year() -> u64 {
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    const SECONDS_PER_YEAR: u64 = 60 * 60 * 24 * 365;
    since_epoch.as_secs() / SECONDS_PER_YEAR + 1970
}

fn generate_vulkan_layer_header_binding(
    vulkan_headers_dir: &Path,
    out_dir: &Path,
    completed_files: mpsc::Sender<PathBuf>,
) -> Vec<Task> {
    info!("Preparing the task to generate vulkan_layer.h...");

    let out_file_relative_path = PathBuf::from_iter(["bindings", "vk_layer", "generated.rs"]);
    let out_file = PathBuf::from_iter([out_dir.to_owned(), out_file_relative_path.clone()]);
    let mut platform_specific_out_relative_path =
        PathBuf::from_iter(["bindings", "vk_layer", "generated"]);
    #[cfg(windows)]
    platform_specific_out_relative_path.push("windows.rs");
    #[cfg(unix)]
    platform_specific_out_relative_path.push("unix.rs");
    let platform_specific_out = PathBuf::from_iter([
        out_dir.to_owned(),
        platform_specific_out_relative_path.clone(),
    ]);

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
    let set_common_bindgen_configs = {
        let rustfmt_path = rustfmt_path.to_owned();
        move |bindgen_builder: bindgen::Builder| -> bindgen::Builder {
            bindgen_builder
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
                .derive_default(true)
                // We expect ash to cover most type definitions for us.
                .allowlist_recursively(false)
        }
    };

    let mut tasks: Vec<Task> = vec![];
    tasks.push(Box::new({
        let license_comments = license_comments.clone();
        let set_common_bindgen_configs = set_common_bindgen_configs.clone();
        let completed_files = completed_files.clone();
        move || {
            info!("generating bindings for vk_layer.h...");
            let mut out_file = create_file_all(out_file).unwrap();
            let mut out_preamble = license_comments;
            out_preamble.push_str(
                "#![allow(missing_docs)]
use super::*;
use ash::vk::*;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;
#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
pub use windows::*;
",
            );
            out_file.write_all(out_preamble.as_bytes()).unwrap();
            set_common_bindgen_configs(Default::default())
                .allowlist_file("vk_layer.h")
                .allowlist_type("VkNegotiateLayerInterface")
                .allowlist_type("VkLayerDeviceCreateInfo")
                .allowlist_type("PFN_GetPhysicalDeviceProcAddr")
                .allowlist_type("VkLayerInstanceLink_?")
                .allowlist_type("PFN_vkSetInstanceLoaderData")
                .allowlist_type("PFN_vkLayerCreateDevice")
                .allowlist_type("PFN_vkLayerDestroyDevice")
                .allowlist_type("VkLayerDeviceLink_?")
                .allowlist_type("PFN_vkSetDeviceLoaderData")
                .generate()
                .unwrap()
                .write(Box::new(out_file))
                .unwrap();
            info!("The binding for vk_layer.h completes generation.");
            completed_files
                .send(out_file_relative_path.clone())
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to send completed file for {}: {}",
                        out_file_relative_path.display(),
                        e
                    )
                });
        }
    }));

    tasks.push(Box::new({
        let platform_specific_out = platform_specific_out.to_owned();
        move || {
            info!("generating platform specific bindings for vk_layer.h...");
            let mut platform_specific_out_file = create_file_all(platform_specific_out).unwrap();
            platform_specific_out_file
                .write_all(license_comments.as_bytes())
                .unwrap();

            set_common_bindgen_configs(Default::default())
                .allowlist_type("VkLayerFunction_?")
                .allowlist_type("VkNegotiateLayerStructType")
                .generate()
                .unwrap()
                .write(Box::new(platform_specific_out_file))
                .unwrap();
            info!("The platform specific binding for vk_layer.h completes generation.");
            completed_files
                .send(platform_specific_out_relative_path.clone())
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to send completed file for {}: {}",
                        platform_specific_out_relative_path.display(),
                        e
                    )
                });
        }
    }));

    tasks
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
    let target = PathBuf::from(target);
    let mut output_file_path = out_dir.clone();
    output_file_path.push(target.clone());
    info!("Generating {}...", output_file_path.display());
    if let Some(parent) = output_file_path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|e| {
            panic!(
                "Failed to create the directory {} for genvk.py output {}: {e}",
                parent.display(),
                output_file_path.display()
            )
        });
    }
    let python_command = guess_python_command().expect("Failed to find installed python.");
    let status = Command::new(python_command)
        .args([
            script_path.as_os_str(),
            target.as_os_str(),
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
    let output = Command::new("rustup")
        .args(["run", "nightly", "rustfmt", "--emit=stdout", "--quiet"])
        .stdin(output_file)
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
    assert!(output.status.success(), "Format failed.");
    let formatted_content = output.stdout;
    let mut output_file = create_file_all(&output_file_path).unwrap();
    output_file.write_all(&formatted_content).unwrap();
    info!("{} completes generation.", output_file_path.display());
}

#[derive(Parser)]
struct Cli {
    /// Run in 'check' mode. Exits with 0 if the generated code is the same as the existing local
    /// files. Exits with 1 and prints a diff if it is different.
    #[arg(long)]
    check: bool,
}

fn main() -> ExitCode {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .filter_module("bindgen", log::LevelFilter::Error)
        .init();
    let cli = Cli::parse();

    let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    assert!(cargo_manifest_dir.is_absolute());
    let mut project_root_dir = cargo_manifest_dir;
    assert!(project_root_dir.pop());

    let mut _tempdir: Option<TempDir> = None;
    let target_root_dir = if cli.check {
        let dir = tempfile::tempdir().expect("Should create the temporary directory successfully.");
        let path = dir.path().to_owned();
        _tempdir = Some(dir);
        path
    } else {
        project_root_dir.clone()
    };

    let mut tasks: Vec<Task> = vec![];

    let mut vulkan_headers_dir = project_root_dir.clone();
    vulkan_headers_dir.push("third_party");
    vulkan_headers_dir.push("Vulkan-Headers");

    let mut vulkan_layer_src_dir = target_root_dir.clone();
    vulkan_layer_src_dir.push("vulkan-layer");
    vulkan_layer_src_dir.push("src");

    let (completed_files_tx, completed_files_rx) = mpsc::channel();
    tasks.append(&mut generate_vulkan_layer_header_binding(
        &vulkan_headers_dir,
        &vulkan_layer_src_dir,
        completed_files_tx.clone(),
    ));

    let mut vulkan_layer_genvk_path = project_root_dir.clone();
    vulkan_layer_genvk_path.push("scripts");
    vulkan_layer_genvk_path.push("vulkan_layer_genvk.py");

    let mut vk_xml_path = vulkan_headers_dir.clone();
    vk_xml_path.push("registry");
    vk_xml_path.push("vk.xml");

    tasks.push({
        let target = OsString::from("layer_trait/generated.rs");
        let target_relative_path = PathBuf::from(target.clone());
        let genvk_args = GenvkArgs {
            script_path: vulkan_layer_genvk_path.clone(),
            target: target.clone(),
            registry: vk_xml_path.clone(),
            out_dir: vulkan_layer_src_dir.clone(),
        };
        let completed_files_tx = completed_files_tx.clone();
        Box::new(move || {
            run_vulkan_layer_genvk(&genvk_args);
            completed_files_tx
                .send(target_relative_path)
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to send completed file for {}: {}",
                        target.to_string_lossy().as_ref(),
                        e
                    )
                });
        })
    });
    tasks.push({
        let target = OsString::from("global_simple_intercept/generated.rs");
        let target_relative_path = PathBuf::from(target.clone());
        let genvk_args = GenvkArgs {
            script_path: vulkan_layer_genvk_path.clone(),
            target: target.clone(),
            registry: vk_xml_path.clone(),
            out_dir: vulkan_layer_src_dir.clone(),
        };
        let completed_files_tx = completed_files_tx.clone();
        Box::new(move || {
            run_vulkan_layer_genvk(&genvk_args);
            completed_files_tx
                .send(target_relative_path)
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to send completed file for {}: {}",
                        target.to_string_lossy().as_ref(),
                        e
                    )
                });
        })
    });
    drop(completed_files_tx);
    let threads = tasks.into_iter().map(thread::spawn).collect::<Vec<_>>();
    let exit_code = if cli.check {
        let has_diff = thread::scope(move |s| {
            let (diff_message_tx, diff_message_rx) = mpsc::channel();
            while let Ok(completed_file) = completed_files_rx.recv() {
                s.spawn({
                    let before_path = PathBuf::from_iter([
                        project_root_dir.clone(),
                        PathBuf::from("vulkan-layer"),
                        PathBuf::from("src"),
                        completed_file.clone(),
                    ]);
                    let after_path = PathBuf::from_iter([
                        target_root_dir.clone(),
                        PathBuf::from("vulkan-layer"),
                        PathBuf::from("src"),
                        completed_file.clone(),
                    ]);
                    let diff_message_tx = diff_message_tx.clone();
                    move || {
                        debug!(
                            "Comparing {} and {}",
                            before_path.display(),
                            after_path.display()
                        );
                        let before = if before_path.exists() {
                            fs::read_to_string(&before_path).unwrap_or_else(|e| {
                                panic!(
                                    "Failed to read {} as before for diff: {}",
                                    before_path.display(),
                                    e
                                )
                            })
                        } else {
                            info!("File to diff {} doesn't exist.", before_path.display());
                            "".to_owned()
                        };
                        let after = if after_path.exists() {
                            fs::read_to_string(&after_path).unwrap_or_else(|e| {
                                panic!(
                                    "Failed to read {} as after for diff: {}",
                                    after_path.display(),
                                    e
                                )
                            })
                        } else {
                            info!("File to diff {} doesn't exist.", after_path.display());
                            "".to_owned()
                        };
                        let input = InternedInput::new(before.as_str(), after.as_str());
                        let diff_builder = UnifiedDiffBuilder::new(&input).with_counter();
                        let diff_result = imara_diff::diff(
                            imara_diff::Algorithm::Histogram,
                            &input,
                            diff_builder,
                        );
                        let diff_message = if diff_result.total() == 0 {
                            None
                        } else {
                            let mut message = completed_file.to_string_lossy().into_owned();
                            message.push('\n');
                            message.push_str(&diff_result.wrapped);
                            Some(message)
                        };
                        diff_message_tx.send(diff_message).unwrap_or_else(|e| {
                            panic!(
                                "Failed to send the diff message for {}: {}",
                                completed_file.display(),
                                e
                            )
                        });
                    }
                });
            }
            drop(diff_message_tx);
            let mut has_diff = false;
            let mut num_of_files = 0;
            while let Ok(diff_message) = diff_message_rx.recv() {
                num_of_files += 1;
                if let Some(message) = diff_message {
                    has_diff = true;
                    println!("{}", message);
                }
            }
            if !has_diff {
                info!("{} files would be left unchanged.", num_of_files);
            }
            has_diff
        });
        if has_diff {
            ExitCode::from(1)
        } else {
            ExitCode::from(0)
        }
    } else {
        ExitCode::from(0)
    };
    for thread in threads {
        thread.join().unwrap();
    }
    exit_code
}
