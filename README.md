# Godot + Rust Template

This is a template repository for starting [Godot](https://godotengine.org/) projects using the Rust programming language. It includes a minimal setup with build scripts for Windows, Linux, and macOS, and is configured to work with [godot-rust](https://github.com/godot-rust/gdext).

## Features

- Preconfigured integration between Rust and Godot 4.2+
- Cross-platform build scripts
- Minimal Godot project structure
- Rust module ready for FFI-based Godot extension

## Requirements

- **Godot**: version **4.2 or newer**
- **Rust**: version **1.86.0 or newer** (latest stable recommended)
- `cargo`, `rustup`, and standard Rust toolchain installed
- installed 'cargo-make' for build scripting. Future developer can install 'cargo-make' by running command 'cargo install cargo-make'
- **Python**: version **3**

## How to build project

## TODO

- [X] Add support for passing compilation target as an argument to build scripts
- [X] Add support for passing debug flag as an argument to build scripts
- [ ] Create a build script for compiling to WebAssembly (WASM)
- [X] Create a build script for compiling to Linux
- [ ] Auto generate and install .gdextension file
- [ ] CI/CD update based on docker/podman
