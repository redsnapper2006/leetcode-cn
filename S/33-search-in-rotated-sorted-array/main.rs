impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 1 {
      return if nums[0] == target { 0 } else { -1 };
    }
    let mut s: i32 = 0;
    let mut e: i32 = nums.len() as i32 - 1;
    while s <= e {
      let m = (s + (e - s) / 2);
      if nums[m as usize] == target {
        return m;
      }
      if nums[m as usize] >= nums[0] {
        if nums[m as usize] > target && nums[0] <= target {
          e = m - 1;
        } else {
          s = m + 1;
        }
      } else {
        if nums[m as usize] < target && nums[nums.len() - 1] >= target {
          s = m + 1;
        } else {
          e = m - 1;
        }
      }
    }
    -1
  }
}
