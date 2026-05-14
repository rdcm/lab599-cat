use crate::hardware::radio::Radio;

macro_rules! poll {
    ($radio:expr, $fetch:ident, $field:ident, $tag:literal) => {
        match $radio.$fetch() {
            Ok(v) => $radio.state_mut().$field = v,
            Err(e) => $radio
                .state_mut()
                .log_error(format!(concat!($tag, ": {}"), e)),
        }
    };
}

pub fn tick(radio: &mut Radio) {
    poll!(radio, get_frequency, frequency, "FA");
    poll!(radio, get_mode, mode, "MD");
    poll!(radio, get_filter, filter, "FL");
    poll!(radio, get_smeter, smeter, "SM");
    poll!(radio, get_ptt, ptt, "PT");
    poll!(radio, get_cmr, cmr, "PR");
    poll!(radio, get_preamp, preamp, "PA");
    poll!(radio, get_attenuator, attenuator, "RA");
    poll!(radio, get_split, split, "SP");
    poll!(radio, get_vox, vox, "VX");
    poll!(radio, get_nr, nr, "NR");
    poll!(radio, get_nb, nb, "NB");
    poll!(radio, get_notch, notch, "NT");
    poll!(radio, get_mon, mon, "MO");
    poll!(radio, get_dif, dif, "IS");
    poll!(radio, get_power, power, "PC");
    poll!(radio, get_af_gain, af_gain, "AG");
    poll!(radio, get_voltage, voltage, "VL");
    poll!(radio, get_busy, busy, "BY");

    if radio.state().ptt {
        poll!(radio, get_swr, swr, "RM");
    } else {
        radio.state_mut().swr = 0;
    }
}
