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
    pub width: i32,
    pub height: i32,
}

impl CoordinateBox {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            c1: (x, y),
            c2: (x + width, y),
            c3: (x, y + height),
            c4: (x + width, y + height),
            width,
            height,
        }
    }
}
