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

run-iq:
	./target/release/lab599 --iq-device "Sound Blaster Play! 3, USB Audio" --iq-rate 48000

run-audio:
	./target/release/lab599 --audio "PipeWire"

list-audio:
	./target/release/lab599 --list-audio

find-port:
	@ls /dev/serial/by-id/ 2>/dev/null | grep -i ftdi | sed 's|^|/dev/serial/by-id/|' \
	|| echo "(no FTDI device found in /dev/serial/by-id/)"

setup-deps-fedora:
	sudo dnf install -y systemd-devel alsa-lib-devel

setup-deps-ubuntu:
	sudo systemctl stop ModemManager
	sudo apt install -y build-essential pkg-config libasound2-dev libudev-dev
	sudo usermod -aG dialout $USER