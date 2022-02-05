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
    pub fn new_alpha(width: u32, height: u32) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Surface {
            surface: sdl2::surface::Surface::new(
                width,
                height,
                sdl2::pixels::PixelFormatEnum::ABGR8888,
            )?,
        }))
    }

    pub fn from_surface(surface: sdl2::surface::Surface<'static>) -> Sdl2Surface {
        Sdl2Surface { surface }
    }

    pub fn blend_sdl2_surface(
        dest_surface: &mut sdl2::surface::Surface<'static>,
        source_surface: &sdl2::surface::Surface<'static>,
        position: (i32, i32),
        blend_mode: BlendMode,
    ) -> Result<(), Box<dyn Error>> {
        let dst_rect = if position == (0, 0) {
            Option::None
        } else {
            Option::Some(sdl2::rect::Rect::new(
                position.0,
                position.1,
                source_surface.width(),
                source_surface.height(),
            ))
        };
        unsafe {
            let const_ptr = source_surface as *const sdl2::surface::Surface<'static>;
            let mut_ptr = const_ptr as *mut sdl2::surface::Surface<'static>;
            let surface = &mut *mut_ptr;
            match blend_mode {
                BlendMode::None => surface.set_blend_mode(sdl2::render::BlendMode::None)?,
                BlendMode::Blend => surface.set_blend_mode(sdl2::render::BlendMode::Blend)?,
                BlendMode::Add => surface.set_blend_mode(sdl2::render::BlendMode::Add)?,
                BlendMode::Mod => surface.set_blend_mode(sdl2::render::BlendMode::Mod)?,
                BlendMode::Mul => surface.set_blend_mode(sdl2::render::BlendMode::Mul)?,
                _ => Err("invalid blend mode")?,
            }
            source_surface.blit(Option::None, dest_surface, dst_rect)?;
            surface.set_blend_mode(sdl2::render::BlendMode::Blend)?;
        }
        
        Ok(())
    }
}

impl Surface for Sdl2Surface {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone(&self) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        let mut surface_copy = sdl2::surface::Surface::new(
            self.surface.width(),
            self.surface.height(),
            sdl2::pixels::PixelFormatEnum::ABGR8888,
        )?;
        self.surface
            .blit(Option::None, &mut surface_copy, Option::None)?;
        Ok(Box::new(Sdl2Surface {
            surface: surface_copy,
        }))
    }

    fn from_surface_and_color(
        &self,
        color: &dyn Color,
    ) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        let mut surface_copy = sdl2::surface::Surface::new(
            self.surface.width(),
            self.surface.height(),
            sdl2::pixels::PixelFormatEnum::ABGR8888,
        )?;
        surface_copy.fill_rect(
            Option::None,
            sdl2::pixels::Color::RGBA(color.r(), color.g(), color.b(), color.a()),
        )?;
        Ok(Box::new(Sdl2Surface {
            surface: surface_copy,
        }))
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
        Rect::new(
            0,
            0,
            self.surface.width() as i32,
            self.surface.height() as i32,
        )
    }

    fn fill(&mut self, color: &dyn Color) -> Result<(), Box<dyn Error>> {
        self.surface.fill_rect(
            Option::None,
            sdl2::pixels::Color::RGBA(color.r(), color.g(), color.b(), color.a()),
        )?;
        Ok(())
    }

    fn blit(
        &mut self,
        source_surface: &dyn Surface,
        position: (i32, i32),
        blend_mode: BlendMode,
    ) -> Result<(), Box<dyn Error>> {
        let sdl2_source_surface: &Sdl2Surface =
            match source_surface.as_any().downcast_ref::<Sdl2Surface>() {
                Some(sdl2_source_surface) => sdl2_source_surface,
                None => Err("not a sdl2 surface")?,
            };
        Sdl2Surface::blend_sdl2_surface(
            &mut self.surface,
            &sdl2_source_surface.surface,
            position,
            blend_mode,
        )
    }
}

#[cfg(test)]
mod test_sdl2_surface {}
