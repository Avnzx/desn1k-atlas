extern crate uom;

use std::{
    thread::{self},
    time::{Duration, Instant},
};

use hardware::switch_controller::SwitchController;
use subsystems::{claw_subsystem::ClawSubsystem, drive_subsystem::DriveSubsystem};

pub mod command;
pub mod hardware;
pub mod robot;
pub mod subsystems;
pub mod util;

// Run at 20ms intervals, AKA 50Hz
const LOOP_TIME: Duration = Duration::from_millis(20);

fn main() {
    let mut controller = SwitchController::default();

    // Create Subsystems
    let mut drive_subsystem = DriveSubsystem::new();
    // let claw_subsystem = ClawSubsystem::new();

    // Main loop, where everything happens
    loop {
        let loop_start = Instant::now();
        let _ = controller.update();

        // Pushing the stick forward pitches down
        // drive_subsystem.drive(
        //     controller.get_left_y(),
        //     controller.get_left_x(),
        //     -controller.get_right_y(),
        // );
        drive_subsystem.drive_tail_normal(
            -controller.get_left_y(),
            -controller.get_right_y(),
        );
        // OLD: drive_subsystem.drive_tail_normal(-controller.get_left_y(), -controller.get_right_y());

        // Claw subsystem
        if controller.get_pov(180) { // TODO: Drop object / Open claw
        } else if controller.get_pov(90) { // TODO: Horizontal pickup
        } else if controller.get_pov(270) { // TODO: Vertical Pickup
        }

        // Loop time - time it took for this iter = time to wait until next iter
        thread::sleep(LOOP_TIME.saturating_sub(Instant::now().duration_since(loop_start)));
    }
}
