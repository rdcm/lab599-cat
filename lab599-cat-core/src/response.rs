use crate::CatError;

#[derive(Debug, Clone)]
pub enum Response {
    Frequency(u64),
    Ok,
    Error,
    Unknown(String),
}

impl Response {
    pub fn parse(input: &str) -> Result<Self, CatError> {
        if input == "?" || input == "?;" {
            return Ok(Response::Error);
        }

        if input.starts_with("FA") {
            let value = input.trim_matches(';').trim_start_matches("FA");
            let freq = value.parse::<u64>().map_err(|_| CatError::ParseError)?;
            return Ok(Response::Frequency(freq));
        }

        Ok(Response::Unknown(input.to_string()))
    }
}
