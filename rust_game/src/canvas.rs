use crate::surface::Surface;
use std::error::Error;

pub trait Canvas {
    fn get_surface<'a>(&'a mut self) -> &'a mut dyn Surface;
    fn update(&mut self) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
mod canvas_test {
    use super::*;
    use crate::color::{Color, ColorU8};
    use crate::test::SurfaceMock;

    struct CanvasMock {
        pub fill_color: ColorU8,
        pub canvas_color: ColorU8,
        pub canvas_surface: SurfaceMock,
    }

    impl CanvasMock {
        pub fn new() -> CanvasMock {
            CanvasMock {
                fill_color: ColorU8::new_gray(0),
                canvas_color: ColorU8::new_gray(0),
                canvas_surface: SurfaceMock::new(),
            }
        }

        fn fill(&mut self, color: &dyn Color) {
            self.fill_color.set(color);
        }
    }

    impl Canvas for CanvasMock {
        fn get_surface<'a>(&'a mut self) -> &'a mut dyn Surface {
            &mut self.canvas_surface
        }
        fn update(&mut self) -> Result<(), Box<dyn Error>> {
            self.canvas_color = self.fill_color.clone();
            Ok(())
        }
    }

    #[test]
    fn test_fill() {
        let mut canvas = CanvasMock::new();
        canvas.fill(&ColorU8::new_rgb(128, 32, 64));
        assert_eq!(ColorU8::new(128, 32, 64, 255), canvas.fill_color);
    }

    #[test]
    fn test_update() {
        let mut canvas = CanvasMock::new();
        canvas.fill(&ColorU8::new_rgb(128, 32, 64));
        canvas.update().unwrap();
        assert_eq!(ColorU8::new(128, 32, 64, 255), canvas.canvas_color);
    }
}
