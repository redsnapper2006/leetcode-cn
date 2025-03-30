impl Solution {
  pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter().enumerate().fold(0, |sum, (idx, &v)| {
      sum
        + if (idx as i32 - k) >= 0 && nums[idx - k as usize] >= v
          || (idx as i32 + k) < nums.len() as i32 && nums[idx + k as usize] >= v
        {
          0
        } else {
          v
        }
    })
  }
}
