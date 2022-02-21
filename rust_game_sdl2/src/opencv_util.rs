use opencv::core;
use rust_game::surface::Surface;
use std::error::Error;

pub unsafe fn surface_to_opencv_mat(surface: &dyn Surface) -> Result<core::Mat, Box<dyn Error>> {
    let w = surface.get_width() as i32;
    let h = surface.get_height() as i32;
    let raw = surface.raw()?;
    let step = (w * 4) as usize;
    Ok(core::Mat::new_rows_cols_with_data(h, w, core::CV_8UC4, raw.as_ptr() as _, step)?)
}
