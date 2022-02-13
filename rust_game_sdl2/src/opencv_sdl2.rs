use crate::surface::Sdl2Surface;
use opencv::core;
use rust_game::rectangle::Rect;
use rust_game::surface::Surface;
use std::error::Error;

pub fn surface_to_sdl2_surface<'a>(
    surface: &'a dyn Surface,
) -> Result<&'a Sdl2Surface, Box<dyn Error>> {
    match surface.as_any().downcast_ref::<Sdl2Surface>() {
        Some(sdl2_surface) => Ok(sdl2_surface),
        None => Err("not a sdl2 surface")?,
    }
}

pub unsafe fn sdl2_surface_to_opencv_mat(
    sdl2_surface: &sdl2::surface::Surface<'static>,
) -> Result<core::Mat, Box<dyn Error>> {
    let raw_surface = sdl2_surface.raw();
    let w = (*raw_surface).w as i32;
    let h = (*raw_surface).h as i32;
    let step = (w * 4) as usize;
    Ok(core::Mat::new_rows_cols_with_data(
        w,
        h,
        core::CV_8UC4,
        (*raw_surface).pixels,
        step,
    )?)
}

pub unsafe fn sdl2_surface_range_to_opencv_mat(
    sdl2_surface: &sdl2::surface::Surface<'static>,
    region: &Rect,
) -> Result<core::Mat, Box<dyn Error>> {
    let raw_surface = sdl2_surface.raw();
    let w = (*raw_surface).w as i32;
    let h = (*raw_surface).h as i32;
    let step = (w * 4) as usize;
    let mat = core::Mat::new_rows_cols_with_data(w, h, core::CV_8UC4, (*raw_surface).pixels, step)?;
    Ok(core::Mat::rowscols(
        &mat,
        &core::Range::new(region.get_top(), region.get_bottom())?,
        &core::Range::new(region.get_left(), region.get_right())?,
    )?)
}
