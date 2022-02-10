use crate::surface::Surface;
use std::error::Error;
use std::path::Path;

pub trait Image {
    fn load(&self, path: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>>;
}

#[cfg(test)]
mod context_test {
    use super::*;

    struct ImageMock {}

    impl Image for ImageMock {
        fn load(&self, _: &Path) -> Result<Box<dyn Surface>, Box<dyn Error>> {
            Err("test")?
        }
    }
}
