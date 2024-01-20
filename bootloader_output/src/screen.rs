/*
uint32 cursorX = 0;
uint32 cursorY = 0;

const uint8 screenWidth = 80;
const uint8 screenHeight = 25;
const uint8 screenDepth = 2;

void clearLine( uint8 from, uint8 to )
{
  string videoMemory = (string) 0xB8000;

  for ( uint16 index = screenWidth * from * screenDepth;
        index < ( screenWidth * ( to + 1) * screenDepth );
        index++ )
  {
    videoMemory[ index++ ] = 0x00;
  }
}
*/

use crate::system::*;

static mut CURSOR_X: u8 = 0;
static mut CURSOR_Y: u8 = 0;

const SCREEN_WIDTH: u8 = 80;
const SCREEN_HEIGHT: u8 = 25;
const SCREEN_DEPTH: u8 = 2;

#[no_mangle]
#[inline(never)]
pub fn clear_line(from: u8, to: u8)
{
    let video_memory = (0xB8000) as *mut u8;

    for index in (SCREEN_WIDTH * from * SCREEN_DEPTH)..(SCREEN_WIDTH * (to + 1) * SCREEN_DEPTH)
    {
        //videoMemory[index as usize] = 0x00;
        unsafe { video_memory.offset(index as isize).write(0x00); }
    }
}
#[no_mangle]
#[inline(never)]
pub fn clear_screen()
{
    clear_line(0, SCREEN_HEIGHT - 1);
    unsafe
    {
        CURSOR_X = 0;
        CURSOR_Y = 0;
        update_cursor();
    }
}

#[no_mangle]
#[inline(never)]
pub fn update_cursor()
{
    let current_position: u32 = unsafe { CURSOR_Y * SCREEN_WIDTH + CURSOR_X } as u32;

    output_byte(0x3D4, 14);
    output_byte(0x3D5, (current_position >> 8) as u8);
    output_byte(0x3D4, 15);
    output_byte(0x3D5, current_position as u8);
}
