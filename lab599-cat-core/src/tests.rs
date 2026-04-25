use crate::{CatError, Command, MeterType, Mode, Protocol, Response, VfoSelect};

// ── encode ────────────────────────────────────────────────────────────────────

#[test]
fn encode_fa_set() {
    assert_eq!(Protocol::encode(&Command::FaSet(7_000_000)), "FA00007000000;");
}

#[test]
fn encode_fa_set_14mhz() {
    assert_eq!(Protocol::encode(&Command::FaSet(14_195_000)), "FA00014195000;");
}

#[test]
fn encode_fa_read() {
    assert_eq!(Protocol::encode(&Command::FaRead), "FA;");
}

#[test]
fn encode_fb_set() {
    assert_eq!(Protocol::encode(&Command::FbSet(14_000_000)), "FB00014000000;");
}

#[test]
fn encode_md_set_usb() {
    assert_eq!(Protocol::encode(&Command::MdSet(Mode::Usb)), "MD2;");
}

#[test]
fn encode_md_read() {
    assert_eq!(Protocol::encode(&Command::MdRead), "MD;");
}

#[test]
fn encode_ag_set() {
    assert_eq!(Protocol::encode(&Command::AgSet(150)), "AG0150;");
}

#[test]
fn encode_ag_set_zero() {
    assert_eq!(Protocol::encode(&Command::AgSet(0)), "AG0000;");
}

#[test]
fn encode_al_set_type2() {
    assert_eq!(Protocol::encode(&Command::AlSet(true)), "AL1;");
}

#[test]
fn encode_by_read() {
    assert_eq!(Protocol::encode(&Command::ByRead), "BY;");
}

#[test]
fn encode_band_down() {
    assert_eq!(Protocol::encode(&Command::BandDown), "BD;");
}

#[test]
fn encode_band_up() {
    assert_eq!(Protocol::encode(&Command::BandUp), "BU;");
}

#[test]
fn encode_cg_set_off() {
    assert_eq!(Protocol::encode(&Command::CgSet(0)), "CG000;");
}

#[test]
fn encode_cg_set_50() {
    assert_eq!(Protocol::encode(&Command::CgSet(50)), "CG050;");
}

#[test]
fn encode_cn_set() {
    assert_eq!(Protocol::encode(&Command::CnSet(10)), "CN10;");
}

#[test]
fn encode_ct_set_on() {
    assert_eq!(Protocol::encode(&Command::CtSet(true)), "CT1;");
}

#[test]
fn encode_fl_set() {
    assert_eq!(Protocol::encode(&Command::FlSet { rx: 2, tx: 1 }), "FL21;");
}

#[test]
fn encode_fr_set_vfo_b() {
    assert_eq!(Protocol::encode(&Command::FrSet(VfoSelect::VfoB)), "FR1;");
}

#[test]
fn encode_gt_set() {
    assert_eq!(Protocol::encode(&Command::GtSet(5)), "GT05;");
}

#[test]
fn encode_id_read() {
    assert_eq!(Protocol::encode(&Command::IdRead), "ID;");
}

#[test]
fn encode_if_read() {
    assert_eq!(Protocol::encode(&Command::IfRead), "IF;");
}

#[test]
fn encode_is_set_on() {
    assert_eq!(Protocol::encode(&Command::IsSet(true)), "IS1;");
}

#[test]
fn encode_ks_set() {
    assert_eq!(Protocol::encode(&Command::KsSet(25)), "KS025;");
}

#[test]
fn encode_lk_set_on() {
    assert_eq!(Protocol::encode(&Command::LkSet(true)), "LK10;");
}

#[test]
fn encode_mc_set() {
    assert_eq!(Protocol::encode(&Command::McSet(7)), "MC007;");
}

#[test]
fn encode_mc_set_99() {
    assert_eq!(Protocol::encode(&Command::McSet(99)), "MC099;");
}

#[test]
fn encode_mg_set() {
    assert_eq!(Protocol::encode(&Command::MgSet(80)), "MG080;");
}

#[test]
fn encode_ml_set() {
    assert_eq!(Protocol::encode(&Command::MlSet(200)), "ML200;");
}

