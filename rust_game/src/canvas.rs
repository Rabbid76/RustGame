pub trait Canvas {
    fn update(&mut self);
}

#[cfg(test)]
mod canvas_test {
    use super::*;

    struct CanvasMock {}

    impl Canvas for CanvasMock {
        fn update(&mut self) {

        }
    }
}
