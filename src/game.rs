use piston_window::*;
use super::block::Block;
use super::board::{GameBoard, Pixel};
use super::tetronimo::{SlideDirection, Shape, Tetromino, TetrominoGenerator, TetronimoState};
use super::transform::{RotationDirection, Point};
use super::input;
use super::limit;



#[derive(Debug, Clone)]
enum KeyAction {
    Press,
    Release,
    Unknown,
}

pub struct Game {
    gameboard: GameBoard,
    upcoming: GameBoard,
    tetronimo_generator: TetrominoGenerator,
    unit_width: f64,
    slide_timer: limit::RateLimiter,
    rotate_timer: limit::RateLimiter,
    gravity_timer: limit::RateLimiter,
    fast_fall_timer: limit::RateLimiter,
    active_piece: Tetromino,
    ghost_piece: Tetromino,
    key_mapping: input::KeyMap,
    command_state: input::CommandState,
}


impl Game {
    pub fn new() -> Self {
        let mut key_map = input::KeyMap::new();
        key_map.insert(Key::Up, input::Command::RotateClockwise);
        key_map.insert(Key::Down, input::Command::DownFast);
        key_map.insert(Key::Left, input::Command::SlideLeft);
        key_map.insert(Key::Right, input::Command::SlideRight);
        key_map.insert(Key::Space, input::Command::Lock);
        Game {
            gameboard: GameBoard::new(10, 22, 2, Pixel::new(20f64, 500f64)),
            upcoming: GameBoard::new(6, 9, 0, Pixel::new(400f64, 500f64)),
            tetronimo_generator: TetrominoGenerator::new(),
            unit_width: 25f64,
            slide_timer: limit::RateLimiter::new(0.05f64, Some(0.3f64)),
            rotate_timer: limit::RateLimiter::new(0.4f64, Some(0.4f64)),
            gravity_timer: limit::RateLimiter::new(0.5f64, None),
            fast_fall_timer: limit::RateLimiter::new(0.05f64, None),
            key_mapping: key_map,
            command_state: input::CommandState::new(),
            active_piece: Tetromino::new(),
            ghost_piece: Tetromino::new(),
        }

    }


    fn new_piece(&mut self) -> Tetromino {
        let mut new_piece = self.tetronimo_generator.pop();
        new_piece.state = TetronimoState::Falling;
        new_piece.put(Point::new(5, 21));
        new_piece
    }

    fn check_and_update(&mut self, direction: RotationDirection) {
        let mut new_piece = self.active_piece.clone();
        new_piece.rotate(&direction);
        let translations = new_piece.wall_kick_options(&direction);
        for test_translate in &translations {
            let mut test_piece = new_piece.clone();
            test_piece.translate(test_translate);
            if self.gameboard.check_piece(&test_piece) {
                self.active_piece = test_piece;
                self.ghost_piece = self.ghost(&self.active_piece);
                return;
            }
        }
    }

    fn slide(&mut self) {
        let event = self.slide_timer.get_event();
        let maybe_direction = self.command_state.do_slide();
        let mut test_piece = self.active_piece.clone();
        match (event, maybe_direction) {
            (Some(_), Some(input::SlideDirection::Left)) => test_piece.slide(SlideDirection::Left),
            (Some(_), Some(input::SlideDirection::Right)) => {
                test_piece.slide(SlideDirection::Right)
            }
            (_, None) => self.slide_timer.reset(),
            (None, _) => {}
        }
        if self.gameboard.check_piece(&test_piece) {
            self.active_piece = test_piece;
            self.ghost_piece = self.ghost(&self.active_piece);
        }
    }



    fn rotate(&mut self) {
        let event = self.rotate_timer.get_event();
        let maybe_direction = self.command_state.do_rotate();
        match (event, maybe_direction) {
            (Some(_), Some(input::RotateDirection::Clockwise)) => {
                self.check_and_update(RotationDirection::Clockwise)
            }
            (Some(_), Some(input::RotateDirection::CounterClockwise)) => {
                self.check_and_update(RotationDirection::CounterClockwise)
            }
            (_, None) => self.rotate_timer.reset(),
            (None, _) => {}
        };
    }


