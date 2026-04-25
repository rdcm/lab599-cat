use lab599_cat_core::{Command, MeterType, Mode, Protocol, VfoSelect};

#[test]
fn encode_fa_set() {
    assert_eq!(
        Protocol::encode(&Command::FaSet(7_000_000)),
        "FA00007000000;"
    );
}

#[test]
fn encode_fa_set_14mhz() {
    assert_eq!(
        Protocol::encode(&Command::FaSet(14_195_000)),
        "FA00014195000;"
    );
}

#[test]
fn encode_fa_read() {
    assert_eq!(Protocol::encode(&Command::FaRead), "FA;");
}

#[test]
fn encode_fb_set() {
    assert_eq!(
        Protocol::encode(&Command::FbSet(14_000_000)),
        "FB00014000000;"
    );
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
        Protocol::encode(&Command::TmSet {
            hour: 14,
            minute: 30,
            second: 0
        }),
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
        Protocol::encode(&Command::AcSet {
            at_on: true,
            start_tuning: false
        }),
        "AC010;"
    );
}
