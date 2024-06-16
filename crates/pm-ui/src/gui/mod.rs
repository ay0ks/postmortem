use crate::{traits::Draw, Object};
use std::{ffi::CString, mem, ptr};
extern crate x11;
use x11::xlib::{
    CWBackPixel, CWBorderPixel, CWColormap, CWCursor, Display, InputOutput, Screen, Visual, Window,
    XCreateWindow, XDefaultRootWindow, XDefaultScreenOfDisplay, XMapWindow, XOpenDisplay,
    XSetWindowAttributes, XUnmapWindow,
};

#[derive(Clone, Debug, Default)]
pub struct GCanvasState {
    pub screen_width: i32,
    pub screen_height: i32,
}

pub struct GCanvas {
    x_display_id: (String, CString),
    x_display: *mut Display,
    x_screen: *mut Screen,
    x_screen_root: Window,
    x_window: Window,
    x_visual: *mut Visual,
    root: Object<GCanvasState>,
}

impl GCanvas {
    pub async unsafe fn new(display_id: Option<String>) -> Self {
        let display_id = display_id.unwrap_or(":0".to_string());
        let x_display_id = CString::new(display_id.clone()).unwrap();
        let x_display = XOpenDisplay(x_display_id.as_ptr());
        let x_screen = XDefaultScreenOfDisplay(x_display.cast());
        let x_screen_root = (*x_screen).root;

        let state = GCanvasState {
            screen_width: (*x_screen).width,
            screen_height: (*x_screen).height,
        };

        let x_window_value_mask = CWBackPixel | CWBorderPixel | CWColormap | CWCursor;
        let mut x_window_attributes = Box::new(XSetWindowAttributes {
            background_pixel: 0,
            border_pixel: 0,
            colormap: 0,
            cursor: 0,
            ..mem::zeroed()
        });
        let x_window = XCreateWindow(
            x_display,
            x_screen_root,
            0,
            0,
            state.screen_width as u32,
            state.screen_height as u32,
            0,
            (*x_screen).root_depth,
            InputOutput as u32,
            (*x_screen).root_visual,
            x_window_value_mask,
            x_window_attributes.as_mut(),
        );

        GCanvas {
            x_display_id: (display_id, x_display_id),
            x_display,
            x_screen,
            x_screen_root,
            root: Object::new_with_state(0, 0, state.screen_width, state.screen_height, state),
            x_window,
        }
    }

    pub async unsafe fn show(&self) {
        XMapWindow(self.x_display, self.x_window);
    }

    pub async unsafe fn hide(&self) {
        XUnmapWindow(self.x_display, self.x_window);
    }
}

impl Draw for GCanvas {
    fn draw(&self) {
        println!("Drawing canvas");
    }
}
