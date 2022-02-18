pub mod animation;

use crate::rectangle::Rect;
use crate::surface::{BlendMode, Surface};
use std::error::Error;

pub trait Sprite {
    fn image<'a>(&'a self) -> &'a dyn Surface;
    fn rect<'a>(&'a self) -> &'a Rect;
    fn set_rect(&mut self, rect: Rect);
    fn image_animation<'a>(&'a mut self) -> Option<(&'a mut dyn ImageAnimation, &'a dyn Surface)> {
        Option::None
    }
    fn rectangle_animation<'a>(&'a mut self) -> Option<(&'a mut dyn RectAnimation, &'a Rect)> {
        Option::None
    }
    fn kill(&mut self) {}
    fn is_killed(&self) -> bool {
        false
    }
    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct DefaultSprite {
    image: Box<dyn Surface>,
    rectangle: Rect,
    image_animation: Option<Box<dyn ImageAnimation>>,
    rectangle_animation: Option<Box<dyn RectAnimation>>,
    is_killed: bool,
}

impl DefaultSprite {
    pub fn new(image: Box<dyn Surface>, center: (i32, i32)) -> Box<dyn Sprite> {
        let size = image.get_size();
        Box::new(DefaultSprite {
            image,
            rectangle: Rect::new_center(center, (size.0 as i32, size.1 as i32)),
            image_animation: Option::None,
            rectangle_animation: Option::None,
            is_killed: false,
        })
    }

    pub fn new_animated(
        image: Box<dyn Surface>,
        image_animation: Option<Box<dyn ImageAnimation>>,
        rectangle_animation: Option<Box<dyn RectAnimation>>,
    ) -> Box<dyn Sprite> {
        let size = image.get_size();
        Box::new(DefaultSprite {
            image,
            rectangle: Rect::new_center((0, 0), (size.0 as i32, size.1 as i32)),
            image_animation,
            rectangle_animation,
            is_killed: false,
        })
    }
}

impl Sprite for DefaultSprite {
    fn image<'a>(&'a self) -> &'a dyn Surface {
        self.image.as_ref()
    }

    fn rect<'a>(&'a self) -> &'a Rect {
        &self.rectangle
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rectangle = rect;
    }

    fn image_animation<'a>(&'a mut self) -> Option<(&'a mut dyn ImageAnimation, &'a dyn Surface)> {
        match &mut self.image_animation {
            Some(animation) => Some((animation.as_mut(), self.image.as_ref())),
            _ => Option::None,
        }
    }

    fn rectangle_animation<'a>(&'a mut self) -> Option<(&'a mut dyn RectAnimation, &'a Rect)> {
        match &mut self.rectangle_animation {
            Some(animation) => Some((animation.as_mut(), &self.rectangle)),
            _ => Option::None,
        }
    }

    fn kill(&mut self) {
        self.is_killed = true;
    }

    fn is_killed(&self) -> bool {
        self.is_killed
    }
}

pub trait RectAnimation {
    fn update_rectangle(&mut self, rect: &Rect) -> Rect;
}

pub trait ImageAnimation {
    fn transform_image(&mut self, image: &dyn Surface) -> Box<dyn Surface>;
}

pub trait SpriteGroup {
    fn sprites<'a>(&'a self) -> &'a Vec<Box<dyn Sprite>>;
    fn sprites_mut<'a>(&'a mut self) -> &'a mut Vec<Box<dyn Sprite>>;

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        for sprite in self.sprites_mut() {
            let rectangle = match sprite.rectangle_animation() {
                Some((rectangle_animation, rectangle)) => Some(rectangle_animation.update_rectangle(rectangle)),
                _ => Option::None,
            };
            match rectangle {
                Some(rectangle) => sprite.set_rect(rectangle),
                _ => {}
            }
        }
        for sprite in self.sprites_mut() {
            sprite.update()?;
        }
        self.sprites_mut().retain(|sprite| !sprite.is_killed());
        Ok(())
    }

    fn draw(&mut self, surface: &mut dyn Surface) -> Result<(), Box<dyn Error>> {
        for sprite in self.sprites_mut() {
            let _ = match sprite.image_animation() {
                Some((image_animation, image)) => {
                    let animated_image = image_animation.transform_image(image);
                    surface.blit(animated_image.as_ref(), sprite.rect().get_top_left(), BlendMode::Blend)
                }
                None => surface.blit(sprite.image(), sprite.rect().get_top_left(), BlendMode::Blend),
            }?;
        }
        Ok(())
    }
}

pub struct Group {
    sprites: Vec<Box<dyn Sprite>>,
}

impl Group {
    pub fn new(sprites: Vec<Box<dyn Sprite>>) -> Box<dyn SpriteGroup> {
        Box::new(Group { sprites })
    }
}

impl SpriteGroup for Group {
    fn sprites<'a>(&'a self) -> &'a Vec<Box<dyn Sprite>> {
        &self.sprites
    }

    fn sprites_mut<'a>(&'a mut self) -> &'a mut Vec<Box<dyn Sprite>> {
        &mut self.sprites
    }
}
