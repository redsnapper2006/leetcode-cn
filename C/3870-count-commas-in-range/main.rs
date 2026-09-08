impl Solution {
  pub fn count_commas(n: i64) -> i32 {
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
    (ans + cnt * (n - base + 1)) as i32
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::count_commas(1000000000));
}
