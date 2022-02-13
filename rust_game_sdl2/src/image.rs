use crate::opencv_sdl2;
use crate::surface::Sdl2Surface;
use image::io::Reader;
use image::{DynamicImage, RgbaImage};
use resvg;
use rust_game::image::Image;
use rust_game::surface::Surface;
use sdl2;
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use usvg;

#[derive(Copy, Clone)]
struct PixelRGBA8888;

pub struct Sdl2Image;

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

    pub fn raw_to_surface(
        size: (u32, u32),
        src_data: &Vec<u8>,
    ) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        let mut sdl2_surface =
            sdl2::surface::Surface::new(size.0, size.1, sdl2::pixels::PixelFormatEnum::ABGR8888)?;
        match sdl2_surface.without_lock_mut() {
            Some(data) => data.copy_from_slice(src_data),
            _ => Err("surface data")?,
        }
        Ok(Box::new(Sdl2Surface::from_surface(sdl2_surface)))
    }

    pub fn load_image(&self, path: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        let rgba_image = match path.to_str() {
            Some(file_path) => Sdl2Image::load_image_to_raw(file_path)?,
            _ => Err("cannot decode file path")?,
        };
        Sdl2Image::raw_to_surface((rgba_image.0 as u32, rgba_image.1 as u32), &rgba_image.2)
    }

    pub fn load_svg(&self, path: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        let mut opt = usvg::Options::default();
        // Get file's absolute directory.
        opt.resources_dir = std::fs::canonicalize(path)
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()));
        opt.fontdb.load_system_fonts();
        let svg_data = std::fs::read(path)?;
        let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref())?;
        let pixmap_size = rtree.svg_node().size.to_screen_size();
        let (width, height) = (pixmap_size.width(), pixmap_size.height());
        let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
        resvg::render(
            &rtree,
            usvg::FitTo::Original,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        )
        .unwrap();
        Sdl2Image::raw_to_surface((width, height), &pixmap.data().to_vec())
    }

    fn save_image(&self, surface: &dyn Surface, path: &Path) -> Result<(), Box<dyn Error>> {
        let sdl2_surface = opencv_sdl2::surface_to_sdl2_surface(surface)?;
        let w = sdl2_surface.get_width();
        let h = sdl2_surface.get_height();
        match sdl2_surface.surface.without_lock() {
            Some(data) => match RgbaImage::from_raw(w, h, data.to_vec()) {
                Some(image) => image.save(path)?,
                _ => Err("cannot create image")?,
            },
            _ => Err("no image data")?,
        };
        Ok(())
    }
}

impl Image for Sdl2Image {
    fn load(&self, path: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        let extension = path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap()
            .to_ascii_lowercase();
        match extension.as_str() {
            "svg" => self.load_svg(path),
            _ => self.load_image(path),
        }
    }

    fn save(&self, surface: &dyn Surface, path: &Path) -> Result<(), Box<dyn Error>> {
        self.save_image(surface, path)
    }
}

/*
Alternative implementations for loading images:

SDL2:

````rust
use crate::opencv_sdl2;
```

```rust
impl Sdl2Image {
    pub fn load_sdl2(&self, path: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>> {
        Ok(Box::new(Sdl2Surface::from_surface(
            sdl2::surface::Surface::load_bmp(path)?.convert(
                &sdl2::pixels::PixelFormat::try_from(sdl2::pixels::PixelFormatEnum::ABGR8888)?,
            )?,
        )))
    }
}
```

OpenCV:

```rust
use opencv;
use opencv::prelude::MatTraitConst;
```

```rust
impl Sdl2Image {
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
}
```
*/
