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

// Run at 20ms intervals, AKA 50Hz
const LOOP_TIME: Duration = Duration::from_millis(20);

fn main() {
    let mut controller = CommandSwitchController::new(None);

    // Create Subsystems
    let mut drive_subsystem = DriveSubsystem::new();
    let claw_subsystem = ClawSubsystem::new();

    let mut scheduler = command::command_scheduler::CommandScheduler {
        disabled: false,
        ..Default::default()
    };


    
    scheduler
        // .register_subsystem(&drive_subsystem)
        .register_subsystem(&claw_subsystem);

    // Main loop, where everything happens
    loop {
        let loop_start = Instant::now();
        // scheduler.run(); // SPWM hates this one trick

        let _ = controller.update(&mut scheduler);

        // drive_subsystem.drive_all_motors(-controller.raw.get_left_y());
        drive_subsystem.drive_tail_normal(-controller.raw.get_left_y(), -controller.raw.get_right_y());

        // Loop time - time it took for this iter = time to wait until next iter
        thread::sleep(LOOP_TIME.saturating_sub(Instant::now().duration_since(loop_start)));
    }
}
