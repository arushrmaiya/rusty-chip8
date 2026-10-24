use std::{thread, env};

mod display;
mod input;

use display::{draw_screen, main_sdl};
use input::get_key;
use sdl3::event::Event::{self, Quit};

use sdl3::pixels::Color;
// use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;
const SCREEN_WIDTH:u32 = 64;
const SCREEN_HEIGHT:u32 = 32;
const SCALE:u32 = 15;

use rusty_chip8::State;

use std::fs::File;
use std::io::Read;

use crate::input::retCodes;

fn main() {
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

    let mut curr_state = State::new();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut rom = File::open(file_path).expect("Should have been able to read the file");;
    let mut buffer: Vec<u8> = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();

    curr_state.load_prog(&buffer);

    'gameloop: loop{
        let retCode:retCodes = get_key(&mut curr_state);
        // draw_screen();
        // let code: i32 = 0x0;
        match retCode{
            retCodes::Quit=>break 'gameloop,
            _=>()
        }

        for _ in 0..10{
            curr_state.cycle();
        }
        
        draw_screen(&mut canvas, curr_state.get_display());
        curr_state.tick_timers();

        thread::sleep(Duration::from_secs_f64(1.0 / 60.0))
    }

}
