use modpadctrl::{command, ModpadApi};

fn main() {
        let modpad_api = ModpadApi::new().expect("Creating MacropadApi failed");

        modpad_api.send_command(&command::Brightness::BrightnessIncrease).expect("Unable to send command");
        let modpad_command = command::Mapping::new(0x15, 1, 1, 2).expect("That's a wrong number");
        modpad_api.send_command(&modpad_command).expect("Unable to send command");
}
