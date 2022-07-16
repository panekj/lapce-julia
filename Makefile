all:

release: build-release copy
dev: build-dev copy

build-release:
	cargo build --target wasm32-wasi --release

build-dev:
	cargo build --target wasm32-wasi

copy:
	cp ./target/wasm32-wasi/release/lapce-*.wasm ./
