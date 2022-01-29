use crate::canvas::Canvas;
use crate::events::Events;
use std::error::Error;

pub trait Context {
    fn new_canvas(&self) -> Result<Box<dyn Canvas>, Box<dyn Error>>;
    fn events(&self) -> Result<Box<dyn Events>, Box<dyn Error>>;
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
    }
}
