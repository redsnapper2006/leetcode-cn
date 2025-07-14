impl Solution {
  pub fn minimum_sum(nums: Vec<i32>) -> i32 {
    let mut left: Vec<i32> = vec![0; nums.len()];
    let mut right: Vec<i32> = vec![0; nums.len()];
    left[0] = nums[0];
    for i in 1..nums.len() {
      left[i] = nums[i].min(left[i - 1]);
    }
    right[nums.len() - 1] = nums[nums.len() - 1];
    for i in (0..nums.len() - 1).rev() {
      right[i] = nums[i].min(right[i + 1]);
    }
    let mut ans: i32 = i32::MAX;
    for i in 1..nums.len() - 1 {
      if left[i] != nums[i] && right[i] != nums[i] {
        ans = ans.min(left[i] + nums[i] + right[i]);
      }
    }
    ans
  }
}
