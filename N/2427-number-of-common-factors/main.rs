struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn common_factors(a: i32, b: i32) -> i32 {
    let a_sqrt: i32 = (a as f32).sqrt() as i32;
    let b_sqrt: i32 = (b as f32).sqrt() as i32;

    let mut a_factors: HashSet<i32> = HashSet::new();
    for i in 1..=a_sqrt {
      if a % i == 0 {
        a_factors.insert(i);
        a_factors.insert(a / i);
      }
    }
    let mut cnt: i32 = 0;
    for i in 1..=b_sqrt {
      if b % i == 0 {
        if a_factors.contains(&i) {
          cnt += 1;
        }
        let bb = b / i;
        if bb == i {
          continue;
        }
        if a_factors.contains(&bb) {
          cnt += 1;
        }
      }
    }
    cnt
  }
}

fn main() {
  println!("{}", Solution::common_factors(6, 12));
  println!("{}", Solution::common_factors(12, 6));
}
