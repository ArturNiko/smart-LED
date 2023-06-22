use alloc::vec::Vec;
use alloc::boxed::Box;
use hal::{gpio::
    {  
        BankGpioRegisterAccess, 
        InteruptStatusRegisterAccess, 
        PinType, 
        GpioSignal,
        GpioPin,
        Input, 
        PullUp, 
        Floating
    },
    timer::{
        Timer0,
        Timer1,
        Timer as ESP_Timer,
        TimerGroupInstance
    }
}; 

enum Timers<TimerGroupInstance> {
    Timer0(Timer0<TimerGroupInstance>),
    Timer1(Timer1<TimerGroupInstance>)
}

type Trigger<const GPIONUM: u8> = GpioPin<Input<PullUp>, Box<dyn BankGpioRegisterAccess>, Box<dyn InteruptStatusRegisterAccess>, Box<dyn PinType>, Box<dyn GpioSignal>, GPIONUM>;
type Echo<const GPIONUM: u8> = GpioPin<Input<Floating>, Box<dyn BankGpioRegisterAccess>, Box<dyn InteruptStatusRegisterAccess>, Box<dyn PinType>, Box<dyn GpioSignal>, GPIONUM>;
type Timer = ESP_Timer<Box<dyn TimerGroupInstance>>;

pub struct USS<const GPIONUM: u8> {
    trigger: Trigger<GPIONUM>,
    echo: Echo<GPIONUM>,
    timer: Timer,

    echo_start: u64,
    echo_end: u64,
    distance_cm: f32
}

pub struct Controller{
    sets: Option<Vec<USS>>,
}

impl Controller {
    pub fn add<const GPIONUM: u8>(&mut self, echo: Echo<GPIONUM>, trigger: Trigger<GPIONUM>, timer: Timer) {
        let sensor_data = USS { trigger, echo, timer, echo_start: 0, echo_end: 0, distance_cm: 0.0 };
        self.sets.push(sensor_data);
    }

    pub fn run(&mut self) {
        // Choose sensor
        self.sets.iter().map(|i| Self::update(self.sets[i]));
    } 

    fn update<const GPIONUM: u8>(sets: USS<GPIONUM>){
        // Wait until pin goes high
        while !sets.echo.is_high().unwrap() {}

        // Kick off timer measurement
        sets.echo_start = sets.timer.now();

        // Wait until pin goes low
        while !sets.echo.is_low().unwrap() {}

        // Collect current timer count
        sets.echo_end = sets.timer.now();

        // Calculate the elapsed timer count, and the distance in cms
        sets.distance_cm = ((sets.echo_end - sets.echo_start) / 16 / 58) as f32;
    }

    pub fn trigger_high(&mut self, id: &Option<u8>) {
             match id {
                Some(i) => self.sets[i].trigger.set_high(),
                None => self.sets.iter().map(|set| set.trigger.set_high())
            }
    }

    pub fn trigger_low(&mut self, id: &Option<u8>) {
        match id {
            Some(i) => self.sets[i].trigger.set_low(),
            None => self.sets.iter().map(|set| set.trigger.set_low())
        }
    }

    pub fn sets(&self) -> Option<Vec<USS>>{
        self.sets
    }
}