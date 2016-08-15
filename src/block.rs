use super::transform::Point;
use super::tetriscolor::Color;


#[derive(Debug, Clone)]
pub struct Block {
    pub color: Color,
    pub point: Point,
}

impl Block {
    pub fn new(c: Color, p: Point) -> Self {
        Block {
            color: c,
            point: p,
        }
    }
}


pub fn into_block(point: &Point) -> Point {
    Point::new(transform_int(point.x), transform_int(point.y))
}

fn transform_int(x: i32) -> i32 {
    x >> 1
}

#[test]
fn zero_to_zero() {
    assert_eq!(0i32, transform_int(0i32));
}

#[test]
fn two_to_one() {
    assert_eq!(1i32, transform_int(2i32));
}
#[test]
fn negative_two_to_one() {
    assert_eq!(-1i32, transform_int(-2i32));
}

#[test]
fn test_range() {
    let xrange = vec![4, 3, 2, 1, 0, -1, -2, -3, -4];
    let result = vec![2, 1, 1, 0, 0, -1, -1, -2, -2];

    for i in 0..xrange.len() {
        assert_eq!(result[i], transform_int(xrange[i]))
    }
}