    fn gravity(&mut self) {
        let event = match self.command_state.get_drop_speed() {
            input::DropSpeed::Fast => self.fast_fall_timer.get_event(),
            input::DropSpeed::Slow => self.gravity_timer.get_event(),
        };

        debug!("Gravity is: {:?}", event);
        match event {
            Some(_) => {
                let mut test_piece = self.active_piece.clone();
                test_piece.move_down();
                if self.gameboard.check_piece(&test_piece) {
                    self.active_piece = test_piece;
                } else {
                    self.lock();
                }
            }
            None => {} //Timer says wait
        }
    }


    fn ghost(&self, piece: &Tetromino) -> Tetromino {
        let mut test = piece.clone();
        let mut ghost = test.clone();

        while self.gameboard.check_piece(&test) {
            ghost = test.clone();
            test.move_down();
        }

        ghost.state = TetronimoState::Ghost;
        ghost
    }

    fn lock(&mut self) {
        self.gameboard.add_blocks(&self.active_piece.blocks());
        self.active_piece.state = TetronimoState::Nonexistant;
        self.gameboard.wipe_full_rows();
        self.command_state.clear_state();
        self.clear_timers();
    }


    pub fn on_input(&mut self, inp: &Input) {
        let (command, action) = match *inp {
            Input::Press(Button::Keyboard(button)) => {
                (self.key_mapping.get(&button), KeyAction::Press)
            }
            Input::Release(Button::Keyboard(button)) => {
                (self.key_mapping.get(&button), KeyAction::Release)
            }
            _ => (None, KeyAction::Unknown),
        };
        debug!("{:?} {:?}", command, action);
        match (command, action) {
            (Some(c), KeyAction::Press) => self.command_state.key_press(*c),
            (Some(c), KeyAction::Release) => self.command_state.key_release(*c),
            (_, _) => {}
        }
    }


    pub fn on_render<G>(&self, g: &mut G, view: math::Matrix2d)
        where G: Graphics
    {
        let Pixel { x, y } = self.gameboard.point;
        let height = self.gameboard.height() as i32;

        for block in self.gameboard.blocks() {
            self.render_block(g, view, x, y, block);
        }

        for block in self.active_piece.blocks() {
            if block.point.y < height {
                self.render_block(g, view, x, y, block);
            }
        }
        for block in self.ghost_piece.blocks() {
            if block.point.y < height {
                self.render_block(g, view, x, y, block);
            }
        }


        let Pixel { x: upcoming_x, y: upcoming_y } = self.upcoming.point;
        for block in self.upcoming.blocks() {
            self.render_block(g, view, upcoming_x, upcoming_y, block);
        }

        for i in 0..3 {
            let p = Point::new(3, 7 - i * 3);
            let mut upcoming_tetronimo = self.tetronimo_generator.peek(i as usize);
            upcoming_tetronimo.state = TetronimoState::Frozen;
            upcoming_tetronimo.translate(&p);
            for block in upcoming_tetronimo.blocks() {
                self.render_block(g, view, upcoming_x, upcoming_y, block);
            }
        }
    }

    fn clear_timers(&mut self) {
        self.gravity_timer.reset();
        self.fast_fall_timer.reset();
        self.slide_timer.reset();
        self.rotate_timer.reset();
    }

    fn update_timers(&mut self, dt: f64) {
        self.gravity_timer.elapsed(dt);
        self.fast_fall_timer.elapsed(dt);
        self.slide_timer.elapsed(dt);
        self.rotate_timer.elapsed(dt);
    }

    pub fn on_update(&mut self, dt: f64) {
        self.update_timers(dt);

        match self.active_piece.state {
            TetronimoState::Falling => {
                debug!("Falling");
                if self.command_state.lock() {
                    self.active_piece = self.ghost_piece.clone();
                    self.lock();
                } else {
                    self.gravity();
                    self.slide();
                    self.rotate();
                }
            }
            TetronimoState::Nonexistant => {
                self.active_piece = self.new_piece();
                self.ghost_piece = self.ghost(&self.active_piece);
            }
            ref state => unreachable!("Found Active Tetronimo in State: {:?}", state),
        }
    }


    fn render_block<G>(&self, g: &mut G, view: math::Matrix2d, x: f64, y: f64, block: Block)
        where G: Graphics
    {
        let draw_x = x + block.point.x as f64 * self.unit_width;
        let draw_y = y - block.point.y as f64 * self.unit_width;
        let square = rectangle::square(draw_x, draw_y, self.unit_width * 0.95);
        rectangle(block.color.as_list(), square, view, g);
    }
}
