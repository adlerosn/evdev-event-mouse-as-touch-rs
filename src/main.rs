mod input_event_mouse;
mod mouse_mover;
mod screen;
mod trackpad;
mod trackpad_behavior;

use evdev::Device;

use crate::input_event_mouse::find_event_mouse;
use crate::mouse_mover::MoveMouse;
use crate::trackpad_behavior::initialize_trackpad;
use crate::trackpad_behavior::Notifiable;

fn main() {
    let evmouse_path = find_event_mouse().expect("No event mouse found on local machine");
    println!("Using event-mouse: {:?}", evmouse_path);
    let mut device = Device::open(&evmouse_path).unwrap();
    device.grab().unwrap();
    let mut trackpad = initialize_trackpad(&device, &evmouse_path);
    println!("{:#?}", trackpad.definition);
    trackpad.action = Box::new(MoveMouse::new());
    println!("Listenting events...");
    while let Ok(events_fetched) = device.fetch_events() {
        for event_fetched in events_fetched {
            trackpad.notify(event_fetched);
        }
    }
}
