impl Solution {
  pub fn consecutive_set_bits(n: i32) -> bool {
    ((n & (n >> 1)) != 0) & ((n & (n >> 1)) & ((n & (n >> 1)) - 1) == 0)
  }
}