#[test]
fn encode_mo_set_mute() {
    assert_eq!(Protocol::encode(&Command::MoSet(true)), "MO1;");
}

#[test]
fn encode_mr_read() {
    assert_eq!(Protocol::encode(&Command::MrRead(5)), "MR005;");
}

#[test]
fn encode_nb_set_on() {
    assert_eq!(Protocol::encode(&Command::NbSet(true)), "NB1;");
}

#[test]
fn encode_nf_set_type2() {
    assert_eq!(Protocol::encode(&Command::NfSet(true)), "NF1;");
}

#[test]
fn encode_nl_set() {
    assert_eq!(Protocol::encode(&Command::NlSet(50)), "NL050;");
}

#[test]
fn encode_nr_set_on() {
    assert_eq!(Protocol::encode(&Command::NrSet(true)), "NR1;");
}

#[test]
fn encode_nt_set_auto() {
    assert_eq!(Protocol::encode(&Command::NtSet(true)), "NT1;");
}

#[test]
fn encode_pa_set_on() {
    assert_eq!(Protocol::encode(&Command::PaSet(true)), "PA1;");
}

#[test]
fn encode_pc_set() {
    assert_eq!(Protocol::encode(&Command::PcSet(100)), "PC100;");
}

#[test]
fn encode_pl_set() {
    assert_eq!(Protocol::encode(&Command::PlSet(50)), "PL050;");
}

#[test]
fn encode_pr_set_on() {
    assert_eq!(Protocol::encode(&Command::PrSet(true)), "PR1;");
}

#[test]
fn encode_ps_set_on() {
    assert_eq!(Protocol::encode(&Command::PsSet(true)), "PS1;");
}

#[test]
fn encode_pt_set_tx() {
    assert_eq!(Protocol::encode(&Command::PtSet(true)), "PT1;");
}

#[test]
fn encode_ra_set_on() {
    assert_eq!(Protocol::encode(&Command::RaSet(true)), "RA01;");
}

#[test]
fn encode_ra_set_off() {
    assert_eq!(Protocol::encode(&Command::RaSet(false)), "RA00;");
}

#[test]
fn encode_rg_set() {
    assert_eq!(Protocol::encode(&Command::RgSet(100)), "RG100;");
}

#[test]
fn encode_rl_set() {
    assert_eq!(Protocol::encode(&Command::RlSet(50)), "RL050;");
}

#[test]
fn encode_rm_set_swr() {
    assert_eq!(Protocol::encode(&Command::RmSet(MeterType::Swr)), "RM1;");
}

#[test]
fn encode_rt_set_on() {
    assert_eq!(Protocol::encode(&Command::RtSet(true)), "RT1;");
}

#[test]
fn encode_rx_mode() {
    assert_eq!(Protocol::encode(&Command::RxMode), "RX;");
}

#[test]
fn encode_sm_read() {
    assert_eq!(Protocol::encode(&Command::SmRead), "SM0;");
}

#[test]
fn encode_sp_set_on() {
    assert_eq!(Protocol::encode(&Command::SpSet(true)), "SP1;");
}

#[test]
fn encode_sq_set() {
    assert_eq!(Protocol::encode(&Command::SqSet(128)), "SQ0128;");
}

#[test]
fn encode_tm_set() {
    assert_eq!(
        Protocol::encode(&Command::TmSet { hour: 14, minute: 30, second: 0 }),
        "TM14:30:00;"
    );
}

#[test]
fn encode_to_set_on() {
    assert_eq!(Protocol::encode(&Command::ToSet(true)), "TO1;");
}

#[test]
fn encode_tp_set() {
    assert_eq!(Protocol::encode(&Command::TpSet(10)), "TP010;");
}

#[test]
fn encode_tx_mode() {
    assert_eq!(Protocol::encode(&Command::TxMode), "TX;");
}

#[test]
fn encode_vd_set() {
    assert_eq!(Protocol::encode(&Command::VdSet(500)), "VD0500;");
}

#[test]
fn encode_vg_set() {
    assert_eq!(Protocol::encode(&Command::VgSet(75)), "VG075;");
}

#[test]
fn encode_vl_read() {
    assert_eq!(Protocol::encode(&Command::VlRead), "VL;");
}

