pub mod animation;

use crate::rectangle::Rect;
use crate::surface::Surface;

pub trait Sprite {
    fn image<'a>(&'a self) -> &'a dyn Surface;
    fn rect<'a>(&'a self) -> &'a Rect;

    fn rect_mut<'a>(&'a mut self) -> Option<&'a mut Rect> {
        Option::None
    }
}

pub trait RectAnimation {
    fn update_rectangle(&mut self, rect: &Rect) -> Rect;
}

pub trait ImageAnimation {
    fn transform_image(&mut self, image: &dyn Surface) -> Box<dyn Surface>;
}
