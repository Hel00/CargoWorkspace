/* MUST RUN AS ADMIN */

use windows_sys::{ core::*, Win32::{UI::WindowsAndMessaging::*, System::{Threading::{PROCESS_ALL_ACCESS, OpenProcess}, Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory}}} };
use std::{ ptr::{ null }, ffi::c_void, ffi::CString, time::Duration };

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

unsafe fn write_memory( handle: isize, address: u64, value: [u8; 4] )
{
    let mut buffer = value.clone();
    const BUFFER_SIZE: usize = 4;

    WriteProcessMemory(
        handle,
        address as *const c_void,
        &mut buffer as *mut _ as *mut c_void,
        BUFFER_SIZE,
        0 as *mut usize
    );
}

fn main()
{
    let process_name = CString::new("Battlefield 4").unwrap();
    const DELAY: Duration = Duration::new(5, 0);
    let mut pid: u32 = 0;

    unsafe
    {
        let hwnd = FindWindowA(null(), process_name.as_ptr() as *const u8);

        GetWindowThreadProcessId(hwnd, &mut pid as *mut u32);

        let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid as u32);

        let sight_recoil = read_memory(handle, vec![0x1423B2EC8, 0x128, 0x30, ]) + 0x430;
        let sight_spread = read_memory(handle, vec![0x1423B2EC8, 0x128, 0x30, ]) + 0x434;

        let hip_recoil = read_memory(handle, vec![0x1423B2EC8, 0x128, 0x30, ]) + 0x438;
        let hip_spread = read_memory(handle, vec![0x1423B2EC8, 0x128, 0x30, ]) + 0x43C;
        
        loop
        {
            write_memory(handle, sight_recoil, [0x00, 0x00, 0x00, 0x3F]);
            write_memory(handle, sight_spread, [0x00, 0x00, 0x00, 0x3F]);

            write_memory(handle, hip_recoil, [0x00, 0x00, 0x00, 0x3F]);
            write_memory(handle, hip_spread, [0x00, 0x00, 0x00, 0x3F]);

            std::thread::sleep( DELAY );
        }
    }
}
