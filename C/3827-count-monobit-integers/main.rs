impl Solution {
  pub fn count_monobit(n: i32) -> i32 {
    32 - (n + 1).leading_zeros() as i32
  }
}
