use super::block::{self, Block};
use super::transform::{self, Orientation, Point, RotationDirection};
use super::tetriscolor::Color;
use na::Origin;

#[derive(Debug, Clone)]
pub enum TetronimoState {
    Falling,
    Ghost,
    Locking,
    Frozen,
    Nonexistant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shape {
    O,
    T,
    I,
    L,
    J,
    S,
    Z,
}

#[derive(Debug, Clone)]
pub enum SlideDirection {
    Left,
    Right,
}



#[derive(Debug, Clone)]
pub struct Tetromino {
    shape: Shape,
    origin: Point,
    orientation: Orientation,
    pub state: TetronimoState,
}

impl Tetromino {
    /// Creates a dummy tetronimo
    pub fn new() -> Self {
        Tetromino {
            shape: Shape::O,
            origin: Point::origin(),
            orientation: Orientation::North,
            state: TetronimoState::Nonexistant,
        }
    }

    /// Creates a tetronimo with a desired shape
    pub fn new_shape(shape: Shape) -> Self {
        Tetromino {
            shape: shape,
            origin: Point::origin(),
            orientation: Orientation::North,
            state: TetronimoState::Nonexistant,
        }
    }

    pub fn put(&mut self, p: Point) {
        self.origin = p;
    }

    /// Color of the tetronimo
    pub fn color(&self) -> Color {

        let alpha: f32 = match self.state {
            TetronimoState::Falling => 1f32,
            TetronimoState::Ghost => 0.2f32,
            TetronimoState::Locking => 1f32,
            TetronimoState::Frozen => 1f32,
            TetronimoState::Nonexistant => 0f32,
        };

        match self.shape {
            Shape::O => {
                Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
                    alpha: alpha,
                }
            }
            Shape::T => {
                Color {
                    red: 0.7,
                    green: 0.7,
                    blue: 0.0,
                    alpha: alpha,
                }
            }
            Shape::I => {
                Color {
                    red: 0.0,
                    green: 0.7,
                    blue: 0.7,
                    alpha: alpha,
                }
            }
            Shape::L => {
                Color {
                    red: 0.5,
                    green: 0.7,
                    blue: 0.7,
                    alpha: alpha,
                }
            }
            Shape::J => {
                Color {
                    red: 0.5,
                    green: 0.0,
                    blue: 0.7,
                    alpha: alpha,
                }
            }
            Shape::S => {
                Color {
                    red: 0.5,
                    green: 0.3,
                    blue: 0.0,
                    alpha: alpha,
                }
            }
            Shape::Z => {
                Color {
                    red: 0.0,
                    green: 0.3,
                    blue: 0.5,
                    alpha: alpha,
                }
            }
        }
    }

    pub fn slide(&mut self, direction: SlideDirection) {
        match direction {
            SlideDirection::Left => self.origin.x -= 1,
            SlideDirection::Right => self.origin.x += 1,
        }
    }

    pub fn translate(&mut self, offset: &Point) {
        self.origin += offset.to_vector();
    }

    pub fn rotate(&mut self, rotation: &RotationDirection) {
        self.orientation = transform::rotate(&self.orientation, rotation);
    }

    pub fn wall_kick_options(&self, direction: &RotationDirection) -> Vec<Point> {
        wall_kicks(&self.shape, &self.orientation, direction)
    }

    pub fn move_down(&mut self) {
        self.origin.y -= 1;
    }

    pub fn blocks(&self) -> Vec<Block> {
        let color = self.color();
        tetronimo_points(self.shape)
            .iter()
            .map(|p| {
                let tetronimo_space = transform::transform(&p, &self.orientation);
                let block_space = block::into_block(&tetronimo_space);
                let final_point: Point = self.origin + block_space.to_vector();
                Block::new(color, final_point)
            })
            .collect()
    }
}

// All the Following information is from the SRS rotation model.
// Reference https://tetris.wiki/SRS

fn wall_kicks(shape: &Shape,
              orientation: &Orientation,
              direction: &RotationDirection)
              -> Vec<Point> {

    match *shape {
        Shape::I => {
            match (orientation, direction) {
                (&Orientation::North, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(-2, 0),
                         Point::new(1, -2),
                         Point::new(-2, 1)]
                }
                (&Orientation::North, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(2, 0),
                         Point::new(-1, 0),
                         Point::new(2, 1),
                         Point::new(-1, -2)]
                }
                (&Orientation::East, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-2, 0),
                         Point::new(1, 0),
                         Point::new(-2, -1),
                         Point::new(1, 2)]
                }
                (&Orientation::East, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(-2, 0),
                         Point::new(1, -2),
                         Point::new(-2, 1)]
                }
                (&Orientation::South, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(2, 0),
                         Point::new(-1, 2),
                         Point::new(2, -1)]
                }
                (&Orientation::South, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-2, 0),
                         Point::new(1, 0),
                         Point::new(-2, -1),
                         Point::new(1, 2)]
                }
                (&Orientation::West, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(2, 0),
                         Point::new(-1, 0),
                         Point::new(2, 1),
                         Point::new(-1, -2)]
                }
                (&Orientation::West, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(2, 0),
                         Point::new(-1, 2),
                         Point::new(2, -1)]
                }
            }
        }
        Shape::O => vec![Point::new(0, 0)],
        _ => {
            match (orientation, direction) {
                (&Orientation::North, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(-1, -1),
                         Point::new(0, 2),
                         Point::new(-1, 2)]
                }
                (&Orientation::North, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(1, -1),
                         Point::new(0, 2),
                         Point::new(1, 2)]
                }
                (&Orientation::East, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(-1, 1),
                         Point::new(0, -2),
                         Point::new(-1, -2)]
                }
                (&Orientation::East, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(-1, 1),
                         Point::new(0, -2),
                         Point::new(-1, -2)]
                }
                (&Orientation::South, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(1, -1),
                         Point::new(0, 2),
                         Point::new(1, 2)]
                }
                (&Orientation::South, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(-1, -1),
                         Point::new(0, 2),
                         Point::new(-1, 2)]
                }
                (&Orientation::West, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(1, 1),
                         Point::new(0, -2),
                         Point::new(1, -2)]
                }
                (&Orientation::West, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(1, 1),
                         Point::new(0, -2),
                         Point::new(1, -2)]
                }
            }
        }
    }
}

fn tetronimo_points(shape: Shape) -> Vec<Point> {
    match shape {
        Shape::O => {
            vec![
                Point::new(-1,1),
                Point::new(-1,-1),
                Point::new(1,1),
                Point::new(1,-1),
            ]
        }
        Shape::I => {
            vec![
                Point::new(-3,1),
                Point::new(-1,1),
                Point::new(1,1),
                Point::new(3,1),
            ]
        }
        Shape::T => {
            vec![
                Point::new(0,0),
                Point::new(-2, 0),
                Point::new(2, 0),
                Point::new(0, 2),
            ]
        }
        Shape::L => {
            vec![
                Point::new(0, 0),
                Point::new(-2, 0),
                Point::new(2, 0),
                Point::new(2, 2),
            ]
        }
        Shape::J => {
            vec![
                Point::new(0, 0),
                Point::new(-2, 0),
                Point::new(2, 0),
                Point::new(-2, 2),
            ]
        }
        Shape::S => {
            vec![
                Point::new(0, 0),
                Point::new(-2, 0),
                Point::new(0, 2),
                Point::new(2, 2),
            ]
        }
        Shape::Z => {
            vec![
                Point::new(0, 2),
                Point::new(-2, 2),
                Point::new(0, 0),
                Point::new(2, 0),
            ]
        }
    }
}
