# Copyright 2024 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

{ pkgs, ... }: {
  channel = "stable-23.11";
  env =
    let
      getRustBindgenEnv = env-name:
        let
          runCommand = (import <nixpkgs> { }).runCommand;
          fileContents = (import <nixpkgs> { }).lib.strings.fileContents;
          setupScriptPath = "${pkgs.rustPlatform.bindgenHook}/nix-support/setup-hook";
          store-path = runCommand
            "get-rust-bindgen-${env-name}"
            { }
            ". ${setupScriptPath} && $postHook && echo \$${env-name} >$out";
          command-output-str = fileContents (builtins.toPath store-path);
        in
        command-output-str;
    in
    {
      BINDGEN_EXTRA_CLANG_ARGS = getRustBindgenEnv "BINDGEN_EXTRA_CLANG_ARGS";
      LIBCLANG_PATH = getRustBindgenEnv "LIBCLANG_PATH";
    };
  packages = [
    pkgs.clang
    pkgs.libclang
    pkgs.llvmPackages.libllvm
    pkgs.llvmPackages.stdenv
    pkgs.llvmPackages.libcxxStdenv
    pkgs.llvmPackages.libcxx
    pkgs.llvmPackages.libcxxClang
    pkgs.llvmPackages.bintools
    pkgs.python39
    pkgs.rustup
    pkgs.cargo-nextest
    pkgs.cargo-make
    pkgs.git
    pkgs.pipenv
    pkgs.go
    pkgs.taplo
  ];
  idx = {
    extensions = [
      "rust-lang.rust-analyzer"
      "ms-python.python"
      "ms-python.debugpy"
      "ms-python.black-formatter"
      "tamasfe.even-better-toml"
    ];
    workspace = {
      onCreate = {
        git-clone-submodules = "git submodule update --init --recursive";
        rust-toolchain-stable-install = "rustup toolchain install stable";
        rust-toolchain-nightly-install =
          "rustup toolchain install nightly --component miri rustfmt rust-src --profile minimal";
        pipenv-install = "pipenv install --ignore-pipfile";
        addlicense-install = "go install github.com/google/addlicense@master";
      };
    };
  };
}
