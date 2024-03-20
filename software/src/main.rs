extern crate uom;

use evdev::EvdevEnum;
use evdev::{AbsoluteAxisType, Device};

pub mod command;
pub mod hardware;
pub mod robot;

fn main() {
    let mut scheduler = command::command_scheduler::CommandScheduler {
        disabled: false,
        ..Default::default()
    };

    let device = Device::open("/dev/input/event0").expect("js0 Not found!");

    // TODO: Ensure a 20ms loop time...
    loop {
        scheduler.run();

        println!("axes {:#?}", device.supported_absolute_axes());
        println!("buttons {:#?}", device.supported_keys());
        println!("switches {:#?}", device.supported_switches());

        println!(
            "{:#?}",
            device.get_abs_state().unwrap()[AbsoluteAxisType::ABS_RZ.to_index()]
        );
    }
}

// let ev = stream.next_event();
// let input =  ev.await.unwrap();
// println!("Button {:?} value {}", input.kind(), input.value());
