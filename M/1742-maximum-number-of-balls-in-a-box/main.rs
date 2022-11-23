struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut m: [i32; 54] = [0; 54];
    for i in low_limit..high_limit + 1 {
      let mut sum = 0;
      let mut b = i;
      while b > 0 {
        let e = b % 10;
        sum += e;
        b /= 10;
      }

      m[sum as usize] += 1;
    }

    let mut max = 0;
    for v in m {
      if v > max {
        max = v;
      }
    }
    max
  }
}

fn main() {
  println!("{}", Solution::count_balls(1, 15));
}
