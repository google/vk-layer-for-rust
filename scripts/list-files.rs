// Copyright 2024 Google LLC
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

use clap::Parser;
use glob::{glob, GlobError, Pattern, PatternError};
use log::error;
use std::fs;
use std::{
    path::{Path, PathBuf},
    process::{self, ExitCode},
};

const ABOUT: &str = "List files and forward the canonical path as the arguments. If the command \
                     failed, the exit code will be non-zero. If a file can't be accessed, the \
                     file will be skipped instead of triggering an error.";

#[derive(Parser, Debug)]
#[command(about = ABOUT)]
struct Cli {
    /// Globs to start search.
    #[arg(long)]
    include: Vec<String>,

    /// Globs to exclude from search.
    #[arg(long)]
    ignore: Vec<String>,

    /// Command to run on the matching files.
    #[arg(last = true)]
    command: Vec<String>,
}

fn included_paths(
    include_patterns: &[String],
) -> Result<impl Iterator<Item = Result<PathBuf, GlobError>>, PatternError> {
    let paths = include_patterns
        .iter()
        .map(|pat| (pat, glob(pat)))
        .map(|(pat, res)| {
            if let Err(e) = &res {
                error!("Failed to parse the pattern {}: {:?}", pat, e);
            }
            res
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(paths.into_iter().flatten())
}

fn build_patterns(patterns: &[String]) -> Result<Vec<Pattern>, PatternError> {
    patterns
        .iter()
        .map(|pat| (pat, Pattern::new(pat)))
        .map(|(pat, res)| {
            if let Err(e) = &res {
                error!("Failed to parse the pattern {}: {:?}", pat, e);
            }
            res
        })
        .collect()
}

fn main() -> ExitCode {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();
    let cli = Cli::parse();
    let paths = match included_paths(&cli.include) {
        Ok(paths) => paths,
        Err(_) => {
            error!("Failed to parse the include pattern.");
            return ExitCode::FAILURE;
        }
    };
    let exclude_patterns = match build_patterns(&cli.ignore) {
        Ok(exclude_patterns) => exclude_patterns,
        Err(_) => {
            error!("Failed to parse the exclude pattern.");
            return ExitCode::FAILURE;
        }
    };

    let mut maybe_command = None;
    let mut visitor: Box<dyn FnMut(&Path)> = if cli.command.is_empty() {
        Box::new(|path: &Path| println!("{}", path.display()))
    } else {
        let mut command = process::Command::new(&cli.command[0]);
        command.args(&cli.command[1..]);
        maybe_command = Some(command);
        Box::new(|path: &Path| {
            maybe_command
                .as_mut()
                .expect("command must be set")
                .arg(path.as_os_str());
        })
    };
    for path in paths {
        let path = match path {
            Ok(path) => path,
            Err(e) => {
                error!("Failed to get the path: {:?}", e);
                continue;
            }
        };
        if exclude_patterns
            .iter()
            .any(|ignore_pattern| ignore_pattern.matches_path(&path))
        {
            continue;
        }
        let file_metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(e) => {
                error!("Failed to read the metadata of {}: {}", path.display(), e);
                continue;
            }
        };
        if !file_metadata.is_file() {
            continue;
        }
        visitor(&path);
    }
    drop(visitor);
    if let Some(mut command) = maybe_command {
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(e) => {
                error!("Failed to spawn the command: {}", e);
                return ExitCode::FAILURE;
            }
        };
        let output = match child.wait() {
            Ok(output) => output,
            Err(e) => {
                error!("Failed to wait for the process: {}", e);
                return ExitCode::FAILURE;
            }
        };
        if output.success() {
            return ExitCode::from(0);
        } else {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::from(0)
}
