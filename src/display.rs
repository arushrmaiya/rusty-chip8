use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::render::Canvas;
use sdl3::video::Window;
use std::time::Duration;
use sdl3::rect::Rect;

use crate::display;

const SCREEN_WIDTH:usize = 64;
const SCREEN_HEIGHT:usize = 32;
const SCALE:usize = 15;


pub fn main_sdl() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl3 demo", (SCREEN_WIDTH*SCALE) as u32, (SCREEN_HEIGHT*SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn draw_screen(canvas: &mut Canvas<Window>, display: &[bool]){
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for (i, px) in display.iter().enumerate(){
        if *px{
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let rect = Rect::new((x * 15) as i32, (y * 15) as i32, 15, 15);
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}