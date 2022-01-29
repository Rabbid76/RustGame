use crate::context::Sdl2Context;
use rust_game::canvas::Canvas;
use rust_game::color::Color;
use std::error::Error;
extern crate sdl2;

pub struct Sdl2Canvas {
    canvas: sdl2::render::WindowCanvas,
}

impl Sdl2Canvas {
    pub fn new(context: &Sdl2Context) -> Result<Sdl2Canvas, Box<dyn Error>> {
        let window = context
            .video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build()?;
        Ok(Sdl2Canvas { canvas })
    }
}

impl Canvas for Sdl2Canvas {
    fn update(&mut self) {
        self.canvas.present();
    }

    fn fill(&mut self, color: &dyn Color) {
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(color.r(), color.g(), color.b()));
        self.canvas.clear();
    }
}
