compile with `cargo zigbuild --target x86_64-pc-windows-gnu`

then go into `target/<triple>/release/deps/` and compile the `.ll` file with `zig build-exe -target x86_64-windows-none -L"/home/hel/.wine/drive_c/windows/system32/" -dynamic -lkernel32 -luser32 -O ReleaseSmall -ffunction-sections --gc-sections`
