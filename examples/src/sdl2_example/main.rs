// TODO: https://docs.rs/crate/sdl2/0.22.0/source/README.md

use rust_game::color::ColorU8;
use rust_game::context::Context;
use rust_game_sdl2::context::Sdl2Context;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() {
    let context = Sdl2Context::new().unwrap();
    let mut canvas = context.new_canvas().unwrap();

    let mut event_pump = context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.fill(&ColorU8::new_rgb(i, 64, 255-i));
       

        canvas.update();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
