use std::ops;
use super::Point;

pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn new() {
    }

    pub fn contains(&self, point:&Point) -> bool {
        self.max.x >= point.x && self.max.y >= point.y && self.min.x <= point.x && self.min.y <= point.y
    }
}
