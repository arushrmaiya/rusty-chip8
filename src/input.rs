use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use rusty_chip8::State;

pub enum retCodes{
    Code(usize),
    Quit,
    NoneType
}
fn key_to_code(key: Keycode) -> Option<usize> {
    match key {
        Keycode::_1 => Some(0x1),
        Keycode::_2 => Some(0x2),
        Keycode::_3 => Some(0x3),
        Keycode::_4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _
        => None,
    }
}
pub fn get_key(curr_state: &mut State)->retCodes{
    let sdl_context = sdl3::init().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // let key_press = [false;16];
    for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    return retCodes::Quit
                },
                Event::KeyDown{keycode: Some(key), ..} => {
                    println!("{key:?} Down");
                    if let Some(code) = key_to_code(key){
                        curr_state.keypress(code, true);
                        return retCodes::Code(code)
                    }
                },
                Event::KeyUp{keycode: Some(key), ..} => {
                    println!("{key:?} Up");
                    if let Some(code) = key_to_code(key){
                        curr_state.keypress(code, false);
                        return retCodes::Code(code)
                    }
                },
                _ => {return retCodes::NoneType}
            }
        }
    
    retCodes::NoneType
}