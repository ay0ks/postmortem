use std::{
    ffi::{c_int, c_void, CString},
    mem,
    process::exit,
    ptr,
    rc::Rc,
    thread,
};

use crate::{traits::Draw, Object};
mod errors;
use errors::X11Error;
extern crate x11;
use x11::{glx, xlib};
extern crate gleam;
use gleam::gl::{self, types::*};
mod coordinate_helper;
pub use coordinate_helper::GCoordinateHelper;

#[derive(Clone, Debug, Default)]
pub struct GCanvasState {
    pub screen_width: u32,
    pub screen_height: u32,
    pub wm_protocols: Option<xlib::Atom>,
    pub wm_delete_window: Option<xlib::Atom>,
}

pub struct GCanvas {
    x_display_id: (String, CString),
    x_display: Box<xlib::Display>,
    x_screen: i32,
    x_screen_root: xlib::Window,
    x_window: xlib::Window,
    x_visual: Box<xlib::XVisualInfo>,
    x_display_white: u64,
    x_display_black: u64,
    x_gc: Box<xlib::GC>,
    x_gl_context: Box<glx::GLXContext>,
    root: Object<GCanvasState>,
    gl: Box<gl::GlFfi>,
}

impl GCanvas {
    pub async unsafe fn new(
        display_id: Option<String>,
        x: i32,
        y: i32,
        height: u32,
        width: u32,
    ) -> Result<Self, X11Error> {
        let display_id = display_id.unwrap_or(":0".to_string());

        let x_display_id = CString::new(display_id.clone()).unwrap();
        let x_display = xlib::XOpenDisplay(x_display_id.as_ptr());

        if x_display.is_null() {
            return Err(X11Error::CouldNotOpen(display_id));
        }

        let mut x_display = Box::from_raw(x_display);

        let x_screen = xlib::XDefaultScreen(x_display.as_mut());
        let x_screen_root = xlib::XRootWindow(x_display.as_mut(), x_screen);
        let x_screen_root_depth = xlib::XDefaultDepth(x_display.as_mut(), x_screen);

        let mut x_visual_info = Box::<xlib::XVisualInfo>::new_zeroed();
        xlib::XMatchVisualInfo(
            x_display.as_mut(),
            x_screen,
            x_screen_root_depth,
            xlib::TrueColor as i32,
            x_visual_info.as_mut_ptr(),
        );
        let mut x_visual_info = x_visual_info.assume_init();

        let state = GCanvasState {
            screen_width: xlib::XDisplayWidth(x_display.as_mut(), x_screen) as u32,
            screen_height: xlib::XDisplayHeight(x_display.as_mut(), x_screen) as u32,
            ..Default::default()
        };

        let x_display_white = xlib::XWhitePixel(x_display.as_mut(), x_screen);
        let x_display_black = xlib::XBlackPixel(x_display.as_mut(), x_screen);

        let x_window_value_mask = xlib::CWBackPixel | xlib::CWEventMask;
        let x_window_event_mask = xlib::KeyPressMask
            | xlib::KeyReleaseMask
            | xlib::ButtonPressMask
            | xlib::ButtonReleaseMask
            | xlib::EnterWindowMask
            | xlib::LeaveWindowMask
            | xlib::PointerMotionMask
            | xlib::Button1MotionMask
            | xlib::Button2MotionMask
            | xlib::Button3MotionMask
            | xlib::Button4MotionMask
            | xlib::Button5MotionMask
            | xlib::ButtonMotionMask
            | xlib::KeymapStateMask
            | xlib::ExposureMask
            | xlib::VisibilityChangeMask
            | xlib::StructureNotifyMask
            | xlib::ResizeRedirectMask
            | xlib::SubstructureNotifyMask
            | xlib::SubstructureRedirectMask
            | xlib::FocusChangeMask
            | xlib::PropertyChangeMask
            | xlib::ColormapChangeMask
            | xlib::OwnerGrabButtonMask;

        let mut x_window_attributes = Box::new(xlib::XSetWindowAttributes {
            background_pixel: x_display_black,
            event_mask: x_window_event_mask,
            ..mem::zeroed()
        });

        println!(
            "depth: {:?} -> {:?}, visual: {:#?}",
            x_screen_root_depth, x_visual_info.depth, x_visual_info.visual
        );

        let x_window = xlib::XCreateWindow(
            x_display.as_mut(),
            x_screen_root,
            x,
            y,
            width,
            height,
            0,
            x_visual_info.depth,
            xlib::InputOutput as u32,
            ptr::null_mut(),
            x_window_value_mask,
            x_window_attributes.as_mut(),
        );

        if x_window == 0 {
            return Err(X11Error::CouldNotCreateWindow);
        }

        let x_gc_value_mask = 0;
        let mut x_gc_values = Box::new(xlib::XGCValues { ..mem::zeroed() });

        let x_gc = Box::new(xlib::XCreateGC(
            x_display.as_mut(),
            x_window,
            x_gc_value_mask,
            x_gc_values.as_mut(),
        ));

        if x_gc.is_null() {
            return Err(X11Error::CouldNotCreateGC);
        }

        let x_gl_context = Box::new(glx::glXCreateContext(
            x_display.as_mut(),
            x_visual_info.as_mut(),
            ptr::null_mut(),
            xlib::True as i32,
        ));

        if x_gl_context.is_null() {
            return Err(X11Error::CouldNotCreateGLContext);
        }

        let gl = Box::new(gl::GlFfi::load_with(|s| {
            let c_str = CString::new(s).unwrap();
            glx::glXGetProcAddress(c_str.as_ptr() as *const u8).unwrap() as *const c_void
        }));

        Ok(GCanvas {
            x_display_id: (display_id, x_display_id),
            x_display,
            x_screen,
            x_screen_root,
            x_window,
            x_visual: x_visual_info,
            x_display_white,
            x_display_black,
            x_gc,
            x_gl_context,
            root: Object::new_with_state(0, 0, state.screen_width, state.screen_height, state),
            gl,
        })
    }

