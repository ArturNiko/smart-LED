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

struct USS<const GPIONUM: u8> {
    trigger: Trigger<GPIONUM>,
    echo: Echo<GPIONUM>,
    timer: Timer,

    echo_start: u64,
    echo_end: u64,
}

struct Controller{
    uss: Option<Vec<USS>>,
}



impl<const GPIONUM: u8> Controller {
    pub fn initialize(&mut self, 
        echo: GpioPin<Input<Floating>, Box<dyn BankGpioRegisterAccess>, Box<dyn InteruptStatusRegisterAccess>, Box<dyn PinType>, Box<dyn GpioSignal>, GPIONUM>, 
        trigger: GpioPin<Input<PullUp>, Box<dyn BankGpioRegisterAccess>, Box<dyn InteruptStatusRegisterAccess>, Box<dyn PinType>, Box<dyn GpioSignal>, GPIONUM>,
        timer: Timer) {

        let sensor_data = USS { trigger, echo, timer, echo_start: 0, echo_end: 0 };
        //self.pins.push(sensor_data);
    }

    pub fn triggers_state(&mut self, state: bool) {
        //self.pins.iter().map(|set| if state { set.trigger.set_high() } else  { set.trigger.set_low() });
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