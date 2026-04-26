# Lab599 CAT Protocol

**Revision 3 / 2026-03**

## Overview

CAT (Computer Aided Transceiver) is a serial protocol for remote control of Lab599 transceivers.

A command consists of:
- An alphabetical command code (2 letters)
- Parameters (fixed-width digit fields)
- A semicolon terminator `;`

**Example — set VFO A to 7 MHz:**

```
FA00007000000;
^^           ^
||           terminator
||
|parameters (11 digits)
command code
```

## Command Types

| Direction | Type | Description |
|-----------|------|-------------|
| PC → TRX | Set | Sets a condition |
| PC → TRX | Read | Requests a value |
| TRX → PC | Answer | Response to a Read |
| TRX → PC | Output | Unsolicited status |

## Error Responses

| Message | Meaning |
|---------|---------|
| `?;` | Syntax error, or command not valid in current state |
| `E;` | Communication error (overrun or framing error on serial) |
| `O;` | Data received but processing not completed |

> **Note:** `?;` may occasionally appear due to microprocessor transients even when the command was correct.

## Parameter Notes

- Blank/unused parameter digits must be filled with `0` (not spaces).
- Control characters 00–1Fh are ignored or cause a `?;` response.
- Do not send commands while the Tuning control is being rotated rapidly.
- Frequency data is not processed if entered from the keypad simultaneously.

---

## Command Reference

### AC — Antenna Tuner Status

> Applicable device: **TX500MP**

| Operation | Format | Example |
|-----------|--------|---------|
| Set | `AC0{P2}{P3};` | `AC001;` |
| Read | `AC;` | |
| Answer | `AC0{P2}{P3};` | `AC001;` |

| Parameter | Values |
|-----------|--------|
| P1 | Always `0` |
| P2 | `0` = AT OFF, `1` = AT ON |
| P3 | `0` = Stop Tuning (Set) / Tuning stopped (Answer) |
| | `1` = Start Tuning (Set) / Tuning active (Answer) |

---

### AG — AF Gain

| Operation | Format |
|-----------|--------|
| Set | `AG0{P2P2P2};` |
| Read | `AG0;` |
| Answer | `AG0{P2P2P2};` |

| Parameter | Values |
|-----------|--------|
| P1 | Always `0` |
| P2 | `000`–`250` |

---

### AL — NF Type

| Operation | Format |
|-----------|--------|
| Set | `AL{P1};` |
| Read | `AL;` |
| Answer | `AL{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = NF type 1, `1` = NF type 2 |

---

### BY — Busy Signal Status

Read-only.

| Operation | Format |
|-----------|--------|
| Read | `BY;` |
| Answer | `BY{P1}{P2};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = Not busy, `1` = Busy |
| P2 | Always `0` |

---

### BD / BU — Band Down / Band Up

> Applicable device: **TX500**

Set-only commands. No parameters.

| Operation | Format |
|-----------|--------|
| Band Down | `BD;` |
| Band Up | `BU;` |

---

### CG — Carrier Level

