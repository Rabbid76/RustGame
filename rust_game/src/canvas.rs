use crate::color::Color;

pub trait Canvas {
    fn update(&mut self);
    fn fill(&mut self, color: &dyn Color);
}

#[cfg(test)]
mod canvas_test {
    use super::*;
    use crate::color::ColorU8;

    struct CanvasMock {
        pub fill_color: ColorU8,
        pub canvas_color: ColorU8,
    }

    impl CanvasMock {
        pub fn new() -> CanvasMock {
            CanvasMock {
                fill_color: ColorU8::new_gray(0),
                canvas_color: ColorU8::new_gray(0),
            }
        }
    }

    impl Canvas for CanvasMock {
        fn update(&mut self) {
            self.canvas_color = self.fill_color.clone();
        }

        fn fill(&mut self, color: &dyn Color) {
            self.fill_color.set(color);
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
        canvas.update();
        assert_eq!(ColorU8::new(128, 32, 64, 255), canvas.canvas_color);
    }
}
