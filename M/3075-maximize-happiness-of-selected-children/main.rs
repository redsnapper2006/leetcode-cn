use std::cmp::Reverse;

impl Solution {
  pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
    let mut happy = happiness.iter().map(|n| n as i64).collect::<Vec<i64>>();
    happy.sort_by_key(|&n| Reverse(n));
    happy
      .iter()
      .take(k)
      .fold((0, 0), |(sum, time), v| (sum + (v - time).abs(0), time + 1))
      .0
  }
}
