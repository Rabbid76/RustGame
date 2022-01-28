use rust_game::window::Window;

pub struct Sdl2Window {}

impl Sdl2Window {
    pub fn new() -> Sdl2Window {
        Sdl2Window {}
    }
}

impl Window for Sdl2Window {}
