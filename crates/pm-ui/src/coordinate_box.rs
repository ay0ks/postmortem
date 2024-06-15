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
    pub c: (i32, i32, i32, i32),
    pub width: i32,
    pub height: i32,
}
