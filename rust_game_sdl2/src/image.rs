use crate::surface::Sdl2Surface;
use rust_game::image::Image;
use rust_game::surface::Surface;
use sdl2;
use std::error::Error;
use std::path::Path;

pub struct Sdl2Image {}

impl Sdl2Image {
    pub fn new() -> Result<Box<dyn Image>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Image {}))
    }
}

impl Image for Sdl2Image {
    // TODO load images using opencv
    fn load(&self, path: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Surface::from_surface(
            sdl2::surface::Surface::load_bmp(path)?.convert(
                &sdl2::pixels::PixelFormat::try_from(sdl2::pixels::PixelFormatEnum::ABGR8888)?,
            )?,
        )))
    }
}
