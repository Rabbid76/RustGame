use opencv::core;
use rust_game::color::Color;
use rust_game::rectangle::Rect;
use rust_game::surface::{BlendMode, Surface};
use sdl2;
use std::any::Any;
use std::error::Error;

pub struct Sdl2Surface {
    pub surface: sdl2::surface::Surface<'static>,
}

impl Sdl2Surface {
    unsafe fn sdl2_surface_range_to_opencv_mat(
        sdl2_surface: &sdl2::surface::Surface<'static>,
        region: &Rect,
    ) -> Result<core::Mat, Box<dyn Error>> {
        let raw_surface = sdl2_surface.raw();
        let w = (*raw_surface).w as i32;
        let h = (*raw_surface).h as i32;
        let step = (w * 4) as usize;
        let mat = core::Mat::new_rows_cols_with_data(h, w, core::CV_8UC4, (*raw_surface).pixels, step)?;
        Ok(core::Mat::rowscols(
            &mat,
            &core::Range::new(region.get_top(), region.get_bottom())?,
            &core::Range::new(region.get_left(), region.get_right())?,
        )?)
    }

    pub fn new_alpha(size: (u32, u32)) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Surface {
            surface: sdl2::surface::Surface::new(size.0, size.1, sdl2::pixels::PixelFormatEnum::ABGR8888)?,
        }))
    }

    pub fn from_surface(surface: sdl2::surface::Surface<'static>) -> Sdl2Surface {
        Sdl2Surface { surface }
    }

    pub fn blend_sdl2_surface(
        dest_surface: &mut sdl2::surface::Surface<'static>,
        source_surface: &sdl2::surface::Surface<'static>,
        dest_rect: &Rect,
        src_rect: &Rect,
        blend_mode: BlendMode,
    ) -> Result<(), Box<dyn Error>> {
        match blend_mode {
            BlendMode::MultiplyRGBA => {
                Sdl2Surface::blend_sdl2_surface_opencv(dest_surface, source_surface, dest_rect, src_rect, blend_mode)
            }
            _ => Sdl2Surface::blend_sdl2_surface_sdl2(
                dest_surface,
                source_surface,
                Some(Sdl2Surface::rect_to_sdl2_rect(&dest_rect)),
                Some(Sdl2Surface::rect_to_sdl2_rect(&src_rect)),
                Sdl2Surface::blend_mode_to_sdl2_blend_mode(blend_mode),
            ),
        }
    }

    fn rect_to_sdl2_rect(rect: &Rect) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(rect.get_left(), rect.get_top(), rect.get_width() as u32, rect.get_height() as u32)
    }

    fn blend_mode_to_sdl2_blend_mode(blend_mode: BlendMode) -> sdl2::render::BlendMode {
        match blend_mode {
            BlendMode::None => sdl2::render::BlendMode::None,
            BlendMode::Blend => sdl2::render::BlendMode::Blend,
            BlendMode::Add => sdl2::render::BlendMode::Add,
            BlendMode::Modulate => sdl2::render::BlendMode::Mod,
            BlendMode::Multiply | BlendMode::MultiplyRGBA => sdl2::render::BlendMode::Mul,
            _ => sdl2::render::BlendMode::Invalid,
        }
    }

    fn blend_sdl2_surface_sdl2(
        dest_surface: &mut sdl2::surface::Surface<'static>,
        source_surface: &sdl2::surface::Surface<'static>,
        dest_rect: Option<sdl2::rect::Rect>,
        src_rect: Option<sdl2::rect::Rect>,
        blend_mode: sdl2::render::BlendMode,
    ) -> Result<(), Box<dyn Error>> {
        unsafe {
            let const_ptr = source_surface as *const sdl2::surface::Surface<'static>;
            let mut_ptr = const_ptr as *mut sdl2::surface::Surface<'static>;
            let surface = &mut *mut_ptr;
            surface.set_blend_mode(blend_mode)?;
            source_surface.blit(src_rect, dest_surface, dest_rect)?;
            surface.set_blend_mode(sdl2::render::BlendMode::Blend)?;
        }
        Ok(())
    }

    fn blend_sdl2_surface_opencv(
        dest_surface: &mut sdl2::surface::Surface<'static>,
        source_surface: &sdl2::surface::Surface<'static>,
        dest_rect: &Rect,
        src_rect: &Rect,
        blend_mode: BlendMode,
    ) -> Result<(), Box<dyn Error>> {
        unsafe {
            match blend_mode {
                BlendMode::MultiplyRGBA => {
                    let mut dest_mat = Sdl2Surface::sdl2_surface_range_to_opencv_mat(dest_surface, dest_rect)?;
                    let src_mat = Sdl2Surface::sdl2_surface_range_to_opencv_mat(source_surface, src_rect)?;
                    core::multiply(&dest_mat.clone(), &src_mat, &mut dest_mat, 1.0 / 255.0, -1)?;
                }
                _ => Err("not yet implemented")?,
            }
        }
        Ok(())
    }
}

