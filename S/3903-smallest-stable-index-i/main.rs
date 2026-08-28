impl Solution {
  pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
    let mut right: Vec<i32> = vec![*nums.last().unwrap(); nums.len()];
    for i in (0..nums.len() - 1).rev() {
      right[i] = nums[i].min(right[i + 1]);
    }

    let mut mx = nums[0];
    for i in 0..nums.len() {
      mx = mx.max(nums[i]);
      if mx - right[i] <= k {
        return i as i32;
      }
    }
    -1
  }
}
