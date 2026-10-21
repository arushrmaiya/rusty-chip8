use sdl3::event::Event;
use sdl3::keyboard::Keycode;

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
pub fn get_key()->Result<usize, ()>{
    let sdl_context = sdl3::init().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // let key_press = [false;16];
    for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    return Err(())
                },
                Event::KeyDown{keycode: Some(key), ..} => {
                    println!("{key:?}");
                    if let Some(code) = key_to_code(key){
                        return Ok(code)
                    }
                },
                _ => {return Err(())}
            }
        }
    
    Err(())
}