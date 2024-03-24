use std::time::Duration;

use rppal::{gpio::{Gpio, OutputPin}, pwm::{Channel, Pwm}};

use crate::command::subsystem::Subsystem;

// BCM numbering https://pinout.xyz/
const MOTOR_L: u8 = 17;
const MOTOR_R: u8 = 27;
const MOTOR_T: u8 = 22;
const SERVO_T: Channel = Channel::Pwm0;

pub struct DriveSubsystem {
  left: OutputPin,
  right: OutputPin,
  tail: OutputPin,
  tailservo: Pwm
}

impl DriveSubsystem {
    pub fn new() -> DriveSubsystem {
        DriveSubsystem {
          left: Gpio::new().unwrap().get(MOTOR_L).unwrap().into_output(),
          right: Gpio::new().unwrap().get(MOTOR_R).unwrap().into_output(),
          tail: Gpio::new().unwrap().get(MOTOR_T).unwrap().into_output(),
          tailservo: Pwm::new(SERVO_T).unwrap()
        }
    }

    // Drive, NWU
    pub fn drive(&mut self, forwardback: f32, leftright: f32, updown: f32, yaw: f32) {
      self.left.set_pwm(Duration::from_micros(2000), Duration::from_micros(1000)).unwrap();
    }
}

impl Subsystem for DriveSubsystem {
    fn periodic(&self) {
    }
}
