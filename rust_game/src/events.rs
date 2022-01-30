use crate::keys::KeyCode;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Quit {},
    KeyDown { key_code: i32, key: Option<KeyCode> },
    KeyUp { key_code: i32 },
}

pub trait Events {
    fn get(&mut self) -> Vec<Event>;
}

#[cfg(test)]
mod event_test {
    use super::*;

    struct EventMock {}

    impl Events for EventMock {
        fn get(&mut self) -> Vec<Event> {
            vec![]
        }
    }
}
