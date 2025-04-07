mod modulafy;

use modulafy::modulafy;
use syn::{
  Expr, Token, Type,
  parse::{Parse, ParseStream},
  parse_macro_input,
};

struct Input {
  expr: Expr,
  modulus: Expr,
  inttype: Type,
}

impl Parse for Input {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let expr = input.parse()?;
    input.parse::<Token![,]>()?;
    let modulus = input.parse()?;
    input.parse::<Token![,]>()?;
    let inttype = input.parse()?;
    Ok(Self { expr, modulus, inttype })
  }
}

#[proc_macro]
pub fn modular(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as Input);

  modulafy(&input.expr, &input.modulus, &input.inttype).into()
}
