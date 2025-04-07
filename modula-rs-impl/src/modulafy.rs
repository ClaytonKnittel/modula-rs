use num_prime::nt_funcs::is_prime64;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{BinOp, Expr, ExprBinary, ExprLit, Lit, Type, spanned::Spanned};

fn modulo(expr: TokenStream, modulus: &Expr) -> TokenStream {
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

fn binary(
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