#[test]
fn encode_vfo_copy_ab() {
    assert_eq!(Protocol::encode(&Command::VfoCopyAb), "VV;");
}

#[test]
fn encode_vx_set_on() {
    assert_eq!(Protocol::encode(&Command::VxSet(true)), "VX1;");
}

#[test]
fn encode_xt_set_on() {
    assert_eq!(Protocol::encode(&Command::XtSet(true)), "XT1;");
}

#[test]
fn encode_ac_set() {
    assert_eq!(
        Protocol::encode(&Command::AcSet { at_on: true, start_tuning: false }),
        "AC010;"
    );
}

// ── decode ────────────────────────────────────────────────────────────────────

#[test]
fn decode_fa_answer() {
    assert_eq!(Protocol::decode("FA00007000000;"), Ok(Response::Fa(7_000_000)));
}

#[test]
fn decode_fa_answer_14mhz() {
    assert_eq!(Protocol::decode("FA00014195000;"), Ok(Response::Fa(14_195_000)));
}

#[test]
fn decode_fb_answer() {
    assert_eq!(Protocol::decode("FB00014000000;"), Ok(Response::Fb(14_000_000)));
}

#[test]
fn decode_md_usb() {
    assert_eq!(Protocol::decode("MD2;"), Ok(Response::Md(Mode::Usb)));
}

#[test]
fn decode_md_cw_r() {
    assert_eq!(Protocol::decode("MD7;"), Ok(Response::Md(Mode::CwR)));
}

#[test]
fn decode_ag_answer() {
    assert_eq!(Protocol::decode("AG0150;"), Ok(Response::Ag(150)));
}

#[test]
fn decode_al_answer() {
    assert_eq!(Protocol::decode("AL1;"), Ok(Response::Al(true)));
}

#[test]
fn decode_by_busy() {
    assert_eq!(Protocol::decode("BY10;"), Ok(Response::By { busy: true }));
}

#[test]
fn decode_by_not_busy() {
    assert_eq!(Protocol::decode("BY00;"), Ok(Response::By { busy: false }));
}

#[test]
fn decode_cg_answer() {
    assert_eq!(Protocol::decode("CG050;"), Ok(Response::Cg(50)));
}

#[test]
fn decode_cn_answer() {
    assert_eq!(Protocol::decode("CN10;"), Ok(Response::Cn(10)));
}

#[test]
fn decode_ct_on() {
    assert_eq!(Protocol::decode("CT1;"), Ok(Response::Ct(true)));
}

#[test]
fn decode_fl_answer() {
    assert_eq!(Protocol::decode("FL21;"), Ok(Response::Fl { rx: 2, tx: 1 }));
}

#[test]
fn decode_fr_vfo_b() {
    assert_eq!(Protocol::decode("FR1;"), Ok(Response::Fr(VfoSelect::VfoB)));
}

#[test]
fn decode_ft_memory() {
    assert_eq!(Protocol::decode("FT2;"), Ok(Response::Ft(VfoSelect::Memory)));
}

#[test]
fn decode_gt_answer() {
    assert_eq!(Protocol::decode("GT05;"), Ok(Response::Gt(5)));
}

#[test]
fn decode_id_tx500() {
    assert_eq!(Protocol::decode("ID500;"), Ok(Response::Id(500)));
}

#[test]
fn decode_id_tx500mp() {
    assert_eq!(Protocol::decode("ID505;"), Ok(Response::Id(505)));
}

#[test]
fn decode_is_on() {
    assert_eq!(Protocol::decode("IS1;"), Ok(Response::Is(true)));
}

#[test]
fn decode_ks_answer() {
    assert_eq!(Protocol::decode("KS025;"), Ok(Response::Ks(25)));
}

#[test]
fn decode_lk_on() {
    // LK10; — P1=1 (locked), P2=0
    assert_eq!(Protocol::decode("LK10;"), Ok(Response::Lk(true)));
}

#[test]
fn decode_ma_answer() {
    assert_eq!(Protocol::decode("MA050;"), Ok(Response::Ma(50)));
}

#[test]
fn decode_mc_answer() {
    assert_eq!(Protocol::decode("MC007;"), Ok(Response::Mc(7)));
}

