/// Operating mode (MD command)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Lsb = 1,
    Usb = 2,
    Cw = 3,
    Fm = 4,
    Am = 5,
    Dig = 6,
    CwR = 7,
}

impl Mode {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Mode::Lsb),
            2 => Some(Mode::Usb),
            3 => Some(Mode::Cw),
            4 => Some(Mode::Fm),
            5 => Some(Mode::Am),
            6 => Some(Mode::Dig),
            7 => Some(Mode::CwR),
            _ => None,
        }
    }
}

/// VFO / Memory selection (FR / FT command)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VfoSelect {
    VfoA = 0,
    VfoB = 1,
    Memory = 2,
}

impl VfoSelect {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(VfoSelect::VfoA),
            1 => Some(VfoSelect::VfoB),
            2 => Some(VfoSelect::Memory),
            _ => None,
        }
    }
}

/// Meter type (RM command)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterType {
    Power = 0,
    Swr = 1,
    MicDig = 2,
    Alc = 3,
}

impl MeterType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(MeterType::Power),
            1 => Some(MeterType::Swr),
            2 => Some(MeterType::MicDig),
            3 => Some(MeterType::Alc),
            _ => None,
        }
    }
}

/// All CAT commands.
///
/// Naming convention:
/// - `XxxSet(...)` — set command with parameters
/// - `XxxRead`     — read command (requests current value)
///
/// Commands that are set-only have no `Read` variant.
/// Commands that are read-only have no `Set` variant.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    // ── AC — Antenna tuner (TX500MP) ─────────────────────────────────────
    /// Set: at_on, start_tuning
    AcSet {
        at_on: bool,
        start_tuning: bool,
    },
    AcRead,

    // ── AG — AF gain ──────────────────────────────────────────────────────
    /// Set: 0–250
    AgSet(u16),
    AgRead,

    // ── AL — NF type ──────────────────────────────────────────────────────
    /// Set: false = NF type 1, true = NF type 2
    AlSet(bool),
    AlRead,

    // ── BY — Busy status (read-only) ──────────────────────────────────────
    ByRead,

    // ── BD / BU — Band Down / Up (TX500, set-only) ───────────────────────
    BandDown,
    BandUp,

    // ── CG — Carrier level ────────────────────────────────────────────────
    /// Set: 0 = OFF TX, 10–100 = TUNE/TONE level
    CgSet(u8),
    CgRead,

    // ── CN — CTCSS frequency ─────────────────────────────────────────────
    /// Set: 0–41 (index into CTCSS table)
    CnSet(u8),
    CnRead,

    // ── CT — CTCSS function (TX500MP) ─────────────────────────────────────
    CtSet(bool),
    CtRead,

    // ── FA — VFO A frequency ──────────────────────────────────────────────
    /// Set: frequency in Hz
    FaSet(u64),
    FaRead,

    // ── FB — VFO B frequency ──────────────────────────────────────────────
    FbSet(u64),
    FbRead,

    // ── FL — Current filter (Lab599) ──────────────────────────────────────
    /// Set: rx_filter (0–3), tx_filter (0–1)
    FlSet {
        rx: u8,
        tx: u8,
    },
    FlRead,

    // ── FR — VFO/Memory select (RX) ───────────────────────────────────────
    FrSet(VfoSelect),
    FrRead,

    // ── FT — VFO/Memory select (TX) ───────────────────────────────────────
    FtSet(VfoSelect),
    FtRead,

    // ── GT — AGC time constant ────────────────────────────────────────────
    /// Set: 1–10
    GtSet(u8),
    GtRead,

    // ── ID — Transceiver ID (read-only) ───────────────────────────────────
    IdRead,

    // ── IF — Full transceiver status (read-only) ──────────────────────────
    IfRead,

    // ── IS — DSP IF set (Lab599) ──────────────────────────────────────────
    IsSet(bool),
    IsRead,

    // ── KS — Keying speed ─────────────────────────────────────────────────
    /// Set: 4–60 WPM
    KsSet(u8),
    KsRead,

    // ── LK — Lock status ──────────────────────────────────────────────────
    LkSet(bool),
    LkRead,

    // ── MA — DIG gain ─────────────────────────────────────────────────────
    /// Set: 0–100
    MaSet(u8),
    MaRead,

    // ── MC — Memory channel ───────────────────────────────────────────────
    /// Set: 0–99
    McSet(u8),
    McRead,

    // ── MD — Operating mode ───────────────────────────────────────────────
    MdSet(Mode),
    MdRead,

    // ── MG — Microphone gain ──────────────────────────────────────────────
    /// Set: 0–100
    MgSet(u8),
    MgRead,

    // ── ML — TX Monitor level ─────────────────────────────────────────────
    /// Set: 0–250
    MlSet(u16),
    MlRead,

    // ── MO — TX Monitor mute ──────────────────────────────────────────────
    /// Set: true = mute
    MoSet(bool),
    MoRead,

    // ── MR — Read memory channel data (Lab599) ────────────────────────────
    /// Read: channel 0–99
    MrRead(u8),

    // ── MW — Write memory channel data (Lab599) ───────────────────────────
    MwWrite(MemoryData),

    // ── NB — Noise Blanker ────────────────────────────────────────────────
    NbSet(bool),
    NbRead,

    // ── NF — Notch Filter type ────────────────────────────────────────────
    /// Set: false = type 1, true = type 2
    NfSet(bool),
    NfRead,

    // ── NL — Noise Blanker level ──────────────────────────────────────────
    /// Set: 30–100
    NlSet(u8),
    NlRead,

    // ── NR — Noise Reduction ──────────────────────────────────────────────
    NrSet(bool),
    NrRead,

    // ── NT — Notch Filter ─────────────────────────────────────────────────
    /// Set: true = Auto Notch
    NtSet(bool),
    NtRead,

    // ── PA — Pre-amplifier ────────────────────────────────────────────────
    PaSet(bool),
    PaRead,

    // ── PC — Output power ─────────────────────────────────────────────────
    /// Set: 10–100
    PcSet(u8),
    PcRead,

    // ── PL — Speech compressor input level ───────────────────────────────
    /// Set: 1–100
    PlSet(u8),
    PlRead,

    // ── PR — Speech compressor ────────────────────────────────────────────
    PrSet(bool),
    PrRead,

    // ── PS — Power on/off ─────────────────────────────────────────────────
    PsSet(bool),
    PsRead,

    // ── PT — PTT status ───────────────────────────────────────────────────
    /// Set: false = RX, true = TX
    PtSet(bool),
    PtRead,

    // ── RA — RF Attenuator ────────────────────────────────────────────────
    RaSet(bool),
    RaRead,

    // ── RG — RF Gain ──────────────────────────────────────────────────────
    /// Set: 0–100
    RgSet(u8),
    RgRead,

    // ── RL — Noise Reduction level ────────────────────────────────────────
    /// Set: 1–100
    RlSet(u8),
    RlRead,

    // ── RM — Meter function (Lab599) ──────────────────────────────────────
    RmSet(MeterType),
    RmRead,

    // ── RT — RIT function ─────────────────────────────────────────────────
    RtSet(bool),
    RtRead,

    // ── RX — Set receiver mode (set-only) ────────────────────────────────
    RxMode,

    // ── SM — S-meter value (read-only) ────────────────────────────────────
    SmRead,

    // ── SP — Split operation (TX500) ──────────────────────────────────────
    SpSet(bool),
    SpRead,

    // ── SQ — Squelch ──────────────────────────────────────────────────────
    /// Set: 0–255
    SqSet(u8),
    SqRead,

    // ── TM — Time ─────────────────────────────────────────────────────────
    TmSet {
        hour: u8,
        minute: u8,
        second: u8,
    },
    TmRead,

    // ── TO — Tone status ──────────────────────────────────────────────────
    ToSet(bool),
    ToRead,

    // ── TP — TX Tune output power ─────────────────────────────────────────
    /// Set: 5–50
    TpSet(u8),
    TpRead,

    // ── TX — Set transmission mode (set-only) ────────────────────────────
    TxMode,

    // ── VD — VOX Delay time ───────────────────────────────────────────────
    /// Set: 0–5000 ms (steps of 100)
    VdSet(u16),
    VdRead,

    // ── VG — VOX Gain ─────────────────────────────────────────────────────
    /// Set: 0–100
    VgSet(u8),
    VgRead,

    // ── VL — Voltage (Lab599, read-only) ─────────────────────────────────
    VlRead,

    // ── VV — VFO copy A=B (TX500, set-only) ──────────────────────────────
    VfoCopyAb,

    // ── VX — VOX status ───────────────────────────────────────────────────
    VxSet(bool),
    VxRead,

    // ── XT — XIT function (TX500) ────────────────────────────────────────
    XtSet(bool),
    XtRead,

    // ── Raw fallback ─────────────────────────────────────────────────────
    Raw(String),
}

/// Memory channel data used by MR/MW commands.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryData {
    pub channel: u8,
    pub frequency: u64,
    pub mode: Mode,
    /// 0 = OFF, 1 = Pre-amp, 2 = Attenuator
    pub preamp_att: u8,
}
