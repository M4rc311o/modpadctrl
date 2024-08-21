use core::fmt;
use std::error::Error;
use hidapi::{HidApi, HidDevice, HidError};

#[derive(Debug)]
#[non_exhaustive]
pub enum MacropadApiError {
    HidApiError(HidError),
    MacropadNotFound
}

impl Error for MacropadApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::HidApiError(ref err) => Some(err),
            _ => None
        }
    }
}

impl fmt::Display for MacropadApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::HidApiError(_) => write!(f, "Underlying HID API error"),
            Self::MacropadNotFound => write!(f, "Macropad not found")
        }
    }
}

impl From<HidError> for MacropadApiError {
    fn from(err: HidError) -> Self {
        Self::HidApiError(err)
    }
}

pub struct MacropadApi {
    hidapi_ctx: HidApi,
    macropad_device: HidDevice
}

impl MacropadApi {
    pub fn new() -> Result<Self, MacropadApiError> {
        const VID: u16 = 0x00;
        const PID: u16 = 0x00;
        const USAGE_PAGE: u16 = 0x00;

        let hidapi_ctx = HidApi::new()?;
        let macropad_device_info_opt = hidapi_ctx.device_list().find(|device| {
            device.vendor_id() == VID &&
            device.product_id() == PID &&
            device.usage_page() == USAGE_PAGE
        });

        let macropad_device_path = match macropad_device_info_opt {
            Some(macropad_device_info) => macropad_device_info.path(),
            None => return Err(MacropadApiError::MacropadNotFound)
        };

        let macropad_device = hidapi_ctx.open_path(macropad_device_path)?;

        Ok(Self {
            hidapi_ctx,
            macropad_device
        })
    }
}
