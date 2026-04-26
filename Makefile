build:
	cargo build --release --workspace

build-all:
	cargo build --release --workspace --features lab599-cat-device/tx500mp

tests:
	cargo test --workspace --features lab599-cat-device/tx500mp --exclude lab599-cat-tui

format:
	cargo sort --workspace
	cargo fmt --all

lint:
	cargo clippy --release --all-targets

tui-build:
	cargo build --release -p lab599-cat-tui

tui-run:
	./target/release/lab599-cat-tui --port /dev/ttyUSB0

tui-list-audio:
	./target/release/lab599-cat-tui --list-audio

tui-with-audio:
	./target/release/lab599-cat-tui --port /dev/ttyUSB0 --audio pipewire

setup-deps:
	sudo dnf install -y systemd-devel alsa-lib-devel
