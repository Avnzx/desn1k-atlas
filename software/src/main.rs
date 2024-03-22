extern crate uom;

use std::time::{Duration, Instant};

use command::command_switch_controller::CommandSwitchController;
use embedded_hal::delay::DelayNs;
use rppal::hal::Delay;

pub mod command;
pub mod hardware;
pub mod robot;
pub mod util;

// Run at 10ms intervals, AKA 100Hz
const LOOP_TIME: Duration = Duration::from_millis(10);

fn main() {
    let mut scheduler = command::command_scheduler::CommandScheduler {
        disabled: false,
        ..Default::default()
    };

    let mut delay = Delay::new();
    let mut controller = CommandSwitchController::default();

    // Main loop, where everything happens
    loop {
        let loop_start = Instant::now();

        let _ = controller.raw.update();
        scheduler.run();

        // Loop time - time it took for this iter = time to wait until next iter
        delay.delay_us((LOOP_TIME - (Instant::now() - loop_start)).subsec_micros());
    }
}
