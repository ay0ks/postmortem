/// ```          
///          
///    c1 ─► ┌─────────────┐ ◄─ c2
///          │ ↖           │       
///          │  x,y        │       
///          │             │       
///          │             │       
///          │             │       
///          │             │       
///    c3 ─► └─────────────┘ ◄─ c4
///                                   
/// ```
pub struct CoordinateBox {
    pub x: i32,
    pub y: i32,
    pub c1: (i32, i32),
    pub c2: (i32, i32),
    pub c3: (i32, i32),
    pub c4: (i32, i32),
    pub width: u32,
    pub height: u32,
}

impl CoordinateBox {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            c1: (x, y),
            c2: (x + (width as i32), y),
            c3: (x, y + (height as i32)),
            c4: (x + (width as i32), y + (height as i32)),
            width,
            height,
        }
    }
}
