use rust_game::color::Color;
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
        match blend_mode {
            BlendMode::None => dest_surface.set_blend_mode(sdl2::render::BlendMode::None)?,
            BlendMode::Blend => dest_surface.set_blend_mode(sdl2::render::BlendMode::Blend)?,
            BlendMode::Add => dest_surface.set_blend_mode(sdl2::render::BlendMode::Add)?,
            BlendMode::Mod => dest_surface.set_blend_mode(sdl2::render::BlendMode::Mod)?,
            BlendMode::Mul => dest_surface.set_blend_mode(sdl2::render::BlendMode::Mul)?,
            _ => Err("invalid blend mode")?,
        }
        source_surface.blit(Option::None, dest_surface, dst_rect)?;
        Ok(())
    }
}

impl Surface for Sdl2Surface {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn fill(&mut self, color: &dyn Color) -> Result<(), Box<dyn Error>> {
        self.surface.fill_rect(
            Option::None,
            sdl2::pixels::Color::RGB(color.r(), color.g(), color.b()),
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
                None => Err("not a sdle surface")?,
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
