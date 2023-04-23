/*
type
    ElfObject* = object
        data*: seq[ uint8 ]
        size*: uint32

proc newElfObject*( objectPath: static[string] ): ElfObject =

    var
        objectFile = open( objectPath, fmRead )
    
    result.size = cast[ uint32 ]( objectFile.getFileSize() )
    result.data = newSeq[ uint8 ]( result.size )

    discard objectFile.readBytes( result.data, 0, result.size )
    
    objectFile.close()

proc isELF*( this: var ElfObject ): bool =

    result = false

    if this.data[0] == 0x7F and
    this.data[1] == cast[ uint8 ]( 'E' ) and
    this.data[2] == cast[ uint8 ]( 'L' ) and
    this.data[3] == cast[ uint8 ]( 'F' ):

        result = true
*/

pub type ElfObject = Vec<u8>;

pub trait ElfObjectConstructor
{
    fn new(file_name: &'static str) -> Option<Self> where Self: Sized;
}

impl<'a> ElfObjectConstructor for ElfObject
{
    fn new(file_name: &'static str) -> Option<Self>
    {
        use std::fs::File;
        #[allow(unused)]
        use std::io::{Read, Seek, SeekFrom};

        let mut buffer: Vec<u8> = vec![];
        let mut elf_file = File::open(file_name).ok()?;

        elf_file.read(&mut buffer);

        Option::Some(buffer)
    }
}
