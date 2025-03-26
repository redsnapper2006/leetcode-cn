use std::collections::HashSet;
impl Solution {
  pub fn minimum_sum(n: i32, k: i32) -> i32 {
    let m = n.min(k / 2);
    m * (m + 1) / 2 + (2 * k + n - m - 1) * (n - m) / 2
  }

  pub fn minimum_sum2(n: i32, k: i32) -> i32 {
    let mut exclude: HashSet<i32> = HashSet::new();

    let mut idx: i32 = 0;
    let mut cnt: i32 = 0;
    let mut sum: i32 = 0;
    while cnt < n {
      idx += 1;
      if exclude.contains(&idx) {
        continue;
      }
      sum += idx;
      exclude.insert((k - idx));
      cnt += 1;
    }

    sum
  }
}
