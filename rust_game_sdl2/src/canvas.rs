use crate::context::Sdl2Context;
use crate::surface::Sdl2Surface;
use rust_game::canvas::Canvas;
use rust_game::surface::Surface;
use std::error::Error;
use std::sync::Arc;
extern crate sdl2;

pub struct Sdl2Canvas {
    pub sdl_context: Arc<sdl2::Sdl>,
    pub window: sdl2::video::Window,
    pub canvas_surface: Sdl2Surface,
}

impl Sdl2Canvas {
    pub fn new(context: &Sdl2Context) -> Result<Sdl2Canvas, Box<dyn Error>> {
        let window = context
            .video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()?;
        let event_pump = context.sdl_context.event_pump()?;
        let window_surface = window.surface(&event_pump)?;
        let canvas_surface = sdl2::surface::Surface::new(
            window_surface.width(),
            window_surface.height(),
            sdl2::pixels::PixelFormatEnum::ABGR8888,
        )?;
        Ok(Sdl2Canvas {
            sdl_context: context.sdl_context.clone(),
            window,
            canvas_surface: Sdl2Surface::from_surface(canvas_surface),
        })
    }
}

impl Canvas for Sdl2Canvas {
    fn get_surface(&mut self) -> &mut dyn Surface {
        &mut self.canvas_surface
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let event_pump = self.sdl_context.event_pump()?;
        let mut window_surface = self.window.surface(&event_pump)?;
        self.canvas_surface
            .surface
            .blit(Option::None, &mut window_surface, Option::None)?;
        window_surface.update_window()?;
        Ok(())
    }
}
