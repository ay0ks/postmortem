use crate::{traits::Draw, CoordinateBox};

pub struct Object<T> {
    pub rect: CoordinateBox,
    pub state: Option<T>,
    pub children: Vec<Box<dyn Draw>>,
}

impl<T> Object<T>
where
    T: Default,
{
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Object {
            rect: CoordinateBox::new(x, y, width, height),
            state: None,
            children: Vec::new(),
        }
    }

    pub fn new_with_state(x: i32, y: i32, width: i32, height: i32, state: T) -> Self {
        Object {
            rect: CoordinateBox::new(x, y, width, height),
            state: Some(state),
            children: Vec::new(),
        }
    }

    pub fn new_with_default_state(x: i32, y: i32, width: i32, height: i32) -> Self {
        Object::<T>::new_with_state(x, y, width, height, T::default())
    }
}
