#[cfg(test)]
mod tests {
  use modula_rs::modular;

  #[test]
  fn test_add() {
    let x = 10;
    let y = 20;
    let answer = modular!(x + y, 7, i32);
    assert_eq!(answer, 2);
  }

  #[test]
  fn test_sub() {
    let x = 10;
    let y = 20;
    let answer = modular!(x - y, 7, i32);
    assert_eq!(answer, 4);
  }
}
