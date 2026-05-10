use std::collections::BTreeMap;
use std::ops::Bound::Included;

impl Solution {
  pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
    let target = target as i64;
    let mut tm: BTreeMap<i64, i64> = BTreeMap::new();
    tm.insert(nums[0] as i64, 0);
    for i in (1..nums.len()) {
      let mut r: i64 = -1;
      let (mut s, mut e) = (nums[i] as i64 - target, nums[i] as i64 + target);
      if s > e {
        (s, e) = (e, s);
      }

      for (&k, &v) in tm.range((Included(s), Included(e))) {
        r = r.max(v);
      }
      if r != -1 {
        tm.insert(nums[i] as i64, r + 1);
      }
    }
    if !tm.contains_key(&(nums[nums.len() - 1] as i64)) {
      -1
    } else {
      *tm.get(&(nums[nums.len() - 1] as i64)).unwrap() as _
    }
  }
}
