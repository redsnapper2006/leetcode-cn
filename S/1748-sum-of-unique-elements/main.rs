struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for i in &nums {
      let mut v = m.entry(*i).or_insert(0);
      *v += 1;
    }
    let mut sum = 0;
    for (k, v) in &m {
      if *v == 1 {
        sum += k;
      }
    }
    sum
  }
}

fn main() {
  println!("{}", Solution::sum_of_unique(vec![0; 5]));
}
