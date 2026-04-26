use lab599_cat::{CatError, Protocol};

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
