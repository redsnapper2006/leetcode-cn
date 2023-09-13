struct Solution {}

impl Solution {
  pub fn distinct_integers(n: i32) -> i32 {
    match n {
      1 => 1,
      _ => n - 1,
    }
  }
}
