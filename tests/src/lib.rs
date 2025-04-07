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
    for m in (1..=11).filter(|&m| (2..m).all(|d| m % d == 0)) {
      for x in -m..=m {
        for y in -m..=m {
          let answer = modular!(x + y, m, i32);
          assert_eq!(answer, (x + y + 2 * m) % m);
        }
      }
    }
  }

  #[test]
  fn test_sub_many() {
    for m in (1..=11).filter(|&m| (2..m).all(|d| m % d == 0)) {
      for x in -m..=m {
        for y in -m..=m {
          let answer = modular!(x - y, m, i32);
          assert_eq!(answer, (x - y + 2 * m) % m);
        }
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
    for m in (1..=11).filter(|&m| (2..m).all(|d| m % d == 0)) {
      for x in -m..=m {
        for y in -m..=m {
          let answer = modular!(x * y, m, i32);
          assert_eq!(answer, (x * y).rem_euclid(m));
        }
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
    for m in (1..=11).filter(|&m| (2..m).all(|d| m % d == 0)) {
      for x in (-(m - 1)..m).filter(|&x| x != 0) {
        for y in (-(m - 1)..m).filter(|&y| y != 0) {
          let answer = modular!(x / y, m, i32);
          assert_eq!((y * answer).rem_euclid(m), x.rem_euclid(m));
        }
      }
    }
  }

  #[test]
  fn test_neg() {
    let x = 10;
    let answer = modular!(-x, 7, i32);
    assert_eq!(answer, 4);
  }

  #[test]
  fn test_neg_many() {
    for m in (1..=11).filter(|&m| (2..m).all(|d| m % d == 0)) {
      for x in -m..=m {
        let answer = modular!(-x, m, i32);
        assert_eq!((x + answer).rem_euclid(m), 0);
      }
    }
  }
}
