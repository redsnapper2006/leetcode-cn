impl Solution {
  pub fn count_commas(n: i32) -> i32 {
    if n < 1000 {
      return 0;
    }
    let mut base: i32 = 1000;
    let mut ans: i32 = 0;
    let mut cnt: i32 = 1;
    while base * 1000 < n {
      ans += cnt * base * 999;
      base *= 1000;
      cnt += 1;
    }
    ans + cnt * (n - base + 1)
  }
}
