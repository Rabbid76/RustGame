use crate::canvas::Canvas;
use std::error::Error;

pub trait Context {
    fn new_canvas(&self) -> Result<Box<dyn Canvas>, Box<dyn Error>>;
}

#[cfg(test)]
mod context_test {
    use super::*;

    struct ContextMock {}

    impl Context for ContextMock {
        fn new_canvas(&self) -> Result<Box<dyn Canvas>, Box<dyn Error>> {
            Err("test")?
        }
    }
}
