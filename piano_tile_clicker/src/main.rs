// https://www.crazygames.com/game/piano-tiles-2-online

use winapi::shared::windef::HDC__;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use std::ptr::null_mut;
use std::{thread, time};

mod colour;
use enigo::*;

fn clear_console()
{
    print!("{esc}c", esc = 27 as char);
}

/*
COORDS

X   Y
620 470

715 470

800 470

900 470
*/

unsafe fn detect_tile(hdc: *mut HDC__, x: i32, y: i32) -> bool
{
    if GetPixel(hdc, x, y) == 0 { true  }
    else                        { false }
}

unsafe fn click_tile(hdc: *mut HDC__, enigo: &mut Enigo, x: i32, y: i32)
{
    if detect_tile(hdc, x, y) == true
    {
        enigo.mouse_move_to(x, y);
        enigo.mouse_down(MouseButton::Left);
        enigo.mouse_up(MouseButton::Left);
    }
}

macro_rules! tile_sensors
{
    ($hdc: expr, $enigo: expr, $($x: expr, $y: expr);+) =>
    {
        $(
            click_tile($hdc, $enigo, $x, $y);
        )*
    }
}

fn main()
{
    unsafe
    {
        let SLEEP_TIME = time::Duration::from_secs_f32(0.01);
        let hdc = GetDC(null_mut());
        let mut enigo: enigo::Enigo = Enigo::new();

        let mut cursor_location: (i32, i32);
        let mut colour_u32: u32;
        let mut colour: colour::Colour;

        loop
        {
            cursor_location = Enigo::mouse_location();

            colour_u32 = GetPixel(hdc,
                cursor_location.0,
                cursor_location.1);

            colour = colour::Colour::new (colour_u32);

            // DEBUG INFO

            /*clear_console();

            println!("Raw colour hex: {:#06X}", colour_u32);
            println!("");
    
            println!("R    G    B    A");
            println!("{:#04X} {:#04X} {:#04X} {:#04X}", colour.r, colour.g, colour.b, colour.a);
            println!("");

            println!("X: {} Y: {}", cursor_location.0, cursor_location.1);*/

            click_tile(hdc, &mut enigo, 620, 570);
            click_tile(hdc, &mut enigo, 715, 570);
            click_tile(hdc, &mut enigo, 800, 570);
            click_tile(hdc, &mut enigo, 900, 570);

            tile_sensors!(hdc, &mut enigo, 620, 570;
                                           715, 570;
                                           800, 570;
                                           900, 570
            );

            //thread::sleep(SLEEP_TIME);
        }
    }
}
