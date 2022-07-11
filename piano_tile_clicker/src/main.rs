#[cfg(feature = "winuser")] pub mod winuser;

use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use std::ptr::null_mut;

mod colour;
use enigo::*;

fn main()
{
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(500, 200);
    unsafe
    {
        let colour_u32 = GetPixel(GetDC(null_mut()), 500, 500);
        println!("ColourRef: {:#X}", colour_u32);

        let colour = colour::Colour::new(colour_u32);

        println!("R: {:#X} G: {:#X} B: {:#X} A: {:#X}", colour.r, colour.g, colour.b, colour.a);
    }
}
