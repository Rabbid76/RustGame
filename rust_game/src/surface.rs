use crate::color::Color;
use crate::rectangle::Rect;
use std::any::Any;
use std::error::Error;

#[repr(i32)]
pub enum BlendMode {
    None,
    Blend,
    Add,
    Mod,
    Mul,
    Invalid,
}

pub trait Surface {
    fn as_any(&self) -> &dyn Any;
    fn clone(&self) -> Result<Box<dyn Surface>, Box<dyn Error>>;
    fn modulate_surface_and_color(
        &self,
        color: &dyn Color,
    ) -> Result<Box<dyn Surface>, Box<dyn Error>>;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_size(&self) -> (u32, u32);
    fn get_rect(&self) -> Rect;
    fn fill(&mut self, color: &dyn Color) -> Result<(), Box<dyn Error>>;
    fn blit(
        &mut self,
        source_surface: &dyn Surface,
        position: (i32, i32),
        blend_mode: BlendMode,
    ) -> Result<(), Box<dyn Error>>;
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
