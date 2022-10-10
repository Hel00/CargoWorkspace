use windows_sys::{ core::*, Win32::{UI::WindowsAndMessaging::*, System::{Threading::{PROCESS_ALL_ACCESS, OpenProcess}, Diagnostics::Debug::ReadProcessMemory}} };
use std::{ ptr::{ null }, ffi::c_void, ffi::CString };

unsafe fn read_memory( handle: isize, offsets: Vec< u64 > ) -> u64
{
    let mut buffer = 0_u64;
    const BUFFER_SIZE: usize = 4;

    for item in offsets
    {
        ReadProcessMemory
        (
            handle,
            ( buffer + item ) as *const c_void,
            &mut buffer as *mut _ as *mut c_void,
            BUFFER_SIZE,
            0 as *mut usize
        );
    }

    buffer
}

fn main()
{
    let process_name = CString::new("Battlefield 4").unwrap();

    unsafe
    {
        let mut pid: u32 = 0;

        let hwnd = FindWindowA(null(), process_name.as_ptr() as *const u8);

        GetWindowThreadProcessId(hwnd, &mut pid as *mut u32);

        let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid as u32);

        let result = read_memory(handle, vec![0x1423B2EC8, 0x128, 0x10, 0xE0]);

        println!("result is {}", result);
    }
}

/* read_memory(handle, vec![0x1423B2EC8, 0x128, 0x10, 0xE0]) = 3_i32 */
