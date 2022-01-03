use std::ptr;
use x11::xlib;

pub struct Display {
    display: *mut xlib::Display,
    screen: *mut xlib::Screen,
    // window: xlib::Window,
}

impl Display {
    pub fn try_new() -> Option<Self> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                return None;
            }
            let screen = xlib::XDefaultScreenOfDisplay(display);
            // let root = xlib::XRootWindowOfScreen(screen);
            Some(Display {
                display,
                screen,
                // window: root,
            })
        }
    }
    pub fn screen_res(&mut self) -> Option<(i32, i32)> {
        let screen: &mut xlib::Screen = &mut unsafe { *self.screen };
        let res = (screen.width, screen.height);
        if res.0 < 0 || res.1 < 0 {
            None
        } else {
            Some(res)
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
    }
}
