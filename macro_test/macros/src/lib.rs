use std::str::FromStr;
use proc_macro::*;

#[proc_macro]
pub fn test(input: TokenStream) -> TokenStream
{
    let mut result = TokenStream::from_str("[1, 2, 3]");

    let x = TokenTree::Literal(Literal::string("HEYA!"));

    let result = TokenStream::from(x);
    // result = Ok(TokenStream::from(x));

    result
}

#[proc_macro]
pub fn print_node(input: TokenStream) -> TokenStream
{
    for token in input.clone()
    {
        let node = match token.clone()
        {
            TokenTree::Group(_) => "Group",
            TokenTree::Ident(_) => "Ident",
            TokenTree::Literal(_) => "Literal",
            TokenTree::Punct(_) => "Punct",
        };

        println!("Token {} is of Node {}", token, node);
    }

    let result = TokenStream::from_str("0").unwrap();

    result
}
