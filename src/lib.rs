extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
  BinOp, Expr, ExprBinary, ExprLit, Lit, LitInt, Token, Type,
  parse::{Parse, ParseStream},
  parse_macro_input,
  spanned::Spanned,
};

struct Input {
  expr: Expr,
  modulus: LitInt,
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

fn modulo(expr: TokenStream, modulus: &LitInt) -> TokenStream {
  quote! { #expr.rem_euclid(#modulus) }
}

fn modulafy(expr: &Expr, modulus: &LitInt, inttype: &Type) -> TokenStream {
  match expr {
    Expr::Binary(ExprBinary { left, op, right, .. }) => match op {
      op @ BinOp::Add(_) | op @ BinOp::Sub(_) => {
        let left = modulafy(left, modulus, inttype);
        let right = modulafy(right, modulus, inttype);
        modulo(quote! { (#left #op #right) }, modulus)
      }
      _ => syn::Error::new(
        expr.span(),
        format!("Unsupported bin op \"{}\"", quote! { #op }),
      )
      .to_compile_error(),
    },
    Expr::Path(_) => {
      let expr = quote! { #inttype::from(#expr) };
      modulo(expr.to_token_stream(), modulus)
    }
    Expr::Lit(ExprLit { lit: Lit::Int(repr), .. }) => {
      if repr.suffix().is_empty() {
        quote! {
            #inttype::from(#expr)
        }
      } else {
        expr.to_token_stream()
      }
    }
    Expr::Lit(_) => expr.to_token_stream(),
    _ => syn::Error::new(
      expr.span(),
      format!("Unexpected expr \"{}\"", quote! { #expr }),
    )
    .to_compile_error(),
  }
  .into()
}

#[proc_macro]
pub fn modular(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as Input);

  modulafy(&input.expr, &input.modulus, &input.inttype).into()
}
