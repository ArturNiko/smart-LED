use esp_backtrace as _;
use esp_println::println;

use hal::{GpioPin, timer::Timer};

struct USS {
    trigger: GpioPin,
    echo: GpioPin,
    timer: Timer,

    echo_start: u64,
    echo_end: u64,
}

struct USSController {
    uss: Vec<USS>
}


impl USSController{
    pub fn initialize(&mut self, echo: GpioPin, trigger: GpioPin, timer: Timer) {
        let sensor_data = USS { trigger, echo, timer, echo_start: 0, echo_end: 0 };
        self.pins.push(sensor_data);
    }

    pub fn triggers_state(&mut self, state: bool) {
        self.pins.iter().map(|set| if state { set.trigger.set_high() } else  { set.trigger.set_low() });
    }

    pub fn run(&mut self, id: Option<u16>) {
        // Choose sensor
        let sensor = self[id.unwrap_or(0)];




        // Wait until pin goes high
        while !sensor.echo.is_high().unwrap() {}


        // Kick off timer measurement
        echo_start = sensor.timer.now();


        // Wait until pin goes low
        while !sensor.echo.is_low().unwrap() {}


        // Collect current timer count
        sensor.echo_end = sensor.timer.now();
    } 


}