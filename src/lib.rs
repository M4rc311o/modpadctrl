use core::fmt;
use std::error::Error;
use hidapi::{HidApi, HidDevice, HidError};

#[derive(Debug)]
#[non_exhaustive]
pub enum ModpadApiError {
    HidApiError(HidError),
    ModpadNotFound
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
            Self::ModpadNotFound => write!(f, "Modpad not found")
        }
    }
}

impl From<HidError> for ModpadApiError {
    fn from(err: HidError) -> Self {
        Self::HidApiError(err)
    }
}

pub struct ModpadApi {
    hidapi_ctx: HidApi,
    modpad_device: HidDevice
}

impl ModpadApi {
    pub fn new() -> Result<Self, ModpadApiError> {
        const VID: u16 = 0x03eb;
        const PID: u16 = 0x2066;
        const USAGE_PAGE: u16 = 0xff;

        let hidapi_ctx = HidApi::new()?;
        let modpad_device_info_opt = hidapi_ctx.device_list().find(|device| {
            device.vendor_id() == VID &&
            device.product_id() == PID &&
            device.usage_page() == USAGE_PAGE
        });

        let modpad_device_path = match modpad_device_info_opt {
            Some(modpad_device_info) => modpad_device_info.path(),
            None => return Err(ModpadApiError::ModpadNotFound)
        };

        let modpad_device = hidapi_ctx.open_path(modpad_device_path)?;

        Ok(Self {
            hidapi_ctx,
            modpad_device
        })
    }
}
