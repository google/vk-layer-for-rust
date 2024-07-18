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

use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, ensure, Context};
use clap::{Error as ClapError, Parser};
use glob::Pattern;
use ignore::gitignore::Gitignore;
use log::error;
use std::borrow::Borrow;
use std::io;
use walkdir::WalkDir;

const ABOUT: &str = "List files and forward the canonical path as the arguments. If the command \
                     failed, the exit code will be non-zero. If a file can't be accessed, the \
                     file will be skipped instead of triggering an error.";

#[derive(Parser, Debug)]
#[command(about = ABOUT)]
pub struct Cli {
    /// Globs to include in search. Paths that match any of include patterns will be included. If
    /// no include pattern is provided, "*" is used.
    #[arg(long)]
    pub include: Vec<String>,

    /// Globs to exclude from search. Paths that match any of exclude patterns will not be
    /// included.
    #[arg(long)]
    pub ignore: Vec<String>,

    /// Respect the .gitignore file. Will find the .gitignore file under the root directory.
    #[arg(long)]
    pub use_gitignore: bool,

    /// Search root.
    pub root: OsString,

    /// Command to run on the matching files.
    #[arg(last = true)]
    pub command: Vec<String>,
}

impl Cli {
    pub fn get_paths(&self) -> anyhow::Result<impl IntoIterator<Item = PathBuf>> {
        fn parse_pattern(pattern: &String) -> anyhow::Result<Pattern> {
            Pattern::new(pattern).map_err(|e| anyhow!("Failed to parse pattern {}: {}", pattern, e))
        }
        let include_patterns = self
            .include
            .iter()
            .map(parse_pattern)
            .collect::<anyhow::Result<Vec<_>>>()
            .context("parse include patterns")?;
        let ignore_patterns = {
            let mut ignore_patterns = self
                .ignore
                .iter()
                .map(parse_pattern)
                .collect::<anyhow::Result<Vec<_>>>()
                .context("parse ignore patterns")?;
            if self.use_gitignore {
                ignore_patterns
                    .push(Pattern::new("**/.git").expect("parsing .git pattern should succeed"));
            }
            ignore_patterns
        };

        let gitignore = if self.use_gitignore {
            let mut gitignore_path = PathBuf::from(&self.root);
            gitignore_path.push(".gitignore");
            let (gitignore, error) = Gitignore::new(&gitignore_path);
            if let Some(error) = error {
                error!(
                    "Failed to create a new gitignore matcher from {}: {}",
                    gitignore_path.display(),
                    error
                );
                None
            } else {
                Some(gitignore)
            }
        } else {
            None
        };
        let walk_dir = WalkDir::new(&self.root);
        let res = FileIteratorBuilder {
            include_patterns,
            ignore_patterns,
            gitignore,
            walk_dir,
        }
        .build()
        .filter_map(|path| match path {
            Ok(path) => Some(path),
            Err(e) => {
                error!("Failed to visit one path: {:?}", e);
                None
            }
        });
        Ok(res)
    }
}

pub fn parse_args<T>(args: impl IntoIterator<Item = T>) -> Result<Cli, ClapError>
where
    T: Into<OsString> + Clone,
{
    let mut cli = Cli::try_parse_from(args)?;
    if cli.include.is_empty() {
        cli.include.push("*".to_string());
    }
    Ok(cli)
}

struct FileIteratorBuilder {
    pub include_patterns: Vec<Pattern>,
    pub ignore_patterns: Vec<Pattern>,
    pub gitignore: Option<Gitignore>,
    pub walk_dir: WalkDir,
}

impl Default for FileIteratorBuilder {
    fn default() -> Self {
        Self {
            include_patterns: vec![],
            ignore_patterns: vec![],
            gitignore: None,
            walk_dir: WalkDir::new("."),
        }
    }
}

impl FileIteratorBuilder {
    fn build(self) -> FileIterator {
        let Self {
            include_patterns,
            ignore_patterns,
            gitignore,
            walk_dir,
        } = self;
        FileIterator {
            include_patterns,
            ignore_patterns,
            gitignore,
            current: walk_dir.into_iter(),
        }
    }
}

struct FileIterator {
    include_patterns: Vec<Pattern>,
    ignore_patterns: Vec<Pattern>,
    gitignore: Option<Gitignore>,
    current: walkdir::IntoIter,
}

