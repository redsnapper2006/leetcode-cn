impl Solution {
  pub fn check_divisibility(n: i32) -> bool {
    let mut sum: i32 = 0;
    let mut product: i32 = 1;
    let mut n = n;
    let orig = n;
    while n > 0 {
      let d = n % 10;
      sum += d;
      product *= d;
      n /= 10;
    }
    orig % (sum + product) == 0
  }
}
