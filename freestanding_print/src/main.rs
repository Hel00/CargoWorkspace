#![no_std]
#![no_main]

use core::arch::asm;

#[no_mangle]
pub unsafe extern "C" fn write(file_descriptor: i32, stream: &str, size: isize)
{
    const SYSCALL_NUMBER_WRITE: i32 = 1;

    asm!
    {
        "syscall",
        in("rax") SYSCALL_NUMBER_WRITE,
        in("rdi") file_descriptor,
        in("rsi") stream.as_ptr(),
        in("rdx") size,
    };
}

#[no_mangle]
pub extern "C" fn main()
{
    unsafe { write(0, "Hey\n", 4); }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> !
{
    loop {}
}
