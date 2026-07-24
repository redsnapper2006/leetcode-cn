impl Solution {
  pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
    let mut left: Vec<i32> = nums.clone();
    let mut right: Vec<i32> = nums.clone();
    let mut mx: i32 = i32::MIN;
    let mut mn: i32 = i32::MAX;
    for i in 0..nums.len() {
      mx = mx.max(nums[i]);
      left[i] = mx;
      mn = mn.min(nums[nums.len() - 1 - i]);
      right[nums.len() - 1 - i] = mn;
    }
    for i in 0..nums.len() {
      if left[i] - right[i] <= k {
        return i as i32;
      }
    }
    -1
  }
}
