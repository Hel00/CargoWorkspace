/* MUST RUN AS ADMIN */

#![no_main]

#[link(name = "kernel32", kind = "dylib")]
extern
{
    fn ReadProcessMemory(handle: u32, read_from: u64, write_to: &mut u64, size: u64, idk: u64);
    fn WriteProcessMemory(handle: u32, read_from: u64, write_to: &mut u64, size: u64, idk: u64);
    fn OpenProcess(desired_access: u32, inherit_handle: bool, process_id: u32) -> u32;
    fn Sleep(miliseconds: u32);
}

#[link(name = "user32", kind = "dylib")]
extern
{
    fn FindWindowA(class_name: u64, window_name: *const u8) -> u32;
    fn GetWindowThreadProcessId(hwnd: u32, process_id: *mut u32) -> u32;
}

unsafe fn read_memory< const SIZE: usize >( handle: u32, offsets: [u64; SIZE] ) -> u64
{
    let mut buffer = 0_u64;
    const BUFFER_SIZE: u64 = 4;

    for item in offsets
    {
        ReadProcessMemory
        (
            handle,
            buffer + item,
            &mut buffer,
            BUFFER_SIZE,
            0
        );
    }

    buffer
}

unsafe fn write_memory( handle: u32, address: u64, value: u64 )
{
    let mut buffer = value;
    const BUFFER_SIZE: u64 = 4;

    WriteProcessMemory(
        handle,
        address,
        &mut buffer,
        BUFFER_SIZE,
        0
    );
}

const PROCESS_ALL_ACCESS: u32 = 2035711; //(0x000F0000L | 0x00100000L | 0xFFF); // 2035711

#[no_mangle]
pub extern "C" fn main()
{
    let process_name = "Battlefield 4\0";
    const DELAY: u32 = 2500;
    let mut pid: u32 = 0;
    const half_float: u64 = 0x3F000000;

    unsafe
    {
        let hwnd = FindWindowA(0, process_name.as_ptr() as *const u8);

        GetWindowThreadProcessId(hwnd, &mut pid as *mut u32);

        let handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid as u32);


        /*let mut result = read_memory(handle, [0x04D81420]);

        println!("Pid is {pid}");
        println!("Result is {result}");

        write_memory(handle, 0x04D81420, 69);

        result = read_memory(handle, [0x04D81420]);
        println!("Now it's is {result}");*/
        
        loop
        {
            let sight_recoil = read_memory(handle, [0x1423B2EC8, 0x128, 0x30, ]) + 0x430;
            let sight_spread = read_memory(handle, [0x1423B2EC8, 0x128, 0x30, ]) + 0x434;

            let hip_recoil = read_memory(handle, [0x1423B2EC8, 0x128, 0x30, ]) + 0x438;
            let hip_spread = read_memory(handle, [0x1423B2EC8, 0x128, 0x30, ]) + 0x43C;

            write_memory(handle, sight_recoil, half_float);
            write_memory(handle, sight_spread, half_float);

            write_memory(handle, hip_recoil, half_float);
            write_memory(handle, hip_spread, half_float);

            Sleep( DELAY );
        }
    }
}
