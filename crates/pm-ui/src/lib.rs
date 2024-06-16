#![feature(new_uninit)]

#[cfg(feature = "gui")]
mod gui;

#[cfg(feature = "tui")]
mod tui;

#[cfg(feature = "gui")]
pub use gui::*;
#[cfg(feature = "tui")]
pub use tui::*;

mod coordinate_box;
pub use coordinate_box::CoordinateBox;
mod object;
pub mod traits;
pub use object::Object;
