use core::arch::asm;
/*
pub fn input_byte(port: u16) -> u8
{
    let mut result: u8 = 0;

    unsafe
    {
        asm!
        {
            "inb {1:x}, {0}",
            out(reg_byte) result,
            in(reg) port,
        }
    }

    result
}
*/
/*
pub fn output_byte(port: u16, data: u8)
{
    unsafe
    {
        asm!
        {
            "outb {0}, {1:x}",
            in(reg_byte) data,
            in(reg) port,
        }
    }
}
*/
pub fn output_byte(port: u16, data: u8) {
    unsafe {
        asm!(
            "out dx, al", // This is the Intel-style assembly syntax for outb
            in("dx") port,
            in("al") data,
        );
    }
}
