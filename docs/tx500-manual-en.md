# Lab599 Discovery TX-500 — User Manual

> Version v1.16.04 / 03.2023  
> HF/50 MHz Transceiver TX-500

---

## Contents

1. [Introduction](#introduction)
2. [Controls and Interface](#controls-and-interface)
3. [Protection](#protection)
4. [Basic Operation](#basic-operation)
5. [Advanced Features](#advanced-features)
6. [Firmware Update](#firmware-update)
7. [Remote Control](#remote-control)
8. [Menu Functions](#menu-functions)
9. [Maintenance](#maintenance)
10. [Accessories](#accessories)
11. [Specifications](#specifications)

---

## Introduction

The TX-500 Discovery is an ultra-compact all-band transceiver designed for portable operation. Its rugged, splash- and dust-resistant enclosure ensures reliability in harsh conditions. Because the TX-500 is an SDR (Software Defined Radio) system, capabilities can be extended through software and regular firmware updates.

---

## Controls and Interface

### Controls (top view)

| Element | Function |
|---------|----------|
| **TUNE/MULTI** | VFO A/B tuning, menu value adjustment |
| **AF GAIN** | Monitor level (TX/RX sidetone) |
| **RIT/XIT** | RX/TX offset, RF/SQL/AGC |
| Function keys | Most frequently used functions |
| **POWER** | Power on/off |
| **BAND+/BAND−** | Band up / band down |
| **MODE** | Modulation mode |
| **FILTER** | Passband filter |
| **MENU** | Menu |
| **R/X** | RIT/XIT on/off |
| **CLR** | Reset RIT/XIT |
| **V/M** | VFO / Memory |
| **Lock** | VFO lock |
| **+/−** | VFO step |

### Connectors (left side)

**DC 9–15V** (power):
- Pin 1 — GND
- Pin 2 — (+) DC 9–15V

**REM/DATA**:
- Pin 1 — PTT
- Pin 2 — PTT output (open collector)
- Pin 3 — AUDIO output (transceiver → PC)
- Pin 4 — AUX Q
- Pin 5 — AUX I
- Pin 6 — AUDIO input (PC → transceiver)
- Pin 7 — GND

**MIC/SP** (microphone/speaker):
- Pin 1 — PHONE −
- Pin 2 — DYNAMIC MIC
- Pin 3 — PHONE +
- Pin 4 — PHONE +
- Pin 5 — MIC (DC+)
- Pin 6 — GND

### Connectors (right side)

**CW KEY**:
- Pin 1 — GND
- Pin 2 — Dot (·)
- Pin 3 — NC
- Pin 4 — NC
- Pin 5 — Dash (−)

**CAT** (CAT interface):
- Pin 1 — GND
- Pin 2 — RX
- Pin 3 — +DC USB
- Pin 4 — TX

**ANT**: Antenna (50 Ω, BNC)

### User interface (main screen)

| Zone | Content |
|------|---------|
| **① ⑨** | Quick menu and function keys (top and bottom rows) |
| **②** | Info bar: RX/TX, VFO step, VFO lock, clock battery, overheat (HOT), AGC, supply voltage, time |
| **③** | VFO A / VFO B — current frequencies |
| **④** | Info block 1: filter number, bandwidth, RX volume, TX monitor, RF gain |
| **⑤** | Bar graph: S-meter (RX) / PWR, SWR, MIC, ALC (TX) |
| **⑥** | Info block 2: NR, NB, NF, ATT, MON, VOX, CMR, PRE, SQL, SPL, DIF |
| **⑦** | Panadapter |
| **⑧** | Main menu bar |

**Block ⑥ indicators:**
- `NR` — Noise reduction on
- `NB` — Noise blanker on
- `NF` — Notch filter on
- `ATT` — Attenuator on
- `MON` — TX monitor on
- `VOX` — VOX on
- `CMR` — Speech compressor on
- `PRE` — RF pre-amp on
- `SQL` — Squelch on
- `SPL` — Split (separate TX/RX frequencies) on
- `DIF` — Virtual IF on

---

## Protection

### Overvoltage protection

Above 15 V the transceiver will not switch to TX; the display inverts. Exceeding 16 V may blow the fuse. Use a 9–15 V supply rated at least 2.5 A.

> **WARNING!** Supply voltage above 15 V may damage the transceiver!

### Reverse polarity protection

With incorrect power polarity the transceiver will not power on.

### Overheat protection

The PA stage has thermal protection. At approximately 60 °C TX is automatically disabled. During prolonged digital-mode transmissions, do not block the ventilation slots on the rear cover.

> **Tip:** Deploy both rear kickstands to improve airflow convection.

### High-SWR protection

At SWR ≥ 3.0 the transceiver automatically reduces output power. Use a matched 50 Ω antenna or an antenna tuner.

---

## Basic Operation

### Getting started

Before use, connect: DC power supply, speaker-microphone, antenna.

### Using the menu

- Press **MENU** to enter
- Use `MenuUp / MenuDown` function keys to navigate
- Press `SubMenu` to enter a sub-menu
- Turn **TUNE/MULTI** to change a value
- Press **MENU** again to exit

### Band selection

TX-500 covers 160–6 m:

| Band (m) | Range (MHz) | Band (m) | Range (MHz) |
|----------|-------------|----------|-------------|
| 160 | 1.8–2.0 | 17 | 18.068–18.168 |
| 80 | 3.5–4.0 | 15 | 21.0–21.45 |
| 60 | ~5.3–5.4 | 12 | 24.89–24.99 |
| 40 | 7.0–7.3 | 10 | 28.0–29.7 |
| 30 | 10.0–10.15 | 6 | 50–54 |
| 20 | 14.0–14.35 | | |

### Modulation mode

Press **MODE** to cycle: USB, CW, DIG, AM, FM.  
Hold **MODE** — quick menu for alternate modes (LSB, CWR, etc.).

Supported modes:
- **SSB (USB/LSB)** — narrowband voice. USB for 20 m and above, LSB for 160/80/40 m.
- **CW** — Morse, low power consumption, popular for DX and contests.
- **CWR** — CW with reversed sideband (reduces QRM).
- **AM** — wideband voice; common on 160, 80, 40, 10 m.
- **FM** — local contacts; effective on 10 m and above.
- **DIG** — digital modes (PSK31, RTTY, JT65, FT8, FT4, etc.). Disables microphone, activates REM/DATA audio input.

### VFO A and B

- **VFO A** — primary, controls RX/TX frequency.
- **VFO B** — register for a second frequency.
- **VFO step**: `+` / `−` keys; SSB/DIG/LSB steps: 0.5/10/100/1 kHz; CW/CWR: 1/10/100/1 kHz; AM/FM: 100 Hz/500 Hz/1/2.5/5 kHz.
- **A→B**: copy VFO A to B.
- **B→A**: copy VFO B to A.
- **A↔B**: swap A and B.

### RIT — receive incremental tuning

Shifts the receive frequency without affecting the TX frequency. Step: 1/10/100/200/300 Hz. Keys `+` / `−`.

### XIT — transmit incremental tuning

Shifts the transmit frequency without affecting RX.  
Enable: press **R/X** while in TX. Reset: **CLR**.

### VFO lock / unlock

Press **Lock**. `LOCK` appears on the display.

### Transmit settings

- **MON (TX monitor)**: key → `MON`. PTT + **AF GAIN** knob.
- **CMR (speech compressor)**: key → `CMR`. MENU → 17 CMR Level.
- **POWER**: press `POWER` in the top row, turn **TUNE/MULTI** (10–100%).
- **VOX**: MENU → 14 VOX Level (sensitivity), MENU → 15 VOX (delay, ms).
- **CW PITCH**: key → `CWPITCH`, turn **TUNE/MULTI** (or MENU → 01).
- **METR**: cycle TX meter view: MIC / ALC / PWR / SWR / PWR Num / SWR Num.

### Receive settings

- **RF gain**: RF amplification level (default 0; reduce for strong signals or digital modes).
- **SQL**: squelch threshold. MENU → 13 SQL.
- **DIF (virtual IF)**: ENABLE/DISABLE. Improves DSP performance.
- **FILTER**: LF/HF filter adjustment (4 RX filters, 2 TX filters). Long press → select cutoff frequency.
- **PRE/ATT**: pre-amp (weak signal) / 20 dB attenuator (strong interfering signal).
- **NR**: DSP noise reduction, 1–100. MENU → 10 NR Level.
- **NB**: DSP noise blanker, 40–100. MENU → 11 NB Level.
- **NF**: Notch filter for tonal interference (SSB/AM). Key → `NF`.
- **TONE**: transmit 1000 Hz tone (NORMAL) or 1000 + 2000 Hz (DUAL).

---

## Advanced Features

### Memory operation

100 memory channels (00–99). Toggle VFO ↔ Memory: **V/M** key.

Channel operations (via `MEM` → quick menu):
1. `Mem→VFO` — recall settings from selected channel
2. `VFO→Mem` — save current settings to channel
3. `CLR` — clear channel

### Split and XIT

Separate RX and TX frequencies:
1. Press → `A→B` (copy VFO A to B)
2. Tune VFO B to the TX frequency (±2 kHz from DX station)
3. Press → `SPL` (`SPL` indicator appears)

`RX↔B` temporarily swaps VFO A and B to monitor your own TX frequency.

### RX audio equalizer (RX EQ)

MENU → 19 EQL → RX. Three bands: High Freq / Low Freq / Mid Freq.  
Defaults: HF=50, LF=100, MF=75.

### TX audio equalizer (TX EQ)

MENU → 19 EQL → TX. Defaults: HF=100, LF=100, MF=100.

### CW-in-SSB split

VFO A — SSB receive, VFO B — CW transmit. Key → `SPL`.

### Band memory

When changing bands the transceiver saves the within-band frequency.

### Voice message memory

2 slots (VOM1, VOM2), 20 seconds each. Record: hold VOM 1/2 (>1 s) → `REC` appears → dictate message → press VOM 1/2. Play: short press VOM 1/2.

### CW message memory

4 slots (CWM1–CWM4), 25 seconds each. Record: hold CWM 1–4 (>1 s) → `REC` appears → key the message → press CWM 1–4. Play: short press CWM 1–4.

### CW tone match indicator

Shows the difference between the correspondent's CW tone and the local CW Pitch setting — helps zero-beat accurately.

### CW decoder

MENU → 31 CW Decode → `SubMenu`:
- **RX** → ENABLE: decode on receive (characters displayed in bottom row)
- **TX** → ENABLE: decode on transmit

### Beacon mode

Automatic transmission of CW (CWM1) or voice (VOM1) at a set interval.  
MENU → 30 Beacon → `SubMenu` → CW or VO → turn **TUNE/MULTI** to set interval 1–240 s.  
Disable: set value to DISABLED.

### Antenna SWR monitor

Measures SWR across the selected band. Graph: Y = SWR (1–6), X = band span.  
Activate: hold **TONE** key → `TONE`.

### DIG modes (audio data modes)

For PSK31, RTTY, JT65, FT8, etc. use DIG mode. DIG disables the microphone and activates the REM/DATA audio input.

Connection: audio cable from REM/DATA connector to PC sound card (pin 3 → PC mic input, pin 6 ← PC headphone output).

Level settings:
- MENU → 14 VOX Level → DIG (VOX sensitivity)
- MENU → 15 VOX → DIG (delay, ms)
- MENU → 09 Gain → DIG

RX/TX switching via CAT or VOX.

### External amplifier control

- Pin 2 (PTT OUT) of REM/DATA → amplifier PTT (open collector, max 25 V / 0.5 A)
- Pin 7 (GND) of REM/DATA → amplifier GND
- BNC "ANT" → amplifier "TRX" input (50 Ω coax)

---

## Firmware Update

1. Download firmware from [www.lab599.com](http://www.lab599.com) → Downloads.
2. Connect DC 9–15 V external power.
3. Connect TX-500 to PC, launch the update utility, select the COM port.
4. Enter update mode: hold the third top function key + press **POWER** → "The loader is waiting…"
5. Click "Update" in the utility. Do not power off until complete.
6. After completion, power cycle the transceiver and verify the firmware version.

The firmware version is shown on the startup screen in the bottom row.

---

## Remote Control

### PC control and logging

The TX-500 is compatible with any software supporting RS-232 or USB. Connection requires the CAT-USB adapter (see Accessories).

The TX-500 CAT interface is compatible with KENWOOD TS-2000 commands:  
`ID  AI  RX  TX  IF  FA  FB  MD (6-DIG)  FR  FT  FN  PA  RA`

For third-party logging and contest software, select RIG Type = **KENWOOD TS-2000** in the application settings.

Commands outside this set (e.g. `PR` — speech compressor) require **Lab599 native** CAT mode (Menu 25: `LAB599`).

### COM port settings

| Parameter | Value |
|-----------|-------|
| RIG Type | KENWOOD TS-2000 |
| **Baud rate** | **9600** |
| Data bits | 8 |
| Parity | NONE |
| Stop bits | 1 |

> The baud rate is **9600** for both modes (TS-2000 and Lab599 native).

### Connection

```
CAT [TX-500] ──── CAT-USB cable ──── USB 2.0 [PC]
```

FTDI (FT232R) adapters work driver-free on Linux (appears as `/dev/ttyUSB0`).  
PL2303 adapters require the driver from www.lab599.com.

---

## Menu Functions

Access: **MENU** → `MenuUp/MenuDown` to navigate → `SubMenu` for sub-menus → **TUNE/MULTI** to change value → **MENU** to exit.

| # | Parameter | Description | Default |
|---|-----------|-------------|---------|
| 00 | **Encoder** | VFO encoder mode: Plain (linear) / Intel (adaptive) | Intel |
| 01 | **CW Pitch** | CW sidetone frequency, 400–1200 Hz | 700 Hz |
| 02 | **CW Speed** | Keyer speed, 10–300 cpm (2–60 wpm) | 100 cpm (20 wpm) |
| 03 | **CW Weight** | Dot/dash ratio, 2:1–4.5:1 | 3:1 |
| 04 | **CW Key** | Key type: Single / Auto (Iambic A/B); Reverse: Disable/Enable | Type=auto; Auto=Iambic A; Rev=Disable |
| 05 | **Beacon** | Beacon interval: 1–240 s / Disable | Disable |
| 06 | **AGC** | AGC time constant for CW/SSB/AM (1=slow, 10=fast) | CW=5; SSB=3; AM=3 |
| 07 | **RF** | RF gain level for CW/SSB/DIG/AM/FM | 0 |
| 08 | **Power** | TX power level, 10–100% | 100% |
| 09 | **Gain** | MIC audio level (1–100) and DIG audio level (1–100) | MIC=5; DIG=20 |
| 10 | **NR Level** | DSP noise reduction, 1–100 | 50 |
| 11 | **NB Level** | DSP noise blanker, 40–100 | 50 |
| 12 | **Notch Filter Type** | NF filter type: 1=high quality, 2=low latency | 1 |
| 13 | **SQL** | Squelch threshold for SSB/AM and FM (0–100) | SSB/AM=0; FM=0 |
| 14 | **VOX Level** | VOX sensitivity for MIC/DIG (1–100) | MIC=50; DIG=50 |
| 15 | **VOX** | VOX delay for CW/MIC/DIG, 100 ms–10 s | CW=400 ms; MIC=1000 ms; DIG=100 ms |
| 16 | **AM/FM** | Allow AM/FM below 29 MHz: Enable/Disable | Enable |
| 17 | **CMR Level** | SSB speech compressor level, 1–100 (max useful compression ~40) | 5 |
| 18 | **Save Band VFO** | On band change, save VFO A only or VFO A&B | VFO A |
| 19 | **EQL** | RX/TX equalizer: High/Low/Mid Freq (1–100) | RX: HF=50,LF=100,MF=75; TX: HF=100,LF=100,MF=100 |
| 20 | **RX Pan Scale** | RX panadapter: AVG (1–100), Scale (0.1–5.0), Shift (−100..+100) | AVG=5; Scale=0.9; Shift=30 |
| 21 | **TX Pan Scale** | TX panadapter: AVG (1–100), Scale (0.1–5.0), Shift (−100..+100) | AVG=5; Scale=2.7; Shift=20 |
| 22 | **TX Metr** | TX meter display: SWR Num / PWR Num / SWR / PWR / ALC / MIC | PWR |
| 23 | **Type Tone** | Tone signal type: Normal (1000 Hz) / Dual (1000+2000 Hz) | Normal |
| 24 | **Audio output** | Audio output power: Normal (1 W) / Outdoor (3 W) | Normal |
| 25 | **Freq Ref** | TCXO correction (24.576 MHz), −1000..+1000 Hz | 0 |
| 26 | **Beep Key** | Key press beep: Enable/Disable | Enable |
| 27 | **Time** | Set clock: Hour/Min | — |
| 28 | **Corr Time** | Clock drift correction, −63..+126 | 0 |
| 29 | **Backlight** | Display backlight: dimly / brightly / auto | brightly |
| 30 | **Contrast** | LCD contrast, 0–50 | 21 |
| 31 | **CW Decode** | CW decoder: RX Enable/Disable; TX Enable/Disable | Disable |
| 32 | **Alt Encoder** | RIT/XIT encoder alternate function: OFF / AGC / SQL | OFF |
| 33 | **Select Memory** | Memory channel switching: Encoder / Keys +/− | Encoder |

### Factory reset

Hold `V/M` and press **POWER**.

---

## Maintenance

### Clock battery replacement (CR2032)

Tools required: 1.5 mm hex key, 3 mm hex key, CR2032 battery.

1. Remove the RF and RIT/XIT control knobs (1.5 mm hex).
2. Remove the indicated screws with the 3 mm hex key.
3. Loosen the side screws (turn 180°) on both sides of the enclosure.
4. Carefully separate the top panel.
5. Replace the CR2032 battery.
6. Reassemble in reverse order.

---

## Accessories

### Hand speaker-microphone

High-quality microphone, speaker, PTT, 3.5 mm external speaker jack. Connects to **MIC/SP**.

### Power cable

DC 9–15 V with 3 A fuse. V+ / GND.

### Microphone/speaker/PTT adapter

Connects a standard headset (3.5 mm MIC, 3.5 mm PHONE) and a PTT button to **MIC/SP**.

### Audio cable (for DIG modes)

Connects the transceiver to a PC sound card for digital modes:
- REM/DATA connector ↔ PC mic input (3.5 mm) + PC headphone output (3.5 mm)

### CW adapter

Connects a CW key via 3.5 mm plug to the **CW KEY** connector.

### CAT-USB adapter

Connects TX-500 to PC via USB:
- FTDI chips — driver-free on Linux (appears as `/dev/ttyUSB0`)
- PL2303 chips — require driver from www.lab599.com

---

## Specifications

### General

- Bands: 160–6 m (amateur)
- Continuous RX: 0.5–56.0 MHz
- Modes: SSB, CW, DIG, AM, FM
- DSP: 32-bit floating point
- Power supply: DC 9–15 V; TX: 1–3 A; RX: up to 110 mA
- Display: monochrome LCD 256×128 px
- Panadapter: 48 kHz real-time
- Firmware update: via USB
- Dimensions: 90 × 207 × 21 mm
- Weight: 0.55 kg

### Receiver

- Sensitivity (MDS): −136 dBm (with pre-amp)
- Quadrature downconversion mixer (SDR-software compatible)
- I/Q outputs for PC sound card connection
- Switchable low-noise pre-amp and 20 dB attenuator
- 3-band RX audio equalizer
- 4 adjustable digital RX filters
- Automatic notch filter
- DSP noise reduction and noise blanker
- External speaker audio output, 3 W

### Transmitter

- Power: 1–10 W HF (7 W on 6 m)
- High-SWR and overheat protection
- Carrier suppression: >50 dB
- Harmonics/spurious: >50 dB below carrier
- CW sidetone: 400–1200 Hz
- 2 adjustable digital TX filters
- Digital speech compressor
- 3-band TX audio equalizer

---

*All specifications apply to amateur radio use only. Measurements performed at 13.8 V DC supply.*
