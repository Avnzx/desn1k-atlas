use std::time::Duration;

use rppal::{gpio::{Gpio, OutputPin}, pwm::{Channel, Polarity, Pwm}};

use crate::command::subsystem::Subsystem;

// BCM numbering https://pinout.xyz/
const MOTOR_L: Channel = Channel::Pwm0;
const MOTOR_R: Channel = Channel::Pwm1;
const MOTOR_T: u8 = 27;
const SERVO_T: u8 = 17;

const PWMPERIOD: Duration = Duration::from_millis(10);

pub struct DriveSubsystem {
  left: Pwm,
  right: Pwm,
  tail: OutputPin,
  tailservo: OutputPin
}

impl DriveSubsystem {
    pub fn new() -> DriveSubsystem {
        DriveSubsystem {
          left: Pwm::with_frequency(MOTOR_L, 100.0, 0.5, Polarity::Normal, true).unwrap(),
          right: Pwm::with_frequency(MOTOR_R, 100.0, 0.5, Polarity::Normal, true).unwrap(),
          tail: Gpio::new().unwrap().get(MOTOR_T).unwrap().into_output(),
          tailservo: Gpio::new().unwrap().get(SERVO_T).unwrap().into_output()
        }
    }

    // Drive, NWU
    // pub fn drive(&mut self, pitch: f32, roll: f32, updown: f32) {
    //   let mut left = 0.0;
    //   let mut right = 0.0;
    //   let mut tail = 0.0;

    //   let pitch = pitch.clamp(-1.0, 1.0);
    //   left -= pitch;
    //   right -= pitch;
    //   tail += pitch;

    //   let roll = roll.clamp(-1.0, 1.0);
    //   left += roll;
    //   right -= roll;

    //   let updown = updown.clamp(-1.0, 1.0);
    //   left += updown;
    //   right += updown;
    //   tail += updown;


    //   // Normalize speeds
    //   let minimum = f32::min(f32::min(left,right),tail);
    //   left += f32::abs(minimum); right += f32::abs(minimum); tail += f32::abs(minimum); // shift all to +ve


    //   let maximum = f32::max(f32::max(left,right),tail);
    //   left = left / maximum;
    //   right = right / maximum;
    //   tail = tail / maximum;

    //   if maximum == 0.0 {
    //       left = 0.0; right = 0.0; tail = 0.0;
    //   }

    //   println!("raw tail: {}, r: {}, l: {}", tail, right, left);


    //   let left_duty = 0.5 + 0.5 * left.clamp(0.0, 1.0);
    //   let right_duty = 0.5 + 0.5 * right.clamp(0.0, 1.0);
    //   let tail_duty = 0.5 + 0.5 * tail.clamp(0.0, 1.0);

    //   println!("tail: {}, r: {}, l: {}", tail_duty, right_duty, left_duty);

    //   self.left.set_duty_cycle(left_duty.into()).unwrap();
    //   self.right.set_duty_cycle(right_duty.into()).unwrap();

    //   let pulse = Duration::from_secs_f64((0.01 * tail_duty).into());
    //   self.tail.set_pwm(Duration::from_millis(10), pulse).unwrap();    }

    pub fn drive_tail_normal(&mut self, tail: f32, front: f32) {
      self.left.set_duty_cycle(Self::throttle_to_duty(front).into()).unwrap();
      self.right.set_duty_cycle(Self::throttle_to_duty(front).into()).unwrap();
      self.tail.set_pwm(PWMPERIOD, Self::throttle_to_pulse(PWMPERIOD, tail)).unwrap();
    }

    #[inline]
    fn throttle_to_duty(throttle: f32) -> f32 {
      0.5 + 0.5 * throttle.clamp(0.0, 0.99)
    }

    #[inline]
    fn throttle_to_pulse(period: Duration, throttle: f32) -> Duration {
      let duty = 0.5 + 0.5 * throttle.clamp(0.0, 0.99);
      period.mul_f32(duty)
    }
}

impl Subsystem for DriveSubsystem {
    fn periodic(&self) {
    }
}
