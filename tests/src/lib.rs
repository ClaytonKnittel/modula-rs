#[cfg(test)]
mod tests {
  use modula_rs::modular;

  #[test]
  fn test_simple() {
    let x = 10;
    let y = 20;
    let answer = modular!(x + y, 7);
    assert_eq!(answer, 2);
  }
}
