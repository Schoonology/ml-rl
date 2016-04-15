use super::Point;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn new() -> Rect {
        Rect::create(Point::new(), Point::new())
    }

    pub fn create(min:Point, max:Point) -> Rect {
        Rect { min: min, max: max }
    }

    pub fn from(r:&Rect) -> Rect {
        Rect::create(Point::from(&r.min), Point::from(&r.max))
    }

    pub fn contains(&self, point:&Point) -> bool {
        self.max.x >= point.x && self.max.y >= point.y && self.min.x <= point.x && self.min.y <= point.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Point;
    use std::mem::size_of;

    #[test]
    fn size_test() {
        assert_eq!(size_of::<Rect>(), 16);
    }

    #[test]
    fn new_test() {
        Rect::new();
    }

    #[test]
    fn create_test() {
        let subject = Rect::create(Point::create(1, 2), Point::create(3, 4));

        assert_eq!(subject.min.x, 1);
        assert_eq!(subject.min.y, 2);
        assert_eq!(subject.max.x, 3);
        assert_eq!(subject.max.y, 4);
    }

    #[test]
    fn from_test() {
        let a = Rect::create(Point::create(1, 2), Point::create(3, 4));
        let subject = Rect::from(&a);

        assert_eq!(subject.min.x, 1);
        assert_eq!(subject.min.y, 2);
        assert_eq!(subject.max.x, 3);
        assert_eq!(subject.max.y, 4);
    }

    #[test]
    fn debug_test() {
        let a = Rect::create(Point::create(1, 2), Point::create(3, 4));

        format!("{:?}", a);
    }
}
