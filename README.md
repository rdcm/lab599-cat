# lab599-cat

Rust implementation of the CAT protocol for the Lab599 TX-500 transceiver. Works with both **Lab599 native** and **TS-2000 compatible** modes.

The TX-500 supports two CAT modes (`LAB599` / `TS-2000`). Most commands work in both modes. The following are confirmed supported in TS-2000 mode:

```
ID  AI  RX  TX  IF  FA  FB  MD  FR  FT  FN  PA  RA
```

Commands outside this set require **Lab599 native mode** — notably `PR` (speech compressor / CMR) and `RM` (meter).

---

## Workspace layout

```
lab599-cat/    — protocol types + Tx500<T> driver
lab599-ctl/    — terminal UI app, binary: lab599
```

---

## Hardware requirements

| Item | Detail |
|---|---|
| Radio | Lab599 TX-500 or TX-500MP |
| Cable | CAT/USB cable (FTDI FT232R) |
| Port | `/dev/ttyUSB0` (Linux) or `COM*` (Windows) |
| Baud rate | **9600**, 8N1 |

On Linux, add your user to the `dialout` group to access the serial port without `sudo`:

```sh
sudo usermod -aG dialout $USER
```

---

## lab599-ctl

Terminal UI for real-time radio control. Builds to a single binary `lab599`. On startup it queries the device ID (`ID;`) and displays the model name in the UI header.

### Build & run

```sh
make build          # build all
make run            # connect to /dev/ttyUSB0
make run-audio      # connect + audio loopback (pipewire)
make list-audio     # list available audio input devices
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
| `[` / `]` | Band down / up |
| `m` | Cycle modulation mode (LSB → USB → CW → CW-R → AM → FM → DIG) |
| `f` | Cycle RX filter (FIL-1 … FIL-4) |
| `p` | Toggle pre-amp |
| `a` | Toggle attenuator (−20 dB) |
| `s` | Toggle split (TX on VFO B, RX on VFO A) |
| `c` | Toggle speech compressor (CMR) |
| `t` | Toggle TX / RX |
| `v` | Toggle VOX |
| `n` | Toggle noise reduction (NR) |
| `b` | Toggle noise blanker (NB) |
| `x` | Toggle notch filter (NF) |
| `o` | Toggle TX monitor |
| `d` | Toggle DSP IF |
| `q` / `Ctrl+C` | Quit |

### S-meter scale

Matches the TX-500 display: S1 · S3 · S5 · S7 · S9 · S9+20 · S9+40 · S9+60.  
The bar is color-coded: green (≤ S9), yellow (S9+20), red (S9+40 and above).

---

## Library usage

`CatDriver<T>` is generic over any `Read + Write` transport — use a real serial port in production, or a `std::io::Cursor` in tests.

```rust
use lab599_cat::CatDriver;

// Real serial port
let port = serialport::new("/dev/ttyUSB0", 9600)
    .timeout(Duration::from_millis(2000))
    .open()?;
let mut radio = CatDriver::new(port);

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
- `RM` (meter read) is a two-step operation: `RM{type};` selects the meter, then `RM;` reads the value
- Voltage (`VL`) is returned as a float string (`"11.7 "`) and stored as tenths of a volt (u16)

---

## Running tests

```sh
make tests
```

Tests use an in-memory mock transport — no hardware required.
