build:
	cargo build --release --workspace

build-all:
	cargo build --release --workspace --features lab599-cat-device/tx500mp

test:
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

# Install system dependencies required by serialport and cpal (audio).
# Detects Fedora/RHEL (dnf) or Debian/Ubuntu (apt).
setup-deps:
	@if command -v dnf >/dev/null 2>&1; then \
		echo "Detected dnf (Fedora/RHEL)"; \
		sudo dnf install -y systemd-devel alsa-lib-devel; \
	elif command -v apt-get >/dev/null 2>&1; then \
		echo "Detected apt (Debian/Ubuntu)"; \
		sudo apt-get install -y libudev-dev libasound2-dev; \
	else \
		echo "Unknown package manager. Install manually: libudev-dev, libasound2-dev (or equivalent)"; \
		exit 1; \
	fi
