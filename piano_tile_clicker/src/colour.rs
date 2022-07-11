pub struct Colour
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Colour
{
    pub fn new(rgba: u32) -> Colour
    {
        let a: u8 = ((rgba >> 24) & 0xff) as u8;
        let b: u8 = ((rgba >> 16) & 0xff) as u8;
        let g: u8 = ((rgba >> 08) & 0xff) as u8;
        let r: u8 = (rgba & 0xff) as u8;

        Colour { a: a, b: b, g: g, r: r }
    }
}
