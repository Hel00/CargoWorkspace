/*
const MAP_CONST*: seq[seq[string]] = @[ # 34x8
    @[". . . . . . . . . . . . . . . . . ."],
    @[". . . . . . . . . . . . . . . . . ."],
    @[". . . . . . . . . . . . . . . . . ."],
    @[". . . . . . . . . . . . . . . . . ."],
    @[". . . . . . . . . . . . . . . . . ."],
    @[". . . . . . . . . . . . . . . . . ."],
    @[". . . . . . . . . . . . . . . . . ."],
    @[". . . . . . . . . . . . . . . . . ."],
    ]
 */

pub fn replace_index(s: &mut String, i: usize, c: char,)
{
    assert!(s.is_ascii());
    assert!(c.is_ascii());
    unsafe { s.as_bytes_mut()[i] = c as u8; }
}

use std::io::*;

pub const MAP: &[ &str ] = // 34x7
&[ 
    ". . . . . . . . . . . . . . . . . .",
    ". . . . . . . . . . . . . . . . . .",
    ". . . . . . . . . . . . . . . . . .",
    ". . . . . . . . . . . . . . . . . .",
    ". . . . . . . . . . . . . . . . . .",
    ". . . . . . . . . . . . . . . . . .",
    ". . . . . . . . . . . . . . . . . .",
    ". . . . . . . . . . . . . . . . . .",
];

macro_rules! clear_screen
{
    () =>
    {
        print!("{esc}c", esc = 27 as char);
    };
}

pub struct Map//< 'a >
{
    pub model: Vec< String >,
    pub height_index: usize,
    pub width_index: usize,
    //pub properties: Vec< &'a str >
}

impl/*< 'a >*/ Map//< 'a >
{
    pub fn new() -> Self
    {
        let mut map: Vec< String > = Vec::new();

        for m in MAP
        {
            map.push( m.to_string() );
        }

        let height = map.len() - 1;
        let width = map[0].len() - 1;

        Map { model: map, height_index: height, width_index: width }
    }

    pub fn update( &self )
    {
        clear_screen!();

        for chunk in &self.model
        {
            stdout().write( chunk.as_bytes() ).unwrap();
            stdout().write( b"\n" ).unwrap();
        }

        stdout().flush().unwrap();
    }
 
    pub fn test( &mut self )
    {
        let mut data = String::from("Ooga booga");
        
        replace_index( &mut data, 0, 'A' );

        println!("ASDASDA {}", data);
    }
}