impl Surface for Sdl2Surface {
    fn as_any<'a>(&'a self) -> &'a dyn Any {
        self
    }

    fn clone(&self) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        let mut surface_copy =
            sdl2::surface::Surface::new(self.surface.width(), self.surface.height(), sdl2::pixels::PixelFormatEnum::ABGR8888)?;
        self.surface.blit(Option::None, &mut surface_copy, Option::None)?;
        Ok(Box::new(Sdl2Surface { surface: surface_copy }))
    }

    fn modulate_surface_and_color(&self, color: &dyn Color) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        let mut color_surface = Sdl2Surface {
            surface: sdl2::surface::Surface::new(self.surface.width(), self.surface.height(), sdl2::pixels::PixelFormatEnum::ABGR8888)?,
        };
        color_surface.fill(color)?;
        //let mut final_surface = self.clone()?;
        //final_surface.blit(&color_surface, (0, 0), BlendMode::Multiply)?;
        //Ok(final_surface)
        color_surface.blit(self, (0, 0), BlendMode::MultiplyRGBA).unwrap();
        Ok(Box::new(color_surface))
    }

    fn get_width(&self) -> u32 {
        self.surface.width()
    }

    fn get_height(&self) -> u32 {
        self.surface.height()
    }

    fn get_size(&self) -> (u32, u32) {
        (self.surface.width(), self.surface.height())
    }

    fn get_rect(&self) -> Rect {
        Rect::new(0, 0, self.surface.width() as i32, self.surface.height() as i32)
    }

    fn raw(&self) -> Result<&[u8], Box<dyn Error>> {
        match self.surface.without_lock() {
            Some(data) => Ok(data),
            _ => Err("cannot retrieve raw data")?,
        }
    }

    fn raw_mut(&mut self) -> Result<&mut [u8], Box<dyn Error>> {
        match self.surface.without_lock_mut() {
            Some(data) => Ok(data),
            _ => Err("cannot retrieve raw data")?,
        }
    }

    fn fill(&mut self, color: &dyn Color) -> Result<(), Box<dyn Error>> {
        self.surface.fill_rect(Option::None, sdl2::pixels::Color::RGBA(color.r(), color.g(), color.b(), color.a()))?;
        Ok(())
    }

    fn blit(&mut self, source_surface: &dyn Surface, position: (i32, i32), blend_mode: BlendMode) -> Result<Rect, Box<dyn Error>> {
        let src_rect = source_surface.get_rect().move_(position.0, position.1);
        let dest_rect = self.get_rect().clip(&src_rect);
        let src_rect = src_rect.clip(&dest_rect).move_(-position.0, -position.1);
        let sdl2_source_surface: &Sdl2Surface = match source_surface.as_any().downcast_ref::<Sdl2Surface>() {
            Some(sdl2_source_surface) => sdl2_source_surface,
            None => Err("not a sdl2 surface")?,
        };
        Sdl2Surface::blend_sdl2_surface(&mut self.surface, &sdl2_source_surface.surface, &dest_rect, &src_rect, blend_mode)?;
        Ok(dest_rect)
    }
}

#[cfg(test)]
mod test_sdl2_surface {}
