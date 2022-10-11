use std::collections::HashMap;

struct Solution {}

impl Solution {
  pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
    let mut m = HashMap::new();
    let mut ret: i32 = 0;
    for i in 0..nums.len() - 1 {
      if nums[i] != key {
        continue;
      }
      if !m.contains_key(&nums[i + 1]) {
        m.insert(nums[i + 1], 1);
      } else {
        let x = m.get_mut(&nums[i + 1]).unwrap();
        *x += 1
      }
      ret = nums[i + 1]
    }
    let mut max: i32 = *m.get(&ret).unwrap();
    for (key, val) in m.iter() {
      if *val > max {
        max = *val;
        ret = *key;
      }
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::most_frequent(vec![1, 2, 3], 2))
}
