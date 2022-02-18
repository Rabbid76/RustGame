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

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> ColorU8 {
        ColorU8 { rgba: [r, g, b, a] }
    }

    pub fn new_rgb(r: u8, g: u8, b: u8) -> ColorU8 {
        ColorU8 { rgba: [r, g, b, 255] }
    }

    pub fn new_gray(grayscale: u8) -> ColorU8 {
        ColorU8 {
            rgba: [grayscale, grayscale, grayscale, 255],
        }
    }

    pub fn new_gray_alpha(grayscale: u8, alpha: u8) -> ColorU8 {
        ColorU8 {
            rgba: [grayscale, grayscale, grayscale, alpha],
        }
    }

    // TODO: HSL/HSV color module

    fn f32_to_u8(c: f32) -> u8 {
        ((c * 255.0).round() as i32).clamp(0, 255) as u8
    }

    fn hue_to_rgb(h: f32) -> (f32, f32, f32) {
        (
            ((h * 6.0 - 3.0).abs() - 1.0).clamp(0.0, 1.0),
            (2.0 - (h * 6.0 - 2.0).abs()).clamp(0.0, 1.0),
            (2.0 - (h * 6.0 - 4.0).abs()).clamp(0.0, 1.0),
        )
    }

    pub fn from_hue(hue: u16) -> ColorU8 {
        let rgb = ColorRGBA::hue_to_rgb((hue % 360) as f32 / 360.0);
        ColorU8 {
            rgba: [
                ColorRGBA::f32_to_u8(rgb.0),
                ColorRGBA::f32_to_u8(rgb.1),
                ColorRGBA::f32_to_u8(rgb.2),
                255,
            ],
        }
    }

    pub fn from_hue_alpha(hue: u16, alpha: u8) -> ColorU8 {
        let rgb = ColorRGBA::hue_to_rgb((hue % 360) as f32 / 360.0);
        ColorU8 {
            rgba: [
                ColorRGBA::f32_to_u8(rgb.0),
                ColorRGBA::f32_to_u8(rgb.1),
                ColorRGBA::f32_to_u8(rgb.2),
                (alpha as f32 * 255.0 / 100.0).round() as u8,
            ],
        }
    }

    pub fn from_hsv(hue: u16, saturation: u8, value: u8) -> ColorU8 {
        let rgb = ColorRGBA::hue_to_rgb((hue % 360) as f32 / 360.0);
        let s = saturation as f32 / 100.0;
        let v = value as f32 / 100.0;
        ColorU8 {
            rgba: [
                ColorRGBA::f32_to_u8(((rgb.0 - 1.0) * s + 1.0) * v),
                ColorRGBA::f32_to_u8(((rgb.1 - 1.0) * s + 1.0) * v),
                ColorRGBA::f32_to_u8(((rgb.2 - 1.0) * s + 1.0) * v),
                255,
            ],
        }
    }

    pub fn from_hsv_alpha(hue: u16, saturation: u8, value: u8, alpha: u8) -> ColorU8 {
        let rgb = ColorRGBA::hue_to_rgb((hue % 360) as f32 / 360.0);
        let s = saturation as f32 / 100.0;
        let v = value as f32 / 100.0;
        ColorU8 {
            rgba: [
                ColorRGBA::f32_to_u8(((rgb.0 - 1.0) * s + 1.0) * v),
                ColorRGBA::f32_to_u8(((rgb.1 - 1.0) * s + 1.0) * v),
                ColorRGBA::f32_to_u8(((rgb.2 - 1.0) * s + 1.0) * v),
                (alpha as f32 * 255.0 / 100.0).round() as u8,
            ],
        }
    }

    pub fn from_hsl(hue: u16, saturation: u8, lightness: u8) -> ColorU8 {
        let rgb = ColorRGBA::hue_to_rgb((hue % 360) as f32 / 360.0);
        let s = saturation as f32 / 100.0;
        let l = lightness as f32 / 100.0;
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        ColorU8 {
            rgba: [
                ColorRGBA::f32_to_u8(((rgb.0 - 0.5) * c) + l),
                ColorRGBA::f32_to_u8(((rgb.1 - 0.5) * c) + l),
                ColorRGBA::f32_to_u8(((rgb.2 - 0.5) * c) + l),
                255,
            ],
        }
    }

    pub fn from_hsl_alpha(hue: u16, saturation: u8, lightness: u8, alpha: u8) -> ColorU8 {
        let rgb = ColorRGBA::hue_to_rgb((hue % 360) as f32 / 360.0);
        let s = saturation as f32 / 100.0;
        let l = lightness as f32 / 100.0;
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        ColorU8 {
            rgba: [
                ColorRGBA::f32_to_u8(((rgb.0 - 0.5) * c) + l),
                ColorRGBA::f32_to_u8(((rgb.1 - 0.5) * c) + l),
                ColorRGBA::f32_to_u8(((rgb.2 - 0.5) * c) + l),
                (alpha as f32 * 255.0 / 100.0).round() as u8,
            ],
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

    #[test]
    fn color_from_hue() {
        let color = ColorU8::from_hue(180);
        assert_eq!(ColorU8::new(0, 255, 255, 255), color);
    }

    #[test]
    fn color_from_hue_alpha() {
        let color = ColorU8::from_hue_alpha(180, 50);
        assert_eq!(ColorU8::new(0, 255, 255, 128), color);
    }

    #[test]
    fn color_from_hsv() {
        let color = ColorU8::from_hsv(180, 100, 50);
        assert_eq!(ColorU8::new(0, 128, 128, 255), color);
    }

    #[test]
    fn color_from_hsv_alpha() {
        let color = ColorU8::from_hsv_alpha(180, 100, 50, 50);
        assert_eq!(ColorU8::new(0, 128, 128, 128), color);
    }

    #[test]
    fn color_from_hsl() {
        let color = ColorU8::from_hsl(180, 50, 50);
        assert_eq!(ColorU8::new(64, 191, 191, 255), color);
    }

    #[test]
    fn color_from_hsl_alpha() {
        let color = ColorU8::from_hsl_alpha(180, 50, 50, 50);
        assert_eq!(ColorU8::new(64, 191, 191, 128), color);
    }
}
