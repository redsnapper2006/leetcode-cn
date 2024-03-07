struct Solution {}

impl Solution {
  pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
    let m = n.min(target / 2);
    let mut sum: i64 = (m as i64 + 1) * m as i64 / 2;
    sum %= 1000000007;
    if n >= m {
      sum += (target as i64 + target as i64 + n as i64 - m as i64 - 1) * (n as i64 - m as i64) / 2;
      sum %= 1000000007;
    }

    sum
  }
}