| Operation | Format |
|-----------|--------|
| Set | `CG{P1P1P1};` |
| Read | `CG;` |
| Answer | `CG{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = OFF TX, `10`–`100` = TUNE/TONE level |

---

### CN — CTCSS Frequency

| Operation | Format |
|-----------|--------|
| Set | `CN{P1P1};` |
| Read | `CN;` |
| Answer | `CN{P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `00`–`41` (index into CTCSS table) |

**CTCSS Frequency Table (TX500MP):**

| No. | Hz | No. | Hz | No. | Hz | No. | Hz |
|-----|----|-----|----|-----|----|-----|----|
| 00 | 67.0 | 11 | 97.4 | 22 | 141.3 | 33 | 206.5 |
| 01 | 69.3 | 12 | 100.0 | 23 | 146.2 | 34 | 210.7 |
| 02 | 71.9 | 13 | 103.5 | 24 | 151.4 | 35 | 218.1 |
| 03 | 74.4 | 14 | 107.2 | 25 | 156.7 | 36 | 225.7 |
| 04 | 77.0 | 15 | 110.9 | 26 | 162.2 | 37 | 229.1 |
| 05 | 79.7 | 16 | 114.8 | 27 | 167.9 | 38 | 233.6 |
| 06 | 82.5 | 17 | 118.8 | 28 | 173.8 | 39 | 241.8 |
| 07 | 85.4 | 18 | 123.0 | 29 | 179.9 | 40 | 250.3 |
| 08 | 88.5 | 19 | 127.3 | 30 | 186.2 | 41 | 254.1 |
| 09 | 91.5 | 20 | 131.8 | 31 | 192.8 | — | — |
| 10 | 94.8 | 21 | 136.5 | 32 | 203.5 | — | — |

---

### CT — CTCSS Function Status

> Applicable device: **TX500MP**

| Operation | Format |
|-----------|--------|
| Set | `CT{P1};` |
| Read | `CT;` |
| Answer | `CT{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = CTCSS OFF, `1` = CTCSS ON |

---

### FA / FB — VFO A / VFO B Frequency

| Operation | Format | Example (14.195 MHz) |
|-----------|--------|----------------------|
| Set | `FA{P1×11};` | `FA00014195000;` |
| Read | `FA;` | |
| Answer | `FA{P1×11};` | `FA00014195000;` |

| Parameter | Values |
|-----------|--------|
| P1 | Frequency in Hz, 11 digits, zero-padded |

Same format applies to `FB` for VFO B.

---

### FL — Current Filter

> Applicable device: **Lab599**

| Operation | Format |
|-----------|--------|
| Set | `FL{P1}{P2};` |
| Read | `FL;` |
| Answer | `FL{P1}{P2};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0`–`3` — RX filter number |
| P2 | `0`–`1` — TX filter number |

---

### FR / FT — VFO or Memory Channel Select

| Operation | Format |
|-----------|--------|
| Set | `FR{P1};` / `FT{P1};` |
| Read | `FR;` / `FT;` |
| Answer | `FR{P1};` / `FT{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = VFO A, `1` = VFO B, `2` = Memory Channel |

---

### GT — AGC Time Constant

| Operation | Format |
|-----------|--------|
| Set | `GT{P1P1};` |
| Read | `GT;` |
| Answer | `GT{P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `01`–`10` (steps of 1) |

---

### ID — Transceiver ID Number

Read-only.

| Operation | Format |
|-----------|--------|
| Read | `ID;` |
| Answer | `ID{P1P1P1};` |

| P1 | Device |
|----|--------|
| `019` | TS-2000 (TS2000 protocol) |
| `500` | TX500 (LAB599 protocol) |
| `505` | TX500MP (LAB599 protocol) |

---

### IF — Full Transceiver Status

Read-only. Cannot be used while in Data mode.

| Operation | Format |
|-----------|--------|
| Read | `IF;` |
| Answer | `IF{P1×11}{P2×5}{P3×5}{P4}{P5}{P6P7}{P8}{P9}{P10}{P11}{P12}{P13}{P14×2}{P15};` |

| Param | Width | Meaning |
|-------|-------|---------|
| P1 | 11 | Displayed frequency in Hz |
| P2 | 5 | Spaces |
| P3 | 5 | RIT/XIT frequency ±9990 Hz |
| P4 | 1 | `0` = RIT OFF, `1` = RIT ON |
| P5 | 1 | `0` = XIT OFF, `1` = XIT ON |
| P6,P7 | 2 | Memory channel number (see MC) |
| P8 | 1 | `0` = RX, `1` = TX |
| P9 | 1 | Operating mode (see MD) |
| P10 | 1 | VFO function (see FR/FT) |
| P11 | 1 | Scan status (see SC) |
| P12 | 1 | `0` = Simplex, `1` = Split |
| P13 | 1 | `0` = OFF, `2` = CTCSS ON |
| P14 | 2 | Tone/CTCSS frequency index (see TN/CN) |
| P15 | 1 | Always `0` |

---

### IS — DSP IF Set

> Applicable device: **Lab599**

| Operation | Format |
|-----------|--------|
| Set | `IS{P1};` |
| Read | `IS;` |
| Answer | `IS{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = IF not set, `1` = IF set |

---

### KS — Keying Speed

| Operation | Format |
|-----------|--------|
| Set | `KS{P1P1P1};` |
| Read | `KS;` |
| Answer | `KS{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `004`–`060` WPM. Values ≤003 → 004; values ≥061 → 060 |

---

### LK — Lock Status

| Operation | Format |
|-----------|--------|
| Set | `LK{P1}{P2};` |
| Read | `LK;` |
| Answer | `LK{P1}{P2};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = Lock OFF, `1` = Lock ON |
| P2 | Always `0` |

---

### MA — DIG Gain

| Operation | Format |
|-----------|--------|
| Set | `MA{P1P1P1};` |
| Read | `MA;` |
| Answer | `MA{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `000`–`100` |

---

### MC — Memory Channel Number

| Operation | Format |
|-----------|--------|
| Set | `MC0{P2P2};` |
| Read | `MC;` |
| Answer | `MC0{P2P2};` |

| Parameter | Values |
|-----------|--------|
| P1 | Always `0` |
| P2 | `00`–`99` (two digits, zero-padded) |

---

### MD — Operating Mode

| Operation | Format |
|-----------|--------|
| Set | `MD{P1};` |
| Read | `MD;` |
| Answer | `MD{P1};` |

| P1 | Mode |
|----|------|
| `0` | None (setting failure) |
| `1` | LSB |
| `2` | USB |
| `3` | CW |
| `4` | FM |
| `5` | AM |
| `6` | DIG (Lab599) |
| `7` | CW-R |

---

### MG — Microphone Gain

| Operation | Format |
|-----------|--------|
| Set | `MG{P1P1P1};` |
| Read | `MG;` |
| Answer | `MG{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `000`–`100` |

---

### ML — TX Monitor Level

| Operation | Format |
|-----------|--------|
| Set | `ML{P1P1P1};` |
| Read | `ML;` |
| Answer | `ML{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `000`–`250` |

---

### MO — TX Monitor Mute

| Operation | Format |
|-----------|--------|
| Set | `MO{P1};` |
| Read | `MO;` |
| Answer | `MO{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = TX Monitor out, `1` = TX Monitor mute |

---

### MR — Read Memory Channel Data

> Applicable device: **Lab599**

| Operation | Format |
|-----------|--------|
| Read | `MR{P1}{P2}{P2};` |
| Answer | (50-char response, see below) |

**Answer parameters:**

| Param | Meaning |
|-------|---------|
| P1,P2 | Always `0` |
| P3 | Channel number (see MC) |
| P4 | Frequency (11 digits; unused high-end digits → 0) |
| P5 | Mode (see MD) |
| P6 | `0` = OFF, `1` = Preamplifier, `2` = Attenuator |
| P7 | Always `0` |
| P8,P9 | Always `0` |
| P10 | Always `000` |
| P11,P12 | Always `0` |
| P13,P14,P15 | Always `0` |
| P16 | Always space |

---

### MW — Write Memory Channel Data

> Applicable device: **Lab599**

Set-only. Same parameter layout as MR answer.

---

### NB — Noise Blanker

| Operation | Format |
|-----------|--------|
| Set | `NB{P1};` |
| Read | `NB;` |
| Answer | `NB{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = NB OFF, `1` = NB ON |

---

### NF — Notch Filter Type

| Operation | Format |
|-----------|--------|
| Set | `NF{P1};` |
| Read | `NF;` |
| Answer | `NF{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = Notch type 1, `1` = Notch type 2 |

---

### NL — Noise Blanker Level

| Operation | Format |
|-----------|--------|
| Set | `NL{P1P1P1};` |
| Read | `NL;` |
| Answer | `NL{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `030`–`100` |

---

### NR — Noise Reduction

| Operation | Format |
|-----------|--------|
| Set | `NR{P1};` |
| Read | `NR;` |
| Answer | `NR{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = NR OFF, `1` = NR ON |

---

### NT — Notch Filter

| Operation | Format |
|-----------|--------|
| Set | `NT{P1};` |
| Read | `NT;` |
| Answer | `NT{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = Notch OFF, `1` = Auto Notch |

---

### PA — Pre-amplifier

| Operation | Format |
|-----------|--------|
| Set | `PA{P1};` |
| Read | `PA;` |
| Answer | `PA{P1}{P2};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = Pre-amp OFF, `1` = Pre-amp ON |
| P2 | Always `0` |

---

### PC — Output Power

| Operation | Format |
|-----------|--------|
| Set | `PC{P1P1P1};` |
| Read | `PC;` |
| Answer | `PC{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `010`–`100` |

---

### PL — Speech Compressor Input Level

| Operation | Format |
|-----------|--------|
| Set | `PL{P1P1P1};` |
| Read | `PL;` |
| Answer | `PL{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `001`–`100` |

---

### PR — Speech Compressor

| Operation | Format |
|-----------|--------|
| Set | `PR{P1};` |
| Read | `PR;` |
| Answer | `PR{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = Compressor OFF, `1` = Compressor ON |

---

### PS — Power ON/OFF

| Operation | Format |
|-----------|--------|
| Set | `PS{P1};` |
| Read | `PS;` |
| Answer | `PS{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `1` = Power ON |

---

### PT — PTT Status

| Operation | Format |
|-----------|--------|
| Set | `PT{P1};` |
| Read | `PT;` |
| Answer | `PT{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = RX, `1` = TX |

---

### RA — RF Attenuator

| Operation | Format |
|-----------|--------|
| Set | `RA{P1P1};` |
| Read | `RA;` |
| Answer | `RA{P1P1}{P2P2};` |

| Parameter | Values |
|-----------|--------|
| P1 | `00` = ATT OFF, `01` = ATT ON |
| P2 | Always `00` |

---

### RG — RF Gain

| Operation | Format |
|-----------|--------|
| Set | `RG{P1P1P1};` |
| Read | `RG;` |
| Answer | `RG{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `000`–`100` |

---

### RL — Noise Reduction Level

| Operation | Format |
|-----------|--------|
| Set | `RL{P1P1P1};` |
| Read | `RL;` |
| Answer | `RL{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `01`–`100` (when NR is ON) |

---

### RM — Meter Function

> Applicable device: **Lab599**

| Operation | Format |
|-----------|--------|
| Set | `RM{P1};` |
| Read | `RM;` |
| Answer | `RM{P1}{P2P2P2P2};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = POWER, `1` = SWR, `2` = MIC/DIG, `3` = ALC |
| P2 | `0000`–`0030` (meter value in dots) |

---

### RT — RIT Function

| Operation | Format |
|-----------|--------|
| Set | `RT{P1};` |
| Read | `RT;` |
| Answer | `RT{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = RIT OFF, `1` = RIT ON |

---

### RX — Set Receiver Mode

Set-only. No parameters.

| Operation | Format |
|-----------|--------|
| Set | `RX;` |

---

### SM — S-Meter Value

Read-only.

| Operation | Format |
|-----------|--------|
| Read | `SM{P1};` |
| Answer | `SM{P1}{P2P2P2P2};` |

| Parameter | Values |
|-----------|--------|
| P1 | Always `0` |
| P2 | `0000`–`0030` — S-meter (RX) or power out (TX) |

---

### SP — Split Operation

> Applicable device: **TX500**

| Operation | Format |
|-----------|--------|
| Set | `SP{P1};` |
| Read | `SP;` |
| Answer | `SP{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = Split OFF, `1` = Split ON |

---

### SQ — Squelch

| Operation | Format |
|-----------|--------|
| Set | `SQ0{P2P2P2};` |
| Read | `SQ0;` |
| Answer | `SQ0{P2P2P2};` |

| Parameter | Values |
|-----------|--------|
| P1 | Always `0` |
| P2 | `000`–`255` |

---

### TM — Time

| Operation | Format | Example |
|-----------|--------|---------|
| Set | `TM{HH}:{MM}:{SS};` | `TM14:30:00;` |
| Read | `TM;` | |
| Answer | `TM{HH}:{MM}:{SS};` | |

| Parameter | Values |
|-----------|--------|
| P1 (HH) | `00`–`23` |
| P2 (MM) | `00`–`59` |
| P3 (SS) | `00`–`59` |

---

### TO — Tone Status

| Operation | Format |
|-----------|--------|
| Set | `TO{P1};` |
| Read | `TO;` |
| Answer | `TO{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = Tone OFF, `1` = Tone ON |

---

### TP — TX Tune Output Power

| Operation | Format |
|-----------|--------|
| Set | `TP{P1P1P1};` |
| Read | `TP;` |
| Answer | `TP{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `005`–`050` |

---

### TX — Set Transmission Mode

Set-only. No parameters.

| Operation | Format |
|-----------|--------|
| Set | `TX;` |

---

### VD — VOX Delay Time

| Operation | Format |
|-----------|--------|
| Set | `VD{P1P1P1P1};` |
| Read | `VD;` |
| Answer | `VD{P1P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0000`–`5000` ms (steps of 100) |

---

### VG — VOX Gain

| Operation | Format |
|-----------|--------|
| Set | `VG{P1P1P1};` |
| Read | `VG;` |
| Answer | `VG{P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `000`–`100` |

---

### VL — Voltage

> Applicable device: **Lab599**

Read-only.

| Operation | Format |
|-----------|--------|
| Read | `VL;` |
| Answer | `VL{P1P1P1P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | Supply voltage in V (4 digits) |

---

### VV — VFO Copy (A=B)

> Applicable device: **TX500**

Set-only. No parameters.

| Operation | Format |
|-----------|--------|
| Set | `VV;` |

---

### VX — VOX Status

| Operation | Format |
|-----------|--------|
| Set | `VX{P1};` |
| Read | `VX;` |
| Answer | `VX{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = VOX OFF, `1` = VOX ON |

---

### XT — XIT Function

> Applicable device: **TX500**

| Operation | Format |
|-----------|--------|
| Set | `XT{P1};` |
| Read | `XT;` |
| Answer | `XT{P1};` |

| Parameter | Values |
|-----------|--------|
| P1 | `0` = XIT OFF, `1` = XIT ON |

---

## Device Command Support Matrix

| Command | TX500 | TX500MP |
|---------|-------|---------|
| AC | — | ✓ |
| CT | — | ✓ |
| BD/BU | ✓ | — |
| SP | ✓ | — |
| VV | ✓ | — |
| XT | ✓ | — |
| All others | ✓ | ✓ |

Lab599-specific commands (FL, IS, MR, MW, RM, VL) apply to all Lab599 devices.
