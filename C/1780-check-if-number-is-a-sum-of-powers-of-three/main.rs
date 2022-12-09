struct Solution {}

impl Solution {
  pub fn check_powers_of_three(n: i32) -> bool {
    let mut m = n;
    while m > 0 {
      if m % 3 == 2 {
        return false;
      }
      m /= 3;
    }
    true
  }
}
