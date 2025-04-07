use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{BinOp, Expr, ExprBinary, ExprLit, Lit, LitInt, Type, spanned::Spanned};

fn modulo(expr: TokenStream, modulus: &LitInt) -> TokenStream {
  quote! { #expr.rem_euclid(#modulus) }
}

pub fn modulafy(expr: &Expr, modulus: &LitInt, inttype: &Type) -> TokenStream {
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
    Expr::Paren(paren) => {
      let expr = modulafy(&paren.expr, modulus, inttype);
      quote! { (#expr) }
    }
    _ => syn::Error::new(
      expr.span(),
      format!("Unexpected expr \"{}\"", quote! { #expr }),
    )
    .to_compile_error(),
  }
  .into()
}
