// https://www.crazygames.com/game/piano-tiles-2-online

use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use std::ptr::null_mut;

mod colour;
use enigo::*;

fn main()
{
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(500, 200);

    let cursor_location: (i32, i32) = Enigo::mouse_location();
    
    unsafe
    {
        let colour_u32 = GetPixel(GetDC(null_mut()), 500, 500);
        let colour = colour::Colour::new(colour_u32);

        println!("Raw colour hex: {:#06X}", colour_u32);
        println!("");

        println!("R    G    B    A");
        println!("{:#04X} {:#04X} {:#04X} {:#04X}", colour.r, colour.g, colour.b, colour.a);
        println!("");

        println!("X: {} Y: {}", cursor_location.0, cursor_location.1);
    }
}
