use crate::window::Window;
use std::error::Error;

pub trait Context {
    fn new_window(&self) -> Result<Box<dyn Window>, Box<dyn Error>>;
}

#[cfg(test)]
mod context_test {
    use super::*;

    struct ContextMock {}

    impl Context for ContextMock {
        fn new_window(&self) -> Result<Box<dyn Window>, Box<dyn Error>> {
            Err("test")?
        }
    }
}
