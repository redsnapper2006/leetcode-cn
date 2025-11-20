impl Solution {
  pub fn remove_zeros(n: i64) -> i64 {
    let mut n = n;
    let mut m: i64 = 1000000000000000;
    let mut ans: i64 = 0;
    while m > 0 {
      if n >= m {
        ans *= 10;
        ans += n / m;
        n %= m;
      }
      m /= 10;
    }
    ans
  }
}
