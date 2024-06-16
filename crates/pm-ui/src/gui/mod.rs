use std::{
    ffi::{c_int, CString},
    mem::{self, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr,
};

use thiserror::Error;

use crate::{traits::Draw, Object};
extern crate x11;
use x11::xlib::{
    Atom, CWBackPixel, CWBorderPixel, CWColormap, CWCursor, ClientMessage, Display,
    False as XFalse, InputOutput, True as XTrue, TrueColor, Visual, Window, XBlackPixel,
    XClientMessageEvent, XCloseDisplay, XCreateWindow, XDefaultDepth, XDefaultScreen,
    XDisplayHeight, XDisplayWidth, XEvent, XInternAtom, XMapWindow, XMatchVisualInfo, XNextEvent,
    XOpenDisplay, XRootWindow, XSetWMProtocols, XSetWindowAttributes, XStoreName, XUnmapWindow,
    XVisualInfo,
};

#[derive(Clone, Debug, Default)]
pub struct GCanvasState {
    pub screen_width: i32,
    pub screen_height: i32,
}

pub struct GCanvas {
    x_display_id: (String, CString),
    x_display: Box<Display>,
    x_screen: i32,
    x_screen_root: Window,
    x_window: Window,
    x_visual: Box<XVisualInfo>,
    root: Object<GCanvasState>,
}

#[derive(Error, Debug)]
pub enum CanvasError {
    #[error("could not open X display {0}")]
    X11CouldNotOpen(String),
}

impl GCanvas {
    pub async unsafe fn new(display_id: Option<String>) -> Result<Self, CanvasError> {
        let display_id = display_id.unwrap_or(":0".to_string());

        let x_display_id = CString::new(display_id.clone()).unwrap();
        let x_display = XOpenDisplay(x_display_id.as_ptr());

        if x_display.is_null() {
            return Err(CanvasError::X11CouldNotOpen(display_id));
        }

        let mut x_display = Box::from_raw(x_display);

        let x_screen = XDefaultScreen(x_display.as_mut());
        let x_screen_root = XRootWindow(x_display.as_mut(), x_screen);
        let x_screen_root_depth = XDefaultDepth(x_display.as_mut(), x_screen);

        let mut x_visual_info = Box::<XVisualInfo>::new_zeroed();
        XMatchVisualInfo(
            x_display.as_mut(),
            x_screen,
            x_screen_root_depth,
            TrueColor as i32,
            x_visual_info.as_mut_ptr(),
        );
        let x_visual_info = x_visual_info.assume_init();

        let state = GCanvasState {
            screen_width: XDisplayWidth(x_display.as_mut(), x_screen),
            screen_height: XDisplayHeight(x_display.as_mut(), x_screen),
        };

        let x_window_value_mask = CWBackPixel;
        let mut x_window_attributes = Box::new(XSetWindowAttributes {
            background_pixel: XBlackPixel(x_display.as_mut(), x_screen),
            ..mem::zeroed()
        });

        println!(
            "depth: {:?} -> {:?}, visual: {:#?}",
            x_screen_root_depth, x_visual_info.depth, x_visual_info.visual
        );

        let x_window = XCreateWindow(
            x_display.as_mut(),
            x_screen_root,
            0,
            0,
            state.screen_width as u32,
            state.screen_height as u32,
            0,
            x_visual_info.depth,
            InputOutput as u32,
            ptr::null_mut(),
            x_window_value_mask,
            x_window_attributes.as_mut(),
        );

        Ok(GCanvas {
            x_display_id: (display_id, x_display_id),
            x_display,
            x_screen,
            x_screen_root,
            x_visual: x_visual_info,
            root: Object::new_with_state(0, 0, state.screen_width, state.screen_height, state),
            x_window,
        })
    }

    pub async unsafe fn set_title(&mut self, title: &str) {
        let title_str = CString::new(title).unwrap();

        XStoreName(self.x_display.as_mut(), self.x_window, title_str.as_ptr());
    }

    pub async unsafe fn open(&mut self) {
        self.show().await;

        // Hook close requests.
        let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
        let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

        let wm_protocols = XInternAtom(self.x_display.as_mut(), wm_protocols_str.as_ptr(), XFalse);
        let wm_delete_window = XInternAtom(
            self.x_display.as_mut(),
            wm_delete_window_str.as_ptr(),
            XFalse,
        );

        let mut protocols = [wm_delete_window];

        XSetWMProtocols(
            self.x_display.as_mut(),
            self.x_window,
            protocols.as_mut_ptr(),
            protocols.len() as c_int,
        );

        // Main loop.
        let mut event = Box::<XEvent>::new_zeroed().assume_init();

        tokio::task::unconstrained(async move {
            loop {
                XNextEvent(self.x_display.as_mut(), event.as_mut() as *mut XEvent);

                match event.get_type() {
                    #[allow(non_upper_case_globals)]
                    ClientMessage => {
                        let client = XClientMessageEvent::from(event.as_ref());

                        if client.message_type == wm_protocols && client.format == 32 {
                            let protocol = client.data.get_long(0) as Atom;

                            if protocol == wm_delete_window {
                                break;
                            }
                        }
                    },

                    _ => (),
                }
            }
        })
        .await;
    }

    pub async unsafe fn show(&mut self) {
        XMapWindow(self.x_display.as_mut(), self.x_window);
    }

    pub async unsafe fn hide(&mut self) {
        XUnmapWindow(self.x_display.as_mut(), self.x_window);
    }

    pub async unsafe fn close(&mut self) {
        self.hide().await;
        XCloseDisplay(self.x_display.as_mut());
    }
}

impl Draw for GCanvas {
    fn draw(&self) {
        println!("Drawing canvas");
    }
}