    pub async unsafe fn set_title(&mut self, title: &str) {
        let title_str = CString::new(title).unwrap();

        xlib::XStoreName(self.x_display.as_mut(), self.x_window, title_str.as_ptr());
        self.flush().await;
    }

    pub async unsafe fn open(&mut self) {
        xlib::XSetBackground(self.x_display.as_mut(), *self.x_gc, self.x_display_black);
        xlib::XSetForeground(self.x_display.as_mut(), *self.x_gc, self.x_display_white);
        self.flush().await;
        self.show().await;

        // Hook close requests.
        let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
        let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

        self.root.state.wm_protocols = Some(xlib::XInternAtom(
            self.x_display.as_mut(),
            wm_protocols_str.as_ptr(),
            false.into(),
        ));
        self.root.state.wm_delete_window = Some(xlib::XInternAtom(
            self.x_display.as_mut(),
            wm_delete_window_str.as_ptr(),
            false.into(),
        ));

        let mut protocols = [self.root.state.wm_delete_window.unwrap()];
        xlib::XSetWMProtocols(
            self.x_display.as_mut(),
            self.x_window,
            protocols.as_mut_ptr(),
            protocols.len() as c_int,
        );
    }

    pub async unsafe fn run(&mut self) {
        let mut event = Box::<xlib::XEvent>::new_zeroed().assume_init();

        loop {
            xlib::XNextEvent(self.x_display.as_mut(), event.as_mut() as *mut xlib::XEvent);

            println!(
                "event: {:?}, {:?}, {:?}",
                event.get_type(),
                event.any.display,
                event.any.window
            );

            match event.get_type() {
                xlib::ClientMessage => {
                    let client = xlib::XClientMessageEvent::from(event.as_ref());

                    if client.message_type == self.root.state.wm_protocols.unwrap()
                        && client.format == 32
                    {
                        let protocol = client.data.get_long(0) as xlib::Atom;

                        if protocol == self.root.state.wm_delete_window.unwrap() {
                            exit(0);
                        }
                    }
                }

                _ => (),
            }
        }
    }

    pub async unsafe fn flush(&mut self) {
        xlib::XFlush(self.x_display.as_mut());
    }

    pub async unsafe fn show(&mut self) {
        xlib::XMapWindow(self.x_display.as_mut(), self.x_window);
        self.flush().await;
    }

    pub async unsafe fn hide(&mut self) {
        xlib::XUnmapWindow(self.x_display.as_mut(), self.x_window);
        self.flush().await;
    }

    pub async unsafe fn close(&mut self) {
        self.hide().await;

        xlib::XCloseDisplay(self.x_display.as_mut());
    }

    pub async unsafe fn add_text(&mut self, text: &str, x: i32, y: i32) {
        let text_str = CString::new(text).unwrap();

        // //draw using opengl
        self.gl.ClearColor(1.0, 0.0, 0.0, 1.0);
        self.gl.Clear(gl::COLOR_BUFFER_BIT);
        self.gl.Flush();

        //glx::glXSwapBuffers(self.x_display.as_mut(), self.x_window);

        // draw using xlib
        xlib::XDrawString(
            self.x_display.as_mut(),
            self.x_window,
            *self.x_gc,
            x,
            y,
            text_str.as_ptr(),
            text.len() as i32,
        );
        self.flush().await;
    }
}

impl Draw for GCanvas {
    fn draw(&self) {
        println!("Drawing canvas");
    }
}
