fn main()
{
    let string = string_operations( String::new() );
    println!("Hello, world! {}", string);
}

fn string_operations(mut data: String) -> String
{
    data = "Hey".to_string();
    data = "Hello".to_string();
    data.push_str(" World!");

    data
}
