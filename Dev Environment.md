# WASM/WASI Dev Environment

This file should contain all the tools and workflows needed to start building WASM applications for one or more languages.


## OS
Most tooling is noticably developed on UNIX systems. Some things don't work right away on Windows, or at all.
This is mostly because the tooling is very immature. Because of this a linux PC, devcontainer or WSL is highly recommended for WASM development.

The rest of this document will assume you are on a unix system (I'm using Ubuntu in WSL).

## Rust 

### Installation

Ensure gcc is installed:
> sudo apt-get install gcc

Install the rust programming language using the nightly toolchain:
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Add the `wasm32-wasi` target
> rustup target add wasm32-wasi

Install `wasm-tools` from bytecodalliance:
> cargo install wasm-tools

### Binary setup

### Library setup

## GO
TODO

## .NET
TODO
