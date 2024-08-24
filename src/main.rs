use std::process;

use modpadctrl::{keyboard_keypad_page::KeyboardKeypadPage, Brightness, Effect, ModpadApi};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// More verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Change effect
    Effect {
        #[arg(value_enum)]
        effect: Effect
    },
    /// Increase/Decrease brightness
    Brightness {
        #[arg(value_enum)]
        direction: Brightness
    },
    /// Switch profile
    Profile {
        #[arg(value_parser = profile_in_range)]
        profile: u8
    },
    /// Remap key
    Map {
        /// Key code that will be mapped to specified key
        #[arg(value_enum)]
        key_code: KeyboardKeypadPage,
        /// Profile where to remap key
        #[arg(short, long, value_parser = profile_in_range)]
        profile: u8,
        /// Key row
        #[arg(short, long, value_parser = row_in_range)]
        row: u8,
        /// Key column
        #[arg(short, long, value_parser = column_in_range)]
        column: u8
    }
}

fn main() {
    let cli = Cli::parse();

    let modpad_api = ModpadApi::new().unwrap_or_else(|err| {
        eprintln!(
            "Creating ModpadApi failed: {}",
            if cli.verbose { format!("{err:?}") } else { format!("{err}") }
        );
        process::exit(1);
    });

    match cli.command {
        Commands::Effect { effect } => {
            modpad_api.set_effect(effect).unwrap_or_else(|err| {
                eprintln!(
                    "Changing effect failed: {}",
                    if cli.verbose { format!("{err:?}") } else { format!("{err}") }
                );
                process::exit(1);
            });
        },
        Commands::Brightness { direction } => {
            modpad_api.change_brightness(direction).unwrap_or_else(|err| {
                eprintln!(
                    "Changing brightness failed: {}",
                    if cli.verbose { format!("{err:?}") } else { format!("{err}") }
                );
                process::exit(1);
            });
        },
        Commands::Profile { profile } => {
            modpad_api.switch_profile(profile).unwrap_or_else(|err| {
                eprintln!(
                    "Swithing profile failed: {}",
                    if cli.verbose { format!("{err:?}") } else { format!("{err}") }
                );
                process::exit(1);
            });
        },
        Commands::Map { key_code, profile, row, column } => {
            modpad_api.map(key_code, profile, row, column).unwrap_or_else(|err| {
                eprintln!(
                    "Mapping key failed: {}",
                    if cli.verbose { format!("{err:?}") } else { format!("{err}") }
                );
                process::exit(1);
            });
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
