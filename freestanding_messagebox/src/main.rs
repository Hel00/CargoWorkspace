#![no_std]
#![no_main]

#[link(name = "kernel32", kind = "dylib")] // kind = "static"
extern "C"
{
    fn MessageBoxA(hdc: i32, x: &str, y: &str, a: u8) -> u32;
}

#[no_mangle]
pub extern "C" fn main() -> isize
{
    unsafe { MessageBoxA(0, "x", "y", 0); }

    0
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> !
{
    loop {}
}

#[no_mangle]
pub extern "C" fn wWinMainCRTStartup() { main(); }
