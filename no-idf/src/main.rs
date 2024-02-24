#![no_std]
#![no_main]

use esp32_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
use esp_backtrace as _;
use esp_println::println;
use esp32_hal::gpio::GpioExt;


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let pins = peripherals.GPIO.split();
    
    let mut led = pins.gpio2.into_push_pull_output();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    println!("Hello world!");
    loop {
        println!("Loop...");
        led.set_high().unwrap();
        delay.delay_ms(500u32);
        led.set_low().unwrap();
        delay.delay_ms(500u32);
    }
}
