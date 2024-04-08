use std::{f32::consts::PI, time::Duration};

use rppal::gpio::{Gpio, OutputPin};

use crate::command::subsystem::Subsystem;

// BCM numbering https://pinout.xyz/
const SERVO_T: u8 = 27;
const PWMPERIOD: Duration = Duration::from_millis(20);

pub struct ClawSubsystem {
    servo: OutputPin,
}

impl ClawSubsystem {
    pub fn new() -> ClawSubsystem {
        ClawSubsystem {
            servo: Gpio::new().unwrap().get(SERVO_T).unwrap().into_output(),
        }
    }

    // CW +ve
    // Angle from normalized [-1, 1]
    pub fn set_angle(&mut self, angle: f32) {
        self.servo
            .set_pwm(PWMPERIOD, Self::duty_to_pulse(PWMPERIOD, angle))
            .unwrap();
    }

    #[inline]
    fn duty_to_pulse(period: Duration, throttle: f32) -> Duration {
        let duty = 0.075 + 0.025 * throttle.clamp(-1.0, 1.0);
        period.mul_f32(duty)
    }
}

impl Subsystem for ClawSubsystem {}
