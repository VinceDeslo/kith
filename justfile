__default:
    just --list

fmt:
    alejandra .
    cargo fmt

run:
    cargo run --quiet src/main.rs

debug:
	RUST_LOG=debug cargo run src/main.rs

info:
	RUST_LOG=info cargo run src/main.rs
