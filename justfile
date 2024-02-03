
build component adapter="reactor":
	cd crates/component-{{ component }} && cargo build --target wasm32-wasi 
	wasm-tools component new ./target/wasm32-wasi/debug/component_{{replace(component, '-', '_') }}.wasm \
		--adapt wasi_snapshot_preview1=./wasm/wasi_snapshot_preview1.{{adapter}}.wasm \
		-o ./wasm/components/rust_{{ replace(component, '-', '_') }}.wasm

run-rust-hello-host:
	cd rust/bin/hello-host && cargo run

build-all: (build "greeter") (build "wasi-fs")
