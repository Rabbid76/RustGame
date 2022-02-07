use crate::color::Color;
use crate::rectangle::Rect;
use crate::surface::Surface;
use std::error::Error;

pub trait Draw {
    fn rectangle(
        &self,
        surface: &mut dyn Surface,
        color: &dyn Color,
        rectangle: Rect,
        width: i32,
    ) -> Result<Rect, Box<dyn Error>>;

    fn circle(
        &self,
        surface: &mut dyn Surface,
        color: &dyn Color,
        center: (i32, i32),
        radius: i32,
        width: i32,
    ) -> Result<Rect, Box<dyn Error>>;

    fn line(
        &self,
        surface: &mut dyn Surface,
        color: &dyn Color,
        start: (i32, i32),
        end: (i32, i32),
        width: i32,
    ) -> Result<Rect, Box<dyn Error>>;

    fn lines(
        &self,
        surface: &mut dyn Surface,
        color: &dyn Color,
        close: bool,
        points: &Vec<(i32, i32)>,
        width: i32,
    ) -> Result<Rect, Box<dyn Error>>;
}

#[cfg(test)]
mod test_surface {
    use super::*;

    struct DrawMock {}

    impl Draw for DrawMock {
        fn rectangle(
            &self,
            _: &mut dyn Surface,
            _: &dyn Color,
            _: Rect,
            _: i32,
        ) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }

        fn circle(
            &self,
            _: &mut dyn Surface,
            _: &dyn Color,
            _: (i32, i32),
            _: i32,
            _: i32,
        ) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }

        fn line(
            &self,
            _: &mut dyn Surface,
            _: &dyn Color,
            _: (i32, i32),
            _: (i32, i32),
            _: i32,
        ) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }

        fn lines(
            &self,
            _: &mut dyn Surface,
            _: &dyn Color,
            _: bool,
            _: &Vec<(i32, i32)>,
            _: i32,
        ) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }
    }
}
