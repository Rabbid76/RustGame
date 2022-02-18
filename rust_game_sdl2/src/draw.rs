use crate::opencv_sdl2;
use opencv::core;
use opencv::imgproc;
use opencv::types;
use rust_game::color::Color;
use rust_game::draw::Draw;
use rust_game::rectangle::Rect;
use rust_game::surface::Surface;
use std::error::Error;
use std::ops::Range;

pub struct Sdl2Draw {}

impl Sdl2Draw {
    pub fn new() -> Result<Box<dyn Draw>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Draw {}))
    }
}

impl Sdl2Draw {
    fn color_to_opencv_scalar(color: &dyn Color) -> core::Scalar {
        core::Scalar::from([
            color.r() as f64,
            color.g() as f64,
            color.b() as f64,
            color.a() as f64,
        ])
    }

    fn tuple_to_opencv_point(point: (i32, i32)) -> core::Point {
        core::Point {
            x: point.0,
            y: point.1,
        }
    }

    fn tuple_to_opencv_size(point: (i32, i32)) -> core::Size {
        core::Size {
            width: point.0,
            height: point.1,
        }
    }

    fn tuple_vec_to_opencv_vector_of_points(points: &Vec<(i32, i32)>) -> types::VectorOfPoint {
        points
            .into_iter()
            .map(|point| core::Point {
                x: point.0,
                y: point.1,
            })
            .collect()
    }

    fn rectangle_to_opencv_rectangle(rectangle: &Rect) -> core::Rect {
        core::Rect {
            x: rectangle.get_left(),
            y: rectangle.get_top(),
            width: rectangle.get_width(),
            height: rectangle.get_height(),
        }
    }

    fn tuple_vec_enclosing_rectangle(points: &Vec<(i32, i32)>) -> Rect {
        let min_x = points.into_iter().map(|point| point.0).min().expect("");
        let max_x = points.into_iter().map(|point| point.0).max().expect("");
        let min_y = points.into_iter().map(|point| point.1).min().expect("");
        let max_y = points.into_iter().map(|point| point.1).max().expect("");
        Rect::new_from_points((min_x, min_y), (max_x, max_y))
    }
}

