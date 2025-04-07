use num_prime::nt_funcs::is_prime64;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Expr, ExprLit, Lit, Type, spanned::Spanned};

use crate::{binary::binary, unary::unary};

pub fn modulo(expr: TokenStream, modulus: &Expr) -> TokenStream {
  quote! { #expr.rem_euclid(#modulus) }
}

fn lit(lit: &ExprLit, inttype: &Type) -> TokenStream {
  match lit {
    ExprLit { lit: Lit::Int(repr), .. } => {
      if repr.suffix().is_empty() {
        quote! {
            #inttype::from(#lit)
        }
      } else {
        lit.to_token_stream()
      }
    }
    _ => lit.to_token_stream(),
  }
}

fn path(expr: &Expr, modulus: &Expr, inttype: &Type) -> TokenStream {
  let expr = quote! { #inttype::from(#expr) };
  modulo(expr.to_token_stream(), modulus)
}

pub fn modulafy(expr: &Expr, modulus: &Expr, inttype: &Type) -> TokenStream {
  if let Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) = modulus {
    let m: u64 = lit.base10_parse().unwrap();
    if !is_prime64(m) {
      return syn::Error::new(
        modulus.span(),
        format!("Modulus is not prime \"{}\"", quote! { #modulus }),
      )
      .to_compile_error();
    }
  } else {
    // TODO: check that the modulus is prime at runtime.
  }

  match expr {
    Expr::Lit(expr) => lit(expr, inttype),
    Expr::Path(_) => path(expr, modulus, inttype),
    Expr::Unary(unary_op) => unary(unary_op, modulus, inttype),
    Expr::Binary(bin_op) => binary(bin_op, modulus, inttype),
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
