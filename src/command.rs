use crate::error::ModpadApiError;

const PROFILE_COUNT: u8 = 4;
const ROW_COUNT: u8 = 2;
const COLUMN_COUNT: u8 = 4;

pub trait ModpadReport {
    fn build_report(&self) -> ModpadCommandReport;
}

pub enum Effect {   
    Off,
    MaxBrightness,
    Breathing,
    ButtonActivated,
    CustomBrightness,
    Random
}

impl ModpadReport for Effect {
    fn build_report(&self) -> ModpadCommandReport {
        ModpadCommandReport {
            report_id: 0x03,
            command_type: 0x01,
            value: match self {
                Self::Off => 0x101,
                Self::MaxBrightness => 0x102,
                Self::Breathing => 0x103,
                Self::ButtonActivated => 0x104,
                Self::CustomBrightness => 0x105,
                Self::Random => 0x106
            },
            profile: 0,
            row: 0,
            column: 0
        }
    }
}

pub enum Brightness {
    BrightnessIncrease,
    BrightnessDecrease
}

impl ModpadReport for Brightness {
    fn build_report(&self) -> ModpadCommandReport {
        ModpadCommandReport {
            report_id: 0x03,
            command_type: 0x02,
            value: match self {
                Self::BrightnessIncrease => 0x20a,
                Self::BrightnessDecrease => 0x20b
            },
            profile: 0,
            row: 0,
            column: 0
        }
    }
}

pub struct Profile {
    profile_number: u8
}

impl Profile {
    pub fn new(profile_number: u8) -> Result<Self, ModpadApiError> {
        if (1..=PROFILE_COUNT).contains(&profile_number) {
            Ok(Self {
                profile_number
            })
        } else {
            Err(ModpadApiError::CommandArgumentInvalid)
        }

    }
}

impl ModpadReport for Profile {
    fn build_report(&self) -> ModpadCommandReport {
        ModpadCommandReport {
            report_id: 0x03,
            command_type: 0x03,
            value: self.profile_number as u16,
            profile: 0,
            row: 0,
            column: 0
        }
    }
}

pub struct  Mapping {
    key_code: u16,
    profile_number: u8,
    row: u8,
    column: u8
}

impl Mapping {
    pub fn new(key_code: u16, profile_number: u8, row: u8, column: u8) -> Result<Self, ModpadApiError> {
        if(1..=PROFILE_COUNT).contains(&profile_number) && (1..=ROW_COUNT).contains(&row) && (1..=COLUMN_COUNT).contains(&column) {
            Ok(Self {
                key_code,
                profile_number,
                row,
                column
            })
        } else {
            Err(ModpadApiError::CommandArgumentInvalid)
        }
    }
}

impl ModpadReport for Mapping {
    fn build_report(&self) -> ModpadCommandReport {
        ModpadCommandReport {
            report_id: 0x03,
            command_type: 0x04,
            value: self.key_code,
            profile: self.profile_number,
            row: self.row,
            column: self.column
        }
    }
}

pub struct ModpadCommandReport {
    pub report_id: u8,
    pub command_type: u16,
    pub value: u16,
    pub profile: u8,
    pub row: u8,
    pub column: u8
}
