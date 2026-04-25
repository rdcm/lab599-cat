use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatError {
    #[error("invalid command format")]
    InvalidFormat,

    #[error("parse error")]
    ParseError,

    #[error("device error: {0}")]
    DeviceError(String),

    #[error("unknown response")]
    UnknownResponse,
}
