use crate::color::{Color, ColorU8};
use crate::surface::Surface;
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
    fn fill(&mut self, color: &dyn Color) -> Result<(), Box<dyn Error>> {
        self.fill_color.set(color);
        Ok(())
    }
}
