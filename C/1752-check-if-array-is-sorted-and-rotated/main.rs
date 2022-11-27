struct Solution {}

impl Solution {
  pub fn check(nums: Vec<i32>) -> bool {
    let mut idx: usize = 0;

    for i in 0..nums.len() - 1 {
      if nums[i] > nums[i + 1] {
        idx = i + 1;
      }
    }

    for i in 0..nums.len() - 1 {
      let mut idx1: usize = (i + idx) % nums.len();
      let mut idx2: usize = (idx1 + 1) % nums.len();
      if nums[idx1] > nums[idx2] {
        return false;
      }
    }
    true
  }
}
