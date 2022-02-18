use crate::color::Color;
use crate::rectangle::Rect;
use crate::sprite::{ImageAnimation, RectAnimation};
use crate::surface::Surface;

pub struct HypotrochoidAnimation {
    pos: f32,
    step: f32,
    center: (i32, i32),
    parameter: (f32, f32, f32),
}

impl HypotrochoidAnimation {
    pub fn new(start: f32, step: f32, center: (i32, i32), parameter: (f32, f32, f32)) -> HypotrochoidAnimation {
        HypotrochoidAnimation {
            pos: start,
            step,
            center,
            parameter,
        }
    }
}

impl RectAnimation for HypotrochoidAnimation {
    fn update_rectangle(&mut self, rect: &Rect) -> Rect {
        let t = self.pos.to_radians();
        self.pos += self.step;
        let (a, b, h) = self.parameter;
        let x = self.center.0 as f32 + (a - b) * t.cos() + h * ((a - b) / b * t).cos();
        let y = self.center.1 as f32 + (a - b) * t.sin() - h * ((a - b) / b * t).sin();
        Rect::new_center((x.round() as i32, y.round() as i32), rect.get_size())
    }
}

pub trait ToColor<T> {
    fn get_color(&mut self, frame: T) -> Box<dyn Color>;
}

pub struct ColorAnimation {
    frame: u32,
    frame_to_color: Box<dyn ToColor<u32>>,
}

impl ColorAnimation {
    pub fn new(frame: u32, frame_to_color: Box<dyn ToColor<u32>>) -> ColorAnimation {
        ColorAnimation { frame, frame_to_color }
    }
}

impl ImageAnimation for ColorAnimation {
    fn transform_image(&mut self, image: &dyn Surface) -> Box<dyn Surface> {
        let color = self.frame_to_color.as_mut().get_color(self.frame);
        self.frame = (self.frame + 1) % 360;
        image.modulate_surface_and_color(color.as_ref()).unwrap()
    }
}
