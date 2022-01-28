use crate::window::Sdl2Window;
use rust_game::context::Context;
use rust_game::window::Window;
use std::error::Error;

pub struct Sdl2Context {}

impl Sdl2Context {
    pub fn new() -> Sdl2Context {
        Sdl2Context {}
    }
}

impl Context for Sdl2Context {
    fn new_window(&self) -> Result<Box<dyn Window>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Window::new()))
    }
}
