struct Solution {}

impl Solution {
  pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
    let mut s: i32 = 1;
    let mut e: i32 = max_sum;

    while s <= e {
      let m: i32 = s + (e - s) / 2;
      let mut left: i32 = m - index;
      let mut left_sum: i64 = 0;
      if left <= 0 {
        left_sum = (1 + m as i64) * m as i64 / 2 + index as i64 - m as i64 + 1;
      } else {
        left_sum = (m as i64 + left as i64) * (index as i64 + 1) / 2;
      }
      let mut right: i32 = n - index;
      let mut right_sum: i64 = 0;
      if right <= m {
        right_sum = (m as i64 + m as i64 - right as i64 + 1) * right as i64 / 2;
      } else {
        right_sum = m as i64 * (m as i64 + 1) / 2 + (n as i64 - index as i64 - m as i64);
      }
      // println!("{} {} {} {}", m, left_sum, right_sum, right);
      if left_sum + right_sum - m as i64 > max_sum as i64 {
        e = m - 1;
      } else {
        s = m + 1;
      }
    }
    e
  }
}
