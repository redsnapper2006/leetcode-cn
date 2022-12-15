struct Solution {}

impl Solution {
  pub fn get_lucky(s: String, k: i32) -> i32 {
    let mut sum: i32 = 0;
    for b in s.as_bytes() {
      let mut offset = (b - 'a' as u8) as i32 + 1;
      while offset > 0 {
        sum += offset % 10;
        offset /= 10;
      }
    }

    for i in 1..k {
      let mut t = sum;
      let mut sum2: i32 = 0;
      while t > 0 {
        sum2 += t % 10;
        t /= 10;
      }
      sum = sum2;
    }
    sum
  }
}
