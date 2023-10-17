impl Solution {
  pub fn sum_of_multiples(n: i32) -> i32 {
    (1..=n)
      .filter(|v| v % 3 == 0 || v % 5 == 0 || v % 7 == 0)
      .sum()
  }
}
