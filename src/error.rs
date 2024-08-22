use std::{error::Error, fmt};
use hidapi::HidError;

#[derive(Debug)]
#[non_exhaustive]
pub enum ModpadApiError {
    HidApiError(HidError),
    ModpadNotFound,
    CommandArgumentInvalid
}

impl Error for ModpadApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::HidApiError(ref err) => Some(err),
            _ => None
        }
    }
}

impl fmt::Display for ModpadApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::HidApiError(_) => write!(f, "Underlying HID API error"),
            Self::ModpadNotFound => write!(f, "Modpad not found"),
            Self::CommandArgumentInvalid => write!(f, "Invalid command argument")
        }
    }
}

impl From<HidError> for ModpadApiError {
    fn from(err: HidError) -> Self {
        Self::HidApiError(err)
    }
}