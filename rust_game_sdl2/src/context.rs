use crate::canvas::Sdl2Canvas;
use crate::events::Sdl2Events;
use rust_game::canvas::Canvas;
use rust_game::context::{Context, ContextData};
use rust_game::events::Events;
use rust_game::time::{Time, TimeStd};
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;

extern crate sdl2;

pub struct Sdl2Context {
    pub sdl_context: Rc<sdl2::Sdl>,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub context_data: Arc<ContextData>,
}

impl Sdl2Context {
    pub fn new() -> Result<Sdl2Context, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        Ok(Sdl2Context {
            sdl_context: Rc::new(sdl_context),
            video_subsystem,
            context_data: Arc::new(ContextData::new()),
        })
    }
}

impl Context for Sdl2Context {
    fn new_canvas(&self) -> Result<Box<dyn Canvas>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Canvas::new(&self)?))
    }
    fn events(&self) -> Result<Box<dyn Events>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Events::from(&self)?))
    }
    fn time(&self) -> Result<Box<dyn Time>, Box<dyn Error>> {
        Ok(Box::new(TimeStd::from(self.context_data.clone())?))
    }
}
