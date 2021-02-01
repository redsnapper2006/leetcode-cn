struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for i in low_limit..high_limit + 1 {
      let mut sum = 0;
      let mut b = i;
      while b > 0 {
        let e = b % 10;
        sum += e;
        b /= 10;
      }

      if !m.contains_key(&sum) {
        m.insert(sum, 1);
      } else {
        let v = m.get_mut(&sum).unwrap();
        *v += 1;
      }
    }
    let mut max = 0;
    for (k, v) in &m {
      if *v > max {
        max = *v;
      }
    }
    max
  }
}

fn main() {
  println!("{}", Solution::count_balls(1, 15));
}
