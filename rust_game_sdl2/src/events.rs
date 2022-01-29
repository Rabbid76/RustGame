use crate::context::Sdl2Context;
use rust_game::events::{Event, Events};
use std::error::Error;
extern crate sdl2;

pub struct Sdl2Events {
    pub sdl_event_pump: sdl2::EventPump,
}

impl Sdl2Events {
    pub fn from(context: &Sdl2Context) -> Result<Sdl2Events, Box<dyn Error>> {
        Ok(Sdl2Events {
            sdl_event_pump: context.sdl_context.event_pump()?,
        })
    }
}

impl Events for Sdl2Events {
    fn get(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        for event in self.sdl_event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => events.push(Event::Quit {}),
                sdl2::event::Event::KeyDown {
                    keycode: Some(key), ..
                } => events.push(Event::KeyDown {
                    key_code: key as i32,
                }),
                sdl2::event::Event::KeyUp {
                    keycode: Some(key), ..
                } => events.push(Event::KeyDown {
                    key_code: key as i32,
                }),
                _ => {}
            }
        }
        events
    }
}
