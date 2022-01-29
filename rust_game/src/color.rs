pub trait Color {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;
    fn set_r(&mut self, _: u8);
    fn set_g(&mut self, _: u8);
    fn set_b(&mut self, _: u8);
    fn set_a(&mut self, _: u8);
    fn set(&mut self, color: &dyn Color) {
        self.set_r(color.r());
        self.set_g(color.g());
        self.set_b(color.b());
        self.set_a(color.a());
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ColorRGBA<T> {
    rgba: [T; 4],
}

pub type ColorU8 = ColorRGBA<u8>;

impl ColorRGBA<u8> {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> ColorU8 {
        ColorU8 { rgba: [r, g, b, a] }
    }

    pub fn new_rgb(r: u8, g: u8, b: u8) -> ColorU8 {
        ColorU8 {
            rgba: [r, g, b, 255],
        }
    }

    pub fn new_gray(grayscale: u8) -> ColorU8 {
        ColorU8 {
            rgba: [grayscale, grayscale, grayscale, 255],
        }
    }
}

impl Color for ColorRGBA<u8> {
    fn r(&self) -> u8 {
        self.rgba[0]
    }
    fn g(&self) -> u8 {
        self.rgba[1]
    }
    fn b(&self) -> u8 {
        self.rgba[2]
    }
    fn a(&self) -> u8 {
        self.rgba[3]
    }
    fn set_r(&mut self, r: u8) {
        self.rgba[0] = r;
    }
    fn set_g(&mut self, g: u8) {
        self.rgba[1] = g;
    }
    fn set_b(&mut self, b: u8) {
        self.rgba[2] = b;
    }
    fn set_a(&mut self, a: u8) {
        self.rgba[3] = a;
    }
}

#[cfg(test)]
mod test_color_u8 {
    use super::*;

    #[test]
    fn color_new() {
        let color = ColorU8::new(32, 64, 96, 128);
        assert_eq!(32, color.r());
        assert_eq!(64, color.g());
        assert_eq!(96, color.b());
        assert_eq!(128, color.a());
    }

    #[test]
    fn color_new_rgb() {
        let color = ColorU8::new_rgb(32, 64, 128);
        assert_eq!(32, color.r());
        assert_eq!(64, color.g());
        assert_eq!(128, color.b());
        assert_eq!(255, color.a());
    }

    #[test]
    fn color_new_gray() {
        let color = ColorU8::new_gray(128);
        assert_eq!(128, color.r());
        assert_eq!(128, color.g());
        assert_eq!(128, color.b());
        assert_eq!(255, color.a());
    }

    #[test]
    fn color_set() {
        let mut color = ColorU8::new_gray(0);
        color.set_r(128);
        color.set_g(96);
        color.set_b(64);
        color.set_a(32);
        assert_eq!(ColorU8::new(128, 96, 64, 32), color);
    }

    #[test]
    fn color_set_rgb() {
        let mut color = ColorU8::new_gray(128);
        color.set(&ColorU8::new(128, 96, 64, 32));
        assert_eq!(ColorU8::new(128, 96, 64, 32), color);
    }
}
