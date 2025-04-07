use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprUnary, Type, UnOp, spanned::Spanned};

use crate::modulafy::modulafy;

pub fn unary(
  ExprUnary { expr, op, .. }: &ExprUnary,
  modulus: &Expr,
  inttype: &Type,
) -> TokenStream {
  match op {
    UnOp::Neg(_) => {
      let expr = modulafy(expr, modulus, inttype);
      quote! { ((#modulus - #expr) % #modulus) }
    }
    _ => syn::Error::new(
      op.span(),
      format!("Unsupported unary op \"{}\"", quote! { #op }),
    )
    .to_compile_error(),
  }
}
