impl Solution {
  pub fn smallest_number(n: i32, t: i32) -> i32 {
    let mut n = n;
    while n <= 100 {
      let mut m = n;
      let mut multi: i32 = 1;
      while m > 0 {
        multi *= m % 10;
        m /= 10;
      }
      if multi % t == 0 {
        break;
      }
      n += 1;
    }
    n
  }
}
