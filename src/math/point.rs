use std::ops;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new() -> Point {
        Point::create(0, 0)
    }

    pub fn create(x:u32, y:u32) -> Point {
        Point { x: x, y: y }
    }

    pub fn from(p:&Point) -> Point {
        Point::create(p.x, p.y)
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs:Point) -> Point {
        Point { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, _rhs:Point) -> Point {
        Point { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    use std::hash::{Hash, SipHasher, Hasher};

    #[test]
    fn size_test() {
        assert_eq!(size_of::<Point>(), 8);
    }

    #[test]
    fn new_test() {
        let subject = Point::new();
    }

    #[test]
    fn create_test() {
        let subject = Point::create(1, 7);

        assert_eq!(subject.x, 1);
        assert_eq!(subject.y, 7);
    }

    #[test]
    fn from_test() {
        let a = Point::create(1, 7);
        let subject = Point::from(&a);

        assert_eq!(subject.x, 1);
        assert_eq!(subject.y, 7);
    }

    #[test]
    fn add_test() {
        let a = Point { x: 2, y: 3 };
        let b = Point { x: 4, y: 7 };

        let subject = a + b;

        assert_eq!(subject.x, 6);
        assert_eq!(subject.y, 10);
    }

    #[test]
    fn subtract_test() {
        let a = Point { x: 2, y: 3 };
        let b = Point { x: 4, y: 7 };

        let subject = b - a;

        assert_eq!(subject.x, 2);
        assert_eq!(subject.y, 4);
    }

    #[test]
    fn debug_test() {
        let a = Point { x: 5, y: 7 };

        let subject = format!("{:?}", a);
    }

    #[test]
    fn hash_test() {
        fn hash<T: Hash>(t: &T) -> u64 {
            let mut s = SipHasher::new();
            t.hash(&mut s);
            s.finish()
        }

        let a = Point { x: 2, y: 3 };
        let b = Point { x: 2, y: 3 };
        let c = Point { x: 5, y: 7 };

        assert_eq!(hash(&a), hash(&b));
        assert!(hash(&a) != hash(&c));
    }

    #[test]
    fn equals_test() {
        let a = Point { x: 2, y: 3 };
        let b = Point { x: 2, y: 3 };
        let c = Point { x: 5, y: 7 };

        assert!(a == b);
        assert!(a != c);
        assert_eq!(a, b);
    }
}
