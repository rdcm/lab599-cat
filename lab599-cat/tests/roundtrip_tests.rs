use lab599_cat::{Command, Mode, Protocol, Response};

#[test]
fn roundtrip_fa() {
    let cmd = Command::FaSet(14_225_000);
    let encoded = Protocol::encode(&cmd);
    assert_eq!(Protocol::decode(&encoded), Ok(Response::Fa(14_225_000)));
}

#[test]
fn roundtrip_md() {
    for mode in [
        Mode::Lsb,
        Mode::Usb,
        Mode::Cw,
        Mode::Fm,
        Mode::Am,
        Mode::Dig,
        Mode::CwR,
    ] {
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
    let cmd = Command::TmSet {
        hour: 23,
        minute: 59,
        second: 59,
    };
    let encoded = Protocol::encode(&cmd);
    assert_eq!(
        Protocol::decode(&encoded),
        Ok(Response::Tm {
            hour: 23,
            minute: 59,
            second: 59
        })
    );
}
