struct Solution {}

impl Solution {
  pub fn count_even(num: i32) -> i32 {
    (2..=num)
      .map(|n| {
        let mut sum: i32 = 0;
        let mut m = n;
        while m > 0 {
          sum += m % 10;
          m /= 10;
        }
        (sum + 1) % 2
      })
      .sum()
  }
}
