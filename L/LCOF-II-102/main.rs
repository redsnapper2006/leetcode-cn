struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut buf: HashMap<i32, i32> = HashMap::new();
    buf.insert(0, 1);
    nums.iter().for_each(|v| {
      let mut m: HashMap<i32, i32> = HashMap::new();
      for (ov, ocnt) in buf.iter() {
        let mut cnt = m.entry(ov - v).or_insert(0);
        *cnt += ocnt;
        let mut cnt2 = m.entry(ov + v).or_insert(0);
        *cnt2 += ocnt;
      }
      buf = m;
    });
    let mut v = buf.entry(target).or_insert(0);
    *v
  }
}
