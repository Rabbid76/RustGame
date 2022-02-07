use rust_game::color::{Color, ColorU8};
use rust_game::context::Context;
use rust_game::draw::Draw;
use rust_game::events::Event;
use rust_game::keys::KeyCode;
use rust_game::rectangle::Rect;
use rust_game::sprite::animation::{ColorAnimation, HypotrochoidAnimation, ToColor};
use rust_game::sprite::{DefaultSprite, Group};
use rust_game::surface::{BlendMode, Surface, SurfaceBuilder};
use rust_game_sdl2::context::Sdl2Context;
use std::error::Error;

pub struct FrameToHSVColor {}

impl ToColor<u32> for FrameToHSVColor {
    fn get_color(&mut self, frame: u32) -> Box<dyn Color> {
        Box::new(ColorU8::from_hsl((frame as u16 + 1) % 360, 100, 50))
    }
}

fn create_smiley(draw: &dyn Draw) -> Result<Box<dyn Surface>, Box<dyn Error>> {
    let mut surface = Sdl2Context::new_surface_alpha((50, 50)).unwrap();
    draw.circle(
        surface.as_mut(),
        &ColorU8::new_rgba(255, 255, 255, 255),
        (25, 25),
        25,
        -1,
    )
    .unwrap();
    draw.circle(
        surface.as_mut(),
        &ColorU8::new_rgba(0, 0, 0, 255),
        (15, 18),
        5,
        -1,
    )
    .unwrap();
    draw.circle(
        surface.as_mut(),
        &ColorU8::new_rgba(0, 0, 0, 255),
        (35, 18),
        5,
        -1,
    )
    .unwrap();
    draw.lines(
        surface.as_mut(),
        &ColorU8::new_rgba(0, 0, 0, 255),
        false,
        &vec![(15, 32), (20, 38), (30, 38), (35, 32)],
        3,
    )
    .unwrap();
    Ok(surface)
}

fn create_ghost(draw: &dyn Draw) -> Result<Box<dyn Surface>, Box<dyn Error>> {
    let mut surface = Sdl2Context::new_surface_alpha((50, 50)).unwrap();
    draw.circle(
        surface.as_mut(),
        &ColorU8::new_rgba(255, 255, 255, 255),
        (25, 25),
        25,
        -1,
    )
    .unwrap();
    draw.rectangle(
        surface.as_mut(),
        &ColorU8::new_rgba(255, 255, 255, 255),
        Rect::new(1, 25, 48, 18),
        -1,
    )
    .unwrap();
    draw.rectangle(
        surface.as_mut(),
        &ColorU8::new_rgba(0, 0, 0, 0),
        Rect::new(0, 43, 50, 7),
        -1,
    )
    .unwrap();
    for n in 0..4 {
        draw.circle(
            surface.as_mut(),
            &ColorU8::new_rgba(255, 255, 255, 255),
            (7 + n * 12, 44),
            6,
            -1,
        )
        .unwrap();
    }
    draw.circle(
        surface.as_mut(),
        &ColorU8::new_rgba(0, 0, 0, 255),
        (15, 18),
        5,
        -1,
    )
    .unwrap();
    draw.circle(
        surface.as_mut(),
        &ColorU8::new_rgba(0, 0, 0, 255),
        (35, 18),
        5,
        -1,
    )
    .unwrap();
    Ok(surface)
}

pub fn main() {
    let context = Sdl2Context::new().unwrap();
    let mut canvas = context.new_canvas().unwrap();
    let mut events = context.events().unwrap();
    let time = context.time().unwrap();
    let mut clock = time.new_clock();
    let draw = context.draw().unwrap();
    let mut background_surf =
        Sdl2Context::new_surface_alpha(canvas.get_surface().get_size()).unwrap();
    background_surf
        .fill(&ColorU8::new_gray_alpha(128, 10))
        .unwrap();
    let smiley_surface = create_smiley(draw.as_ref()).unwrap();
    let ghost_surface = create_ghost(draw.as_ref()).unwrap();

    let mut sprite_group = Group::new(vec![
        DefaultSprite::new_animated(
            smiley_surface.clone().unwrap(),
            Some(Box::new(ColorAnimation::new(
                0,
                Box::new(FrameToHSVColor {}),
            ))),
            Some(Box::new(HypotrochoidAnimation::new(
                0.0,
                1.0,
                (400, 300),
                (100.0, 60.0, 100.0),
            ))),
        ),
        DefaultSprite::new_animated(
            smiley_surface.clone().unwrap(),
            Some(Box::new(ColorAnimation::new(
                120,
                Box::new(FrameToHSVColor {}),
            ))),
            Some(Box::new(HypotrochoidAnimation::new(
                360.0 * 3.0 / 4.0,
                1.0,
                (400, 300),
                (100.0, 60.0, 100.0),
            ))),
        ),
        DefaultSprite::new_animated(
            smiley_surface.clone().unwrap(),
            Some(Box::new(ColorAnimation::new(
                240,
                Box::new(FrameToHSVColor {}),
            ))),
            Some(Box::new(HypotrochoidAnimation::new(
                360.0 * 3.0 / 4.0 * 2.0,
                1.0,
                (400, 300),
                (100.0, 60.0, 100.0),
            ))),
        ),
        DefaultSprite::new_animated(
            ghost_surface.clone().unwrap(),
            Option::None,
            Some(Box::new(HypotrochoidAnimation::new(
                360.0 * 3.0 / 4.0 * 3.0,
                1.0,
                (400, 300),
                (100.0, 60.0, 100.0),
            ))),
        ),
    ]);

    canvas.get_surface().fill(&ColorU8::new_gray_alpha(128, 255)).unwrap();
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
        sprite_group.draw(canvas.get_surface()).unwrap();
        canvas.update().unwrap();
    }
}
