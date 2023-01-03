struct Solution {}

impl Solution {
  pub fn are_numbers_ascending(s: String) -> bool {
    let mut prev: i32 = 0;
    let mut sum: i32 = 0;
    for b in s.as_bytes() {
      if *b < '0' as u8 || *b > '9' as u8 {
        if sum > 0 {
          if prev > 0 && sum <= prev {
            return false;
          }
          prev = sum;
          sum = 0;
        }
        continue;
      }
      sum = sum * 10 + (*b - '0' as u8) as i32;
    }
    sum == 0 || sum > 0 && sum > prev
  }
}
