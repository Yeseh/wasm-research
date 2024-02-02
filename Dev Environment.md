# WASM/WASI Dev Environment

This file should contain all the tools and workflows needed to start building WASM applications for one or more languages.


## OS
Most tooling is noticably developed on UNIX systems. Some things don't work right away on Windows, or at all.
This is mostly because the tooling is still very immature. Because of this a linux PC, devcontainer or WSL is highly recommended for WASM development.

The rest of this document will assume you are on a unix system (I'm using Ubuntu in WSL).

## Toolchain 

### General Tools 

Ensure gcc is installed:
> sudo apt-get install gcc

Install the rust programming language, configure the nightly toolchain when asked to modify the installation:
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Install `wasm-tools`:
> cargo install wasm-tools

Install `wit-deps`:
> cargo install --git https://github.com/bytecodealliance/wit-deps

Install `wit-bindgen`:
> cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli

Install `wasmtime`:
> curl https://wasmtime.dev/install.sh -sSf | bash

### VSCode extensions
- [Even Better Toml](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
- [Rust](https://marketplace.visualstudio.com/items?itemName=1YiB.rust-bundle)
- [WIT](https://marketplace.visualstudio.com/items?itemName=BytecodeAlliance.wit-idl)
- [WebAssembly](https://marketplace.visualstudio.com/items?itemName=dtsvet.vscode-wasm)

## Rust

Add the `wasm32-wasi` target
> rustup target add wasm32-wasi

Install `cargo wasi`:
> cargo install cargo-wasi

Install `cargo expand`:
> cargo install cargo-expand

### Binary setup

### Library setup

## GO
TODO

## .NET
TODO

## Installing WIT files
- Create a folder in the root of your project
