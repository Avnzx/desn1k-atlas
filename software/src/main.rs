#![no_std]
#![no_main]

use esp32_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay};
use esp_backtrace as _;

extern crate uom;

pub mod command;
pub mod robot;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // let mut timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    // timg0.timer0.reset_counter();
    
    // setup logger
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");

    let mut scheduler = command::command_scheduler::CommandScheduler {
        disabled: false,
        ..Default::default()
    };

    // TODO: Ensure a 20ms loop time... how?
    loop {
        scheduler.run();
        delay.delay_ms(500u32);
    }
}
