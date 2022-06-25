use core::fmt;
use std::{io, error::Error, fmt::Display};

#[derive(Debug)]
pub enum ArduinoError {
    Timeout,
    IoError,
}

impl From<io::Error> for ArduinoError {
    fn from(_err: io::Error) -> Self {
        Self::IoError
    }
}

impl Display for ArduinoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArduinoError::IoError => {
                write!(f, "IO error with Arduino")
            },
            ArduinoError::Timeout => {
                write!(f, "Arduino timed out")
            }
        }
    }
}

impl Error for ArduinoError {}