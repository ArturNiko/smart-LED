#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay,
    Rtc,
};

mod uss_controller;

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

    let mut trig = io.pins.gpio32.into_push_pull_output();
    let echo = io.pins.gpio33.into_floating_input();

    let uss_controller = uss_controller::Controller { uss: None };

    uss_controller.add(io.pins.gpio15.into_floating_input(), io.pins.gpio2.into_push_pull_output(), timer_group0.timer0);
    uss_controller.add(io.pins.gpio33.into_floating_input(), io.pins.gpio32.into_push_pull_output(), timer_group0.timer0);    
    // Initialize the Delay peripheral and activate timer
    let mut delay: Delay = Delay::new(&clocks);


    loop {
        // Clears the trigPin
        //trig.set_low().unwrap();
        //delay.delay_ms(5_u32);
        

        // Sets the trigPin on HIGH state for 10 micro seconds
        //trig.set_high().unwrap();
        //delay.delay_ms(10_u32);
        //trig.set_low().unwrap();


     
        // Calculate the elapsed timer count, and the distance in cms
        //let distance_cm = (echo_end - echo_start) / 16 / 58;


        // Print the distance output
        //println!("Distance {} cm\r", distance_cm);
    }
    
}
