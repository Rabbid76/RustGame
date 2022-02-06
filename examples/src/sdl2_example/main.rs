use rust_game::canvas::Canvas;
use rust_game::color::{Color, ColorU8};
use rust_game::context::Context;
use rust_game::events::Event;
use rust_game::keys::KeyCode;
use rust_game::rectangle::Rect;
use rust_game::sprite::animation::{ColorAnimation, HypotrochoidAnimation, ToColor};
use rust_game::sprite::{ImageAnimation, RectAnimation};
use rust_game::surface::{BlendMode, Surface, SurfaceBuilder};
use rust_game_sdl2::context::Sdl2Context;
use std::error::Error;

pub struct FrameToHSVColor {}

impl ToColor<u32> for FrameToHSVColor {
    fn get_color(&mut self, frame: u32) -> Box<dyn Color> {
        Box::new(ColorU8::from_hsl((frame as u16 + 1) % 360, 100, 50))
    }
}

pub struct Sprite {
    image: Box<dyn Surface>,
    rect: Rect,
    image_animation: Option<ColorAnimation>,
    rect_animation: HypotrochoidAnimation,
}

impl Sprite {
    pub fn new(
        image_animation: Option<ColorAnimation>,
        rect_animation: HypotrochoidAnimation,
        image: Box<dyn Surface>,
    ) -> Sprite {
        let rect = image.get_rect();
        Sprite {
            image,
            rect,
            image_animation,
            rect_animation,
        }
    }
    pub fn update(&mut self) {
        self.rect = self.rect_animation.update_rectangle(&self.rect);
    }
    pub fn draw(&mut self, canvas: &mut Box<dyn Canvas>) -> Result<(), Box<dyn Error>> {
        let _ = match &mut self.image_animation {
            Some(image_animation) => {
                let animated_image = image_animation.transform_image(self.image.as_ref());
                canvas.get_surface().blit(
                    animated_image.as_ref(),
                    self.rect.get_top_left(),
                    BlendMode::Blend,
                )
            }
            None => canvas.get_surface().blit(
                self.image.as_ref(),
                self.rect.get_top_left(),
                BlendMode::Blend,
            ),
        }?;
        Ok(())
    }
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
    let mut test_surface = Sdl2Context::new_surface_alpha((50, 50)).unwrap();
    draw.circle(
        test_surface.as_mut(),
        &ColorU8::new_rgba(255, 255, 255, 255),
        (25, 25),
        25,
    )
    .unwrap();

    let mut sprites = vec![
        Sprite::new(
            Some(ColorAnimation::new(0, Box::new(FrameToHSVColor {}))),
            HypotrochoidAnimation::new(0, (400, 300), (100.0, 60.0, 100.0)),
            test_surface.clone().unwrap(),
        ),
        Sprite::new(
            Some(ColorAnimation::new(120, Box::new(FrameToHSVColor {}))),
            HypotrochoidAnimation::new(360 * 3 / 4, (400, 300), (100.0, 60.0, 100.0)),
            test_surface.clone().unwrap(),
        ),
        Sprite::new(
            Some(ColorAnimation::new(240, Box::new(FrameToHSVColor {}))),
            HypotrochoidAnimation::new(360 * 3 / 4 * 2, (400, 300), (100.0, 60.0, 100.0)),
            test_surface.clone().unwrap(),
        ),
        Sprite::new(
            Option::None,
            HypotrochoidAnimation::new(360 * 3 / 4 * 3, (400, 300), (100.0, 60.0, 100.0)),
            test_surface.clone().unwrap(),
        ),
    ];

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

        for sprite in &mut sprites {
            sprite.update();
        }

        canvas
            .get_surface()
            .blit(background_surf.as_ref(), (0, 0), BlendMode::Blend)
            .unwrap();
        for sprite in &mut sprites {
            sprite.draw(&mut canvas).unwrap();
        }
        canvas.update().unwrap();
    }
}
