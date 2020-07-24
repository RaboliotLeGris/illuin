.DEFAULT_GOAL := build-release
.PHONY: all run build build-release clean lint test check CI

build:
	cargo build

build-release:
	cargo build --release

clean:
	rm -rf target/

run:
	cargo run

lint:
	cargo clippy

test:
	cargo test

check:
	cargo check

CI: check lint test