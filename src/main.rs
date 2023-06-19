#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;

use hal::{
    clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay,
    Rtc,
};

#[entry]
fn main() -> ! {
    let (mut echo_end, mut echo_start): (u64, u64);


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

    let mut trig = io.pins.gpio2.into_push_pull_output();
    let echo = io.pins.gpio15.into_floating_input();

    
    // Initialize the Delay peripheral and activate timer
    let mut delay: Delay = Delay::new(&clocks);
    timer_group0.timer0.set_counter_active(true);


    loop {
        // Clears the trigPin
        trig.set_low().unwrap();
        delay.delay_ms(5_u32);
        

        // Sets the trigPin on HIGH state for 10 micro seconds
        trig.set_high().unwrap();
        delay.delay_ms(10_u32);
        trig.set_low().unwrap();


        // Wait until pin goes high
        while !echo.is_high().unwrap() {}


        // Kick off timer measurement
        echo_start = timer_group0.timer0.now();


        // Wait until pin goes low
        while !echo.is_low().unwrap() {}


        // Collect current timer count
        echo_end = timer_group0.timer0.now();


        // Calculate the elapsed timer count, and the distance in cms
        let distance_cm = (echo_end - echo_start) / 16 / 58;


        // Print the distance output
        println!("Distance {} cm\r", distance_cm);
    }
    
}
