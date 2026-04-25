/// Represents CAT command
#[derive(Debug, Clone)]
pub enum Command {
    /// Set frequency VFO A
    SetFrequency(u64),

    /// Read frequency VFO A
    GetFrequency,

    /// Raw command fallback
    Raw(String),
}

impl Command {
    pub fn to_string(&self) -> String {
        match self {
            Command::SetFrequency(freq) => {
                format!("FA{:011};", freq)
            }
            Command::GetFrequency => "FA;".to_string(),
            Command::Raw(cmd) => cmd.clone(),
        }
    }
}
