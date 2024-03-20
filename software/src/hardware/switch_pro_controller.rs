use evdev::{AbsInfo, AbsoluteAxisType, Device, EvdevEnum};

// PowerA Switch Pro Controller
pub struct SwitchProController {
    device: Device,

    axes: [f32; 4],
    buttons: [u8; 6],
    dpad: [(u8, u8); 1],
}

impl SwitchProController {
    fn update(&mut self) {
        let axes = self.device.get_abs_state().unwrap();
        self.axes = [
            decode_axis(0, 255, axes[AbsoluteAxisType::ABS_X.to_index()].value),
            decode_axis(0, 255, axes[AbsoluteAxisType::ABS_Y.to_index()].value),
            decode_axis(0, 255, axes[AbsoluteAxisType::ABS_Z.to_index()].value),
            decode_axis(0, 255, axes[AbsoluteAxisType::ABS_RZ.to_index()].value)
        ]

    }
}

impl Default for SwitchProController {
    fn default() -> Self {
        Self {
            device: Device::open("/dev/input/event0").expect("Input device not found"),
            axes: [0.0, 0.0, 0.0, 0.0],
            buttons: [0, 0, 0, 0, 0, 0],
            dpad: [(0,0)]
        }
    }
}

enum Buttons {
    LeftBumper,
    RightBumper,
    LeftTrigger,  // BTN_TL
    RightTrigger, // BTN_TR
    LeftStick,
    RightStick,
    A,
    B,
    X,
    Y,
    Back,
    Start,
    Minus,
    Plus,
}

enum Axes {
    LeftX,      // ABS_X
    LeftY,      // ABS_Y
    RightX,     // ABS_Z
    RightY,     // ABS_RZ
}

enum Dpad {
    Dpad0
}

fn decode_axis(min: i32, max: i32, value: i32) -> f32 {
    (value as f32) / (((max+1-min) / 2) as f32) - 1.0
}