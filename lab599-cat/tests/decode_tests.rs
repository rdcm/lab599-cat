use lab599_cat::{MeterType, Mode, Protocol, Response, VfoSelect};

#[test]
fn decode_fa_answer() {
    assert_eq!(
        Protocol::decode("FA00007000000;"),
        Ok(Response::Fa(7_000_000))
    );
}

#[test]
fn decode_fa_answer_14mhz() {
    assert_eq!(
        Protocol::decode("FA00014195000;"),
        Ok(Response::Fa(14_195_000))
    );
}

#[test]
fn decode_fb_answer() {
    assert_eq!(
        Protocol::decode("FB00014000000;"),
        Ok(Response::Fb(14_000_000))
    );
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
    assert_eq!(
        Protocol::decode("FT2;"),
        Ok(Response::Ft(VfoSelect::Memory))
    );
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
        Ok(Response::Rm {
            meter: MeterType::Swr,
            value: 15
        })
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
        Ok(Response::Tm {
            hour: 14,
            minute: 30,
            second: 0
        })
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
    assert_eq!(Protocol::decode("VL11.7 ;"), Ok(Response::Vl(117)));
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
        Ok(Response::Ac {
            at_on: true,
            tuning: true
        })
    );
}
