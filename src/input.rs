
use piston_window::Key;

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub enum SlideDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum RotateDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone, Copy)]
pub enum DropSpeed {
    Slow,
    Fast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    SlideLeft,
    SlideRight,
    DownFast,
    Lock,
    RotateClockwise,
    RotateCounterClockwise,
}


pub struct KeyMap {
    map: BTreeMap<Key, Command>,
}

impl KeyMap {
    pub fn new() -> Self {
        KeyMap { map: BTreeMap::new() }
    }
    pub fn get(&self, key: &Key) -> Option<&Command> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: Key, value: Command) -> Option<Command> {
        self.map.insert(key, value)
    }
}


pub struct CommandState {
    slide: Option<SlideDirection>,
    rotate: Option<RotateDirection>,
    drop: DropSpeed,
    lock: bool,
    key_active: BTreeMap<Command, bool>,
}

impl CommandState {
    pub fn new() -> Self {
        CommandState {
            slide: None,
            rotate: None,
            lock: false,
            drop: DropSpeed::Slow,
            key_active: BTreeMap::new(),
        }
    }

    pub fn clear_state(&mut self) {
        *self = CommandState::new();
    }

    pub fn get_drop_speed(&self) -> DropSpeed {
        self.drop
    }

    pub fn lock(&self) -> bool {
        self.lock
    }

    pub fn key_press(&mut self, key: Command) {
        match key {
            Command::SlideLeft => self.slide = Some(SlideDirection::Left),
            Command::SlideRight => self.slide = Some(SlideDirection::Right),
            Command::DownFast => self.drop = DropSpeed::Fast,
            Command::Lock => self.lock = true,
            Command::RotateClockwise => self.rotate = Some(RotateDirection::Clockwise),
            Command::RotateCounterClockwise => {
                self.rotate = Some(RotateDirection::CounterClockwise)
            }
        }
        self.key_active.insert(key, true);
        if self.key_active.get(&Command::SlideLeft) == self.key_active.get(&Command::SlideRight) {
            self.slide = None;
        }
        if self.key_active.get(&Command::RotateClockwise) ==
           self.key_active.get(&Command::RotateCounterClockwise) {
            self.rotate = None;
        }
    }

    pub fn key_release(&mut self, key: Command) {
        self.key_active.insert(key, false);
        match key {
            Command::DownFast => self.drop = DropSpeed::Slow,
            _ => {}
        }
    }

    pub fn do_slide(&mut self) -> Option<SlideDirection> {
        let direction = self.slide;
        let key_state = match direction {
            Some(SlideDirection::Left) => self.key_active.get(&Command::SlideLeft),
            Some(SlideDirection::Right) => self.key_active.get(&Command::SlideRight),
            None => None,
        };
        if let Some(key_pressed) = key_state {
            if !key_pressed {
                self.slide = None;
            }
        }
        direction
    }

    pub fn do_rotate(&mut self) -> Option<RotateDirection> {
        let direction = self.rotate;
        let key_state = match direction {
            Some(RotateDirection::Clockwise) => self.key_active.get(&Command::RotateClockwise),
            Some(RotateDirection::CounterClockwise) => {
                self.key_active.get(&Command::RotateCounterClockwise)
            }
            None => None,
        };
        if let Some(key_pressed) = key_state {
            if !key_pressed {
                self.rotate = None;
            }
        }
        direction
    }
}
