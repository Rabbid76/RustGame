use crate::color::{Color, ColorU8};
use crate::surface::{BlendMode, Surface};
use std::any::Any;
use std::error::Error;

pub struct SurfaceMock {
    pub fill_color: ColorU8,
}

impl SurfaceMock {
    pub fn new() -> SurfaceMock {
        SurfaceMock {
            fill_color: ColorU8::new_gray(0),
        }
    }
}

impl Surface for SurfaceMock {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn fill(&mut self, color: &dyn Color) -> Result<(), Box<dyn Error>> {
        self.fill_color.set(color);
        Ok(())
    }

    fn blit(&mut self, _: &dyn Surface, _: (i32, i32), _: BlendMode) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
