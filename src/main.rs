use modpadctrl::{Brightness, ModpadApi};

fn main() {
        let modpad_api = ModpadApi::new().expect("Creating MacropadApi failed");

        modpad_api.change_brightness(Brightness::BrightnessIncrease).unwrap();
        modpad_api.remap(0x15, 1, 1, 2).unwrap();
}
