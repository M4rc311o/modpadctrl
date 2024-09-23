
use clap::ValueEnum;
use error::ModpadApiError;
use hidapi::{HidApi, HidDevice};
use keyboard_keypad_page::KeyboardKeypadPage;

pub mod error;
pub mod keyboard_keypad_page;

pub struct ModpadApi {
    modpad_slider: HidDevice,
    modpad_feature: HidDevice
}

impl ModpadApi {
    pub const PROFILE_COUNT: u8 = 4;
    pub const ROW_COUNT: u8 = 2;
    pub const COLUMN_COUNT: u8 = 4;
    pub const KEY_COUNT: u8 = Self::ROW_COUNT * Self::COLUMN_COUNT;
    pub const SLIDER_COUNT: u8 = 3;

    pub fn new() -> Result<Self, ModpadApiError> {
        const VID: u16 = 0x03eb;
        const PID: u16 = 0x2066;
        const USAGE_PAGE: u16 = 0xff;
        const INTERFACE_NUMBER: i32 = 1;

        let mut hidapi_ctx = HidApi::new_without_enumerate()?;
        hidapi_ctx.add_devices(VID, PID)?;

        let modpad_feature_info_opt = hidapi_ctx.device_list().find(|device| {
            device.usage_page() == USAGE_PAGE
        });
        let modpad_slider_info_opt = hidapi_ctx.device_list().find(|device| {
            device.interface_number() == INTERFACE_NUMBER
        });

        let modpad_feature_path = match modpad_feature_info_opt {
            Some(modpad_device_info) => modpad_device_info.path(),
            None => return Err(ModpadApiError::ModpadNotFound)
        };
        let modpad_slider_path = match modpad_slider_info_opt {
            Some(modpad_device_info) => modpad_device_info.path(),
            None => return Err(ModpadApiError::ModpadNotFound)
        };

        let modpad_feature = hidapi_ctx.open_path(modpad_feature_path)?;
        let modpad_slider = hidapi_ctx.open_path(modpad_slider_path)?;

        Ok(Self {
            modpad_slider,
            modpad_feature
        })
    }

    fn send_command(&self, modpad_command_report: ModpadCommandReport) -> Result<(), ModpadApiError> {
        let mut buffer = [0u8; 8];

        buffer[0] = modpad_command_report.report_id;
        buffer[2] = (modpad_command_report.command >> 8) as u8;
        buffer[1] = (modpad_command_report.command & 0xff) as u8;
        buffer[4] = (modpad_command_report.value >> 8) as u8;
        buffer[3] = (modpad_command_report.value & 0xff) as u8;
        buffer[5] = modpad_command_report.optional_1;
        buffer[6] = modpad_command_report.optional_2;
        buffer[7] = modpad_command_report.optional_3;

        self.modpad_feature.send_feature_report(&buffer)?;
        log::debug!("Sent feature report: {buffer:?}");

        Ok(())
    }

    pub fn read_sliders(&self) -> Result<Vec<u8>, ModpadApiError> {
        let mut buf = [0u8; 8];
        let len = self.modpad_slider.read(&mut buf)?;
        let data: Vec<u8> = buf[..len].to_vec();
        Ok(data)
    }

    pub fn set_effect(&self, effect: Effect, module: Module) -> Result<(), ModpadApiError> {
        self.send_command(ModpadCommandReport {
            report_id: 0x03,
            command: 0x01,
            value: match effect {
                Effect::Off => 0x101,
                Effect::MaxBrightness => 0x102,
                Effect::Breathing => 0x103,
                Effect::InputActivated => 0x104,
                Effect::CustomBrightness => 0x105,
                Effect::Random => 0x106
            },
            optional_1: 0,
            optional_2: 0,
            optional_3: module as u8
        })
    }

    pub fn change_brightness(&self, brightness_dir: Brightness, module: Module) -> Result<(), ModpadApiError> {
        self.send_command(ModpadCommandReport {
            report_id: 0x03,
            command: 0x02,
            value: match brightness_dir {
                Brightness::Increase => 0x20a,
                Brightness::Decrease => 0x20b
            },
            optional_1: 0,
            optional_2: 0,
            optional_3: module as u8
        })
    }

    pub fn switch_profile(&self, profile_number: u8, module: Module) -> Result<(), ModpadApiError> {
        if (1..=Self::PROFILE_COUNT).contains(&profile_number) {
            self.send_command(ModpadCommandReport {
                report_id: 0x03,
                command: 0x03,
                value: (profile_number - 1) as u16,
                optional_1: 0,
                optional_2: 0,
                optional_3: module as u8
            })
        } else {
            Err(ModpadApiError::CommandArgumentInvalid)
        }
    }

    pub fn map(&self, key_code: KeyboardKeypadPage, profile_number: u8, key_number: u8, module: Module) -> Result<(), ModpadApiError> {
        if (1..=Self::PROFILE_COUNT).contains(&profile_number) && (1..=Self::KEY_COUNT).contains(&key_number) {
            self.send_command(ModpadCommandReport {
                report_id: 0x03,
                command: 0x04,
                value: key_code as u16,
                optional_1: profile_number - 1,
                optional_2: key_number - 1,
                optional_3: module as u8
            })
        } else {
            Err(ModpadApiError::CommandArgumentInvalid)
        }
    }
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Effect {   
    Off,
    MaxBrightness,
    Breathing,
    InputActivated,
    CustomBrightness,
    Random
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Brightness {
    Increase,
    Decrease
}

#[derive(Clone, ValueEnum, Debug)]
#[repr(u8)]
pub enum Module {
    Modpad,
    Down,
    Left,
    Right
}

struct ModpadCommandReport {
    report_id: u8,
    command: u16,
    value: u16,
    optional_1: u8,
    optional_2: u8,
    optional_3: u8
}
