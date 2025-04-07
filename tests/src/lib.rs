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

  #[test]
  fn test_add_many() {
    for x in -10..10 {
      for y in -10..10 {
        let answer = modular!(x + y, 10, i32);
        assert_eq!(answer, (x + y + 20) % 10);
      }
    }
  }

  #[test]
  fn test_sub_many() {
    for x in -10..10 {
      for y in -10..10 {
        let answer = modular!(x - y, 10, i32);
        assert_eq!(answer, (x - y + 20) % 10);
      }
    }
  }

  #[test]
  fn test_add_recursive() {
    let x = 10;
    let y = 20;
    let answer = modular!(x + (x + y), 7, i32);
    assert_eq!(answer, 5);
  }
}
