struct Solution {}

impl Solution {
  pub fn reinitialize_permutation(n: i32) -> i32 {
    let mut m: i32 = n / 2 + (1 - 1) / 2;
    let mut steps: i32 = 1;
    while m != 1 {
      if m % 2 == 0 {
        m = m / 2;
      } else {
        m = n / 2 + (m - 1) / 2;
      }
      steps+=1;
    }
    steps
  }
}
