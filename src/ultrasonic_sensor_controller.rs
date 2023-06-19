use esp_backtrace as _;
use esp_println::println;

use hal::{GpioPin};

struct PinSet {
    trigger: GpioPin,
    echo: GpioPin,
    //time: 
}


pub struct USS {
    pins: Vec<PinSet>,
}


impl USS{
    pub fn initialize(&mut self, echo: GpioPin, trigger: GpioPin) {
        let pin_set = PinSet { trigger, echo };
        self.pins.push(PinSet);
    }

    pub fn triggers_state(&mut self, state: bool) {
        self.pins.iter().map(|set| if state { set.trigger.set_high() } else  { set.trigger.set_low() });
    }
}