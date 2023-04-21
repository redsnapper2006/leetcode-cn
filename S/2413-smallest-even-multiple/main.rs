struct Solution {}

impl Solution {
  pub fn smallest_even_multiple(n: i32) -> i32 {
    // 1
    match n % 2 {
      0 => n,
      _ => 2 * n,
    }
    // 2
    // (n % 2 + 1) * n
  }
}
