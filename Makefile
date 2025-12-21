build-program:
	cargo build-sbf --manifest-path router/Cargo.toml

build-client:
	cargo build --manifest-path client/Cargo.toml

build-all: build-program build-client

test-program: build-program
	cargo test --manifest-path router/Cargo.toml -- --no-capture

test-client: build-client
	cargo test --manifest-path client/Cargo.toml

test-all: test-program test-client
