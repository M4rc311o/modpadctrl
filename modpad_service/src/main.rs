use std::fs;
use serde::{Deserialize, Serialize};
use modpadctrl::ModpadApi;
use modpad_service::windows_volume_control::ApplicationManager;

#[derive(Debug, Serialize, Deserialize)]
struct Slider {
    application: String,
    #[serde(default)]
    session: Option<usize>
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    sliders: Vec<Slider>
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let application_manager = ApplicationManager::new().expect("Failed to create application manager");

    let config_str = fs::read_to_string("sliders.toml").expect("Failed to read config");
    let config: Config = toml::from_str(&config_str).expect("Failed to parse config");

    let modpad_api = ModpadApi::new().expect("Failed to create Modpad Api");

    let mut prev_sliders_data: Vec<u8> = vec![0u8;ModpadApi::SLIDER_COUNT.into()];
    loop {
        let sliders_data = modpad_api.read_sliders().expect("Failed ot read sliders");
        for (index, slider) in sliders_data.iter().enumerate() {
            if *slider != prev_sliders_data[index] {
                prev_sliders_data[index] = *slider;

                let config_slider = match config.sliders.get(index) {
                    Some(slider) =>  slider,
                    None => continue
                };
                let app_name = &config_slider.application;
                let app = match application_manager.find(&app_name) {
                    Some(app) => app,
                    None => continue
                };

                match config_slider.session {
                    Some(session) => app.set_session_volume((*slider as f32) / 100.0, session).expect("Failed to set volume"),
                    None => app.set_volume((*slider as f32) / 100.0).expect("Failed to set volume")
                }
            }
        }
    }
}
