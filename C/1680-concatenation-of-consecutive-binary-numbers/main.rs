impl Solution {
  pub fn concatenated_binary(n: i32) -> i32 {
    (1..=n as i64).fold(0, |aggr, v| (aggr << (64 - v.leading_zeros()) | v) % 1000000007) as _
  }
}
