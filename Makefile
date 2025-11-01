.PHONY: all clean-all build 

all: clean-all build

build: 
	cargo build --release

clean-all:
	rm -rf cargo-test*
	cargo clean
	rm -rf ./target/debug

verify:
	cargo clippy --all-targets --all-features

fmt:
	rustfmt --check src/*.rs --edition 2024

cargo-fmt:
	cargo fmt
