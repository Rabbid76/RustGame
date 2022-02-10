use rust_game::color::{Color, ColorU8};
use rust_game::context::Context;
use rust_game::events::Event;
use rust_game::keys::KeyCode;
use rust_game::sprite::animation::{HypotrochoidAnimation, ToColor};
use rust_game::sprite::{DefaultSprite, Group};
use rust_game::surface::{BlendMode, SurfaceBuilder};
use rust_game_sdl2::context::Sdl2Context;
use std::path::Path;

pub struct FrameToHSVColor {}

impl ToColor<u32> for FrameToHSVColor {
    fn get_color(&mut self, frame: u32) -> Box<dyn Color> {
        Box::new(ColorU8::from_hsl((frame as u16 + 1) % 360, 100, 50))
    }
}

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

    let mut sprite_group = Group::new(vec![
        DefaultSprite::new_animated(
            test_bmp.clone().unwrap(),
            Option::None,
            Some(Box::new(HypotrochoidAnimation::new(
                0.0,
                1.0,
                (400, 300),
                (100.0, 60.0, 100.0),
            ))),
        ),
        DefaultSprite::new_animated(
            test_bmp.clone().unwrap(),
            Option::None,
            Some(Box::new(HypotrochoidAnimation::new(
                360.0 * 3.0 / 4.0,
                1.0,
                (400, 300),
                (100.0, 60.0, 100.0),
            ))),
        ),
        DefaultSprite::new_animated(
            test_bmp.clone().unwrap(),
            Option::None,
            Some(Box::new(HypotrochoidAnimation::new(
                360.0 * 3.0 / 4.0 * 2.0,
                1.0,
                (400, 300),
                (100.0, 60.0, 100.0),
            ))),
        ),
        DefaultSprite::new_animated(
            test_bmp.clone().unwrap(),
            Option::None,
            Some(Box::new(HypotrochoidAnimation::new(
                360.0 * 3.0 / 4.0 * 3.0,
                1.0,
                (400, 300),
                (100.0, 60.0, 100.0),
            ))),
        ),
    ]);

    canvas
        .get_surface()
        .fill(&ColorU8::new_gray_alpha(128, 255))
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

        sprite_group.update().unwrap();
        canvas
            .get_surface()
            .blit(background_surf.as_ref(), (0, 0), BlendMode::Blend)
            .unwrap();
        //canvas.get_surface().fill(&ColorU8::new_gray_alpha(128, 255)).unwrap();
        sprite_group.draw(canvas.get_surface()).unwrap();
        canvas.update().unwrap();
    }
}
