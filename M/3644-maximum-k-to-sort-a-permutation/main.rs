use std::collections::HashMap;
impl Solution {
  pub fn sort_permutation(nums: Vec<i32>) -> i32 {
    if nums[0] != 0 {
      return 0;
    }

    let mut ans: i32 = 0xFFFFFF;
    for i in (0..nums.len()).rev() {
      if nums[i] == i as i32 {
        continue;
      }
      ans &= nums[i];
    }
    if ans == 0xFFFFFF { 0 } else { ans }
  }

  pub fn sort_permutation2(nums: Vec<i32>) -> i32 {
    let mut pos: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
      pos.insert(nums[i], i);
    }
    let mut nums = nums;
    let mut n = nums.len() as i32 - 1;
    let mut k = 0xFFFFFF;
    for i in (0..nums.len()).rev() {
      let ii = i as i32;
      if nums[i] == ii {
        continue;
      }
      let rp = *pos.get(&ii).unwrap();
      k = k.min(k & ii & nums[i]);
      nums[rp] = nums[i];
      pos.insert(nums[i], rp);
    }
    if k == 0xFFFFFF { 0 } else { k }
  }
}
