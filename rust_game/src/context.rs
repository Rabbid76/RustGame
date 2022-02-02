use crate::canvas::Canvas;
use crate::draw::Draw;
use crate::events::Events;
use crate::surface::Surface;
use crate::time::Time;
use std::error::Error;
use std::time::Instant;

pub trait Context {
    fn new_canvas(&self) -> Result<Box<dyn Canvas>, Box<dyn Error>>;
    fn events(&self) -> Result<Box<dyn Events>, Box<dyn Error>>;
    fn time(&self) -> Result<Box<dyn Time>, Box<dyn Error>>;
    fn new_surface_alpha(
        &self,
        width: u32,
        height: u32,
    ) -> Result<Box<dyn Surface>, Box<dyn Error>>;
    fn draw(&self) -> Result<Box<dyn Draw>, Box<dyn Error>>;
}

pub struct ContextData {
    pub initialization_time: Instant,
}

impl ContextData {
    pub fn new() -> ContextData {
        ContextData {
            initialization_time: Instant::now(),
        }
    }
}

#[cfg(test)]
mod context_test {
    use super::*;

    struct ContextMock {}

    impl Context for ContextMock {
        fn new_canvas(&self) -> Result<Box<dyn Canvas>, Box<dyn Error>> {
            Err("test")?
        }
        fn events(&self) -> Result<Box<dyn Events>, Box<dyn Error>> {
            Err("test")?
        }
        fn time(&self) -> Result<Box<dyn Time>, Box<dyn Error>> {
            Err("test")?
        }
        fn new_surface_alpha(&self, _: u32, _: u32) -> Result<Box<dyn Surface>, Box<dyn Error>> {
            Err("test")?
        }
        fn draw(&self) -> Result<Box<dyn Draw>, Box<dyn Error>> {
            Err("test")?
        }
    }
}
