use rust_game::color::ColorU8;
use rust_game::context::Context;
use rust_game::events::Event;
use rust_game::keys::KeyCode;
use rust_game::surface::BlendMode;
use rust_game_sdl2::context::Sdl2Context;

pub fn main() {
    let context = Sdl2Context::new().unwrap();
    let mut canvas = context.new_canvas().unwrap();
    let mut events = context.events().unwrap();
    let time = context.time().unwrap();
    let mut clock = time.new_clock();
    let draw = context.draw().unwrap();
    let mut test_surface = context.new_surface_alpha(50, 50).unwrap();
    test_surface
        .fill(&ColorU8::new_rgba(255, 255, 0, 128))
        .unwrap();
    draw.circle(
        test_surface.as_mut(),
        &ColorU8::new_rgba(0, 0, 255, 255),
        (25, 25),
        25,
    )
    .unwrap();

    let center = (400.0, 300.0);
    let mut frame: u32 = 0;
    'running: loop {
        let _ = clock.tick_frame_rate(100);
        let _ = time.get_ticks();

        for event in events.get().unwrap() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    key: Some(KeyCode::ESC),
                    ..
                } => break 'running,
                Event::KeyDown { key_code, key } => {
                    println!("{}, {:?}", key_code, key)
                }
                _ => {}
            }
        }

        let t = (frame as f64 * 4.0).to_radians();
        let (a, b, h) = (100.0, 60.0, 100.0);
        let x = center.0 + (a - b) * t.cos() + h * ((a - b) / b * t).cos();
        let y = center.1 + (a - b) * t.sin() - h * ((a - b) / b * t).sin();
        let x = x.round() as i32;
        let y = y.round() as i32;

        canvas
            .get_surface()
            .fill(&ColorU8::from_hsl((frame as u16 + 1) % 360, 50, 20))
            .unwrap();
        canvas
            .get_surface()
            .blit(test_surface.as_ref(), (x - 25, y - 25), BlendMode::Blend)
            .unwrap();
        canvas.update().unwrap();
        frame += 1
    }
}
