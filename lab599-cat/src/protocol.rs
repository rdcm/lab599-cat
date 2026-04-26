use crate::{CatError, Command, MemoryData, Response};

pub struct Protocol;

impl Protocol {
    pub fn encode(cmd: &Command) -> String {
        match cmd {
            Command::AcSet {
                at_on,
                start_tuning,
            } => {
                format!("AC0{}{};", flag(*at_on), flag(*start_tuning))
            }
            Command::AcRead => "AC;".into(),

            Command::AgSet(v) => format!("AG0{v:03};"),
            Command::AgRead => "AG0;".into(),

            Command::AlSet(v) => format!("AL{};", flag(*v)),
            Command::AlRead => "AL;".into(),

            Command::ByRead => "BY;".into(),

            Command::BandDown => "BD;".into(),
            Command::BandUp => "BU;".into(),

            Command::CgSet(v) => format!("CG{v:03};"),
            Command::CgRead => "CG;".into(),

            Command::CnSet(v) => format!("CN{v:02};"),
            Command::CnRead => "CN;".into(),

            Command::CtSet(v) => format!("CT{};", flag(*v)),
            Command::CtRead => "CT;".into(),

            Command::FaSet(freq) => format!("FA{freq:011};"),
            Command::FaRead => "FA;".into(),

            Command::FbSet(freq) => format!("FB{freq:011};"),
            Command::FbRead => "FB;".into(),

            Command::FlSet { rx, tx } => format!("FL{rx}{tx};"),
            Command::FlRead => "FL;".into(),

            Command::FrSet(sel) => format!("FR{};", *sel as u8),
            Command::FrRead => "FR;".into(),

            Command::FtSet(sel) => format!("FT{};", *sel as u8),
            Command::FtRead => "FT;".into(),

            Command::GtSet(v) => format!("GT{v:02};"),
            Command::GtRead => "GT;".into(),

            Command::IdRead => "ID;".into(),

            Command::IfRead => "IF;".into(),

            Command::IsSet(v) => format!("IS{};", flag(*v)),
            Command::IsRead => "IS;".into(),

            Command::KsSet(v) => format!("KS{v:03};"),
            Command::KsRead => "KS;".into(),

            Command::LkSet(v) => format!("LK{}0;", flag(*v)),
            Command::LkRead => "LK;".into(),

            Command::MaSet(v) => format!("MA{v:03};"),
            Command::MaRead => "MA;".into(),

            Command::McSet(v) => format!("MC0{v:02};"),
            Command::McRead => "MC;".into(),

            Command::MdSet(mode) => format!("MD{};", *mode as u8),
            Command::MdRead => "MD;".into(),

            Command::MgSet(v) => format!("MG{v:03};"),
            Command::MgRead => "MG;".into(),

            Command::MlSet(v) => format!("ML{v:03};"),
            Command::MlRead => "ML;".into(),

            Command::MoSet(v) => format!("MO{};", flag(*v)),
            Command::MoRead => "MO;".into(),

            Command::MrRead(ch) => format!("MR0{ch:02};"),

            Command::MwWrite(d) => encode_mw(d),

            Command::NbSet(v) => format!("NB{};", flag(*v)),
            Command::NbRead => "NB;".into(),

            Command::NfSet(v) => format!("NF{};", flag(*v)),
            Command::NfRead => "NF;".into(),

            Command::NlSet(v) => format!("NL{v:03};"),
            Command::NlRead => "NL;".into(),

            Command::NrSet(v) => format!("NR{};", flag(*v)),
            Command::NrRead => "NR;".into(),

            Command::NtSet(v) => format!("NT{};", flag(*v)),
            Command::NtRead => "NT;".into(),

            Command::PaSet(v) => format!("PA{};", flag(*v)),
            Command::PaRead => "PA;".into(),

            Command::PcSet(v) => format!("PC{v:03};"),
            Command::PcRead => "PC;".into(),

            Command::PlSet(v) => format!("PL{v:03};"),
            Command::PlRead => "PL;".into(),

            Command::PrSet(v) => format!("PR{};", flag(*v)),
            Command::PrRead => "PR;".into(),

            Command::PsSet(v) => format!("PS{};", flag(*v)),
            Command::PsRead => "PS;".into(),

            Command::PtSet(v) => format!("PT{};", flag(*v)),
            Command::PtRead => "PT;".into(),

            Command::RaSet(v) => format!("RA{:02};", if *v { 1u8 } else { 0u8 }),
            Command::RaRead => "RA;".into(),

            Command::RgSet(v) => format!("RG{v:03};"),
            Command::RgRead => "RG;".into(),

            Command::RlSet(v) => format!("RL{v:03};"),
            Command::RlRead => "RL;".into(),

            Command::RmSet(m) => format!("RM{};", *m as u8),
            Command::RmRead => "RM;".into(),

            Command::RtSet(v) => format!("RT{};", flag(*v)),
            Command::RtRead => "RT;".into(),

            Command::RxMode => "RX;".into(),

            Command::SmRead => "SM0;".into(),

            Command::SpSet(v) => format!("SP{};", flag(*v)),
            Command::SpRead => "SP;".into(),

            Command::SqSet(v) => format!("SQ0{v:03};"),
            Command::SqRead => "SQ0;".into(),

            Command::TmSet {
                hour,
                minute,
                second,
            } => {
                format!("TM{hour:02}:{minute:02}:{second:02};")
            }
            Command::TmRead => "TM;".into(),

            Command::ToSet(v) => format!("TO{};", flag(*v)),
            Command::ToRead => "TO;".into(),

            Command::TpSet(v) => format!("TP{v:03};"),
            Command::TpRead => "TP;".into(),

            Command::TxMode => "TX;".into(),

            Command::VdSet(v) => format!("VD{v:04};"),
            Command::VdRead => "VD;".into(),

            Command::VgSet(v) => format!("VG{v:03};"),
            Command::VgRead => "VG;".into(),

            Command::VlRead => "VL;".into(),

            Command::VfoCopyAb => "VV;".into(),

            Command::VxSet(v) => format!("VX{};", flag(*v)),
            Command::VxRead => "VX;".into(),

            Command::XtSet(v) => format!("XT{};", flag(*v)),
            Command::XtRead => "XT;".into(),

            Command::Raw(s) => s.clone(),
        }
    }

    pub fn decode(input: &str) -> Result<Response, CatError> {
        Response::parse(input)
    }
}

fn flag(v: bool) -> u8 {
    if v {
        1
    } else {
        0
    }
}

fn encode_mw(d: &MemoryData) -> String {
    // MW00{CH:02}{FREQ:011}{MODE}{PA}0000000000000000000000000000000 ;
    // Trailing fixed fields: P7=0, P8P9=00, P10=000, P11P12=00, P13P14P15=000, P16=space
    format!(
        "MW00{:02}{:011}{}{}0000000000000000000000000000000 ;",
        d.channel, d.frequency, d.mode as u8, d.preamp_att,
    )
}
