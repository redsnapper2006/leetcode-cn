impl Solution {
  pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
    let mut idx: usize = 0;
    while idx < nums.len() - 1 && nums[idx] < nums[idx + 1] {
      idx += 1;
    }
    if idx == nums.len() - 1 {
      return (nums.len() * (nums.len() + 1)) as i32 / 2;
    }
    let mut idx: i32 = idx as i32;
    let mut ans = idx + 2;
    let mut j = nums.len() - 1;
    while j == nums.len() - 1 || nums[j] < nums[j + 1] {
      while idx >= 0 && nums[idx as usize] >= nums[j] {
        idx -= 1;
      }
      ans += idx + 2;
      j -= 1;
    }
    ans
  }
}
