use rust_game::color::Color;
use rust_game::surface;
use sdl2;
use std::error::Error;

pub struct Sdl2Surface {
    pub surface: sdl2::surface::Surface<'static>,
}

impl Sdl2Surface {
    pub fn from_surface(surface: sdl2::surface::Surface<'static>) -> Sdl2Surface {
        Sdl2Surface { surface }
    }
}

impl surface::Surface for Sdl2Surface {
    fn fill(&mut self, color: &dyn Color) -> Result<(), Box<dyn Error>> {
        self.surface.fill_rect(
            Option::None,
            sdl2::pixels::Color::RGB(color.r(), color.g(), color.b()),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod test_sdl2_surface {}
