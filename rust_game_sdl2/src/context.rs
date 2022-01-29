use crate::canvas::Sdl2Canvas;
use rust_game::canvas::Canvas;
use rust_game::context::Context;
use std::error::Error;
extern crate sdl2;

pub struct Sdl2Context {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
}

impl Sdl2Context {
    pub fn new() -> Result<Sdl2Context, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        Ok(Sdl2Context {
            sdl_context,
            video_subsystem,
        })
    }

    // TODO delete
    pub fn event_pump(&self) -> Result<sdl2::EventPump, Box<dyn Error>> {
        Ok(self.sdl_context.event_pump()?)
    }
}

impl Context for Sdl2Context {
    fn new_canvas(&self) -> Result<Box<dyn Canvas>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Canvas::new(&self)?))
    }
}
