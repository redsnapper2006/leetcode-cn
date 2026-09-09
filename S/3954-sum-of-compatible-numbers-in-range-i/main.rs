impl Solution {
  pub fn sum_of_good_integers(n: i32, k: i32) -> i32 {
    let mut p = (n - k).max(0);
    let mut ans = 0i32;
    while p <= n + k {
      if p & n == 0 {
        ans += p;
      }
      p += 1;
    }
    ans
  }
}
