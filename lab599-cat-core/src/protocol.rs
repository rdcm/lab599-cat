use crate::{CatError, Command, Response};

/// CAT protocol encoder/decoder
pub struct Protocol;

impl Protocol {
    pub fn encode(cmd: Command) -> String {
        cmd.to_string()
    }

    pub fn decode(input: &str) -> Result<Response, CatError> {
        Response::parse(input)
    }
}
