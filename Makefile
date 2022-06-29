prepare:
	rustup target add wasm32-unknown-unknown

build-contracts:
	cargo build --release --target wasm32-unknown-unknown
	wasm-strip ./target/wasm32-unknown-unknown/release/vault.wasm 2>/dev/null | true
	wasm-strip ./target/wasm32-unknown-unknown/release/remove_named_key.wasm 2>/dev/null | true
	wasm-strip ./target/wasm32-unknown-unknown/release/purse_transfer.wasm 2>/dev/null | true
	wasm-strip ./target/wasm32-unknown-unknown/release/create_writable_purse.wasm 2>/dev/null | true

clippy:
	cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

clean:
	cargo clean