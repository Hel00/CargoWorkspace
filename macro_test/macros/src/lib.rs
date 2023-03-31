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