#[test]
fn decode_mg_answer() {
    assert_eq!(Protocol::decode("MG080;"), Ok(Response::Mg(80)));
}

#[test]
fn decode_ml_answer() {
    assert_eq!(Protocol::decode("ML200;"), Ok(Response::Ml(200)));
}

#[test]
fn decode_mo_mute() {
    assert_eq!(Protocol::decode("MO1;"), Ok(Response::Mo(true)));
}

#[test]
fn decode_nb_on() {
    assert_eq!(Protocol::decode("NB1;"), Ok(Response::Nb(true)));
}

#[test]
fn decode_nf_type2() {
    assert_eq!(Protocol::decode("NF1;"), Ok(Response::Nf(true)));
}

#[test]
fn decode_nl_answer() {
    assert_eq!(Protocol::decode("NL050;"), Ok(Response::Nl(50)));
}

#[test]
fn decode_nr_on() {
    assert_eq!(Protocol::decode("NR1;"), Ok(Response::Nr(true)));
}

#[test]
fn decode_nt_auto() {
    assert_eq!(Protocol::decode("NT1;"), Ok(Response::Nt(true)));
}

#[test]
fn decode_pa_on() {
    assert_eq!(Protocol::decode("PA10;"), Ok(Response::Pa(true)));
}

#[test]
fn decode_pc_answer() {
    assert_eq!(Protocol::decode("PC100;"), Ok(Response::Pc(100)));
}

#[test]
fn decode_pl_answer() {
    assert_eq!(Protocol::decode("PL050;"), Ok(Response::Pl(50)));
}

#[test]
fn decode_pr_on() {
    assert_eq!(Protocol::decode("PR1;"), Ok(Response::Pr(true)));
}

#[test]
fn decode_ps_on() {
    assert_eq!(Protocol::decode("PS1;"), Ok(Response::Ps(true)));
}

#[test]
fn decode_pt_tx() {
    assert_eq!(Protocol::decode("PT1;"), Ok(Response::Pt(true)));
}

#[test]
fn decode_ra_on() {
    assert_eq!(Protocol::decode("RA0100;"), Ok(Response::Ra(true)));
}

#[test]
fn decode_ra_off() {
    assert_eq!(Protocol::decode("RA0000;"), Ok(Response::Ra(false)));
}

#[test]
fn decode_rg_answer() {
    assert_eq!(Protocol::decode("RG100;"), Ok(Response::Rg(100)));
}

#[test]
fn decode_rl_answer() {
    assert_eq!(Protocol::decode("RL050;"), Ok(Response::Rl(50)));
}

#[test]
fn decode_rm_swr() {
    assert_eq!(
        Protocol::decode("RM10015;"),
        Ok(Response::Rm { meter: MeterType::Swr, value: 15 })
    );
}

#[test]
fn decode_rt_on() {
    assert_eq!(Protocol::decode("RT1;"), Ok(Response::Rt(true)));
}

#[test]
fn decode_sm_answer() {
    assert_eq!(Protocol::decode("SM00015;"), Ok(Response::Sm(15)));
}

#[test]
fn decode_sp_on() {
    assert_eq!(Protocol::decode("SP1;"), Ok(Response::Sp(true)));
}

#[test]
fn decode_sq_answer() {
    assert_eq!(Protocol::decode("SQ0128;"), Ok(Response::Sq(128)));
}

#[test]
fn decode_tm_answer() {
    assert_eq!(
        Protocol::decode("TM14:30:00;"),
        Ok(Response::Tm { hour: 14, minute: 30, second: 0 })
    );
}

#[test]
fn decode_to_on() {
    assert_eq!(Protocol::decode("TO1;"), Ok(Response::To(true)));
}

#[test]
fn decode_tp_answer() {
    assert_eq!(Protocol::decode("TP010;"), Ok(Response::Tp(10)));
}

#[test]
fn decode_vd_answer() {
    assert_eq!(Protocol::decode("VD0500;"), Ok(Response::Vd(500)));
}

#[test]
fn decode_vg_answer() {
    assert_eq!(Protocol::decode("VG075;"), Ok(Response::Vg(75)));
}

