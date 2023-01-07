struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let remain: i32 = nums.iter().sum::<i32>() - x;
    if remain < 0 {
      return -1;
    }
    if remain == 0 {
      return nums.len() as i32;
    }
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(0, -1);
    let mut sum: i32 = 0;
    let mut ret: i32 = nums.len() as i32;
    for i in 0..nums.len() {
      sum += nums[i];
      if sum == x && ret > i as i32 + 1 {
        ret = i as i32 + 1;
      }
      let d = sum - remain;
      if m.contains_key(&d) && ret > (nums.len() - i) as i32 + m.get(&d).unwrap() {
        ret = (nums.len() - i) as i32 + m.get(&d).unwrap();
      }
      m.insert(sum, i as i32);
    }
    if ret == nums.len() as i32 {
      return -1;
    }
    ret
  }

  pub fn min_operations2(nums: Vec<i32>, x: i32) -> i32 {
    let mut left: HashMap<i32, i32> = HashMap::new();
    let mut right: HashMap<i32, i32> = HashMap::new();
    let mut sum: i32 = 0;
    for i in 0..nums.len() {
      sum += nums[i];
      left.insert(sum, i as i32);
    }
    left.insert(0, -1);
    sum = 0;
    for i in (0..nums.len()).rev() {
      sum += nums[i];
      right.insert(sum, i as i32);
    }
    right.insert(0, nums.len() as i32);
    if sum < x {
      return -1;
    }
    if sum == x {
      return nums.len() as i32;
    }

    let mut ret: i32 = nums.len() as i32;
    for (k, v) in &left {
      let remain = x - k;
      if right.contains_key(&remain) {
        let idx = right.get(&remain).unwrap();
        if *idx > *v && ret > nums.len() as i32 - *idx + *v + 1 {
          ret = nums.len() as i32 - *idx + *v + 1;
        }
      }
    }
    for (k, v) in &right {
      let remain = x - k;
      if left.contains_key(&remain) {
        let idx = left.get(&remain).unwrap();
        if *idx < *v && ret > nums.len() as i32 - *v + *idx + 1 {
          ret = nums.len() as i32 - *v + *idx + 1;
        }
      }
    }
    if ret == nums.len() as i32 {
      return -1;
    }
    ret
  }
}