impl Draw for Sdl2Draw {
    fn rectangle(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        rectangle: Rect,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>> {
        let sdl2_surface = opencv_sdl2::surface_to_sdl2_surface(surface)?;
        unsafe {
            let mut mat = opencv_sdl2::sdl2_surface_to_opencv_mat(&sdl2_surface.surface)?;
            let line_type = if antialias {
                imgproc::LINE_AA
            } else {
                imgproc::LINE_8
            };
            let shift = 0;
            imgproc::rectangle(
                &mut mat,
                Sdl2Draw::rectangle_to_opencv_rectangle(&rectangle),
                Sdl2Draw::color_to_opencv_scalar(color),
                line_width,
                line_type,
                shift,
            )?;
        }
        Ok(rectangle.clip(&surface.get_rect()))
    }

    fn circle(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        radius: i32,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>> {
        let sdl2_surface = opencv_sdl2::surface_to_sdl2_surface(surface)?;
        unsafe {
            let mut mat = opencv_sdl2::sdl2_surface_to_opencv_mat(&sdl2_surface.surface)?;
            let line_type = if antialias {
                imgproc::LINE_AA
            } else {
                imgproc::FILLED
            };
            let shift = 0;
            imgproc::circle(
                &mut mat,
                Sdl2Draw::tuple_to_opencv_point(center),
                radius,
                Sdl2Draw::color_to_opencv_scalar(color),
                line_width,
                line_type,
                shift,
            )?;
        }
        Ok(Rect::new(
            center.0 - radius as i32,
            center.1 - radius as i32,
            radius * 2,
            radius * 2,
        )
        .clip(&surface.get_rect()))
    }

    fn ellipse(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        size: (i32, i32),
        angle: f32,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>> {
        let sdl2_surface = opencv_sdl2::surface_to_sdl2_surface(surface)?;
        unsafe {
            let mut mat = opencv_sdl2::sdl2_surface_to_opencv_mat(&sdl2_surface.surface)?;
            let line_type = if antialias {
                imgproc::LINE_AA
            } else {
                imgproc::FILLED
            };
            let shift = 0;
            imgproc::ellipse(
                &mut mat,
                Sdl2Draw::tuple_to_opencv_point(center),
                Sdl2Draw::tuple_to_opencv_size(size),
                angle as f64, 
                0.0,
                360.0,
                Sdl2Draw::color_to_opencv_scalar(color),
                line_width,
                line_type,
                shift,
            )?;
        }
        let radius = size.0.max(size.1);
        Ok(Rect::new(
            center.0 - radius as i32,
            center.1 - radius as i32,
            radius * 2,
            radius * 2,
        )
        .clip(&surface.get_rect()))
    }

    fn arc(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        radius: i32,
        arc_angle: Range<f32>,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>> {
        let sdl2_surface = opencv_sdl2::surface_to_sdl2_surface(surface)?;
        unsafe {
            let mut mat = opencv_sdl2::sdl2_surface_to_opencv_mat(&sdl2_surface.surface)?;
            let line_type = if antialias {
                imgproc::LINE_AA
            } else {
                imgproc::FILLED
            };
            let shift = 0;
            imgproc::ellipse(
                &mut mat,
                Sdl2Draw::tuple_to_opencv_point(center),
                Sdl2Draw::tuple_to_opencv_size((radius, radius)),
                0.0, 
                arc_angle.start as f64,
                arc_angle.end as f64,
                Sdl2Draw::color_to_opencv_scalar(color),
                line_width,
                line_type,
                shift,
            )?;
        }
        Ok(Rect::new(
            center.0 - radius as i32,
            center.1 - radius as i32,
            radius * 2,
            radius * 2,
        )
        .clip(&surface.get_rect()))
    }

    fn elliptical_arc(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        size: (i32, i32),
        angle: f32,
        arc_angle: Range<f32>,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>> {
        let sdl2_surface = opencv_sdl2::surface_to_sdl2_surface(surface)?;
        unsafe {
            let mut mat = opencv_sdl2::sdl2_surface_to_opencv_mat(&sdl2_surface.surface)?;
            let line_type = if antialias {
                imgproc::LINE_AA
            } else {
                imgproc::FILLED
            };
            let shift = 0;
            imgproc::ellipse(
                &mut mat,
                Sdl2Draw::tuple_to_opencv_point(center),
                Sdl2Draw::tuple_to_opencv_size(size),
                angle as f64, 
                arc_angle.start as f64,
                arc_angle.end as f64,
                Sdl2Draw::color_to_opencv_scalar(color),
                line_width,
                line_type,
                shift,
            )?;
        }
        let radius = size.0.max(size.1);
        Ok(Rect::new(
            center.0 - radius as i32,
            center.1 - radius as i32,
            radius * 2,
            radius * 2,
        )
        .clip(&surface.get_rect()))
    }

    fn polygon(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        points: &Vec<(i32, i32)>,
    ) -> Result<Rect, Box<dyn Error>> {
        let sdl2_surface = opencv_sdl2::surface_to_sdl2_surface(surface)?;
        unsafe {
            let mut mat = opencv_sdl2::sdl2_surface_to_opencv_mat(&sdl2_surface.surface)?;
            let line_type = if antialias {
                imgproc::LINE_AA
            } else {
                imgproc::FILLED
            };
            let shift = 0;
            imgproc::fill_poly(
                &mut mat,
                &Sdl2Draw::tuple_vec_to_opencv_vector_of_points(points),
                Sdl2Draw::color_to_opencv_scalar(color),
                line_type,
                shift,
                Sdl2Draw::tuple_to_opencv_point((0, 0)),
            )?;
        }
        Ok(Sdl2Draw::tuple_vec_enclosing_rectangle(points).clip(&surface.get_rect()))
    }

    fn line(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        start: (i32, i32),
        end: (i32, i32),
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>> {
        let sdl2_surface = opencv_sdl2::surface_to_sdl2_surface(surface)?;
        unsafe {
            let mut mat = opencv_sdl2::sdl2_surface_to_opencv_mat(&sdl2_surface.surface)?;
            let line_type = if antialias {
                imgproc::LINE_AA
            } else {
                imgproc::LINE_8
            };
            let shift = 0;
            imgproc::line(
                &mut mat,
                Sdl2Draw::tuple_to_opencv_point(start),
                Sdl2Draw::tuple_to_opencv_point(end),
                Sdl2Draw::color_to_opencv_scalar(color),
                line_width,
                line_type,
                shift,
            )?;
        }
        Ok(Rect::new_from_points(start, end).clip(&surface.get_rect()))
    }

    fn lines(
        &self,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        close: bool,
        points: &Vec<(i32, i32)>,
        line_width: i32,
    ) -> Result<Rect, Box<dyn Error>> {
        let sdl2_surface = opencv_sdl2::surface_to_sdl2_surface(surface)?;
        unsafe {
            let mut mat = opencv_sdl2::sdl2_surface_to_opencv_mat(&sdl2_surface.surface)?;
            let line_type = if antialias {
                imgproc::LINE_AA
            } else {
                imgproc::LINE_8
            };
            let shift = 0;
            imgproc::polylines(
                &mut mat,
                &Sdl2Draw::tuple_vec_to_opencv_vector_of_points(points),
                close,
                Sdl2Draw::color_to_opencv_scalar(color),
                line_width,
                line_type,
                shift,
            )?;
        }
        Ok(Sdl2Draw::tuple_vec_enclosing_rectangle(points).clip(&surface.get_rect()))
    }
}
