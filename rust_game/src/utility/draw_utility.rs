use crate::color::Color;
use crate::context::Context;
use crate::rectangle::Rect;
use crate::surface::{BlendMode, Surface};
use std::error::Error;
use std::ops::Range;

pub struct DrawUtility;

impl DrawUtility {
    fn tuple_vec_enclosing_rectangle(points: &Vec<(i32, i32)>) -> Rect {
        let min_x = points.into_iter().map(|point| point.0).min().expect("");
        let max_x = points.into_iter().map(|point| point.0).max().expect("");
        let min_y = points.into_iter().map(|point| point.1).min().expect("");
        let max_y = points.into_iter().map(|point| point.1).max().expect("");
        Rect::new_from_points((min_x, min_y), (max_x, max_y))
    }

    fn transform_points(points: &Vec<(i32, i32)>, offset: (i32, i32)) -> Vec<(i32, i32)> {
        points.into_iter().map(|point| (point.0 + offset.0, point.1 + offset.1)).collect()
    }

    pub fn blit_rectangle(
        context: &dyn Context,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        rectangle: Rect,
        blend_mode: BlendMode,
    ) -> Result<Rect, Box<dyn Error>> {
        let (w, h) = rectangle.get_size();
        let (x, y) = rectangle.get_top_left();
        let mut blend_surface = context.new_surface_alpha_from_size((w as u32, h as u32))?;
        let _ = context.draw()?.rectangle(blend_surface.as_mut(), antialias, color, rectangle.move_(-x, -y), -1)?;
        surface.blit(blend_surface.as_ref(), (x, y), blend_mode)?;
        Ok(rectangle.clip(&surface.get_rect()))
    }

    pub fn blit_circle(
        context: &dyn Context,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        radius: i32,
        blend_mode: BlendMode,
    ) -> Result<Rect, Box<dyn Error>> {
        let rectangle = Rect::new(center.0 - radius as i32, center.1 - radius as i32, radius * 2, radius * 2);
        let (w, h) = rectangle.get_size();
        let (x, y) = rectangle.get_top_left();
        let mut blend_surface = context.new_surface_alpha_from_size((w as u32, h as u32))?;
        let _ = context.draw()?.circle(blend_surface.as_mut(), antialias, color, (center.0 - x, center.1 - y), radius, -1)?;
        surface.blit(blend_surface.as_ref(), (x, y), blend_mode)?;
        Ok(rectangle.clip(&surface.get_rect()))
    }

    pub fn blit_ellipse(
        context: &dyn Context,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        size: (i32, i32),
        angle: f32,
        blend_mode: BlendMode,
    ) -> Result<Rect, Box<dyn Error>> {
        let radius = size.0.max(size.1);
        let rectangle = Rect::new(center.0 - radius as i32, center.1 - radius as i32, radius * 2, radius * 2);
        let (w, h) = rectangle.get_size();
        let (x, y) = rectangle.get_top_left();
        let mut blend_surface = context.new_surface_alpha_from_size((w as u32, h as u32))?;
        let _ = context.draw()?.ellipse(blend_surface.as_mut(), antialias, color, (center.0 - x, center.1 - y), size, angle, -1)?;
        surface.blit(blend_surface.as_ref(), (x, y), blend_mode)?;
        Ok(rectangle.clip(&surface.get_rect()))
    }

    pub fn blit_arc(
        context: &dyn Context,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        radius: i32,
        arc_angle: Range<f32>,
        blend_mode: BlendMode,
    ) -> Result<Rect, Box<dyn Error>> {
        let rectangle = Rect::new(center.0 - radius as i32, center.1 - radius as i32, radius * 2, radius * 2);
        let (w, h) = rectangle.get_size();
        let (x, y) = rectangle.get_top_left();
        let mut blend_surface = context.new_surface_alpha_from_size((w as u32, h as u32))?;
        let _ = context.draw()?.arc(blend_surface.as_mut(), antialias, color, (center.0 - x, center.1 - y), radius, arc_angle, -1)?;
        surface.blit(blend_surface.as_ref(), (x, y), blend_mode)?;
        Ok(rectangle.clip(&surface.get_rect()))
    }

    pub fn blit_elliptical_arc(
        context: &dyn Context,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        center: (i32, i32),
        size: (i32, i32),
        angle: f32,
        arc_angle: Range<f32>,
        blend_mode: BlendMode,
    ) -> Result<Rect, Box<dyn Error>> {
        let radius = size.0.max(size.1);
        let rectangle = Rect::new(center.0 - radius as i32, center.1 - radius as i32, radius * 2, radius * 2);
        let (w, h) = rectangle.get_size();
        let (x, y) = rectangle.get_top_left();
        let mut blend_surface = context.new_surface_alpha_from_size((w as u32, h as u32))?;
        let _ = context.draw()?.elliptical_arc(
            blend_surface.as_mut(),
            antialias,
            color,
            (center.0 - x, center.1 - y),
            size,
            angle,
            arc_angle,
            -1,
        )?;
        surface.blit(blend_surface.as_ref(), (x, y), blend_mode)?;
        Ok(rectangle.clip(&surface.get_rect()))
    }

    pub fn blit_polygon(
        context: &dyn Context,
        surface: &mut dyn Surface,
        antialias: bool,
        color: &dyn Color,
        points: &Vec<(i32, i32)>,
        blend_mode: BlendMode,
    ) -> Result<Rect, Box<dyn Error>> {
        let rectangle = DrawUtility::tuple_vec_enclosing_rectangle(points);
        let (w, h) = rectangle.get_size();
        let (x, y) = rectangle.get_top_left();
        let mut blend_surface = context.new_surface_alpha_from_size((w as u32, h as u32))?;
        let _ = context.draw()?.polygon(blend_surface.as_mut(), antialias, color, &DrawUtility::transform_points(points, (-x, -y)), -1)?;
        surface.blit(blend_surface.as_ref(), (x, y), blend_mode)?;
        Ok(rectangle.clip(&surface.get_rect()))
    }
}
