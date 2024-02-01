# WebAssembly ecosystem
This file contains an overview of the more important elements of the WebAssembly ecousystem. 

## Concepts

__WebAssembly Binary__: a .wasm file that represents a webassembly file in binary format. The specification of the binary format can be found [here](https://webassembly.github.io/spec/core/binary/index.html)

__WebAssembly Text__: an s-expression based human-readable text representation of a webassembly binary

__WIT__: WebAssembly Interface Types. An IDL meant for specifying the interface a webassembly component implements. WIT files are used to generate host/client code to interact with WASM binary files.

# Main Projects 


## WebAssembly
[__Organization__](https://github.com/WebAssembly)

Main github organization for the webassembly standards and tooling.

### __Important projects__

__WASI__

[__Repo__](https://github.com/WebAssembly/WASI)

Repo for standardizing the WebAssembly systems interface that allows WASM modules to interact with their execution environment. Currently on version 'preview1'. 

__Specification__

[__Repo__](https://github.com/WebAssembly/spec)

The specification and reference interpreter for core webassembly.

__Proposals__

[__Repo__](https://github.com/WebAssembly/proposals)

Tracking repository for proposals to modify the specification.

__WIT implementations__

The WebAssembly organization contains many implementations of WIT worlds that are in one stage of the standardization process or another. The most interesting are the `wasi-*` worlds that are going to be a part of wasi preview2 and preview3.

__WABT__

[__Repo__](https://github.com/WebAssembly/wabt)

Toolkit for interacting with WASM binaries. Main uses:

- Translating to and from WASM binary and WebAssembly Text
- Dumping metadata about wasm binaries 
- Manipulate sections


## Bytecode Alliance
People working to develop standards and tooling for WebAssembly. 

[__Repo__](https://github.com/bytecodealliance)

### __Important projects__

__Wasmtime__

[__Repo__](https://github.com/bytecodealliance/wasmtime)

A webassembly runtime with bindings for several programming languages. As this runtime is built by the people that are very closely connected to the WASM/WASI standardization process i trust this the most. It is going to have the latest features first and is going to be built with alignment to the specifications and standards.

- [__.NET Runtime__](https://github.com/bytecodealliance/wasmtime-dotnet)
- [__Go Runtime__](https://github.com/bytecodealliance/wasmtime-go)

__wasm-tools__

[__Repo__](https://github.com/bytecodealliance/wasm-tools)

 Webassembly toolkit. Similar to WABT but additionally includes utilities to work with WASM components. Prefer this over WABT or install both.

__wit-bindgen__

[__Repo__](https://github.com/bytecodealliance/wit-bindgen)

Tooling to generate source code in several different programming languages from WIT interfaces. Allows applications to interact with WASM components.

__wit-deps__

[__Repo__](https://github.com/bytecodealliance/wit-deps)

'Package manager' for WebAssembly interfaces. Allows a project to specify which interfaces are needed and keeps them up-to-date.

__cargo-component__

[__Repo__](https://github.com/bytecodealliance/cargo-component)

Cargo subcommand for creating WebAssembly components based on the component model proposal. Only used for rust-based component development.

__registry (WARG)__

[__Repo__](https://github.com/bytecodealliance/registry)

Specification and reference implementation of a webassembly component registry.

__wizer__

[__Repo__](https://github.com/bytecodealliance/wizer)

Tooling to pre-initialize some state into a webassembly binary. Might create significant startup time improvements. 


### Implementations

### Tools

## Third party projects

__spiderlightning__: A set of WebAssembly interfaces to build cloud applications with. Now deprecated and being standardized in WASI preview2 as the cloud-core world.

__krustlet__: run webassembly in kubernetes.




