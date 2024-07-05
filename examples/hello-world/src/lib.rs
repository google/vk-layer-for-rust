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

// Disable the non_snake_case lint so that we can have non-snake-case binary name which is required
// Android. See https://github.com/rust-lang/rust/issues/45127 for details on Rust lint.
#![allow(non_snake_case)]

// All the implementation is in a separate module with the non_snake_case lint reenabled.
#[warn(non_snake_case)]
mod r#impl;
