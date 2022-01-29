use num_derive::{FromPrimitive, ToPrimitive};
use keycodes;

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum KeyCode {
    TAB = keycodes::KEY_TAB as i32,
    ESC = keycodes::KEY_ESCAPE as i32,
}