impl Iterator for FileIterator {
    type Item = anyhow::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(res) = self.current.next() {
            let entry = match res {
                Ok(entry) => entry,
                Err(e) => return Some(Err(anyhow!("Failed to visit the next entry: {:?}", e))),
            };
            let mut should_ignore = self
                .ignore_patterns
                .iter()
                .any(|pat| pat.matches_path(entry.path()));
            let file_type = entry.file_type();
            if let Some(gitignore) = &self.gitignore {
                should_ignore = should_ignore
                    || gitignore
                        .matched(entry.path(), file_type.is_dir())
                        .is_ignore();
            }
            if should_ignore && file_type.is_dir() {
                self.current.skip_current_dir();
                continue;
            }
            if !file_type.is_file() {
                continue;
            }
            let mut should_include = !should_ignore;
            should_include = should_include
                && self
                    .include_patterns
                    .iter()
                    .any(|pat| pat.matches_path(entry.path()));
            if should_include {
                return Some(Ok(entry.into_path()));
            }
        }
        None
    }
}

pub struct TaskBuilder {
    command: Vec<String>,
}

impl TaskBuilder {
    pub fn new(command: Vec<String>) -> Self {
        Self { command }
    }

    pub fn build_task<'a>(&self, paths: impl IntoIterator<Item = &'a Path>) -> Task {
        if self.command.is_empty() {
            let mut to_print = OsString::new();
            let mut first = true;
            for path in paths {
                if first {
                    first = false;
                } else {
                    to_print.push(" ");
                }
                to_print.push(path);
            }
            Task::Print(to_print)
        } else {
            let mut command = std::process::Command::new(&self.command[0]);
            for arg in self.command.iter().skip(1) {
                command.arg(arg);
            }
            for path in paths {
                command.arg(path);
            }
            Task::SpawnProcess(command)
        }
    }
}

pub enum Task {
    Print(OsString),
    SpawnProcess(std::process::Command),
}

impl Task {
    pub fn run(
        self,
        stdout_buffer: &mut impl io::Write,
        stderr_buffer: &mut impl io::Write,
    ) -> anyhow::Result<()> {
        match self {
            Self::Print(to_print) => {
                let to_print = to_print.to_string_lossy();
                let to_print: &str = to_print.borrow();
                writeln!(stdout_buffer, "{}", to_print).context("print matched paths")
            }
            Self::SpawnProcess(mut command) => {
                let command_str = {
                    let mut command_strs =
                        vec![command.get_program().to_string_lossy().to_string()];
                    for arg in command.get_args() {
                        command_strs.push(arg.to_string_lossy().to_string());
                    }
                    let command = shlex::try_join(command_strs.iter().map(String::as_ref))
                        .expect("should not fail to join");
                    command
                };
                let output = command
                    .output()
                    .with_context(|| format!("spawn and wait for the command {}", &command_str))?;
                ensure!(output.status.success(), {
                    match &output.status.code() {
                        Some(exit_code) => format!(
                            "The process {} failed with exit code {}.",
                            command_str, exit_code
                        ),
                        None => format!("The process {} failed without an exit code.", command_str),
                    }
                });
                if let Err(e) = stdout_buffer.write_all(&output.stdout) {
                    error!(
                        "Failed to write the stdout for process {}: {:?}",
                        command_str, e
                    );
                }
                if let Err(e) = stderr_buffer.write_all(&output.stderr) {
                    error!(
                        "Failed to write the stderr for process {}: {:?}",
                        command_str, e
                    );
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        let args = [
            "progname",
            "--include=*.yaml",
            "--include",
            "*.yml",
            "--ignore=.git",
            "--ignore=__pycache__",
            "tmp/subdir",
            "--",
            "ls",
            "-lah",
        ];
        let cli = parse_args(args).expect("should parse successfully");
        assert_eq!(cli.include, vec!["*.yaml", "*.yml"]);
        assert_eq!(cli.ignore, vec![".git", "__pycache__"]);
        assert_eq!(cli.command, vec!["ls", "-lah"]);
        assert_eq!(cli.root, "tmp/subdir");
    }

    #[test]
    fn test_parse_args_missing_root() {
        let res = parse_args(["progname"]);
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_args_missing_include() {
        let cli = parse_args(["progname", "."]).expect("should parse successfully");
        assert_eq!(cli.include, vec!["*"]);
    }
}
