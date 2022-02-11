use crate::surface::Sdl2Surface;
use image::io::Reader;
use image::{DynamicImage};
use rust_game::image::Image;
use rust_game::surface::Surface;
use sdl2;
use std::error::Error;
use std::path::Path;

//use crate::opencv_sdl2;
//use opencv;
//use opencv::prelude::MatTraitConst;
//use std::io::{self, Write};

pub struct Sdl2Image {}

impl Sdl2Image {
    pub fn new() -> Result<Box<dyn Image>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Image {}))
    }
}

impl Sdl2Image {
    pub fn load_image_to_raw(filename: &str) -> Result<(usize, usize, Vec<u8>), Box<dyn Error>> {
        let rgba_image = Reader::open(filename)?.decode()?.to_rgba8();
        Ok((
            rgba_image.width() as usize,
            rgba_image.height() as usize,
            DynamicImage::ImageRgba8(rgba_image).into_bytes(),
        ))
    }

    pub fn load_image(&self, path: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        let rgba_image = match path.to_str() {
            Some(file_path) => Sdl2Image::load_image_to_raw(file_path)?,
            _ => Err("cannot decode file path")?,
        };
        let mut sdl2_surface = sdl2::surface::Surface::new(
            rgba_image.0 as u32,
            rgba_image.1 as u32,
            sdl2::pixels::PixelFormatEnum::ABGR8888,
        )?;
        match sdl2_surface.without_lock_mut() {
            Some(data) => data.copy_from_slice(&rgba_image.2[0..(rgba_image.0 * rgba_image.1 * 4)]),
            _ => Err("surface data")?,
        }

        Ok(Box::new(Sdl2Surface::from_surface(sdl2_surface)))
    }

    /*
    pub fn load_sdl2(&self, path: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Surface::from_surface(
            sdl2::surface::Surface::load_bmp(path)?.convert(
                &sdl2::pixels::PixelFormat::try_from(sdl2::pixels::PixelFormatEnum::ABGR8888)?,
            )?,
        )))
    }
    */

    /*
    pub fn load_opencv(&self, path: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        let opencv_image = match path.to_str() {
            Some(file_path) => opencv::imgcodecs::imread(file_path, -1)?,
            _ => Err("cannot decode file path")?
        };
        let r = opencv_image.rows();
        let c = opencv_image.cols();
        let f = opencv_image.flags();
        println!("{}, {}, {:#08x}", c, r, f);
        io::stdout().flush()?;
        let sdl2_surface = sdl2::surface::Surface::new(
            c as u32,
            r as u32,
            sdl2::pixels::PixelFormatEnum::ABGR8888,
        )?;
        unsafe {
            let mut dest_mat = opencv_sdl2::sdl2_surface_to_opencv_mat(&sdl2_surface)?;
            opencv::imgproc::cvt_color(&opencv_image, &mut dest_mat, opencv::imgproc::COLOR_RGBA2BGRA, 4)?;
        }
        Ok(Box::new(Sdl2Surface::from_surface(sdl2_surface)))
    }
    */
}

impl Image for Sdl2Image {
    fn load(&self, path: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        self.load_image(path)
        //self.load_sdl2(path)
        //self.load_opencv(path)
    }
}
