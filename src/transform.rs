use na;

pub type Point = na::Point2<i32>;
pub type Matrix = na::Matrix2<i32>;



#[derive(Debug, Clone)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

pub fn transform(point: &Point, orientation: &Orientation) -> Point {
    rotation_matrix(orientation) * *point
}

fn rotation_matrix(o: &Orientation) -> Matrix {
    match *o {
        Orientation::North => Matrix::new(1, 0, 0, 1),
        Orientation::East => Matrix::new(0, 1, -1, 0),
        Orientation::South => Matrix::new(-1, 0, 0, -1),
        Orientation::West => Matrix::new(0, -1, 1, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_add() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(1, 2);
        let presult = Point::new(2, 3);
        assert_eq!(presult, p1 + p2.to_vector());
    }

    fn test_star() -> Vec<Point> {
        vec![
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(1, 0),
            Point::new(1, -1),
            Point::new(0, -1),
            Point::new(-1, -1),
            Point::new(-1, 0),
            Point::new(-1, 1),
        ]
    }

    #[test]
    /// This is a test that the crate works as expected
    fn point_with_identity() {
        let p = Point::new(1i32,0i32);
        let identity = Matrix::new(1, 0, 0, 1);
        let p2 = identity * p;
        assert_eq!(p2, p);
    }

    #[test]
    fn transform_north() {
        let rotate = Orientation::North;
        let test_points = test_star();
        for i in 0..test_points.len() {
            let p = test_points[i];
            assert_eq!(p, transform(&p, &rotate));
        }
    }

    #[test]
    fn transform_east() {
        let rotate = Orientation::East;
        let test_points = test_star();
        for i in 0..test_points.len() {
            let p = test_points[i];
            let p_east = test_points[(i + 2) % test_points.len()];
            assert_eq!(p_east, transform(&p, &rotate));
        }
    }

    #[test]
    fn transform_south() {
        let rotate = Orientation::South;
        let test_points = test_star();
        for i in 0..test_points.len() {
            let p = test_points[i];
            let p_south = test_points[(i + 4) % test_points.len()];
            assert_eq!(p_south, transform(&p, &rotate));
        }
    }

    #[test]
    fn transform_west() {
        let rotate = Orientation::West;
        let test_points = test_star();
        for i in 0..test_points.len() {
            let p = test_points[i];
            let p_west = test_points[(i + 6) % test_points.len()];
            assert_eq!(p_west, transform(&p, &rotate));
        }
    }
}
