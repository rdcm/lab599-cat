use crate::hardware::radio_state::RadioState;
use crate::ui::widgets::{
    error_log::ErrorLogWidget, radio_help::RadioHelpWidget, radio_info::RadioInfoWidget,
    smeter::SmeterWidget, status_flags::StatusFlagsWidget, tui_help::TuiHelpWidget,
};

impl From<&RadioState> for ErrorLogWidget {
    fn from(s: &RadioState) -> Self {
        let entries = s
            .errors
            .iter()
            .map(|(ts, msg)| {
                let secs = ts.elapsed().as_secs();
                if secs < 60 {
                    format!(" [{secs:>3}s ago]  {msg}")
                } else {
                    format!(" [{:>3}m ago]  {msg}", secs / 60)
                }
            })
            .collect();
        Self { entries }
    }
}

impl From<&RadioState> for RadioHelpWidget {
    fn from(_: &RadioState) -> Self {
        Self
    }
}

impl From<&RadioState> for RadioInfoWidget {
    fn from(s: &RadioState) -> Self {
        Self {
            freq: s.freq_display(),
            mode: s.mode_str(),
            step: s.step.label(),
            filter: s.filter_str(),
            ptt: s.ptt,
            power: s.power,
            voltage: s.voltage_display(),
            swr: s.swr_display(),
            af_gain: s.af_gain,
            audio: s.audio_active,
        }
    }
}

impl From<&RadioState> for SmeterWidget {
    fn from(s: &RadioState) -> Self {
        Self {
            smeter: s.smeter,
            label: s.smeter_label(),
        }
    }
}

impl From<&RadioState> for StatusFlagsWidget {
    fn from(s: &RadioState) -> Self {
        Self {
            preamp: s.preamp,
            attenuator: s.attenuator,
            split: s.split,
            cmr: s.cmr,
            vox: s.vox,
            mon: s.mon,
            nr: s.nr,
            nb: s.nb,
            notch: s.notch,
            dif: s.dif,
            busy: s.busy,
        }
    }
}

impl From<&RadioState> for TuiHelpWidget {
    fn from(s: &RadioState) -> Self {
        Self {
            dc_suppress: s.dc_suppress,
        }
    }
}
