use crate::context::Sdl2Context;
use rust_game::events::{Event, Events};
use std::error::Error;
use std::sync::Arc;
extern crate sdl2;
use num_traits::FromPrimitive;

pub struct Sdl2Events {
    sdl_context: Arc<sdl2::Sdl>,
}

impl Sdl2Events {
    pub fn from(context: &Sdl2Context) -> Result<Sdl2Events, Box<dyn Error>> {
        Ok(Sdl2Events {
            sdl_context: context.sdl_context.clone(),
        })
    }

    /*fn sdl2_mousebutton_as_u8(button: sdl2::mouse::MouseButton) -> u8 {

    }*/
}

impl Events for Sdl2Events {
    fn get(&mut self) -> Result<Vec<Event>, Box<dyn Error>> {
        let mut events = Vec::new();
        for event in self.sdl_context.event_pump()?.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => events.push(Event::Quit {}),
                sdl2::event::Event::KeyDown { keycode: Some(key), .. } => events.push(Event::KeyDown {
                    key_code: key as i32,
                    key: FromPrimitive::from_i32(key as i32),
                }),
                sdl2::event::Event::KeyUp { keycode: Some(key), .. } => events.push(Event::KeyDown {
                    key_code: key as i32,
                    key: FromPrimitive::from_i32(key as i32),
                }),
                sdl2::event::Event::MouseMotion { x, y, xrel, yrel, .. } => events.push(Event::MouseMotion {
                    pos: (x, y),
                    rel: (xrel, yrel),
                }),
                sdl2::event::Event::MouseButtonDown { x, y, mouse_btn, .. } => events.push(Event::MouseButtonDown {
                    pos: (x, y),
                    button: mouse_btn as u8,
                }),
                sdl2::event::Event::MouseButtonUp { x, y, mouse_btn, .. } => events.push(Event::MouseButtonUp {
                    pos: (x, y),
                    button: mouse_btn as u8,
                }),
                sdl2::event::Event::MouseWheel {
                    x, y, which, direction, ..
                } => events.push(Event::MouseWheel {
                    pos: (x, y),
                    which: which,
                    flipped: direction == sdl2::mouse::MouseWheelDirection::Flipped,
                }),
                _ => {}
            }
        }
        Ok(events)
    }
}
