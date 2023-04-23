use proc_macro::TokenStream;
use proc_macro::TokenTree;

#[proc_macro]
pub fn greeter_macro(input: TokenStream) -> TokenStream
{
    println!("ASD");
    "println!(\"Hello world! Nyaa! :3\")".parse().unwrap()
}

#[proc_macro]
pub fn TEST(input: TokenStream) -> TokenStream
{
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let literal = match &tokens[0] {
        TokenTree::Literal(lit) => lit,
        _ => panic!(),
    };
    let s = literal.to_string();

    s[1..s.len() - 1].parse().unwrap()
}
