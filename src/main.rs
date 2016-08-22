#![feature(box_syntax)]

#![feature(zero_one)]
extern crate piston_window;

extern crate rand;

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate toml;

extern crate nalgebra as na;

use piston_window::*;
use std::ops::Add;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

mod error;
mod limit;
mod input;
mod tetronimo;
mod point;
mod tetriscolor;
mod transform;
mod block;
mod board;
mod game;


type Result<T> = std::result::Result<T, error::Error>;


use tetronimo::Shape;



fn main() {
    env_logger::init().unwrap();
    let mut window: PistonWindow = WindowSettings::new("Tetris", [540, 580])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = game::Game::new();
    while let Some(e) = window.next() {
        match e {
            Event::Update(UpdateArgs { dt }) => game.on_update(dt),
            Event::Input(ref input) => game.on_input(input),
            Event::Render(_) => {
                window.draw_2d(&e, |c, g| {
                    clear([0.5; 4], g);
                    game.on_render(g, c.transform);
                });
            }
            _ => debug!("Unknown Window Event {:?}", e),
        }
    }
}
