# lab599-ctl

Terminal UI for real-time control of the Lab599 TX-500 transceiver. Builds to a single binary `lab599`.

## Hardware requirements

| Item | Detail |
|------|--------|
| Radio | Lab599 TX-500 or TX-500MP |
| Cable | CAT/USB cable (FTDI FT232R) |
| Port | `/dev/ttyUSB0` (Linux) or `COM*` (Windows) |
| Baud rate | **9600**, 8N1 |

On Linux, add your user to the `dialout` group:

```sh
sudo usermod -aG dialout $USER
```

## Dependency setup

**Fedora:**
```sh
sudo dnf install -y systemd-devel alsa-lib-devel
```

**Ubuntu/Debian:**
```sh
sudo apt install -y build-essential pkg-config libasound2-dev libudev-dev
```

## Build & run

```sh
cargo build --release
```

Add the binary to `PATH` to call it from anywhere:

```sh
# Option 1 — copy to a system-wide location
sudo install -m755 lab599 /usr/local/bin/lab599

# Option 2 — add to ~/.bashrc or ~/.zshrc
export PATH="$PATH:/path/to/lab599"
```

```sh
# Auto-detects the TX-500 on the first FTDI port
lab599

# Explicit serial port
lab599 --port <PORT>                          # e.g. /dev/ttyUSB0

# With IQ spectrum display
lab599 --iq-device <IQ-DEVICE>                # your audio device name, run with --list-audio to see available
lab599 --iq-device <IQ-DEVICE> --iq-rate <RATE>   # custom sample rate, e.g. 48000

# With RX audio loopback
lab599 --audio <AUDIO-DEVICE>                 # e.g. "PipeWire"

# List available audio input devices
lab599 --list-audio

# Find the TX-500 serial port (prints ready-to-use path)
ls /dev/serial/by-id/ | grep -i ftdi | xargs -I{} readlink -f /dev/serial/by-id/{}
```

## CLI options

| Flag | Default | Description |
|------|---------|-------------|
| `--port <PATH>` | — | Serial port (e.g. `/dev/ttyUSB0`) |
| `--baud <N>` | `9600` | Baud rate |
| `--poll-ms <N>` | `200` | CAT poll interval in milliseconds |
| `--audio <NAME>` | — | Audio input device (substring match) |
| `--list-audio` | — | Print available audio inputs and exit |
| `--iq-device <NAME>` | — | IQ input device for spectrum display |
| `--iq-rate <N>` | `192000` | IQ sample rate in Hz |
| `--rx-socket <PATH>` | `/tmp/lab599-rx.sock` | Unix socket for RX audio streaming |

## Remote audio

Stream RX audio over SSH to a remote machine:

```sh
# On the remote host — run lab599 and forward the Unix socket over SSH
ssh -t -L /tmp/lab599-rx.sock:/tmp/lab599-rx.sock <USER@HOST> "lab599 --audio '<AUDIO_DEVICE>'"

# On the local host — play the streamed audio
nc -U /tmp/lab599-rx.sock | aplay -f FLOAT_LE -r <RATE> -c 1
```

## Pages

Navigate between pages with `Tab`.

| Page | Content |
|------|---------|
| **Main** | Frequency, S-meter, status flags, IQ spectrum |
| **Help** | Key binding reference |
| **Logs** | Error log |

## Key bindings

### Frequency

| Key | Action |
|-----|--------|
| `←` / `→` | Tune by current step |
| `↑` / `↓` | Step size up / down |
| `+` / `PgUp` | Jump +1 MHz |
| `-` / `PgDn` | Jump −1 MHz |
| `[` / `]` | Band down / up |

Step sizes cycle through: 10 Hz → 100 → 500 → 1 kHz → 2.5 kHz → 5 kHz → 10 kHz.

### Radio controls

| Key | Action |
|-----|--------|
| `m` | Cycle mode: LSB → USB → CW → CW-R → AM → FM → DIG |
| `f` | Cycle RX filter: FIL-1 → FIL-2 → FIL-3 → FIL-4 |
| `p` | Toggle pre-amplifier |
| `a` | Toggle attenuator (−20 dB) |
| `s` | Toggle split (TX on VFO B) |
| `c` | Toggle speech compressor (CMR) |
| `t` | Toggle TX / RX |
| `v` | Toggle VOX |
| `n` | Toggle noise reduction (NR) |
| `b` | Toggle noise blanker (NB) |
| `x` | Toggle notch filter |
| `o` | Toggle TX monitor |
| `d` | Toggle DSP IF |
| `z` | Toggle DC spike suppression on spectrum |
| `Tab` | Next page |
| `q` / `Ctrl+C` | Quit |
