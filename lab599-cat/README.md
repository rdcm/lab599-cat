# lab599-cat

Rust implementation of the CAT protocol for the Lab599 TX-500 transceiver.

Supports both **Lab599 native** and **TS-2000 compatible** CAT modes. The driver is generic over any `Read + Write` transport — serial port in production.

## Usage

Add to `Cargo.toml`:

```toml
lab599-cat = { version = "0.1", features = ["tx500"] }
```

Basic example:

```rust
use lab599_cat::CatDriver;
use std::time::Duration;

let port = serialport::new("/dev/ttyUSB0", 9600)
    .timeout(Duration::from_millis(2000))
    .open()?;

let mut radio = CatDriver::new(port);

// Read state
let freq = radio.get_frequency_a()?;   // u64, Hz
let mode = radio.get_mode()?;          // Mode enum
let s    = radio.get_smeter()?;        // u16, 0–30

// Set state
radio.set_frequency_a(7_048_500)?;
radio.set_mode(Mode::Usb)?;
radio.set_preamp(true)?;
radio.set_attenuator(false)?;
radio.set_split(true)?;
radio.set_speech_compressor(true)?;
```

## Feature flags

| Feature | What it enables |
|---------|----------------|
| `tx500` | `CatDriver<T>` + TX-500-specific commands: `BD`, `BU`, `SP`, `VV`, `XT` |
| `tx500mp` | TX-500MP-specific commands: `AC` (antenna tuner), `CT` (CTCSS). Implies `tx500`. |

## Implemented commands

### Universal (Lab599 native + TS-2000 compatible)

| Command | Description |
|---------|-------------|
| `ID` | Transceiver model ID |
| `IF` | Full status snapshot |
| `FA` / `FB` | VFO A / B frequency |
| `FR` / `FT` | RX / TX VFO select |
| `MD` | Operating mode (LSB, USB, CW, CW-R, AM, FM, DIG) |
| `PA` | Pre-amplifier |
| `RA` | RF attenuator (−20 dB) |
| `SM` | S-meter / TX power (0–30) |
| `PC` | Output power |
| `AG` | AF gain |
| `RG` | RF gain |
| `MG` | Microphone gain |
| `NB` / `NL` | Noise blanker + level |
| `NR` / `RL` | Noise reduction + level |
| `NT` | Notch filter |
| `GT` | AGC time constant |
| `VX` / `VG` / `VD` | VOX on/off, gain, delay |
| `RT` | RIT |
| `MC` | Memory channel |
| `KS` | CW keying speed |
| `CG` | Carrier level |
| `ML` / `MO` | TX monitor level / mute |
| `SQ` | Squelch |
| `LK` | Lock |
| `PS` | Power on/off |
| `RX` / `TX` | Receive / Transmit |
| `PT` | PTT status |
| `BY` | Busy signal |

### Lab599-native only

| Command | Description |
|---------|-------------|
| `FL` | RX/TX filter select (FIL-1 … FIL-4) |
| `IS` | DSP IF set |
| `MA` | DIG gain |
| `MR` / `MW` | Read / write memory channel data |
| `PL` | Speech compressor input level |
| `PR` | Speech compressor (CMR) |
| `RM` | Meter function (POWER, SWR, MIC/DIG, ALC) |
| `VL` | Supply voltage |

### TX-500 specific (`tx500` feature)

| Command | Description |
|---------|-------------|
| `BD` / `BU` | Band down / up |
| `SP` | Split operation |
| `VV` | Copy VFO A → B |
| `XT` | XIT |

### TX-500MP specific (`tx500mp` feature)

| Command | Description |
|---------|-------------|
| `AC` | Antenna tuner (on/off, start/stop) |
| `CT` | CTCSS on/off |

## Protocol notes

- Commands are ASCII strings terminated with `;` — e.g. `FA00007048500;`
- Frequency is zero-padded to 11 digits
- Filter index is 0-based in the protocol (FL-0 … FL-3), displayed as FIL-1 … FIL-4
- S-meter range is **0–30** (not 0–9 like Kenwood TS-2000)
- `SM` reads signal level in RX, output power in TX
- `RM` is two-step: `RM{type};` selects the meter, then `RM;` reads the value
- `VL` returns voltage as a float string (`"11.7 "`), stored internally as tenths of a volt (`u16`)

Full protocol reference: [docs/cat-protocol-en.md](../docs/cat-protocol-en.md)

## Running tests

```sh
make tests
```

Tests use an in-memory mock transport — no hardware required.
