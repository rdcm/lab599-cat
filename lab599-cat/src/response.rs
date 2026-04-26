use crate::{CatError, MemoryData, MeterType, Mode, VfoSelect};

/// Full transceiver status returned by the IF command.
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatus {
    pub frequency: u64,
    /// RIT/XIT offset in Hz (±9990)
    pub rit_xit_freq: i32,
    pub rit_on: bool,
    pub xit_on: bool,
    pub memory_channel: u8,
    pub tx: bool,
    pub mode: Mode,
    pub vfo_func: VfoSelect,
    pub scan: bool,
    pub split: bool,
    /// false = OFF, true = CTCSS ON
    pub ctcss_on: bool,
    /// CTCSS/Tone frequency index (0–42)
    pub tone_freq_index: u8,
}

/// All responses the transceiver can produce.
#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    // ── AC ───────────────────────────────────────────────────────────────
    Ac { at_on: bool, tuning: bool },

    // ── AG ───────────────────────────────────────────────────────────────
    Ag(u16),

    // ── AL ───────────────────────────────────────────────────────────────
    Al(bool),

    // ── BY ───────────────────────────────────────────────────────────────
    By { busy: bool },

    // ── CG ───────────────────────────────────────────────────────────────
    Cg(u8),

    // ── CN ───────────────────────────────────────────────────────────────
    Cn(u8),

    // ── CT ───────────────────────────────────────────────────────────────
    Ct(bool),

    // ── FA / FB ──────────────────────────────────────────────────────────
    Fa(u64),
    Fb(u64),

    // ── FL ───────────────────────────────────────────────────────────────
    Fl { rx: u8, tx: u8 },

    // ── FR / FT ──────────────────────────────────────────────────────────
    Fr(VfoSelect),
    Ft(VfoSelect),

    // ── GT ───────────────────────────────────────────────────────────────
    Gt(u8),

    // ── ID ───────────────────────────────────────────────────────────────
    Id(u16),

    // ── IF ───────────────────────────────────────────────────────────────
    If(IfStatus),

    // ── IS ───────────────────────────────────────────────────────────────
    Is(bool),

    // ── KS ───────────────────────────────────────────────────────────────
    Ks(u8),

    // ── LK ───────────────────────────────────────────────────────────────
    Lk(bool),

    // ── MA ───────────────────────────────────────────────────────────────
    Ma(u8),

    // ── MC ───────────────────────────────────────────────────────────────
    Mc(u8),

    // ── MD ───────────────────────────────────────────────────────────────
    Md(Mode),

    // ── MG ───────────────────────────────────────────────────────────────
    Mg(u8),

    // ── ML ───────────────────────────────────────────────────────────────
    Ml(u16),

    // ── MO ───────────────────────────────────────────────────────────────
    Mo(bool),

    // ── MR ───────────────────────────────────────────────────────────────
    Mr(MemoryData),

    // ── NB ───────────────────────────────────────────────────────────────
    Nb(bool),

    // ── NF ───────────────────────────────────────────────────────────────
    Nf(bool),

    // ── NL ───────────────────────────────────────────────────────────────
    Nl(u8),

    // ── NR ───────────────────────────────────────────────────────────────
    Nr(bool),

    // ── NT ───────────────────────────────────────────────────────────────
    Nt(bool),

    // ── PA ───────────────────────────────────────────────────────────────
    Pa(bool),

    // ── PC ───────────────────────────────────────────────────────────────
    Pc(u8),

    // ── PL ───────────────────────────────────────────────────────────────
    Pl(u8),

    // ── PR ───────────────────────────────────────────────────────────────
    Pr(bool),

    // ── PS ───────────────────────────────────────────────────────────────
    Ps(bool),

    // ── PT ───────────────────────────────────────────────────────────────
    Pt(bool),

    // ── RA ───────────────────────────────────────────────────────────────
    Ra(bool),

    // ── RG ───────────────────────────────────────────────────────────────
    Rg(u8),

    // ── RL ───────────────────────────────────────────────────────────────
    Rl(u8),

    // ── RM ───────────────────────────────────────────────────────────────
    Rm { meter: MeterType, value: u16 },

    // ── RT ───────────────────────────────────────────────────────────────
    Rt(bool),

    // ── SM ───────────────────────────────────────────────────────────────
    Sm(u16),

    // ── SP ───────────────────────────────────────────────────────────────
    Sp(bool),

    // ── SQ ───────────────────────────────────────────────────────────────
    Sq(u8),

    // ── TM ───────────────────────────────────────────────────────────────
    Tm { hour: u8, minute: u8, second: u8 },

    // ── TO ───────────────────────────────────────────────────────────────
    To(bool),

    // ── TP ───────────────────────────────────────────────────────────────
    Tp(u8),

    // ── VD ───────────────────────────────────────────────────────────────
    Vd(u16),

    // ── VG ───────────────────────────────────────────────────────────────
    Vg(u8),

    // ── VL ───────────────────────────────────────────────────────────────
    Vl(u16),

    // ── VX ───────────────────────────────────────────────────────────────
    Vx(bool),

    // ── XT ───────────────────────────────────────────────────────────────
    Xt(bool),
}

