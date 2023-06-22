#![no_std]
#![no_main]

extern crate alloc;

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay,
    Rtc,
};

mod uss_controller {
    use alloc;
}

#[entry]
fn main() -> ! {

    // Boilerplate
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt = timer_group0.wdt;
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);


    // Disable MWDT and RWDT (Watchdog) flash boot protection
    wdt.disable();
    rtc.rwdt.disable();


    // Set GPIO5 as an output and GPIO15 analog input
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let uss_controller = uss_controller::Controller { sets: None };

    uss_controller.add(io.pins.gpio15.into_floating_input(), io.pins.gpio2.into_push_pull_output(), timer_group0.timer0);
    uss_controller.add(io.pins.gpio33.into_floating_input(), io.pins.gpio32.into_push_pull_output(), timer_group0.timer1);    
    // Initialize the Delay peripheral and activate timer
    let mut delay: Delay = Delay::new(&clocks);


    loop {
        uss_controller.trigger_low(&None);
        delay.delay_ms(5_u32);

        uss_controller.trigger_high(&None);
        delay.delay_ms(10_u32);
        uss_controller.trigger_low(&None);

        uss_controller.run();


   
        for set in uss_controller.sets().iter() {
            match set {
                Some(s) => println!("Distance {} cm\r", s.distance_cm),
                None => ()
    
            }
        }


     
        // Calculate the elapsed timer count, and the distance in cms
        //let distance_cm = (echo_end - echo_start) / 16 / 58;


        // Print the distance output
        //println!("Distance {} cm\r", distance_cm);
    }
    
}
