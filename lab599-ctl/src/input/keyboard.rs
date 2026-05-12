use std::time::Duration;
use std::{error::Error, fmt};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub struct Keyboard;

impl Keyboard {
    pub fn read_key(millis: u64) -> Result<Option<KeyEvent>> {
        if event::poll(Duration::from_millis(millis))? {
            if let Event::Key(k) = event::read()? {
                if matches!(
                    (k.code, k.modifiers),
                    (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL)
                ) {
                    return Err(Quit.into());
                }
                return Ok(Some(k));
            }
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub struct Quit;

impl fmt::Display for Quit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("quit")
    }
}

impl Error for Quit {}
