use modpadctrl::{Brightness, Effect, ModpadApi};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Effect {
        #[arg(value_enum)]
        effect: EffectType
    },
    Brightness {
        #[arg(value_enum)]
        direction: BrightnessDir
    },
    Profile {
        #[arg(value_parser = profile_in_range)]
        profile: u8
    },
    Map {
        key_code: u16,
        #[arg(value_parser = profile_in_range)]
        profile: u8,
        #[arg(value_parser = row_in_range)]
        row: u8,
        #[arg(value_parser = column_in_range)]
        column: u8
    }
}

#[derive(Clone, ValueEnum, Debug)]
enum EffectType {
    Off,
    MaxBrightness,
    Breathing,
    ButtonActivated,
    CustomBrightness,
    Random
}

#[derive(Clone, ValueEnum, Debug)]
enum BrightnessDir {
    Inc,
    Dec
}

fn main() {
    let cli = Cli::parse();

    let modpad_api = ModpadApi::new().expect("Creating MacropadApi failed");

    match cli.command {
        Commands::Effect { effect } => {
            let effect_type = match effect {
                EffectType::Off => Effect::Off,
                EffectType::MaxBrightness => Effect::MaxBrightness,
                EffectType::Breathing => Effect::Breathing,
                EffectType::ButtonActivated => Effect::ButtonActivated,
                EffectType::CustomBrightness => Effect::CustomBrightness,
                EffectType::Random => Effect::Random
            };
            modpad_api.set_effect(effect_type).expect("Can't set effect");
        },
        Commands::Brightness { direction } => {
            let brightness_dir = match direction {
                BrightnessDir::Inc => Brightness::BrightnessIncrease,
                BrightnessDir::Dec => Brightness::BrightnessDecrease
            };
            modpad_api.change_brightness(brightness_dir).expect("Can't set brightness");
        },
        Commands::Profile { profile } => {
            modpad_api.switch_profile(profile).expect("Can't change profile");
        },
        Commands::Map { key_code, profile, row, column } => {
            modpad_api.map(key_code, profile, row, column).expect("Mapping failed");
        }
    }
}

fn profile_in_range(s: &str) -> Result<u8, String> {
    let profile_range = 1..=ModpadApi::PROFILE_COUNT;

    let profile = s.parse::<u8>().map_err(|_| format!("`{s}` isn't a profile number"))?;

    if profile_range.contains(&profile) {
        Ok(profile)
    } else {
        Err(format!(
            "profile not in range {}-{}",
            profile_range.start(),
            profile_range.end()
        ))
    }
}

fn row_in_range(s: &str) -> Result<u8, String> {
    let row_range = 1..=ModpadApi::ROW_COUNT;

    let row = s.parse::<u8>().map_err(|_| format!("`{s}` isn't a row number"))?;

    if row_range.contains(&row) {
        Ok(row)
    } else {
        Err(format!(
            "row not in range {}-{}",
            row_range.start(),
            row_range.end()
        ))
    }
}

fn column_in_range(s: &str) -> Result<u8, String> {
    let column_range = 1..=ModpadApi::COLUMN_COUNT;

    let column = s.parse::<u8>().map_err(|_| format!("`{s}` isn't a column number"))?;

    if column_range.contains(&column) {
        Ok(column)
    } else {
        Err(format!(
            "column not in range {}-{}",
            column_range.start(),
            column_range.end()
        ))
    }
}
