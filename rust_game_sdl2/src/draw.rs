use crate::surface::Sdl2Surface;
use opencv::core;
use opencv::imgproc;
use rust_game::color::Color;
use rust_game::draw::Draw;
use rust_game::rectangle::Rect;
use rust_game::surface::Surface;
use std::error::Error;

pub struct Sdl2Draw {}

impl Sdl2Draw {
    pub fn new() -> Result<Box<dyn Draw>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Draw {}))
    }
}

impl Draw for Sdl2Draw {
    fn circle(
        &self,
        surface: &mut dyn Surface,
        color: &dyn Color,
        center: (i32, i32),
        radius: i32,
    ) -> Result<Rect, Box<dyn Error>> {
        let sdl2_surface: &Sdl2Surface = match surface.as_any().downcast_ref::<Sdl2Surface>() {
            Some(sdl2_surface) => sdl2_surface,
            None => Err("not a sdl2 surface")?,
        };

        let raw_surface = sdl2_surface.surface.raw();
        unsafe {
            let w = (*raw_surface).w as i32;
            let h = (*raw_surface).h as i32;
            let step = (w * 4) as usize;
            let mut mat = core::Mat::new_rows_cols_with_data(
                w,
                h,
                core::CV_8UC4,
                (*raw_surface).pixels,
                step,
            )
            .unwrap();

            let center = core::Point {
                x: center.0,
                y: center.1,
            };
            let color = core::Scalar::from([
                color.r() as f64,
                color.g() as f64,
                color.b() as f64,
                color.a() as f64,
            ]);
            let thickness = -1;
            let line_type = 8;
            let shift = 0;
            imgproc::circle(&mut mat, center, radius, color, thickness, line_type, shift)?;
        }

        Ok(Rect::new(
            center.0 - radius as i32,
            center.1 - radius as i32,
            radius * 2,
            radius * 2,
        ))
    }
}
