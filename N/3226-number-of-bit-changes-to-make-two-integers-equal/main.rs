struct Solution {}

impl Solution {
  pub fn min_changes(n: i32, k: i32) -> i32 {
    if n & k != k {
      return -1;
    }
    u32::count_ones((n ^ k) as u32) as i32
  }
}
