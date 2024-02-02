run-rust-hello-wasi:
	cd rust/bin/hello-wasi && cargo wasi run \
		&& wasm-tools component new ./target/wasm32-wasi/debug/hello_wasi.wasm \
			--adapt ../../../wasm/wasi_snapshot_preview1.wasm \
			-o ../../../wasm/components/rust_hello_wasi.wasm

build-rust-hello-guest:
	cd rust/lib/hello-guest \
	 	&& cargo build --target wasm32-wasi \
		&& wasm-tools component new ./target/wasm32-wasi/debug/hello_guest.wasm \
			--adapt wasi_snapshot_preview1=../../../wasm/wasi_snapshot_preview1.reactor.wasm \
			-o ../../../wasm/components/rust_hello_guest.wasm

run-rust-hello-host:
	cd rust/bin/hello-host && cargo run


