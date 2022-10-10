use windows_sys::{ core::*, Win32::{UI::WindowsAndMessaging::*, System::{Threading::{PROCESS_ALL_ACCESS, OpenProcess}, Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory}}} };
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

    unsafe
    {
        let mut pid: u32 = 0;

        let hwnd = FindWindowA(null(), process_name.as_ptr() as *const u8);

        GetWindowThreadProcessId(hwnd, &mut pid as *mut u32);

        let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid as u32);

        let result = read_memory(handle, vec![0x1423B2EC8, 0x128, 0x10]) + 0xE0;

        write_memory(handle, result, [0, 0, 0, 0]);

        println!("result is {:#02X?}", result);
    }
}

/* read_memory(handle, vec![0x1423B2EC8, 0x128, 0x10, 0xE0]) = 3_i32 */

/*
import winim

proc getOffset(processHandle: HANDLE, offsets: seq[int64]): int64 =
  var
    value: int64
  for i in 0 .. offsets.len - 1:
    discard ReadProcessMemory(processHandle, cast[LPVOID](value + offsets[i]), LPVOID(addr(value)), SIZE_T(sizeof(value)), nil)
  return value

type
  InvadeProc = proc(base: clonglong, signType: cint) {.fastcall.}
  AddSoulProc = proc(base: clonglong, effect: cint) {.fastcall.}

let
  processHandle: HANDLE = GetCurrentProcess()
  BaseB: int64 = 0x144768E78
  addSoulBase: int64 = getOffset(processHandle, @[BaseB, 0x80, 0x1FA0])
  invade = cast[InvadeProc](REDACTED)
  addSoul = cast[AddSoulProc](0x1405A3310)

addSoul(addSoulBase, 42069)
*/

/*
println!("{:#02x}", 10);
println!("{:#0x}", 10);
println!("{:#x}", 10);
println!("{:#x?}", 10);
println!("{:#0x?}", 10);
println!("{:#02x?}", 10);
*/