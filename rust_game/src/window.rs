pub trait Window {}

#[cfg(test)]
mod window_test {
    use super::*;

    struct WindowMock {}

    impl Window for WindowMock {}
}