#[test]
fn decode_vl_answer() {
    assert_eq!(Protocol::decode("VL1350;"), Ok(Response::Vl(1350)));
}

#[test]
fn decode_vx_on() {
    assert_eq!(Protocol::decode("VX1;"), Ok(Response::Vx(true)));
}

#[test]
fn decode_xt_on() {
    assert_eq!(Protocol::decode("XT1;"), Ok(Response::Xt(true)));
}

#[test]
fn decode_ac_answer() {
    assert_eq!(
        Protocol::decode("AC011;"),
        Ok(Response::Ac { at_on: true, tuning: true })
    );
}

// ── error responses ───────────────────────────────────────────────────────────

#[test]
fn decode_command_error() {
    assert_eq!(Protocol::decode("?;"), Err(CatError::CommandError));
}

#[test]
fn decode_command_error_bare() {
    assert_eq!(Protocol::decode("?"), Err(CatError::CommandError));
}

#[test]
fn decode_comm_error() {
    assert_eq!(Protocol::decode("E;"), Err(CatError::CommError));
}

#[test]
fn decode_busy_error() {
    assert_eq!(Protocol::decode("O;"), Err(CatError::Busy));
}

#[test]
fn decode_unknown_response() {
    assert!(matches!(
        Protocol::decode("ZZ99;"),
        Err(CatError::UnknownResponse(_))
    ));
}

// ── roundtrip encode → decode ────────────────────────────────────────────────

#[test]
fn roundtrip_fa() {
    let cmd = Command::FaSet(14_225_000);
    let encoded = Protocol::encode(&cmd);
    assert_eq!(Protocol::decode(&encoded), Ok(Response::Fa(14_225_000)));
}

#[test]
fn roundtrip_md() {
    for mode in [Mode::Lsb, Mode::Usb, Mode::Cw, Mode::Fm, Mode::Am, Mode::Dig, Mode::CwR] {
        let encoded = Protocol::encode(&Command::MdSet(mode));
        assert_eq!(Protocol::decode(&encoded), Ok(Response::Md(mode)));
    }
}

#[test]
fn roundtrip_ag() {
    for v in [0u16, 1, 127, 250] {
        let encoded = Protocol::encode(&Command::AgSet(v));
        assert_eq!(Protocol::decode(&encoded), Ok(Response::Ag(v)));
    }
}

#[test]
fn roundtrip_tm() {
    let cmd = Command::TmSet { hour: 23, minute: 59, second: 59 };
    let encoded = Protocol::encode(&cmd);
    assert_eq!(
        Protocol::decode(&encoded),
        Ok(Response::Tm { hour: 23, minute: 59, second: 59 })
    );
}

// ── IF parsing ────────────────────────────────────────────────────────────────

#[test]
fn decode_if_answer() {
    // IF + 11(freq) + 5(spaces) + 5(rit) + rit_on + xit_on + 2(mem) + tx + mode + vfo + scan + split + ctcss + 2(tone) + always0
    // "IF00014175000     +0000001002010002000;"
    let raw = "IF00014175000     +000000100201000200 0;";
    let resp = Protocol::decode(raw).unwrap();
    if let Response::If(status) = resp {
        assert_eq!(status.frequency, 14_175_000);
        assert!(!status.rit_on);
        assert!(!status.xit_on);
        assert_eq!(status.memory_channel, 10);
        assert!(!status.tx);
        assert_eq!(status.mode, Mode::Usb);
    } else {
        panic!("expected IF response, got {resp:?}");
    }
}

// ── MR parsing ────────────────────────────────────────────────────────────────

#[test]
fn decode_mr_answer() {
    // After "MR": P1P2(2) + channel(2) + freq(11) + mode(1) + preamp_att(1) + padding
    // "00" + "05" + "00014225000" + "2" + "0" + padding
    let raw = "MR00050001422500020000000000000000000000000000000 ;";
    let resp = Protocol::decode(raw).unwrap();
    if let Response::Mr(data) = resp {
        assert_eq!(data.channel, 5);
        assert_eq!(data.frequency, 14_225_000);
        assert_eq!(data.mode, Mode::Usb);
        assert_eq!(data.preamp_att, 0);
    } else {
        panic!("expected MR response, got {resp:?}");
    }
}
