#![feature(box_syntax)]
extern crate piston_window;

extern crate rand;

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate toml;


use piston_window::*;
use std::ops::Add;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

mod error;
mod limit;
mod input;

type Result<T> = std::result::Result<T, error::Error>;



impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }
    pub fn origin() -> Self {
        Point::new(0, 0)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Color {
    pub fn as_list(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
    pub fn black() -> Self {
        Color {
            red: 0f32,
            green: 0f32,
            blue: 0f32,
            alpha: 1f32,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct GameBoard {
    size_x: usize,
    size_y: usize,
    blocks: Vec<Option<Color>>,
}


#[derive(Debug, Clone)]
struct Block {
    color: Color,
    point: Point,
}


#[derive(Debug, Clone)]
struct Tetromino {
    shape: Shape,
    origin: Point,
    orientation: Orientation,
    state: TetronimoState,
}

impl Tetromino {
    pub fn new() -> Self {
        Tetromino {
            shape: Shape::O,
            origin: Point { x: 0, y: 0 },
            orientation: Orientation::North,
            state: TetronimoState::Nonexistant,
        }
    }

    pub fn new_shape(shape: Shape) -> Self {
        Tetromino {
            shape: shape,
            origin: Point { x: 5, y: 18 },
            orientation: Orientation::North,
            state: TetronimoState::Falling,
        }
    }

    pub fn color(&self) -> Color {
        let alpha: f32 = match self.state {
            TetronimoState::Falling => 1f32,
            TetronimoState::Ghost => 0.1f32,
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

    pub fn slide(&mut self, direction: Slide) {
        match direction {
            Slide::Left => self.origin.x -= 1,
            Slide::Right => self.origin.x += 1,
        }
    }

    pub fn rotate_clockwise(&mut self) {
        self.orientation = match self.orientation {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        };
        info!("Active Orientation: {:?}", self.orientation);
    }

    pub fn move_down(&mut self) {
        self.origin.y -= 1;
    }

    pub fn blocks(&self) -> Vec<Block> {
        let color = self.color();
        let points: Vec<Point> = match self.shape {
            Shape::O => {
                vec![
                Point {x: self.origin.x, y: self.origin.y},
                Point {x: self.origin.x - 1, y: self.origin.y},
                Point {x: self.origin.x , y: self.origin.y - 1},
                Point {x: self.origin.x - 1, y: self.origin.y - 1},
            ]
            }
            Shape::I => {
                vec![
                    Point {x: self.origin.x - 1, y: self.origin.y + 1 },
                    Point {x: self.origin.x - 1, y: self.origin.y },
                    Point {x: self.origin.x - 1, y: self.origin.y - 1},
                    Point {x: self.origin.x - 1, y: self.origin.y - 2},
                ]
            }
            Shape::T => {
                vec![
                    Point {x: self.origin.x, y: self.origin.y},
                    Point {x: self.origin.x , y: self.origin.y + 1},
                    Point {x: self.origin.x -1 , y: self.origin.y },
                    Point {x: self.origin.x +1 , y: self.origin.y },
                ]
            }
        }.iter().map(|p| {
            rotate(&p, &self.origin, &RotationMatrix::orient(&self.orientation))

        }).collect();

        points.iter()
            .map(|point| {
                Block {
                    point: *point,
                    color: color,
                }
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
enum RowPopulation {
    Empty,
    Full,
    Mixed,
}

impl GameBoard {
    pub fn new(x: usize, y: usize) -> Self {
        GameBoard {
            size_x: x,
            size_y: y,
            blocks: vec![None; x * y],
        }
    }

    fn copy_row(&mut self, lower: usize, upper: usize) {
        for i in 0..self.size_x {
            self.blocks[lower * self.size_x + i] = self.blocks[upper * self.size_x + i];
        }
    }

    fn clear_row(&mut self, row: usize) {
        for i in 0..self.size_x {
            self.blocks[row * self.size_x + i] = None;
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

    fn wipe_full_rows(&mut self) {
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

    fn check_piece(&self, piece: &Tetromino, offset: &Point) -> bool {
        piece.blocks()
            .iter()
            .map(|b| {
                let p = b.point + *offset;
                if self.index(&p).is_ok() {
                    self.is_empty(&p)
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

    fn add_blocks(&mut self, blocks: &Vec<Block>) {
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

    pub fn render_block<G>(&self,
                           g: &mut G,
                           view: math::Matrix2d,
                           unit_width: f64,
                           x: f64,
                           y: f64,
                           block: Block)
        where G: Graphics
    {
        let draw_x = x + block.point.x as f64 * unit_width;
        let draw_y = y - block.point.y as f64 * unit_width;
        let square = rectangle::square(draw_x, draw_y, unit_width * 0.95);
        rectangle(block.color.as_list(), square, view, g);
    }
    pub fn render<G>(&self, g: &mut G, view: math::Matrix2d, unit_width: f64, x: f64, y: f64)
        where G: Graphics
    {
        let mut draw_x = x;
        let mut draw_y = y;
        for jx in 0..self.size_y {
            for ix in 0..self.size_x {
                let p = Point {
                    x: ix as i32,
                    y: jx as i32,
                };
                let block = Block {
                    color: self.get_color(&p),
                    point: p,
                };
                self.render_block(g, view, unit_width, x, y, block);
            }
        }
    }
}

#[derive(Debug, Clone)]
enum TetronimoState {
    Falling,
    Ghost,
    Locking,
    Frozen,
    Nonexistant,
}

#[derive(Debug, Clone)]
enum Shape {
    O,
    T,
    I,
}

#[derive(Debug, Clone)]
enum Orientation {
    North,
    East,
    South,
    West,
}

struct RotationMatrix {
    matrix: [i32; 4],
}

impl RotationMatrix {
    fn orient(o: &Orientation) -> Self {
        match *o {
            Orientation::North => RotationMatrix { matrix: [1, 0, 0, 1] },
            Orientation::East => RotationMatrix { matrix: [0, 1, -1, 0] },
            Orientation::South => RotationMatrix { matrix: [-1, 0, 0, -1] },
            Orientation::West => RotationMatrix { matrix: [0, -1, 1, 0] },
        }
    }
}

fn rotate(p: &Point, o: &Point, r: &RotationMatrix) -> Point {
    let x = r.matrix[0] * (p.x - o.x) - r.matrix[1] * (p.y - o.y) + o.x;
    let y = r.matrix[2] * (p.x - o.x) - r.matrix[3] * (p.y - o.y) + o.y;
    Point::new(x, y)
}

enum Slide {
    Left,
    Right,
}

struct Game {
    gameboard: GameBoard,
    unit_width: f64,
    time_between_gravity: f64,
    time_between_down_speed: f64,
    time_since_gravity: f64,
    time_between_slides: f64,
    time_since_slide: f64,
    time_between_rotate: f64,
    time_since_rotate: f64,
    active_piece: Tetromino,
    ghost_piece: Tetromino,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    space: bool,
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    space_pressed: bool,
    tetromino_disttribution: Vec<Weighted<Shape>>,
    rng: rand::ThreadRng,
}


impl Game {
    fn new() -> Self {
        let tetromino_choice = vec!(
                Weighted { weight: 5, item: Shape::O },
                Weighted { weight: 1, item: Shape::I },
                Weighted { weight: 1, item: Shape::T },
            );

        Game {
            gameboard: GameBoard::new(10, 20),
            unit_width: 25f64,
            time_between_gravity: 0.5f64,
            time_between_down_speed: 0.05f64,
            time_since_gravity: 0f64,
            time_between_slides: 0.05f64,
            time_since_slide: 0f64,
            time_between_rotate: 0.1f64,
            time_since_rotate: 0f64,
            active_piece: Tetromino::new(),
            ghost_piece: Tetromino::new(),
            up: false,
            down: false,
            left: false,
            right: false,
            space: false,
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
            space_pressed: false,
            tetromino_disttribution: tetromino_choice,
            rng: rand::thread_rng(),
        }
    }

    fn clear_input(&mut self) {
        self.up_pressed = false;
        self.down_pressed = false;
        self.left_pressed = false;
        self.right_pressed = false;
        self.space_pressed = false;
    }

    fn new_piece(&mut self) -> Tetromino {
        let wc = WeightedChoice::new(&mut self.tetromino_disttribution);
        let shape = wc.ind_sample(&mut self.rng);
        Tetromino::new_shape(shape)
    }

    fn slide(&mut self, dt: f64) {
        self.time_since_slide += dt;
        if self.time_since_slide > self.time_between_slides {
            self.time_since_slide = 0f64;
            match (self.left || self.left_pressed, self.right || self.right_pressed) {
                (true, true) => {}
                (false, false) => {}
                (true, false) => {
                    if self.gameboard.check_piece(&self.active_piece, &Point::new(-1, 0)) {
                        self.active_piece.slide(Slide::Left);
                        self.ghost_piece = self.ghost(&self.active_piece);
                    }
                }
                (false, true) => {
                    if self.gameboard.check_piece(&self.active_piece, &Point::new(1, 0)) {
                        self.active_piece.slide(Slide::Right);
                        self.ghost_piece = self.ghost(&self.active_piece);
                    }
                }
            }
            self.clear_input();
        }
    }

    fn rotate(&mut self, dt: f64) {
        self.time_since_rotate += dt;
        if self.time_since_rotate > self.time_between_rotate {
            self.time_since_rotate = 0f64;
            if self.up || self.up_pressed {
                self.active_piece.rotate_clockwise();
                self.ghost_piece = self.ghost(&self.active_piece);
            }
            self.clear_input();
        }
    }


    fn gravity(&mut self, dt: f64) {
        self.time_since_gravity += dt;
        let down_timer = if self.down {
            self.time_between_down_speed
        } else {
            self.time_between_gravity
        };
        if self.time_since_gravity > down_timer {
            self.time_since_gravity = 0f64;
            if self.gameboard.check_piece(&self.active_piece, &Point::new(0, -1)) {
                self.active_piece.move_down();
            } else {
                self.lock()
            }
        }
    }


    fn ghost(&self, piece: &Tetromino) -> Tetromino {
        let mut ghost = piece.clone();
        let down_one = Point::new(0, -1);

        while self.gameboard.check_piece(&ghost, &down_one) {
            ghost.move_down()
        }
        ghost.state = TetronimoState::Ghost;
        ghost
    }

    fn lock(&mut self) {
        self.gameboard.add_blocks(&self.active_piece.blocks());
        self.active_piece.state = TetronimoState::Nonexistant;
        self.gameboard.wipe_full_rows();
        self.clear_input();
    }

    fn on_input(&mut self, inp: &Input) {
        match *inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up = true;
                        self.up_pressed = true;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down = true;
                        self.down_pressed = true;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left = true;
                        self.left_pressed = true;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right = true;
                        self.right_pressed = true;
                    }
                    Button::Keyboard(Key::Space) => {
                        self.space = true;
                        self.space_pressed = true;
                    }
                    _ => {}
                }
            }
            Input::Release(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up = false;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down = false;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left = false;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right = false;
                    }
                    Button::Keyboard(Key::Space) => {
                        self.space = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }


    fn on_update(&mut self, upd: UpdateArgs) {}

    //     fn on_draw(&mut self, ren: RenderArgs, e: PistonWindow) {
    //         e.draw_2d(&ren.viewport(), |c, g| {
    //             clear([0.0, 0.0, 0.0, 1.0], g);
    //             let center = c.transform.trans((ren.width / 2) as f64, (ren.height / 2) as f64);
    //             let square = rectangle::square(0.0, 0.0, 100.0);
    //             let red = [1.0, 0.0, 0.0, 1.0];
    //             rectangle(red, square, c.transform, g); // We translate the rectangle slightly so that it's centered; otherwise only the top left corner would be centered
    //
    //         });
    //     }
}


fn main() {


    env_logger::init().unwrap();
    let mut game = Game::new();
    let mut window: PistonWindow = WindowSettings::new("Miranda Tetris", [540, 580])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut block_x = 0f64;
    while let Some(e) = window.next() {
        debug!("{:?}", e);
        match e {
            Event::Update(UpdateArgs { dt }) => {
                match game.active_piece.state {
                    TetronimoState::Falling => {
                        if game.space_pressed {
                            game.clear_input();
                            game.active_piece = game.ghost_piece.clone();
                            game.lock();
                        }
                        game.gravity(dt);
                        game.slide(dt);
                        game.rotate(dt);
                    }
                    TetronimoState::Nonexistant => {
                        game.active_piece = game.new_piece();
                        game.ghost_piece = game.ghost(&game.active_piece);
                    }
                    _ => (),
                }
            }
            Event::Input(ref input) => game.on_input(input),
            _ => {}
        }
        block_x += 1f64;
        window.draw_2d(&e, |c, g| {
            clear([0.5; 4], g);
            game.gameboard.render(g, c.transform, game.unit_width, 20f64, 500f64);
            for block in game.ghost_piece.blocks() {
                game.gameboard.render_block(g, c.transform, game.unit_width, 20f64, 500f64, block);
            }
            for block in game.active_piece.blocks() {
                game.gameboard.render_block(g, c.transform, game.unit_width, 20f64, 500f64, block);
            }
        });
    }
}
