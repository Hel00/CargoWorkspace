#![no_std]
#![no_main]

mod screen;
mod system;

use core::arch::*;

use screen::*;

#[no_mangle]
pub extern "C" fn Main() -> !
{
    clear_screen();
    loop {}
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> !
{
    loop {};
}
