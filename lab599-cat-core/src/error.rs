use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CatError {
    #[error("command syntax error or not valid in current state")]
    CommandError,

    #[error("communication error (overrun or framing)")]
    CommError,

    #[error("receive data not completed")]
    Busy,

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("device error: {0}")]
    DeviceError(String),

    #[error("unknown response: {0}")]
    UnknownResponse(String),
}
