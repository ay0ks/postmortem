use crate::traits::Draw;

pub struct Object<T> {
    pub state: Option<T>,
    pub children: Vec<Box<dyn Draw>>,
}
