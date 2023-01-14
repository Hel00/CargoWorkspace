mod map;

pub use map::*;

fn main()
{
    let mut map = Map::new();

    map.update();
    map.test();

}
