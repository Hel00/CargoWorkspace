pub struct Person<'a>
{
    pub name: & 'a str,
    pub age: u8
}

impl<'a> Person<'a>
{
    pub fn new(name: & 'a str, age: u8) -> Self
    {
        Person { name: name, age: age }
    }
}