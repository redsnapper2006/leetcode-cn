struct Solution {}

impl Solution {
  pub fn number_of_child(n: i32, k: i32) -> i32 {
    let div = k / (n - 1);
    let m = k % (n - 1);
    match div % 2 {
      0 => m,
      _ => n - m - 1,
    }
  }
}
