use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

use crate::command::subsystem::Subsystem;

// BCM numbering https://pinout.xyz/
const SERVO_T: u8 = 17; // TODO: Fix
const PWMPERIOD: Duration = Duration::from_millis(100); // TODO: see what min is

pub struct ClawSubsystem {
    servo: OutputPin,
}

impl ClawSubsystem {
    pub fn new() -> ClawSubsystem {
        ClawSubsystem {
            servo: Gpio::new().unwrap().get(SERVO_T).unwrap().into_output(),
        }
    }

    #[inline] // TODO: Fix this method
    fn angle_to_pulse(period: Duration, throttle: f32) -> Duration {
        let duty = 0.5 + 0.5 * throttle.clamp(0.0, 0.99);
        period.mul_f32(duty)
    }
}

impl Subsystem for ClawSubsystem {}
