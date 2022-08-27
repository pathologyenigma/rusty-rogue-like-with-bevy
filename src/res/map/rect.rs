#[derive(Clone)]
pub struct Rect {
    pub x: (i32, i32),
    pub y: (i32, i32),
}

impl Rect {
    pub fn new(x:i32, y: i32, w:i32, h:i32) -> Rect {
        Rect{
            x: (x, x+w),
            y: (y, y+h),
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other:&Rect) -> bool {
        self.x.0 <= other.x.1 && self.x.1 >= other.x.0 && self.y.0 <= other.y.1 && self.y.1 >= other.y.0
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x.0 + self.x.1)/2, (self.y.0 + self.y.1)/2)
    }
}