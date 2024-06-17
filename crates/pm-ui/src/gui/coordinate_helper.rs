use super::X11Error;
use std::ffi::CString;
extern crate x11;
use x11::xlib;

pub struct GCoordinateHelper;

impl GCoordinateHelper {
    pub async unsafe fn center(display_id: Option<String>) -> Result<(i32, i32), X11Error> {
        let display_id = display_id.unwrap_or(":0".to_string());

        let x_display_id = CString::new(display_id.clone()).unwrap();
        let x_display = xlib::XOpenDisplay(x_display_id.as_ptr());

        if x_display.is_null() {
            return Err(X11Error::CouldNotOpen(display_id));
        }

        let mut x_display = Box::from_raw(x_display);
        let x_screen = xlib::XDefaultScreen(x_display.as_mut());

        let x_screen_width = xlib::XDisplayWidth(x_display.as_mut(), x_screen);
        let x_screen_height = xlib::XDisplayHeight(x_display.as_mut(), x_screen);

        let x = x_screen_width / 2;
        let y = x_screen_height / 2;

        xlib::XCloseDisplay(x_display.as_mut());

        Ok((x, y))
    }
}
