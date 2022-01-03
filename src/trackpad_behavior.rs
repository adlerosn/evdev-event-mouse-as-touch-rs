use crate::trackpad::TrackedTouchState;
use crate::trackpad::TrackpadActor;
use crate::trackpad::TrackpadDefinition;
use evdev::{AbsoluteAxisType, Device, InputEvent, InputEventKind, Key};
use std::path::Path;

pub fn initialize_trackpad(device: &Device, evmouse_path: &Path) -> TrackpadActor {
    let mut trackpad = TrackpadActor::default();
    trackpad.definition.name = device.name().unwrap_or("").to_string();
    trackpad.definition.log_path = evmouse_path.to_path_buf();
    trackpad.definition.phy_path = device.physical_path().unwrap_or("").to_string();
    println!("Physical path: {:?}", device.physical_path().unwrap_or(""));
    let supported_keys = device
        .supported_keys()
        .map(|x| x.iter().collect::<Vec<Key>>())
        .unwrap_or_default();
    let supported_abxs = device
        .supported_absolute_axes()
        .map(|x| x.iter().collect::<Vec<AbsoluteAxisType>>())
        .unwrap_or_default();
    trackpad.definition.slotable = supported_abxs.contains(&AbsoluteAxisType::ABS_MT_SLOT);
    trackpad.definition.touchable = supported_keys.contains(&Key::BTN_TOUCH);
    trackpad.definition.pressurable = supported_abxs.contains(&AbsoluteAxisType::ABS_PRESSURE);
    let input_absinfos = device.get_abs_state().unwrap();
    trackpad.definition.state.left = false;
    trackpad.definition.state.right = false;
    trackpad.definition.state.middle = false;
    trackpad.definition.state.touching = false;
    trackpad.definition.state.slot = if trackpad.definition.slotable { -1 } else { 0 };
    trackpad.definition.state.touch_limits.min_x =
        input_absinfos[AbsoluteAxisType::ABS_X.0 as usize].minimum;
    trackpad.definition.state.touch_limits.max_x =
        input_absinfos[AbsoluteAxisType::ABS_X.0 as usize].maximum;
    trackpad.definition.state.touch_limits.min_y =
        input_absinfos[AbsoluteAxisType::ABS_Y.0 as usize].minimum;
    trackpad.definition.state.touch_limits.max_y =
        input_absinfos[AbsoluteAxisType::ABS_Y.0 as usize].maximum;
    trackpad.definition.state.touch_limits.min_p =
        input_absinfos[AbsoluteAxisType::ABS_PRESSURE.0 as usize].minimum;
    trackpad.definition.state.touch_limits.max_p =
        input_absinfos[AbsoluteAxisType::ABS_PRESSURE.0 as usize].maximum;
    trackpad
}

pub trait Notifiable<T> {
    fn notify(&mut self, event: T);
}

impl Notifiable<InputEvent> for TrackpadActor {
    fn notify(&mut self, event: InputEvent) {
        match event.kind() {
            InputEventKind::Synchronization(_) => self.action.act(&self.definition),
            _ => self.definition.notify(event),
        }
    }
}

impl Notifiable<InputEvent> for TrackpadDefinition {
    fn notify(&mut self, event: InputEvent) {
        let value = event.value();
        match event.kind() {
            InputEventKind::Key(button) => match button {
                Key::BTN_TOUCH => self.state.touching = value != 0,
                Key::BTN_LEFT => self.state.left = value != 0,
                Key::BTN_RIGHT => self.state.right = value != 0,
                Key::BTN_MIDDLE => self.state.middle = value != 0,
                _ => (),
            },
            InputEventKind::AbsAxis(axis) => match axis {
                AbsoluteAxisType::ABS_X
                | AbsoluteAxisType::ABS_Y
                | AbsoluteAxisType::ABS_PRESSURE
                | AbsoluteAxisType::ABS_MT_POSITION_X
                | AbsoluteAxisType::ABS_MT_POSITION_Y => {
                    if self.state.slot >= 0 {
                        self.state
                            .touches
                            .entry(self.state.slot)
                            .or_insert_with(TrackedTouchState::default);
                        self.state
                            .touches
                            .get_mut(&self.state.slot)
                            .unwrap()
                            .notify(event);
                    }
                }
                AbsoluteAxisType::ABS_MT_SLOT => {
                    if value < 0 {
                        if self.state.slot >= 0 && self.state.touches.contains_key(&self.state.slot)
                        {
                            self.state.touches.remove(&self.state.slot);
                        }
                    } else {
                        self.state.slot = value;
                    }
                }
                AbsoluteAxisType::ABS_MT_TRACKING_ID => {
                    if value < 0
                        && self.state.slot >= 0
                        && self.state.touches.contains_key(&self.state.slot)
                    {
                        self.state.touches.remove(&self.state.slot);
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}

impl Notifiable<InputEvent> for TrackedTouchState {
    fn notify(&mut self, event: InputEvent) {
        let value = event.value();
        if let InputEventKind::AbsAxis(axis) = event.kind() {
            match axis {
                AbsoluteAxisType::ABS_MT_POSITION_X | AbsoluteAxisType::ABS_X => self.x = value,
                AbsoluteAxisType::ABS_MT_POSITION_Y | AbsoluteAxisType::ABS_Y => self.y = value,
                AbsoluteAxisType::ABS_PRESSURE => self.p = value,
                _ => (),
            }
        }
    }
}
