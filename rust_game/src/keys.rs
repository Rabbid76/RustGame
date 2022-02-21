use keycodes;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum KeyCode {
    TAB = keycodes::KEY_TAB as i32,
    ESC = keycodes::KEY_ESCAPE as i32,
}

impl KeyCode {
    pub const fn key(c: char) -> i32 {
        c.to_ascii_lowercase() as i32
    }
}