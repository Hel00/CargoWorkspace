use windows_sys::{ core::*, Win32::{UI::WindowsAndMessaging::*, System::{Threading::{PROCESS_ALL_ACCESS, OpenProcess}, Diagnostics::Debug::ReadProcessMemory}} };
use std::{ ptr::{self, null}, ffi::c_void, ffi::CString };

fn main()
{
    let game_name = CString::new("Ring0 Atomic 7.4").unwrap();

    unsafe
    {
        let mut pid: u32 = 0;
        let mut status: [u8; 4] = [0, 0, 0, 0];

        let hwnd = FindWindowA(null(), game_name.as_ptr() as *const u8);

        let wtpid = GetWindowThreadProcessId(hwnd, &mut pid as *mut u32);

        let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid as u32);

        ReadProcessMemory(
            handle,
            0x07C7C388 as *const c_void, // should read 3_i32
            &mut status as *mut _ as *mut c_void,
            status.len(),
            0 as *mut usize
        );

        println!("status is {:?}", status);
    }
}
