
use std::ops::Add;

use std::num::Zero;

#[derive(Debug, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}


impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x: x, y: y }
    }

    pub fn origin() -> Self
        where T: Zero
    {
        Point {
            x: T::zero(),
            y: T::zero(),
        }
    }
}
impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, _rhs: Point<T>) -> Point<T> {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}


#[test]
fn add_i32() {
    let p = Point::new(1i32, 2i32);
    let r = Point::new(3i32, 4i32);
    let s = p + r;
    assert_eq!(s.x, 4i32);
    assert_eq!(s.y, 6i32);
}

#[test]
fn add_usize() {
    let p = Point::new(1usize, 2usize);
    let r = Point::new(3usize, 4usize);
    let s = p + r;
    assert_eq!(s.x, 4usize);
    assert_eq!(s.y, 6usize);
}

#[test]
fn point_i32_origin() {
    let p: Point<i32> = Point::origin();
    assert_eq!(p.x, 0i32);
    assert_eq!(p.y, 0i32);
}

#[test]
fn point_usize_origin() {
    let p: Point<usize> = Point::origin();
    assert_eq!(p.x, 0usize);
    assert_eq!(p.y, 0usize);
}
