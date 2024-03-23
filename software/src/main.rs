extern crate uom;

use std::{
    thread::{self},
    time::{Duration, Instant},
};

use command::command_switch_controller::CommandSwitchController;
use subsystems::{claw_subsystem::ClawSubsystem, drive_subsystem::DriveSubsystem};

pub mod command;
pub mod hardware;
pub mod robot;
pub mod subsystems;
pub mod util;

// Run at 10ms intervals, AKA 100Hz
const LOOP_TIME: Duration = Duration::from_millis(10);

fn main() {
    let mut controller = CommandSwitchController::new(None);
    let mut scheduler = command::command_scheduler::CommandScheduler {
        disabled: false,
        ..Default::default()
    };

    // Create Subsystems
    let drive_subsystem = DriveSubsystem::new();
    let claw_subsystem = ClawSubsystem::new();
    scheduler
        .register_subsystem(&drive_subsystem)
        .register_subsystem(&claw_subsystem);

    // Main loop, where everything happens
    loop {
        let loop_start = Instant::now();

        let _ = controller.update(&mut scheduler);
        scheduler.run();

        // Loop time - time it took for this iter = time to wait until next iter
        thread::sleep(LOOP_TIME - (Instant::now() - loop_start));
    }
}
