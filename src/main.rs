#![no_std]
#![no_main]

use esp32_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
use esp_backtrace as _;
use esp_println::println;

extern crate uom;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // setup logger
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");

    loop {
        println!("Loop...");
        delay.delay_ms(500u32);
    }
}
