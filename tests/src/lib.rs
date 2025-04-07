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
    for x in -11..11 {
      for y in -11..11 {
        let answer = modular!(x + y, 11, i32);
        assert_eq!(answer, (x + y + 22) % 11);
      }
    }
  }

  #[test]
  fn test_sub_many() {
    for x in -11..11 {
      for y in -11..11 {
        let answer = modular!(x - y, 11, i32);
        assert_eq!(answer, (x - y + 22) % 11);
      }
    }
  }

  #[test]
  fn test_add_recursive() {
    let x = 10;
    let y = 20;
    let answer = modular!(x - (x + y), 7, i32);
    assert_eq!(answer, 1);
  }

  #[test]
  fn test_mul() {
    let x = 10;
    let y = 20;
    let answer = modular!(x * y, 7, i32);
    assert_eq!(answer, 4);
  }

  #[test]
  fn test_mul_many() {
    for x in -11i32..11 {
      for y in -11i32..11 {
        let answer = modular!(x * y, 7, i32);
        assert_eq!(answer, (x * y).rem_euclid(7));
      }
    }
  }

  #[test]
  fn test_div() {
    let x = 10;
    let y = 20;
    let answer = modular!(x / y, 7, i32);
    assert_eq!(answer, 4);
  }

  #[test]
  fn test_div_many() {
    for x in (-10i32..=10).filter(|&x| x != 0) {
      for y in (-10i32..=10).filter(|&y| y != 0) {
        let answer = modular!(x / y, 11, i32);
        assert_eq!((y * answer).rem_euclid(11), x.rem_euclid(11));
      }
    }
  }
}
