use crate::{traits::Draw, Object};
use x11::xlib::{Display, Screen, Window, XCreateWindow, XDefaultScreenOfDisplay, XOpenDisplay};

#[derive(Clone, Debug, Default)]
pub struct GCanvasState {
    pub screen_width: i32,
    pub screen_height: i32,
}

pub struct GCanvas {
    x_display: *mut Display,
    x_screen: *mut Screen,
    x_window: *mut Window,
    root: Object<GCanvasState>,
}

macro_rules! c_string {
    ($s:expr) => {
        ($s.as_ptr() as *const i8)
    };
}

impl GCanvas {
    pub unsafe fn new() -> Self {
        let x_display = XOpenDisplay(c_string!(":0"));
        let x_screen = XDefaultScreenOfDisplay(x_display.cast());

        let state = GCanvasState {
            screen_width: (*x_screen).width,
            screen_height: (*x_screen).height,
        };

        // TODO(ay0ks):
        // let x_window = XCreateWindow(12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1)

        GCanvas {
            x_display,
            x_screen,
            root: Object::new_with_state(0, 0, state.screen_width, state.screen_height, state),
            x_window: std::ptr::null_mut(), // TODO(ay0ks)
        }
    }
}

impl Draw for GCanvas {
    fn draw(&self) {
        println!("Drawing canvas");
    }
}
