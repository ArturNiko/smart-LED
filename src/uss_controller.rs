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
}

pub struct Controller{
    uss: Option<Vec<USS>>,
}

impl Controller {
    pub fn add<const GPIONUM: u8>(&mut self, echo: Echo<GPIONUM>, trigger: Trigger<GPIONUM>, timer: Timer) {
        let sensor_data = USS { trigger, echo, timer, echo_start: 0, echo_end: 0 };
        self.uss.push(sensor_data);
    }

    pub fn trigger_state_toggle(&mut self, state: bool, id: &Option<u8>) {
        &self.uss.iter().map(|set| if state { set.trigger.set_high() } else { set.trigger.set_low() });
    }

    pub fn run(&mut self, id: &Option<u8>) {

        // Choose sensor
        match id {
            Some(i) => Self::update(self.uss[i]),
            None => self.uss.iter().map(|i| Self::update(self.uss[i]))

        }
        let sensor = self.uss[id.unwrap_or(0)];

       
    } 

    fn update<const GPIONUM: u8>(uss: USS<GPIONUM>){
        // Wait until pin goes high
        while !uss.echo.is_high().unwrap() {}


        // Kick off timer measurement
        uss.echo_start = uss.timer.now();


        // Wait until pin goes low
        while !uss.echo.is_low().unwrap() {}


        // Collect current timer count
        uss.echo_end = uss.timer.now();
    }


}