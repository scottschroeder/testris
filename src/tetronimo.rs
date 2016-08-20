use super::block::{self, Block};
use super::transform::{self, Orientation, Point};
use super::tetriscolor::Color;
use na::Origin;

#[derive(Debug, Clone)]
enum TetronimoState {
    Falling,
    Ghost,
    Locking,
    Frozen,
    Nonexistant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Shape {
    O,
    T,
    I,
}

#[derive(Debug, Clone)]
enum SlideDirection {
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum RotationDirection {
    Clockwise,
    CounterClockwise,
}


#[derive(Debug, Clone)]
struct Tetromino {
    shape: Shape,
    origin: Point,
    orientation: Orientation,
    state: TetronimoState,
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
        }
    }

    pub fn slide(&mut self, direction: SlideDirection) {
        match direction {
            SlideDirection::Left => self.origin.x -= 1,
            SlideDirection::Right => self.origin.x += 1,
        }
    }

    pub fn rotate(&mut self, rotation: RotationDirection) {
        self.orientation = match rotation {
            RotationDirection::Clockwise => {
                match self.orientation {
                    Orientation::North => Orientation::East,
                    Orientation::East => Orientation::South,
                    Orientation::South => Orientation::West,
                    Orientation::West => Orientation::North,
                }
            }
            RotationDirection::CounterClockwise => {
                match self.orientation {
                    Orientation::North => Orientation::West,
                    Orientation::East => Orientation::North,
                    Orientation::South => Orientation::East,
                    Orientation::West => Orientation::South,
                }
            }
        }
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


fn tetronimo_points(shape: Shape) -> Vec<Point> {
    match shape {
        Shape::O => {
            vec![
                Point::new(-1,1),
                Point::new(-1,-1),
                Point::new(1,1),
                Point::new(1,-1),
            ]
        },
        Shape::I => {
            vec![
                Point::new(-1,3),
                Point::new(-1,1),
                Point::new(-1,-1),
                Point::new(-1,-3),
            ]
        },
        Shape::T => {
            vec![
                Point::new(0,0),
                Point::new(-2, 0),
                Point::new(2, 0),
                Point::new(0, 2),
            ]
        },
    }
}
