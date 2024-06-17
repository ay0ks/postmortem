use crate::{traits::Draw, CoordinateBox};

pub struct Object<T> {
    pub rect: CoordinateBox,
    pub state: T,
    pub children: Vec<Box<dyn Draw>>,
}

impl<T> Object<T>
where
    T: Default,
{
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Object {
            rect: CoordinateBox::new(x, y, width, height),
            state: T::default(),
            children: Vec::new(),
        }
    }

    pub fn new_with_state(x: i32, y: i32, width: u32, height: u32, state: T) -> Self {
        Object {
            rect: CoordinateBox::new(x, y, width, height),
            state: state,
            children: Vec::new(),
        }
    }
}
