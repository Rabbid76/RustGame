use rust_game::color::ColorU8;
use rust_game::context::Context;
use rust_game::events::Event;
use rust_game::keys::KeyCode;
use rust_game::surface::{BlendMode, SurfaceBuilder};
use rust_game_sdl2::context::Sdl2Context;
use std::path::Path;

pub fn main() {
    let context = Sdl2Context::new().unwrap();
    let mut canvas = context.new_canvas().unwrap();
    let mut events = context.events().unwrap();
    let time = context.time().unwrap();
    let mut clock = time.new_clock();
    let image = context.image().unwrap();
    let mut background_surf =
        Sdl2Context::new_surface_alpha(canvas.get_surface().get_size()).unwrap();
    background_surf
        .fill(&ColorU8::new_gray_alpha(128, 10))
        .unwrap();
    let test_bmp = image
        .load(&Path::new("./resource/bitmap/test.bmp"))
        .unwrap();
    let test1_png = image
        .load(&Path::new("./resource/icon/Apple64.png"))
        .unwrap();
    let test2_png = image
        .load(&Path::new("./resource/icon/Banana64.png"))
        .unwrap();
    let test3_png = image
        .load(&Path::new("./resource/icon/Pear64.png"))
        .unwrap();
    let test4_png = image
        .load(&Path::new("./resource/icon/Plums64.png"))
        .unwrap();
    let test_svg = image
        .load(&Path::new("./resource/clipart/ice-001.svg"))
        .unwrap();

    'running: loop {
        let _ = clock.tick_frame_rate(100);
        let _ = time.get_ticks();

        for event in events.get().unwrap() {
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

        canvas.get_surface().fill(&ColorU8::new_gray(128)).unwrap();
        canvas
            .get_surface()
            .blit(test_bmp.as_ref(), (10, 10), BlendMode::Blend)
            .unwrap();
        canvas
            .get_surface()
            .blit(test1_png.as_ref(), (10, 100), BlendMode::Blend)
            .unwrap();
        canvas
            .get_surface()
            .blit(test2_png.as_ref(), (110, 100), BlendMode::Blend)
            .unwrap();
        canvas
            .get_surface()
            .blit(test3_png.as_ref(), (210, 100), BlendMode::Blend)
            .unwrap();
        canvas
            .get_surface()
            .blit(test4_png.as_ref(), (310, 100), BlendMode::Blend)
            .unwrap();
            canvas
            .get_surface()
            .blit(test_svg.as_ref(), (10, 200), BlendMode::Blend)
            .unwrap();
        canvas.update().unwrap();
    }
}
