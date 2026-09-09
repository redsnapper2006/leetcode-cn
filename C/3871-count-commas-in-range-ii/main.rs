impl Solution {
  pub fn count_commas(n: i64) -> i64 {
    let mut p = 1000i64;
    let mut ans = 0i64;
    while p <= n {
      ans += n - p + 1;
      p *= 1000;
    }
    ans
  }

  pub fn count_commas2(n: i64) -> i64 {
    if n < 1000 {
      return 0;
    }
    let n = n as u64;
    let mut base = 1000u64;
    let mut ans = 0u64;
    let mut cnt = 1u64;
    while base * 1000 <= n {
      ans += cnt * base * 999;
      base *= 1000;
      cnt += 1;
    }
    (ans + cnt * (n - base + 1)) as i64
  }
}
