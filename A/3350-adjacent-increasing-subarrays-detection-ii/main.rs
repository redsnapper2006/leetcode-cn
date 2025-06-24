impl Solution {
  pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
    let mut left: Vec<i32> = vec![-1; nums.len()];
    let mut right: Vec<i32> = vec![-1; nums.len()];
    (0..nums.len()).for_each(|idx| {
      if idx == 0 {
        left[idx] = 0;
      } else {
        if nums[idx] > nums[idx - 1] {
          left[idx] = left[idx - 1];
        } else {
          left[idx] = idx as i32;
        }
      }

      if idx == 0 {
        right[nums.len() - 1] = nums.len() as i32 - 1;
      } else {
        if nums[nums.len() - 1 - idx] < nums[nums.len() - idx] {
          right[nums.len() - 1 - idx] = right[nums.len() - idx];
        } else {
          right[nums.len() - 1 - idx] = (nums.len() - 1 - idx) as i32;
        }
      }
    });

    let mut ans : i32 = 0;
    for idx in (0..nums.len() - 1) {
      ans = ans.max((idx as i32 - left[idx] + 1).min(right[idx + 1] - idx as i32));
    }
    ans
  }
}
