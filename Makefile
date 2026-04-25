build:
	cargo build --release --workspace

format:
	cargo sort --workspace
	cargo fmt --all

lint:
	cargo clippy --release --all-targets