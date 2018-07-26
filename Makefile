default:
	@RUST_LOG=sleeping_forest=info cargo run

test:
	cargo test

clean:
	cargo clean

.PHONY: default test clean
