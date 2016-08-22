use super::tetriscolor::Color;
use super::transform::Point;
use super::block::Block;
use super::tetronimo::Tetromino;
use super::error;
use super::Result;

use na;

pub type Pixel = na::Point2<f64>;

#[derive(Debug, PartialEq)]
enum RowPopulation {
    Empty,
    Full,
    Mixed,
}

#[derive(Debug)]
pub struct GameBoard {
    size_x: usize,
    size_y: usize,
    size_hidden: usize,
    blocks: Vec<Option<Color>>,
    pub point: Pixel,
}

impl GameBoard {
    pub fn new(x: usize, y: usize, h: usize, p: Pixel) -> Self {
        GameBoard {
            size_x: x,
            size_y: y,
            blocks: vec![None; x * y],
            point: p,
            size_hidden: h,
        }
    }

    pub fn height(&self) -> usize {
        self.size_y - self.size_hidden
    }

    fn copy_row(&mut self, lower: usize, upper: usize) {
        for i in 0..self.size_x {
            self.blocks[lower * self.size_x + i] = if upper < self.size_y {
                self.blocks[upper * self.size_x + i]
            } else {
                None
            };
        }
    }

    fn row_status(&self, row: usize) -> RowPopulation {
        let mut row_pop: usize = 0;
        for i in 0..self.size_x {
            if self.blocks[row * self.size_x + i].is_some() {
                row_pop += 1;
            }
        }

        if row_pop == self.size_x {
            RowPopulation::Full
        } else if row_pop == 0 {
            RowPopulation::Empty
        } else {
            RowPopulation::Mixed
        }
    }

    pub fn wipe_full_rows(&mut self) {
        let status: Vec<RowPopulation> = (0..self.size_y)
            .map(|row| self.row_status(row))
            .collect();

        let mut lower = 0;
        let mut upper = 0;


        while lower < self.size_y {
            if status[lower] == RowPopulation::Empty {
                return;
            }
            while status[upper] == RowPopulation::Full {
                upper += 1;
            }
            self.copy_row(lower, upper);
            lower += 1;
            upper += 1;
        }
    }

    pub fn check_piece(&self, piece: &Tetromino) -> bool {
        piece.blocks()
            .iter()
            .map(|b| {
                if self.index(&b.point).is_ok() {
                    self.is_empty(&b.point)
                } else {
                    false
                }
            })
            .fold(true, |a, b| a && b)
    }

    fn is_empty(&self, p: &Point) -> bool {
        match self.blocks[self.index(p).unwrap()] {
            Some(_) => false,
            None => true,
        }
    }

    fn get_color(&self, p: &Point) -> Color {
        match self.blocks[self.index(p).unwrap()] {
            Some(c) => c,
            None => GameBoard::default_color(),
        }
    }

    fn index(&self, p: &Point) -> Result<usize> {
        let width: i32 = self.size_x as i32;
        let height: i32 = self.size_y as i32;
        if p.x < 0 || p.x >= width {
            return Err(error::Error::OutOfBounds);
        }
        if p.y < 0 || p.y >= height {
            return Err(error::Error::OutOfBounds);
        }
        let x = p.x as usize;
        let y = p.y as usize;
        Ok(y * self.size_x + x)
    }

    fn default_color() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    pub fn add_blocks(&mut self, blocks: &Vec<Block>) {
        for &Block { point: p, color: mut c } in blocks {
            let index = self.index(&p).unwrap();
            if self.blocks[index].is_some() {
                error!("State: {:?}", self);
                error!("Block: {:?}", p);
                // panic!("Trying to take over existing block!");
                c = Color::black();

            }
            c.alpha = 0.8f32;
            self.blocks[index] = Some(c);
        }
    }


    pub fn blocks(&self) -> Vec<Block> {
        let mut result: Vec<Block> = Vec::with_capacity(self.size_x * self.size_y);
        for jx in 0..self.height() {
            for ix in 0..self.size_x {
                let p = Point {
                    x: ix as i32,
                    y: jx as i32,
                };
                let block = Block {
                    color: self.get_color(&p),
                    point: p,
                };
                result.push(block);
            }
        }
        result
    }
}


#[test]
fn new_board() {
    let a = GameBoard::new(2, 3, 0, Pixel::new(0f64, 0f64));
    assert_eq!(6, a.blocks.len());
    let b = GameBoard::new(20, 10, 0, Pixel::new(0f64, 0f64));
    assert_eq!(200, b.blocks.len());
    let c = GameBoard::new(12, 5, 0, Pixel::new(0f64, 0f64));
    assert_eq!(60, c.blocks.len());
}

#[test]
fn copy_rows() {
    let mut board = GameBoard::new(2, 2, 0, Pixel::new(0f64, 0f64));
    let c1 = Color::black();
    let c2 = Color::white();
    board.blocks = vec![None, Some(c1), Some(c2), None];
    board.copy_row(0, 1);
    assert_eq!(vec![Some(c2), None, Some(c2), None], board.blocks);
}

#[test]
fn row_status_check() {
    let mut board = GameBoard::new(2, 3, 0, Pixel::new(0f64, 0f64));
    let c1 = Color::black();
    board.blocks = vec![None, None, Some(c1), None, Some(c1), Some(c1)];
    assert_eq!(RowPopulation::Empty, board.row_status(0));
    assert_eq!(RowPopulation::Mixed, board.row_status(1));
    assert_eq!(RowPopulation::Full, board.row_status(2));
}
