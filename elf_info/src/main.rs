
const DEBUG_BYTES: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];

mod elf_info;
mod elf_object;

use elf_info::*;
use elf_object::*;
use macros::*;

use crate::elf_object::ElfObject;

fn main()
{
    greeter_macro!();

    TEST!("println!()");

    let elf_object_bin = <ElfObject as ElfObjectConstructor>::new("TEST");
    
    let x = ElfIdentifier::new( DEBUG_BYTES );

    //println!("Hello, world! {}", x.abi_version);
    println!("Hello, world! {:?}", elf_object_bin);
}
