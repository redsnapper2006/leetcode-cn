impl Solution {
  pub fn check_divisibility(n: i32) -> bool {
    let mut sum: i32 = 0;
    let mut product: i32 = 1;
    let mut n = n;
    let nn = n;
    while n > 0 {
      sum += n % 10;
      product *= n % 10;
      n /= 10;
    }
    nn % (sum + product) == 0
  }
}
