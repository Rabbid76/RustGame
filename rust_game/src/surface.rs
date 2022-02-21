use crate::color::Color;
use crate::rectangle::Rect;
use std::any::Any;
use std::error::Error;

#[repr(i32)]
pub enum BlendMode {
    None,
    Blend,
    Add,
    Modulate,
    Multiply,
    MultiplyRGBA,
    Invalid,
}

pub trait Surface {
    fn as_any<'a>(&'a self) -> &'a dyn Any;
    fn clone(&self) -> Result<Box<dyn Surface>, Box<dyn Error>>;
    fn modulate_surface_and_color(&self, color: &dyn Color) -> Result<Box<dyn Surface>, Box<dyn Error>>;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_size(&self) -> (u32, u32);
    fn get_rect(&self) -> Rect;
    fn raw(&self) -> Result<&[u8], Box<dyn Error>>;
    fn raw_mut(&mut self) -> Result<&mut [u8], Box<dyn Error>>;
    fn fill(&mut self, color: &dyn Color) -> Result<(), Box<dyn Error>>;
    fn blit(&mut self, source_surface: &dyn Surface, position: (i32, i32), blend_mode: BlendMode) -> Result<Rect, Box<dyn Error>>;
}

pub trait SurfaceBuilder {
    fn new_surface_alpha(size: (u32, u32)) -> Result<Box<dyn Surface>, Box<dyn Error>>;
    fn new_surface_with_color(size: (u32, u32), color: &dyn Color) -> Result<Box<dyn Surface>, Box<dyn Error>>;
}

#[cfg(test)]
mod test_surface {
    use super::*;
    use crate::color::ColorU8;
    use crate::test::SurfaceMock;

    #[test]
    fn test_fill() {
        let mut surface = SurfaceMock::new();
        surface.fill(&ColorU8::new_rgb(128, 32, 64)).unwrap();
    }
}
