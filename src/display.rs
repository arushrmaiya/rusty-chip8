use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;

const SCREEN_WIDTH:u32 = 64;
const SCREEN_HEIGHT:u32 = 32;
const SCALE:u32 = 15;


pub fn main_sdl() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl3 demo", SCREEN_WIDTH*SCALE, SCREEN_HEIGHT*SCALE)
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

pub fn draw_screen(){
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl3 demo", SCREEN_WIDTH*SCALE, SCREEN_HEIGHT*SCALE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 64, 255));
    canvas.present();

}