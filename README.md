# lab599-cat

Rust implementation of the CAT protocol for the Lab599 TX-500 transceiver. Works with both **Lab599 native** (rev. 3) and **TS-2000 compatible** modes.

The TX-500 supports two CAT modes (Menu 25: `LAB599` / `TS-2000`). Most commands work in both modes. The following are confirmed supported in TS-2000 mode:

```
ID  AI  RX  TX  IF  FA  FB  MD  FR  FT  FN  PA  RA
```

Commands outside this set require **Lab599 native mode** — notably `PR` (speech compressor / CMR).

---

## Workspace layout

```
lab599-cat/      — protocol types + Tx500<T> driver
examples/tui/    — terminal UI (ratatui) for live radio control
```

---

## Hardware requirements

| Item | Detail |
|---|---|
| Radio | Lab599 TX-500 |
| Cable | CAT/USB cable (FTDI FT232R) |
| Port | `/dev/ttyUSB0` (Linux) or `COM*` (Windows) |
| Baud rate | **9600**, 8N1 |

On Linux, add your user to the `dialout` group to access the serial port without `sudo`:

```sh
sudo usermod -aG dialout $USER
```

---

## TUI

A terminal UI for real-time radio control.

### Build & run

```sh
make tui-build        # build release binary
make tui-run          # connect to /dev/ttyUSB0
make tui-with-audio   # connect + audio loopback (pipewire)
make tui-list-audio   # list available audio input devices
```

### CLI options

| Flag | Default | Description |
|---|---|---|
| `--port <PATH>` | required | Serial port (e.g. `/dev/ttyUSB0`) |
| `--baud <N>` | `9600` | Baud rate |
| `--poll-ms <N>` | `500` | Poll interval in milliseconds |
| `--audio <NAME>` | — | Audio input device substring match |
| `--list-audio` | — | Print available audio inputs and exit |

### Key bindings

| Key | Action |
|---|---|
| `←` / `→` | Tune frequency by current step |
| `↑` / `↓` | Step size up / down (10 Hz → 100 → 500 → 1k → 2.5k → 5k → 10k) |
| `+` / `-` or `PgUp` / `PgDn` | Jump ±1 MHz |
| `m` | Cycle modulation mode (LSB → USB → CW → CW-R → AM → FM → DIG) |
| `f` | Cycle RX filter (FIL-1 … FIL-4) |
| `p` | Toggle pre-amp (boosts weak signals) |
| `a` | Toggle attenuator (−20 dB, cuts strong interference) |
| `s` | Toggle split (TX on VFO B, RX on VFO A) |
| `c` | Toggle speech compressor (evens mic level for SSB voice) |
| `t` | Toggle TX / RX |
| `q` / `Ctrl+C` | Quit |

### S-meter scale

Matches the TX-500 display: S1 · S3 · S5 · S7 · S9 · S9+20 · S9+40 · S9+60.  
The bar is color-coded: green (≤ S9), yellow (S9+20), red (S9+40 and above).

---

## Library usage

`Tx500<T>` is generic over any `Read + Write` transport — use a real serial port in production, or a `std::io::Cursor` in tests.

```rust
use lab599_cat::Tx500;

// Real serial port
let port = serialport::new("/dev/ttyUSB0", 9600)
    .timeout(Duration::from_millis(2000))
    .open()?;
let mut radio = Tx500::new(port);

// Query and set
let freq = radio.get_frequency_a()?;       // → u64 Hz
radio.set_frequency_a(7_048_500)?;

let mode = radio.get_mode()?;              // → Mode (Lsb / Usb / Cw / …)
radio.set_mode(Mode::Usb)?;

let s = radio.get_smeter()?;              // → u16, range 0–30
radio.set_preamp(true)?;
radio.set_attenuator(false)?;
radio.set_split(true)?;
radio.set_speech_compressor(true)?;
```

### Feature flags

| Feature | Enables |
|---|---|
| `tx500` | `Tx500<T>` driver + TX-500-specific commands: `BD`, `BU`, `SP`, `VV`, `XT` |
| `tx500mp` | TX-500MP-specific commands: `AC` (antenna tuner), `CT` (CTCSS). Implies `tx500`. |

Add to your `Cargo.toml`:

```toml
lab599-cat = { version = "0.1", features = ["tx500"] }
```

---

## Protocol notes

- Commands and responses are ASCII strings terminated with `;`
- Frequency is zero-padded to 11 digits: `FA00007048500;`
- Filter index is **0-based** in the protocol (FL-0 … FL-3), displayed as FIL-1 … FIL-4
- S-meter range is **0–30** (not 0–9 as in Kenwood TS-2000 compatible sets)
- `SM` command reads main receiver signal meter (`SM0;`)

---

## Running tests

```sh
cargo test
```

Tests use an in-memory mock transport — no hardware required.
