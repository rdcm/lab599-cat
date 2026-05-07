build:
	cargo build --release --workspace

build-all:
	cargo build --release --workspace --features lab599-cat/tx500mp

tests:
	cargo nextest run --workspace --features lab599-cat/tx500mp --exclude lab599-ctl

format:
	cargo sort --workspace
	cargo fmt --all

lint:
	cargo clippy --release --all-targets

run:
	./target/release/lab599 --port /dev/ttyUSB0

run-audio:
	./target/release/lab599 --port /dev/ttyUSB0 --audio pipewire

list-audio:
	./target/release/lab599 --list-audio

setup-deps-fedora:
	sudo dnf install -y systemd-devel alsa-lib-devel

setup-deps-ubuntu:
	sudo apt install -y build-essential pkg-config libasound2-dev libudev-dev