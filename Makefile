prepare:
	rustup target add wasm32-unknown-unknown

build-contracts:
	cd vault && cargo build --release --target wasm32-unknown-unknown
	wasm-strip ./target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true

clippy:
	cd vault && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd vault && cargo fmt -- --check

lint: clippy
	cd vault && cargo fmt

clean:
	cd vault && cargo clean