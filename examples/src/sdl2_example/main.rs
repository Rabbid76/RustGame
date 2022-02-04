use rust_game::canvas::Canvas;
use rust_game::color::ColorU8;
use rust_game::context::Context;
use rust_game::events::Event;
use rust_game::keys::KeyCode;
use rust_game::rectangle::Rect;
use rust_game::surface::{BlendMode, Surface};
use rust_game_sdl2::context::Sdl2Context;
use std::error::Error;

pub struct HypotrochoidAnimation {
    frame: u32,
    center: (i32, i32),
}

impl HypotrochoidAnimation {
    pub fn new(frame: u32, center: (i32, i32)) -> HypotrochoidAnimation {
        HypotrochoidAnimation { frame, center }
    }

    pub fn update_rect(&mut self, rect: &mut Rect) {
        let t = (self.frame as f64).to_radians();
        self.frame += 1;
        let (a, b, h) = (100.0, 60.0, 100.0);
        let x = self.center.0 as f64 + (a - b) * t.cos() + h * ((a - b) / b * t).cos();
        let y = self.center.1 as f64 + (a - b) * t.sin() - h * ((a - b) / b * t).sin();
        rect.set_center((x.round() as i32, y.round() as i32))
    }
}

pub struct Sprite {
    animation: HypotrochoidAnimation,
    image: Box<dyn Surface>,
    rect: Rect,
}

impl Sprite {
    pub fn new(animation: HypotrochoidAnimation, image: Box<dyn Surface>) -> Sprite {
        let rect = image.get_rect();
        Sprite {
            animation,
            image,
            rect: rect,
        }
    }
    pub fn update(&mut self) {
        self.animation.update_rect(&mut self.rect);
    }
    pub fn draw(&self, canvas: &mut Box<dyn Canvas>) -> Result<(), Box<dyn Error>> {
        canvas.get_surface().blit(
            self.image.as_ref(),
            self.rect.get_top_left(),
            BlendMode::Blend,
        )
    }
}

pub fn main() {
    let context = Sdl2Context::new().unwrap();
    let mut canvas = context.new_canvas().unwrap();
    let mut events = context.events().unwrap();
    let time = context.time().unwrap();
    let mut clock = time.new_clock();
    let draw = context.draw().unwrap();
    let mut test_surface = context.new_surface_alpha(50, 50).unwrap();
    draw.circle(
        test_surface.as_mut(),
        &ColorU8::new_rgba(255, 255, 255, 255),
        (25, 25),
        25,
    )
    .unwrap();

    let mut sprites = vec![
        Sprite::new(
            HypotrochoidAnimation::new(0, (400, 300)),
            test_surface.clone().unwrap(),
        ),
        Sprite::new(
            HypotrochoidAnimation::new(360 * 3 / 4, (400, 300)),
            test_surface.clone().unwrap(),
        ),
        Sprite::new(
            HypotrochoidAnimation::new(360 * 3 / 4 * 2, (400, 300)),
            test_surface.clone().unwrap(),
        ),
        Sprite::new(
            HypotrochoidAnimation::new(360 * 3 / 4 * 3, (400, 300)),
            test_surface.clone().unwrap(),
        ),
    ];
    let mut frame = 0;
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
            .fill(&ColorU8::from_hsl((frame as u16 + 1) % 360, 50, 20))
            .unwrap();
        for sprite in &sprites {
            sprite.draw(&mut canvas).unwrap();
        }
        canvas.update().unwrap();
        frame += 1
    }
}
