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
    let mut claw_subsystem = ClawSubsystem::new();

    // Main loop, where everything happens
    loop {
        let loop_start = Instant::now();
        let _ = controller.update();

        // Pushing the stick forward pitches down
        drive_subsystem.drive(
            controller.get_left_y(),
            controller.get_left_x(),
            -controller.get_right_y(),
        );
        // drive_subsystem.drive_tail_normal(
        //     -controller.get_left_y(),
        //     -controller.get_right_y(),
        // );

        // Claw subsystem
        match controller.get_current_pov() {
            180 => claw_subsystem.set_angle(60.0 / 360.0), // Drop object
            90 => claw_subsystem.set_angle(0.0 / 360.0),   // Horizontal Pickup
            270 => claw_subsystem.set_angle(20.0 / 360.0), // Vertical Pickup
            _ => (),
        }

        // Loop time - time it took for this iter = time to wait until next iter
        thread::sleep(LOOP_TIME.saturating_sub(Instant::now().duration_since(loop_start)));
    }
}
