struct Solution {}

impl Solution {
  pub fn number_of_cuts(n: i32) -> i32 {
    match n % 2 {
      0 => n / 2,
      _ => match n {
        1 => 0,
        _ => n,
      },
    }
  }
}
