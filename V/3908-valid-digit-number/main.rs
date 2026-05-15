impl Solution {
  pub fn valid_digit(n: i32, x: i32) -> bool {
    let mut n = n;
    let mut match1: bool = false;
    while n > 9 {
      if n % 10 == x {
        match1 = true;
      }
      n /= 10;
    }
    match1 && n != x
  }
}
