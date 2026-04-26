use lab599_cat::{Mode, Protocol, Response};

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
