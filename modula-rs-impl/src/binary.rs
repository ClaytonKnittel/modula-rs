use proc_macro2::TokenStream;
use quote::quote;
use syn::{BinOp, Expr, ExprBinary, Type, spanned::Spanned};

use crate::modulafy::{modulafy, modulo};

fn add_op(left: &Expr, op: &BinOp, right: &Expr, modulus: &Expr, inttype: &Type) -> TokenStream {
  let left = modulafy(left, modulus, inttype);
  let right = modulafy(right, modulus, inttype);
  modulo(quote! { (#left #op #right) }, modulus)
}

fn mul_op(left: &Expr, right: &Expr, modulus: &Expr, inttype: &Type) -> TokenStream {
  let left = modulafy(left, modulus, inttype);
  let right = modulafy(right, modulus, inttype);
  modulo(quote! { (#left * #right) }, modulus)
}

fn div_op(left: &Expr, right: &Expr, modulus: &Expr, inttype: &Type) -> TokenStream {
  let left = modulafy(left, modulus, inttype);
  let right = modulafy(right, modulus, inttype);
  let rinv = modulo(
    quote! { ::modula_rs::num_integer::Integer::extended_gcd(&#right, &#modulus).x },
    modulus,
  );
  modulo(quote! { (#left * #rinv) }, modulus)
}

pub fn binary(
  ExprBinary { left, op, right, .. }: &ExprBinary,
  modulus: &Expr,
  inttype: &Type,
) -> TokenStream {
  match op {
    op @ BinOp::Add(_) | op @ BinOp::Sub(_) => add_op(left, op, right, modulus, inttype),
    BinOp::Mul(_) => mul_op(left, right, modulus, inttype),
    BinOp::Div(_) => div_op(left, right, modulus, inttype),
    _ => syn::Error::new(
      op.span(),
      format!("Unsupported bin op \"{}\"", quote! { #op }),
    )
    .to_compile_error(),
  }
}
