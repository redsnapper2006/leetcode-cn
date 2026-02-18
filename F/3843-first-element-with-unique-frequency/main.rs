use std::collections::HashMap;

impl Solution {
  pub fn first_unique_freq(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut cnt2: HashMap<i32, i32> = HashMap::new();

    nums.iter().for_each(|&v| {
      *cnt.entry(v).or_insert(0) += 1;
    });
    for (k, c) in cnt.iter() {
      *cnt2.entry(*c).or_insert(0) += 1;
    }
    for k in nums {
      if *cnt2.get(&cnt.get(&k).unwrap()).unwrap() == 1 {
        return k;
      }
    }
    -1
  }
}
©leetcode
