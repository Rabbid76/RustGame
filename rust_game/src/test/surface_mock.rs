use crate::color::{Color, ColorU8};
use crate::rectangle::Rect;
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
    fn as_any<'a>(&'a self) -> &'a dyn Any {
        self
    }

    fn clone(&self) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        Ok(Box::new(SurfaceMock {
            fill_color: self.fill_color,
        }))
    }

    fn modulate_surface_and_color(
        &self,
        color: &dyn Color,
    ) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        Ok(Box::new(SurfaceMock {
            fill_color: ColorU8::new_rgba(color.r(), color.g(), color.b(), color.a()),
        }))
    }

    fn get_width(&self) -> u32 {
        0
    }

    fn get_height(&self) -> u32 {
        0
    }

    fn get_size(&self) -> (u32, u32) {
        (0, 0)
    }

    fn get_rect(&self) -> Rect {
        Rect::new(0, 0, 0, 0)
    }

    fn fill(&mut self, color: &dyn Color) -> Result<(), Box<dyn Error>> {
        self.fill_color.set(color);
        Ok(())
    }

    fn blit(
        &mut self,
        _: &dyn Surface,
        _: (i32, i32),
        _: BlendMode,
    ) -> Result<Rect, Box<dyn Error>> {
        Ok(Rect::new(0, 0, 0, 0))
    }
}
