impl Solution {
  pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
    let mut dp: Vec<i32> = vec![-1; 32];
    let mut ans: Vec<i32> = vec![0; nums.len()];
    for i in (0..nums.len()).rev() {
      let mut idx = i as i32;
      for j in 0..32 {
        if nums[i] & (1 << j) > 0 {
          dp[j] = i as i32;
        } else if dp[j] != -1 {
          idx = idx.max(dp[j]);
        }
      }
      ans[i] = idx - i as i32 + 1;
    }
    ans
  }
}
