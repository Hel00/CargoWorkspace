mod person;

use person::*;

fn main()
{
    let hel = Person::new("Hel", 42);
    println!("Person is {} years old and their name is {}", hel.age, hel.name);
}
