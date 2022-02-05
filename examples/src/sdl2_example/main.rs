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

pub struct ColorAnimation {
    frame: u32,
}

impl ColorAnimation {
    pub fn new(frame: u32) -> ColorAnimation {
        ColorAnimation { frame }
    }
    pub fn transform_image(&mut self, image: &mut dyn Surface) -> Box<dyn Surface> {
        let color = ColorU8::from_hsl((self.frame as u16 + 1) % 360, 100, 50);
        self.frame = (self.frame + 1) % 360;
        let mut color_surface = image.from_surface_and_color(&color).unwrap();
        
        let mut final_image = image.clone().unwrap();
        final_image
            .blit(color_surface.as_mut(), (0, 0), BlendMode::Mul)
            .unwrap();
        final_image
        
        //color_surface.blit(image, (0, 0), BlendMode::Mul).unwrap();
        //color_surface
    }
}

/*class HueColorAnimation:
    def __init__(self, start):
        self.frame = start
    def update_image(self, image):
        colouredImage = pygame.Surface(image.get_size())
        colouredImage.fill(color)
        finalImage = image.copy()
        finalImage.blit(colouredImage, (0, 0), special_flags = pygame.BLEND_MULT)
        return finalImage
*/

/* TODO
trait RectAnimation {

}
trait ImageAnimation {

}
trait Sprite {
    pub fn update(&mut self);
    pub fn rectangleAnimation() -> &dny RectAnimation;
    pub fn imageAnimation() -> &dny SpriteAnimation;
    pub fn rectangle(&self) -> Rect;
    pub fn image(&self) -> &dyn Sprite
}
*/
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
        self.rect_animation.update_rect(&mut self.rect);
    }
    pub fn draw(&mut self, canvas: &mut Box<dyn Canvas>) -> Result<(), Box<dyn Error>> {
        match &mut self.image_animation {
            Some(image_animation) => {
                let mut animated_image = image_animation.transform_image(self.image.as_mut());
                canvas.get_surface().blit(
                    animated_image.as_mut(),
                    self.rect.get_top_left(),
                    BlendMode::Blend,
                )
            }
            None => canvas.get_surface().blit(
                self.image.as_mut(),
                self.rect.get_top_left(),
                BlendMode::Blend,
            ),
        }
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
            Some(ColorAnimation::new(0)),
            HypotrochoidAnimation::new(0, (400, 300)),
            test_surface.clone().unwrap(),
        ),
        Sprite::new(
            Some(ColorAnimation::new(120)),
            HypotrochoidAnimation::new(360 * 3 / 4, (400, 300)),
            test_surface.clone().unwrap(),
        ),
        Sprite::new(
            Some(ColorAnimation::new(240)),
            HypotrochoidAnimation::new(360 * 3 / 4 * 2, (400, 300)),
            test_surface.clone().unwrap(),
        ),
        Sprite::new(
            Option::None,
            HypotrochoidAnimation::new(360 * 3 / 4 * 3, (400, 300)),
            test_surface.clone().unwrap(),
        ),
    ];
    //let mut frame = 0;
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
            .fill(&ColorU8::new_gray(64))
            .unwrap();
        for sprite in &mut sprites {
            sprite.draw(&mut canvas).unwrap();
        }
        canvas.update().unwrap();
        //frame += 1
    }
}
