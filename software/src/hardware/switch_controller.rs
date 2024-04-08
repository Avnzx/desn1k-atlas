use std::io::{self, ErrorKind};

use evdev::{AbsoluteAxisType, AttributeSet, Device, EvdevEnum, Key};

// PowerA Switch Pro Controller
// waiting for std::mem::variant_count to be stable...
pub struct SwitchController {
    pub device_path: String,
    pub device: Option<Device>,

    pub axes: [f32; 4],
    pub buttons: AttributeSet<Key>,
    pub dpad: [(i8, i8); 1],
}

impl Default for SwitchController {
    fn default() -> Self {
        Self {
            device_path: "/dev/input/event0".into(),
            device: None,
            axes: [0.0, 0.0, 0.0, 0.0],
            buttons: AttributeSet::default(),
            dpad: [(0, 0)],
        }
    }
}

impl SwitchController {
    /// Update internal state of controller, must be called in a tight loop
    pub fn update(&mut self) -> Result<(), io::Error> {
        if !self
            .device
            .as_ref()
            .is_some_and(|x| x.get_abs_state().is_ok())
        {
            self.device = Device::open(&self.device_path).ok();
            return Err(io::Error::new(ErrorKind::Other, "Gamepad disconnected!"));
        }

        self.buttons = self.device.as_ref().unwrap().get_key_state()?;
        let axes = self.device.as_ref().unwrap().get_abs_state()?;

        self.axes = [
            decode_axis(0, 255, axes[AbsoluteAxisType::ABS_X.to_index()].value),
            decode_axis(0, 255, axes[AbsoluteAxisType::ABS_Y.to_index()].value),
            decode_axis(0, 255, axes[AbsoluteAxisType::ABS_Z.to_index()].value),
            decode_axis(0, 255, axes[AbsoluteAxisType::ABS_RZ.to_index()].value),
        ];

        self.dpad = [(
            axes[AbsoluteAxisType::ABS_HAT0X.to_index()]
                .value
                .try_into()
                .expect("DPad Axis value too large!"),
            axes[AbsoluteAxisType::ABS_HAT0Y.to_index()]
                .value
                .try_into()
                .expect("DPad Axis value too large!"),
        )];

        Ok(())
    }

    fn get_raw_button(&self, button: Buttons) -> bool {
        self.buttons.contains(Key::new(button as u16))
    }

    // Clockwise, starts at up (0)
    fn decode_pov(&self) -> u16 {
        match self.dpad[Dpad::Dpad0 as usize] {
            (0, -1) => 0,
            (1, -1) => 45,
            (1, 0) => 90,
            (1, 1) => 135,
            (0, 1) => 180,
            (-1, 1) => 225,
            (-1, 0) => 270,
            (-1, -1) => 315,
            _ => 0,
        }
    }

    // -------------------
    //      Getters
    // -------------------
    pub fn get_left_x(&self) -> f32 {
        self.axes[Axes::LeftX as usize]
    }

    pub fn get_left_y(&self) -> f32 {
        self.axes[Axes::LeftY as usize]
    }

    pub fn get_right_x(&self) -> f32 {
        self.axes[Axes::RightX as usize]
    }

    pub fn get_right_y(&self) -> f32 {
        self.axes[Axes::RightY as usize]
    }

    pub fn get_left_bumper(&self) -> bool {
        self.get_raw_button(Buttons::LeftBumper)
    }
    pub fn get_right_bumper(&self) -> bool {
        self.get_raw_button(Buttons::RightBumper)
    }
    pub fn get_left_trigger(&self) -> bool {
        self.get_raw_button(Buttons::LeftTrigger)
    }
    pub fn get_right_trigger(&self) -> bool {
        self.get_raw_button(Buttons::RightTrigger)
    }

    pub fn get_left_stick(&self) -> bool {
        self.get_raw_button(Buttons::LeftStick)
    }
    pub fn get_right_stick(&self) -> bool {
        self.get_raw_button(Buttons::RightStick)
    }
    pub fn get_a(&self) -> bool {
        self.get_raw_button(Buttons::A)
    }
    pub fn get_b(&self) -> bool {
        self.get_raw_button(Buttons::B)
    }
    pub fn get_x(&self) -> bool {
        self.get_raw_button(Buttons::X)
    }
    pub fn get_y(&self) -> bool {
        self.get_raw_button(Buttons::Y)
    }
    pub fn get_back(&self) -> bool {
        self.get_raw_button(Buttons::Back)
    }
    pub fn get_start(&self) -> bool {
        self.get_raw_button(Buttons::Start)
    }
    pub fn get_minus(&self) -> bool {
        self.get_raw_button(Buttons::Minus)
    }
    pub fn get_plus(&self) -> bool {
        self.get_raw_button(Buttons::Plus)
    }

    /// # Arguments
    /// * `angle` angle in degrees to match against, CW +ve and starts at "up" == 0
    pub fn get_pov(&self, angle: u16) -> bool {
        self.decode_pov() == angle
    }

    pub fn get_current_pov(&self) -> u16 {
        self.decode_pov()
    }
}

enum Buttons {
    LeftBumper = Key::BTN_WEST.0 as isize,
    RightBumper = Key::BTN_Z.0 as isize,
    LeftTrigger = Key::BTN_TL.0 as isize,
    RightTrigger = Key::BTN_TR.0 as isize,
    LeftStick = Key::BTN_SELECT.0 as isize,
    RightStick = Key::BTN_START.0 as isize,
    A = Key::BTN_C.0 as isize,
    B = Key::BTN_EAST.0 as isize,
    X = Key::BTN_NORTH.0 as isize,
    Y = Key::BTN_SOUTH.0 as isize,
    Back = Key::BTN_THUMBL.0 as isize,
    Start = Key::BTN_MODE.0 as isize,
    Minus = Key::BTN_TL2.0 as isize,
    Plus = Key::BTN_TR2.0 as isize,
}

enum Axes {
    LeftX,  // ABS_X
    LeftY,  // ABS_Y
    RightX, // ABS_Z
    RightY, // ABS_RZ
}

enum Dpad {
    Dpad0,
}

fn decode_axis(min: i32, max: i32, value: i32) -> f32 {
    (value as f32) / (((max + 1 - min) / 2) as f32) - 1.0
}
