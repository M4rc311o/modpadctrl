use modpadctrl::{Brightness, ModpadApi};
use keycode::KeyMappingId;

fn main() {
        let modpad_api = ModpadApi::new().expect("Creating MacropadApi failed");

        modpad_api.change_brightness(Brightness::BrightnessIncrease).unwrap();
        modpad_api.remap(KeyMappingId::UsB, 1, 1, 2).unwrap();
}
