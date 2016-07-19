build:
	cargo build

install:
	cargo install

run:
	./target/debug/lefortovo --lang Haskell

fmt:
	RUST_BACKTRACE=1 cargo fmt

test:
	RUST_BACKTRACE=1 cargo test

testv:
	RUST_BACKTRACE=1 cargo test -- --nocapture
