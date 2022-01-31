use crate::color::Color;
use std::error::Error;

pub trait Surface {
    fn fill(&mut self, color: &dyn Color) -> Result<(), Box<dyn Error>>;
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
