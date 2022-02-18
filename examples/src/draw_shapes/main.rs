use rust_game::color::ColorU8;
use rust_game::context::Context;
use rust_game::events::Event;
use rust_game::keys::KeyCode;
use rust_game::surface::SurfaceBuilder;
use rust_game::rectangle::Rect;
use rust_game_sdl2::context::Sdl2Context;

pub fn main() {
    let context = Sdl2Context::new().unwrap();
    let mut canvas = context.new_canvas().unwrap();
    let draw = context.draw().unwrap();
    let mut events = context.events().unwrap();
    let time = context.time().unwrap();
    let mut clock = time.new_clock();
    let mut background_surf =
        Sdl2Context::new_surface_alpha(canvas.get_surface().get_size()).unwrap();
    background_surf
        .fill(&ColorU8::new_gray_alpha(128, 10))
        .unwrap();

    let mut angle = 0;
    let mut step = 1;
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

        canvas.get_surface().fill(&ColorU8::new_gray(64)).unwrap();
        draw.rectangle(canvas.get_surface(), true, &ColorU8::from_hue(0), Rect::new(50, 50, 50, 50), -1).unwrap();
        draw.circle(canvas.get_surface(), true, &ColorU8::from_hue(30), (175, 75), 25, -1).unwrap();
        draw.ellipse(canvas.get_surface(), true, &ColorU8::from_hue(60), (275, 75), (25, 15), angle as f32, -1).unwrap();
        draw.arc(canvas.get_surface(), true, &ColorU8::from_hue(120), (375, 75), 25, 0.0..angle as f32 ,-1).unwrap();
        draw.elliptical_arc(canvas.get_surface(), true, &ColorU8::from_hue(180), (475, 75), (25, 15), angle as f32, 0.0..angle as f32, -1).unwrap();
        draw.polygon(canvas.get_surface(), true, &ColorU8::from_hue(240), &vec![(550, 100), (600, 100), (575, 50)]).unwrap();
        draw.rectangle(canvas.get_surface(), true, &ColorU8::from_hue(0), Rect::new(50, 150, 50, 50), 3).unwrap();
        draw.circle(canvas.get_surface(), true, &ColorU8::from_hue(30), (175, 175), 25, 3).unwrap();
        draw.ellipse(canvas.get_surface(), true, &ColorU8::from_hue(60), (275, 175), (25, 15), angle as f32, 3).unwrap();
        draw.arc(canvas.get_surface(), true, &ColorU8::from_hue(120), (375, 175), 25, 0.0..angle as f32 ,3).unwrap();
        draw.elliptical_arc(canvas.get_surface(), true, &ColorU8::from_hue(180), (475, 175), (25, 15), angle as f32, 0.0..angle as f32, 3).unwrap();
        draw.lines(canvas.get_surface(), true, &ColorU8::from_hue(300), true, &vec![(650, 100), (700, 100), (675, 50)], 3).unwrap();
        draw.line(canvas.get_surface(), true, &ColorU8::from_hue(300), (650, 150), (700, 200), 3).unwrap();
        draw.line(canvas.get_surface(), true, &ColorU8::from_hue(300), (650, 200), (700, 150), 3).unwrap();
        canvas.update().unwrap();

        angle += step;
        if angle >= 360 || angle <= 0 {
            step *= -1;
        }
    }
}