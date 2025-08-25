impl Solution {
  pub fn min_sensors(n: i32, m: i32, k: i32) -> i32 {
    ((n + 2 * k) / (2 * k + 1)) as i32 * ((m + 2 * k) / (2 * k + 1)) as i32
  }
}
