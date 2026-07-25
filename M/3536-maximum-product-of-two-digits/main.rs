impl Solution {
  pub fn max_product(n: i32) -> i32 {
    let mut first: i32 = -1;
    let mut second: i32 = -1;
    let mut n = n;
    while n > 0 {
      let m = n % 10;
      if m > first {
        second = first;
        first = m;
      } else if m > second {
        second = m;
      }
      n /= 10;
    }
    first * second
  }
}
