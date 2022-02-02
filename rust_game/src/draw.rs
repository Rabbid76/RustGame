use crate::color::Color;
use crate::rectangle::Rect;
use crate::surface::Surface;
use std::error::Error;

pub trait Draw {
    fn circle(
        &self,
        surface: &mut dyn Surface,
        color: &dyn Color,
        center: (i32, i32),
        radius: i32,
    ) -> Result<Rect, Box<dyn Error>>;
}

#[cfg(test)]
mod test_surface {
    use super::*;

    struct DrawMock {}

    impl Draw for DrawMock {
        fn circle(
            &self,
            _: &mut dyn Surface,
            _: &dyn Color,
            _: (i32, i32),
            _: i32,
        ) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }
    }
}
