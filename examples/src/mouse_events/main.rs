use rust_game::color::ColorU8;
use rust_game::context::Context;
use rust_game::events::Event;
use rust_game::keys::KeyCode;
use rust_game::rectangle::Rect;
use rust_game_sdl2::context::Sdl2Context;

pub fn main() {
    let context = Sdl2Context::new().unwrap();
    let mut canvas = context.new_canvas().unwrap();
    let draw = context.draw().unwrap();
    let mut events = context.events().unwrap();
    let time = context.time().unwrap();
    let mut clock = time.new_clock();

    let mut objects: Vec<(Rect, ColorU8, ColorU8, bool, bool)> = vec![
        (Rect::new(100, 100, 100, 100), ColorU8::from_hsl(0, 50, 50), ColorU8::from_hsl(0, 100, 50), false, false),
        (Rect::new(250, 100, 100, 100), ColorU8::from_hsl(60, 50, 50), ColorU8::from_hsl(60, 100, 50), false, false),
        (Rect::new(100, 250, 100, 100), ColorU8::from_hsl(120, 50, 50), ColorU8::from_hsl(120, 100, 50), false, false),
        (Rect::new(250, 250, 100, 100), ColorU8::from_hsl(240, 50, 50), ColorU8::from_hsl(240, 100, 50), false, false),
    ];

    'running: loop {
        let _ = clock.tick_frame_rate(100);
        let _ = time.get_ticks();

        for event in events.get().unwrap() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    key: Some(KeyCode::ESC), ..
                } => break 'running,
                Event::KeyDown { key_code, key } => {
                    println!("{}, {:?}", key_code, key)
                }
                Event::MouseMotion { pos, rel: _ } => {
                    //println!("position: ({}, {}), relative position: ({}, {})", pos.0, pos.1, rel.0, rel.1);
                    for mut object in &mut objects {
                        object.3 = object.0.collide_point(pos.0, pos.1);
                    }
                }
                Event::MouseButtonDown { pos, button } => {
                    println!("position: ({}, {}), button: {}", pos.0, pos.1, button);
                    for mut object in &mut objects {
                        if object.0.collide_point(pos.0, pos.1) {
                            object.4 = !object.4;
                        };
                    }
                }
                Event::MouseButtonUp { pos, button } => {
                    println!("position: ({}, {}), button: {}", pos.0, pos.1, button);
                }
                Event::MouseWheel { pos, which, flipped } => {
                    println!("position: ({}, {}), which: {}, flipped: {}", pos.0, pos.1, which, flipped);
                }
                _ => {}
            }
        }

        canvas.get_surface().fill(&ColorU8::new_gray(128)).unwrap();

        for object in objects.iter() {
            let color = if object.4 { object.2 } else { object.1 };
            draw.rectangle(canvas.get_surface(), true, &color, object.0.clone(), -1).unwrap();
            if object.3 {
                draw.rectangle(canvas.get_surface(), true, &ColorU8::new_gray(255), object.0.clone(), 3).unwrap();
            }
        }

        canvas.update().unwrap();
    }
}
