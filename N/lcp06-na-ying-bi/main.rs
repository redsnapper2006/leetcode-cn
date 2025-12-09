impl Solution {
  pub fn min_count(coins: Vec<i32>) -> i32 {
    coins.iter().fold(0, |aggr, v| aggr + (v + 1) / 2)
  }
}