impl Response {
    pub fn parse(input: &str) -> Result<Self, CatError> {
        let s = input.trim_end_matches(';');

        if s == "?" || s.is_empty() {
            return Err(CatError::CommandError);
        }
        if s == "E" {
            return Err(CatError::CommError);
        }
        if s == "O" {
            return Err(CatError::Busy);
        }

        let code = &s[..s.len().min(2)];
        let params = &s[code.len()..];

        match code {
            "AC" => {
                let b = params.as_bytes();
                if b.len() < 3 {
                    return Err(parse_err("AC", params));
                }
                Ok(Response::Ac {
                    at_on: b[1] == b'1',
                    tuning: b[2] == b'1',
                })
            }

            "AG" => {
                // AG0XXX — skip leading '0'
                let v = params.get(1..).ok_or_else(|| parse_err("AG", params))?;
                Ok(Response::Ag(parse_u16(v, "AG")?))
            }

            "AL" => Ok(Response::Al(parse_bool(params, "AL")?)),

            "BY" => {
                let b = params.as_bytes();
                Ok(Response::By {
                    busy: b.first().copied() == Some(b'1'),
                })
            }

            "CG" => Ok(Response::Cg(parse_u8(params, "CG")?)),

            "CN" => Ok(Response::Cn(parse_u8(params, "CN")?)),

            "CT" => Ok(Response::Ct(parse_bool(params, "CT")?)),

            "FA" => Ok(Response::Fa(parse_u64(params, "FA")?)),

            "FB" => Ok(Response::Fb(parse_u64(params, "FB")?)),

            "FL" => {
                let b = params.as_bytes();
                if b.len() < 2 {
                    return Err(parse_err("FL", params));
                }
                Ok(Response::Fl {
                    rx: digit(b[0], "FL")?,
                    tx: digit(b[1], "FL")?,
                })
            }

            "FR" => {
                let v = parse_u8(params, "FR")?;
                VfoSelect::from_u8(v)
                    .map(Response::Fr)
                    .ok_or_else(|| parse_err("FR", params))
            }

            "FT" => {
                let v = parse_u8(params, "FT")?;
                VfoSelect::from_u8(v)
                    .map(Response::Ft)
                    .ok_or_else(|| parse_err("FT", params))
            }

            "GT" => Ok(Response::Gt(parse_u8(params, "GT")?)),

            "ID" => Ok(Response::Id(parse_u16(params, "ID")?)),

            "IF" => Ok(Response::If(parse_if(params)?)),

            "IS" => Ok(Response::Is(parse_bool(params, "IS")?)),

            "KS" => Ok(Response::Ks(parse_u8(params, "KS")?)),

            "LK" => {
                let b = params.as_bytes();
                Ok(Response::Lk(b.first().copied() == Some(b'1')))
            }

            "MA" => Ok(Response::Ma(parse_u8(params, "MA")?)),

            "MC" => {
                // MC0XX — skip leading '0'
                let v = params.get(1..).ok_or_else(|| parse_err("MC", params))?;
                Ok(Response::Mc(parse_u8(v, "MC")?))
            }

            "MD" => {
                let v = parse_u8(params, "MD")?;
                Mode::from_u8(v)
                    .map(Response::Md)
                    .ok_or_else(|| parse_err("MD", params))
            }

            "MG" => Ok(Response::Mg(parse_u8(params, "MG")?)),

            "ML" => Ok(Response::Ml(parse_u16(params, "ML")?)),

            "MO" => Ok(Response::Mo(parse_bool(params, "MO")?)),

            "MR" => Ok(Response::Mr(parse_mr(params)?)),

            "NB" => Ok(Response::Nb(parse_bool(params, "NB")?)),

            "NF" => Ok(Response::Nf(parse_bool(params, "NF")?)),

            "NL" => Ok(Response::Nl(parse_u8(params, "NL")?)),

            "NR" => Ok(Response::Nr(parse_bool(params, "NR")?)),

            "NT" => Ok(Response::Nt(parse_bool(params, "NT")?)),

            "PA" => {
                let b = params.as_bytes();
                Ok(Response::Pa(b.first().copied() == Some(b'1')))
            }

            "PC" => Ok(Response::Pc(parse_u8(params, "PC")?)),

            "PL" => Ok(Response::Pl(parse_u8(params, "PL")?)),

            "PR" => Ok(Response::Pr(parse_bool(params, "PR")?)),

            "PS" => Ok(Response::Ps(parse_bool(params, "PS")?)),

            "PT" => Ok(Response::Pt(parse_bool(params, "PT")?)),

            "RA" => {
                // P1 is 2-digit: "01" = ON
                let on = params.get(..2).map(|s| s == "01").unwrap_or(false);
                Ok(Response::Ra(on))
            }

            "RG" => Ok(Response::Rg(parse_u8(params, "RG")?)),

            "RL" => Ok(Response::Rl(parse_u8(params, "RL")?)),

            "RM" => {
                let b = params.as_bytes();
                let meter = MeterType::from_u8(digit(b.first().copied().unwrap_or(0), "RM")?)
                    .ok_or_else(|| parse_err("RM", params))?;
                let value = parse_u16(params.get(1..).unwrap_or(""), "RM")?;
                Ok(Response::Rm { meter, value })
            }

            "RT" => Ok(Response::Rt(parse_bool(params, "RT")?)),

            "SM" => {
                // SM0XXXX — skip leading '0'
                let v = params.get(1..).ok_or_else(|| parse_err("SM", params))?;
                Ok(Response::Sm(parse_u16(v, "SM")?))
            }

            "SP" => Ok(Response::Sp(parse_bool(params, "SP")?)),

            "SQ" => {
                // SQ0XXX — skip leading '0'
                let v = params.get(1..).ok_or_else(|| parse_err("SQ", params))?;
                Ok(Response::Sq(parse_u8(v, "SQ")?))
            }

            "TM" => parse_tm(params),

            "TO" => Ok(Response::To(parse_bool(params, "TO")?)),

            "TP" => Ok(Response::Tp(parse_u8(params, "TP")?)),

            "VD" => Ok(Response::Vd(parse_u16(params, "VD")?)),

            "VG" => Ok(Response::Vg(parse_u8(params, "VG")?)),

            "VL" => Ok(Response::Vl(parse_u16(params, "VL")?)),

            "VX" => Ok(Response::Vx(parse_bool(params, "VX")?)),

            "XT" => Ok(Response::Xt(parse_bool(params, "XT")?)),

            other => Err(CatError::UnknownResponse(other.to_string())),
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn parse_err(cmd: &str, params: &str) -> CatError {
    CatError::ParseError(format!("{cmd}: invalid params '{params}'"))
}

fn digit(b: u8, cmd: &str) -> Result<u8, CatError> {
    if b.is_ascii_digit() {
        Ok(b - b'0')
    } else {
        Err(CatError::ParseError(format!(
            "{cmd}: expected digit, got '{}'",
            b as char
        )))
    }
}

fn parse_bool(s: &str, cmd: &str) -> Result<bool, CatError> {
    match s.trim() {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Err(parse_err(cmd, s)),
    }
}

fn parse_u8(s: &str, cmd: &str) -> Result<u8, CatError> {
    s.trim().parse::<u8>().map_err(|_| parse_err(cmd, s))
}

fn parse_u16(s: &str, cmd: &str) -> Result<u16, CatError> {
    s.trim().parse::<u16>().map_err(|_| parse_err(cmd, s))
}

fn parse_u64(s: &str, cmd: &str) -> Result<u64, CatError> {
    s.trim().parse::<u64>().map_err(|_| parse_err(cmd, s))
}

/// Parse IF answer parameters (everything after "IF").
///
/// Format: {11 freq}{5 spaces}{5 rit}{1 rit_on}{1 xit_on}{2 mem}{1 tx}{1 mode}
///         {1 vfo}{1 scan}{1 split}{1 ctcss_flag}{2 tone_freq}{1 always0}
fn parse_if(p: &str) -> Result<IfStatus, CatError> {
    if p.len() < 35 {
        return Err(CatError::ParseError(format!(
            "IF: response too short ({} chars)",
            p.len()
        )));
    }

    let frequency = p[0..11]
        .trim()
        .parse::<u64>()
        .map_err(|_| parse_err("IF", &p[0..11]))?;

    // bytes 11..16 are spaces (P2)
    let rit_str = p[16..21].trim();
    let rit_xit_freq = rit_str.parse::<i32>().unwrap_or(0);

    let b = p.as_bytes();
    let rit_on = b[21] == b'1';
    let xit_on = b[22] == b'1';

    let memory_channel = p[23..25]
        .trim()
        .parse::<u8>()
        .map_err(|_| parse_err("IF", &p[23..25]))?;

    let tx = b[25] == b'1';
    let mode_v = digit(b[26], "IF")?;
    let mode = Mode::from_u8(mode_v).ok_or_else(|| parse_err("IF", &p[26..27]))?;

    let vfo_v = digit(b[27], "IF")?;
    let vfo_func = VfoSelect::from_u8(vfo_v).ok_or_else(|| parse_err("IF", &p[27..28]))?;

    let scan = b[28] == b'1';
    let split = b[29] == b'1';

    let ctcss_flag = b[30];
    let ctcss_on = ctcss_flag == b'2';

    let tone_freq_index = if p.len() >= 33 {
        p[31..33].trim().parse::<u8>().unwrap_or(0)
    } else {
        0
    };

    Ok(IfStatus {
        frequency,
        rit_xit_freq,
        rit_on,
        xit_on,
        memory_channel,
        tx,
        mode,
        vfo_func,
        scan,
        split,
        ctcss_on,
        tone_freq_index,
    })
}

/// Parse MR answer parameters.
fn parse_mr(p: &str) -> Result<MemoryData, CatError> {
    // P1,P2 (always 0), P3 (channel 2 digits), P4 (11 digits freq), P5 (mode), P6 (preamp_att)
    // layout: 00 + CH(2) + FREQ(11) + MODE(1) + PA(1) + ...
    if p.len() < 17 {
        return Err(CatError::ParseError(format!(
            "MR: response too short ({} chars)",
            p.len()
        )));
    }
    let channel = p[2..4]
        .trim()
        .parse::<u8>()
        .map_err(|_| parse_err("MR", &p[2..4]))?;
    let frequency = p[4..15]
        .trim()
        .parse::<u64>()
        .map_err(|_| parse_err("MR", &p[4..15]))?;
    let mode_v = digit(p.as_bytes()[15], "MR")?;
    let mode = Mode::from_u8(mode_v).ok_or_else(|| parse_err("MR", &p[15..16]))?;
    let preamp_att = digit(p.as_bytes()[16], "MR")?;

    Ok(MemoryData {
        channel,
        frequency,
        mode,
        preamp_att,
    })
}

/// Parse TM (time) parameters: "HH:MM:SS"
fn parse_tm(p: &str) -> Result<Response, CatError> {
    // expected: "HH:MM:SS"
    let parts: Vec<&str> = p.splitn(3, ':').collect();
    if parts.len() != 3 {
        return Err(parse_err("TM", p));
    }
    let hour = parts[0].parse::<u8>().map_err(|_| parse_err("TM", p))?;
    let minute = parts[1].parse::<u8>().map_err(|_| parse_err("TM", p))?;
    let second = parts[2].parse::<u8>().map_err(|_| parse_err("TM", p))?;
    Ok(Response::Tm {
        hour,
        minute,
        second,
    })
}
