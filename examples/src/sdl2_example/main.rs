// TODO: https://docs.rs/crate/sdl2/0.22.0/source/README.md

use rust_game::color::ColorU8;
use rust_game::context::Context;
use rust_game::events::Event;
use rust_game::keys::KeyCode;
use rust_game_sdl2::context::Sdl2Context;

use std::time::Duration;

pub fn main() {
    let context = Sdl2Context::new().unwrap();
    let mut canvas = context.new_canvas().unwrap();
    let mut events = context.events().unwrap();
    let time = context.time().unwrap();
    let mut clock = time.new_clock();

    let mut i = 0;
    'running: loop {
        let delta = clock.tick_frame_rate(50);
        let current = time.get_ticks();
        println!("{}, {}", delta, current);        

        for event in events.get() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    key: Some(KeyCode::ESC),
                    ..
                } => break 'running,
                Event::KeyDown { key_code, key } => {
                    println!("{}, {:?}", key_code, key)
                }
                _ => {}
            }
        }

        i = (i + 1) % 255;
        canvas.fill(&ColorU8::new_rgb(i, 64, 255 - i));

        canvas.update();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
