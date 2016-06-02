// http://rosettacode.org/wiki/Animate_a_pendulum
extern crate sdl2;
extern crate sdl2_gfx;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2_gfx::primitives::DrawRenderer;

const BACKGROUND_COLOR: Color = Color::RGB(101, 208, 246);
const BLACK: Color = Color::RGB(0, 0, 0);
const YELLOW: Color = Color::RGB(255, 255, 0);

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut timer = ctx.timer().unwrap();

    let mut window = video_ctx.window("pendulum", 320, 280)
        .position_centered()
        .build()
        .unwrap();

    {
        window.show();
    }

    let mut renderer = window.renderer().build().unwrap();
    let mut events = ctx.event_pump().unwrap();

    let center_x = 160.0f32;
    let center_y = 30.0f32;
    let rotation_radius = 100.0f32;
    let pendulum_radius = 10i16;

    let mut angle = 0.0f32;

    'event: loop {
        let x = (center_x + rotation_radius * angle.cos()) as i16;
        let y = (center_y + rotation_radius * angle.sin().abs()) as i16;
        angle += 0.1;

        let _ = renderer.set_draw_color(BACKGROUND_COLOR);
        let _ = renderer.clear();

        let _ = renderer.hline(5, 315, center_y as i16, BLACK);
        let _ = renderer.filled_circle(x, y, pendulum_radius, YELLOW);
        let _ = renderer.line(center_x as i16,
                              center_y as i16,
                              x,
                              y,
                              BLACK);
        let _ = renderer.present();
        timer.delay(50);

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'event,
                _ => continue,
            }
        }
    }
}
