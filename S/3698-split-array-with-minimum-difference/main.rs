impl Solution {
  pub fn split_array(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut right: Vec<(i64, bool)> = vec![(0, true); n];
    right[n - 1] = (nums[n - 1] as i64, true);
    for i in (0..nums.len() - 1).rev() {
      right[i].0 += right[i + 1].0 + nums[i] as i64;
      right[i].1 = right[i + 1].1 & (nums[i] > nums[i + 1]);
    }

    let mut ans = i64::MAX;
    let mut sum = nums[0] as i64;
    let mut valid: bool = true;
    for i in 1..nums.len() {
      if !valid {
        break;
      }
      if right[i].1 {
        ans = ans.min((sum - right[i].0).abs());
      }
      valid &= nums[i] > nums[i - 1];
      sum += nums[i] as i64;
    }
    match ans {
      i64::MAX => -1,
      _ => ans,
    }
  }
}
