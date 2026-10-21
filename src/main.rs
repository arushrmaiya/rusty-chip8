use std::thread;

mod display;
mod input;

use display::{draw_screen, main_sdl};
use input::get_key;
use sdl3::event::Event;

use sdl3::pixels::Color;
// use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;
const SCREEN_WIDTH:u32 = 64;
const SCREEN_HEIGHT:u32 = 32;
const SCALE:u32 = 15;

use rusty_chip8::State;

fn main() {
    // println!("Hello, world!");
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

    let curr_state = State::new();


    loop{
        let ok = get_key();
        // draw_screen();

    }

}
