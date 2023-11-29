impl Solution {
  pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    (n * (n + 1) >> 1) - m * (1 + n / m) * (n / m)
  }
}
