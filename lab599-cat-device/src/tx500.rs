use lab599_cat_core::{CatError, Command, Protocol, Response};

/// TX-500 device abstraction
pub struct Tx500<T>
where
    T: std::io::Read + std::io::Write,
{
    io: T,
}

impl<T> Tx500<T>
where
    T: std::io::Read + std::io::Write,
{
    pub fn new(io: T) -> Self {
        Self { io }
    }

    pub fn set_frequency(&mut self, freq: u64) -> Result<(), CatError> {
        let cmd = Protocol::encode(Command::SetFrequency(freq));
        self.io
            .write_all(cmd.as_bytes())
            .map_err(|_| CatError::InvalidFormat)?;
        Ok(())
    }

    pub fn get_frequency(&mut self) -> Result<u64, CatError> {
        let cmd = Protocol::encode(Command::GetFrequency);
        self.io
            .write_all(cmd.as_bytes())
            .map_err(|_| CatError::InvalidFormat)?;

        let mut buf = [0u8; 64];
        let n = self
            .io
            .read(&mut buf)
            .map_err(|_| CatError::InvalidFormat)?;
        let resp = std::str::from_utf8(&buf[..n]).unwrap();

        match Protocol::decode(resp)? {
            Response::Frequency(f) => Ok(f),
            _ => Err(CatError::UnknownResponse),
        }
    }
}
