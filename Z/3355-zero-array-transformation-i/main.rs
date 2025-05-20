impl Solution {
  pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
    let mut dp: Vec<i32> = vec![0; nums.len() + 1];
    queries.iter().for_each(|q| {
      dp[q[0] as usize] += 1;
      dp[q[1] as usize + 1] -= 1;
    });
    let mut idx: usize = 0;
    while idx < nums.len() {
      if idx > 0 {
        dp[idx] += dp[idx - 1];
      }
      if dp[idx] < nums[idx] {
        return false;
      }
      idx += 1;
    }

    true
  }
}
