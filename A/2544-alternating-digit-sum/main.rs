struct Solution {}

impl Solution {
  pub fn alternate_digit_sum(n: i32) -> i32 {
    let mut m = n;
    let mut b: Vec<i32> = Vec::new();
    while m > 0 {
      b.push(m % 10);
      m /= 10;
    }

    let mut ret: i32 = 0;
    let mut times: i32 = 1;
    (0..b.len()).rev().for_each(|idx| {
      ret += b[idx] * times;
      times *= -1;
    });
    ret
  }
}
