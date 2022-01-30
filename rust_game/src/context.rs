use crate::canvas::Canvas;
use crate::events::Events;
use crate::time::Time;
use std::error::Error;
use std::time::Instant;

pub trait Context {
    fn new_canvas(&self) -> Result<Box<dyn Canvas>, Box<dyn Error>>;
    fn events(&self) -> Result<Box<dyn Events>, Box<dyn Error>>;
    fn time(&self) -> Result<Box<dyn Time>, Box<dyn Error>>;
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
    }
}
