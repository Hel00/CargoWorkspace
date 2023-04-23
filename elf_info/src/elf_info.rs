pub struct ElfIdentifier
{
    pub elf_signature: [u8; 4],

    pub object_bitmode: u8, // 0 = None; 1 = 32 bit; 2 = 64 bit
    pub endianness: u8,     // 0 = None; 1 = Little; 2 = Big
    pub elf_version: u8,    // Always 1
    pub os_abi: u8,         // 0 = None/System V; 1 = HP-UX; 2 = NetBSD; 3 Linux
    pub abi_version: u8,    // Almost never used

    pub padding: [u8; 7]
}

pub enum BitLength
{
    _32(u32),
    _64(u64)
}

#[allow(unused)]
pub struct ElfHeader
{
    pub elf_identifier: ElfIdentifier,

    pub elf_type: u16,
    // Elf type
    // 
    // ET_NONE 0 = No file type
    // ET_REL  1 = Relocatable file type
    // ET_EXEC 2 = Executable file
    // ET_DYN  3 = Shared object type
    // ET_CORE 4 = Core file

    pub machine: u16,
    // Architecture
    // 
    // 0x03 = x86
    // 0x08 = MIPS
    // 0x28 = ARM
    // 0x3E = amd64
    // 0xB7 = ARMv8
    // 0xF3 = RISC-V

    pub version: u32,
    // Always 1

    pub entry: BitLength,
    // Entry point for executables
    // Or
    // Constructor address for shared libraries

    pub programHeaderOffset: BitLength,
    pub sectionHeaderOffset: BitLength,

    pub flags: u32,

    pub elfHeaderSize: u16,

    pub programHeaderEntrySize: u16,
    pub programHeaderAmount: u16,

    pub sectionHeaderEntrySize: u16,
    pub sectionHeaderAmount: u16,

    pub sectionHeaderStringIndex: u16,
}

impl ElfIdentifier
{
    pub fn new(elf_data: &[u8]) -> Self
    {
        Self
        {
            elf_signature: <[u8; 4]>::try_from(&elf_data[0..4]).unwrap(),

            object_bitmode: elf_data[4],
            endianness: elf_data[5],
            elf_version: elf_data[6],
            os_abi: elf_data[7],
            abi_version: elf_data[8],
        
            padding: <[u8; 7]>::try_from(&elf_data[10..17]).unwrap()
        }
    }
}
