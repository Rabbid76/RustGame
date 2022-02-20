use rust_game::color::ColorU8;
use rust_game::context::Context;
use rust_game::draw::DrawUtility;
use rust_game::events::Event;
use rust_game::keys::KeyCode;
use rust_game::rectangle::Rect;
use rust_game::surface::{BlendMode, SurfaceBuilder};
use rust_game_sdl2::context::Sdl2Context;

pub fn main() {
    let context = Sdl2Context::new().unwrap();
    let mut canvas = context.new_canvas().unwrap();
    let draw = context.draw().unwrap();
    let mut events = context.events().unwrap();
    let time = context.time().unwrap();
    let mut clock = time.new_clock();

    let mut background_surf = Sdl2Context::new_surface_alpha(canvas.get_surface().get_size()).unwrap();
    let (w, h) = background_surf.get_size();
    let colors = vec![ColorU8::new_gray(96), ColorU8::new_gray(64)];
    let ts: i32 = 60;
    for x in 0..(w as i32 + ts - 1) / ts {
        for y in 0..(h as i32 + ts - 1) / ts {
            draw.rectangle(background_surf.as_mut(), false, &colors[((x + y) % 2) as usize], Rect::new(x * ts, y * ts, ts, ts), -1)
                .unwrap();
        }
    }

    let mut angle = 0;
    let mut step = 1;
    'running: loop {
        let _ = clock.tick_frame_rate(100);
        let _ = time.get_ticks();

        for event in events.get().unwrap() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    key: Some(KeyCode::ESC), ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.get_surface().blit(background_surf.as_ref(), (0, 0), BlendMode::Blend).unwrap();

        draw.rectangle(canvas.get_surface(), true, &ColorU8::from_hue(0), Rect::new(50, 50, 50, 50), -1).unwrap();
        draw.circle(canvas.get_surface(), true, &ColorU8::from_hue(30), (175, 75), 25, -1).unwrap();
        draw.ellipse(canvas.get_surface(), true, &ColorU8::from_hue(60), (275, 75), (25, 15), angle as f32, -1).unwrap();
        draw.arc(canvas.get_surface(), true, &ColorU8::from_hue(120), (375, 75), 25, 0.0..angle as f32, -1).unwrap();
        draw.elliptical_arc(canvas.get_surface(), true, &ColorU8::from_hue(180), (475, 75), (25, 15), angle as f32, 0.0..angle as f32, -1)
            .unwrap();
        draw.polygon(canvas.get_surface(), true, &ColorU8::from_hue(240), &vec![(550, 100), (600, 100), (575, 50)], -1).unwrap();

        draw.rectangle(canvas.get_surface(), true, &ColorU8::from_hue(0), Rect::new(50, 150, 50, 50), 3).unwrap();
        draw.circle(canvas.get_surface(), true, &ColorU8::from_hue(30), (175, 175), 25, 3).unwrap();
        draw.ellipse(canvas.get_surface(), true, &ColorU8::from_hue(60), (275, 175), (25, 15), angle as f32, 3).unwrap();
        draw.arc(canvas.get_surface(), true, &ColorU8::from_hue(120), (375, 175), 25, 0.0..angle as f32, 3).unwrap();
        draw.elliptical_arc(canvas.get_surface(), true, &ColorU8::from_hue(180), (475, 175), (25, 15), angle as f32, 0.0..angle as f32, 3)
            .unwrap();
        draw.polygon(canvas.get_surface(), true, &ColorU8::from_hue(240), &vec![(550, 200), (600, 200), (575, 150)], 3).unwrap();

        DrawUtility::blit_rectangle(
            &context,
            canvas.get_surface(),
            true,
            &ColorU8::from_hue_alpha(0, 25),
            Rect::new(50, 250, 50, 50),
            BlendMode::Blend,
        )
        .unwrap();
        DrawUtility::blit_circle(&context, canvas.get_surface(), true, &ColorU8::from_hue_alpha(30, 25), (175, 275), 25, BlendMode::Blend)
            .unwrap();
        DrawUtility::blit_ellipse(
            &context,
            canvas.get_surface(),
            true,
            &ColorU8::from_hue_alpha(60, 25),
            (275, 275),
            (25, 15),
            angle as f32,
            BlendMode::Blend,
        )
        .unwrap();
        DrawUtility::blit_arc(
            &context,
            canvas.get_surface(),
            true,
            &ColorU8::from_hue_alpha(120, 25),
            (375, 275),
            25,
            0.0..angle as f32,
            BlendMode::Blend,
        )
        .unwrap();
        DrawUtility::blit_elliptical_arc(
            &context,
            canvas.get_surface(),
            true,
            &ColorU8::from_hue_alpha(180, 25),
            (475, 275),
            (25, 15),
            angle as f32,
            0.0..angle as f32,
            BlendMode::Blend,
        )
        .unwrap();
        DrawUtility::blit_polygon(
            &context,
            canvas.get_surface(),
            true,
            &ColorU8::from_hue_alpha(240, 25),
            &vec![(550, 300), (600, 300), (575, 250)],
            BlendMode::Blend,
        )
        .unwrap();

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
