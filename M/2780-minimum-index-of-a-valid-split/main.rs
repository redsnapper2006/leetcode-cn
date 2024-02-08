struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn minimum_index(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    nums.iter().for_each(|&v| {
      let count = m.entry(v).or_insert(0);
      *count += 1;
    });

    let mut dominant: i32 = 0;
    let mut cnt: i32 = 0;
    m.iter().for_each(|(k, v)| {
      if *v > cnt {
        cnt = *v;
        dominant = *k;
      }
    });

    let mut cur: i32 = 0;
    let mut idx: usize = 0;
    while idx < nums.len() {
      if nums[idx] == dominant {
        cur += 1;
      }
      if cur * 2 > idx as i32 + 1 && (cnt - cur) * 2 > nums.len() as i32 - idx as i32 - 1 {
        return idx as i32;
      }
      idx += 1;
    }
    -1
  }
}
