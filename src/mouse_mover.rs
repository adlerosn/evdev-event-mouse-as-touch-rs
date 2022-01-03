use crate::screen::Display;
use crate::trackpad::TrackedTouchState;
use crate::trackpad::TrackpadAction;
use crate::trackpad::TrackpadDefinition;
use mouse_rs::{types::keys::Keys, Mouse};

pub struct MoveMouse {
    display: Display,
    mouse: Mouse,
}

impl MoveMouse {
    pub fn new() -> Self {
        let d = Display::try_new().expect("Could not open default X11 display");
        let m = Mouse::new();
        Self {
            display: d,
            mouse: m,
        }
    }
}

impl TrackpadAction for MoveMouse {
    fn act(&mut self, defition: &TrackpadDefinition) {
        let (screen_width, screen_height) = self
            .display
            .screen_res()
            .expect("Could not get resolution from display");
        let touch_opt = {
            let mut touches = defition
                .state
                .touches
                .values()
                .filter(|x| {
                    x.x >= defition.state.touch_limits.min_x
                        && x.x <= defition.state.touch_limits.max_x
                        && x.y >= defition.state.touch_limits.min_y
                        && x.y <= defition.state.touch_limits.max_y
                        && x.p >= defition.state.touch_limits.min_p
                        && x.p <= defition.state.touch_limits.max_p
                })
                .collect::<Vec<&TrackedTouchState>>();
            touches.sort_by_key(|x| -x.p);
            touches.first().copied()
        };
        if let Some(touch) = touch_opt {
            let rx = ((touch.x as f64 - defition.state.touch_limits.min_x as f64)
                / (defition.state.touch_limits.max_x - defition.state.touch_limits.min_x) as f64)
                .min(1.0)
                .max(0.0);
            let ry = ((touch.y as f64 - defition.state.touch_limits.min_y as f64)
                / (defition.state.touch_limits.max_y - defition.state.touch_limits.min_y) as f64)
                .min(1.0)
                .max(0.0);
            let ax = (rx * screen_width as f64).round() as i32;
            let ay = (ry * screen_height as f64).round() as i32;
            self.mouse.move_to(ax, ay).expect("Could not move mouse");
        }
        if defition.state.left
            || touch_opt.is_some()
                && touch_opt.unwrap().p
                    >= (0.25
                        * (defition.state.touch_limits.max_p - defition.state.touch_limits.min_p)
                            as f64)
                        .round() as i32
        {
            self.mouse.press(&Keys::LEFT)
        } else {
            self.mouse.release(&Keys::LEFT)
        }
        .expect("Cannot change click status of LEFT button");
        if defition.state.right {
            self.mouse.press(&Keys::RIGHT)
        } else {
            self.mouse.release(&Keys::RIGHT)
        }
        .expect("Cannot change click status of RIGHT button");
        if defition.state.middle {
            self.mouse.press(&Keys::MIDDLE)
        } else {
            self.mouse.release(&Keys::MIDDLE)
        }
        .expect("Cannot change click status of MIDDLE button");
    }
}
