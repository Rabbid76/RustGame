use crate::color::Color;
use crate::rectangle::Rect;
use crate::surface::Surface;
use std::error::Error;
use std::ops::Range;

pub trait Draw {
    fn rectangle(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        rectangle: Rect,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>>;

    fn circle(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        radius: i32,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>>;

    fn ellipse(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        size: (i32, i32),
        angle: f32,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>>;

    fn arc(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        radius: i32,
        arc_angle: Range<f32>,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>>;

    fn elliptical_arc(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        size: (i32, i32),
        angle: f32,
        arc_angle: Range<f32>,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>>;

    fn polygon(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        points: &Vec<(i32, i32)>,
    ) -> Result<Rect, Box<dyn Error>>;

    fn line(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        start: (i32, i32),
        end: (i32, i32),
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>>;

    fn lines(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        close: bool,
        points: &Vec<(i32, i32)>,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>>;
}

#[cfg(test)]
mod test_surface {
    use super::*;

    struct DrawMock {}

    impl Draw for DrawMock {
        fn rectangle(&self, _: &mut dyn Surface, _: bool, _: &dyn Color, _: Rect, _: i32) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }

        fn circle(&self, _: &mut dyn Surface, _: bool, _: &dyn Color, _: (i32, i32), _: i32, _: i32) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }

        fn ellipse(
            &self,
            _: &mut dyn Surface,
            _: bool,
            _: &dyn Color,
            _: (i32, i32),
            _: (i32, i32),
            _: f32,
            _: i32,
        ) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }

        fn arc(
            &self,
            _: &mut dyn Surface,
            _: bool,
            _: &dyn Color,
            _: (i32, i32),
            _: i32,
            _: Range<f32>,
            _: i32,
        ) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }

        fn elliptical_arc(
            &self,
            _: &mut dyn Surface,
            _: bool,
            _: &dyn Color,
            _: (i32, i32),
            _: (i32, i32),
            _: f32,
            _: Range<f32>,
            _: i32,
        ) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }

        fn polygon(&self, _: &mut dyn Surface, _: bool, _: &dyn Color, _: &Vec<(i32, i32)>) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }

        fn line(&self, _: &mut dyn Surface, _: bool, _: &dyn Color, _: (i32, i32), _: (i32, i32), _: i32) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }

        fn lines(&self, _: &mut dyn Surface, _: bool, _: &dyn Color, _: bool, _: &Vec<(i32, i32)>, _: i32) -> Result<Rect, Box<dyn Error>> {
            Err("test")?
        }
    }
}
