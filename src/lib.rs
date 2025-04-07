extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
  BinOp, Expr, ExprBinary, LitInt, Token,
  parse::{Parse, ParseStream},
  parse_macro_input,
  spanned::Spanned,
};

struct Input {
  expr: Expr,
  modulus: LitInt,
}

impl Parse for Input {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let expr = input.parse()?;
    input.parse::<Token![,]>()?;
    let modulus = input.parse()?;
    Ok(Self { expr, modulus })
  }
}

fn translate_pluses(expr: Expr, modulus: LitInt) -> TokenStream {
  match &expr {
    Expr::Binary(ExprBinary { left, op, right, .. }) => match op {
      BinOp::Add(_) => quote! {
        (#left + #right) % #modulus
      }
      .into(),
      _ => expr.to_token_stream().into(),
    },
    Expr::Lit(_) | Expr::Group(_) => expr.to_token_stream().into(),
    _ => syn::Error::new(
      expr.span(),
      format!("Unexpected expr \"{}\"", quote! { #expr }),
    )
    .to_compile_error()
    .into(),
  }
}

#[proc_macro]
pub fn modular(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as Input);

  translate_pluses(input.expr, input.modulus)
}
