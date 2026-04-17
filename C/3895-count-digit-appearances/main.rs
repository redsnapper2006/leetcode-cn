impl Solution {
  pub fn count_digit_occurrences(nums: Vec<i32>, digit: i32) -> i32 {
    nums
      .iter()
      .map(|&n| {
        let mut n = n;
        let mut cnt: i32 = 0;
        while n > 0 {
          if n % 10 == digit {
            cnt += 1;
          }
          n /= 10;
        }
        cnt
      })
      .sum()
  }
}
