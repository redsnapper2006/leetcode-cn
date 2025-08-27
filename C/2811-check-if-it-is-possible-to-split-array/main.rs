impl Solution {
  pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
    if nums.len() <= 2 {
      return true;
    }
    for i in 1..nums.len() {
      if nums[i] + nums[i - 1] >= m {
        return true;
      }
    }
    false
  }

  pub fn can_split_array2(nums: Vec<i32>, m: i32) -> bool {
    let total: i32 = nums.iter().sum();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; nums.len()]; nums.len()];
    fn dfs(nums: &Vec<i32>, s: usize, e: usize, sum: i32, m: i32, dp: &mut Vec<Vec<i32>>) -> bool {
      if dp[s][e] != 0 {
        return dp[s][e] == 2;
      }
      if s == e || s + 1 == e {
        dp[s][e] = 2;
        return true;
      }

      let mut ans: bool = false;
      if sum - nums[s] >= m {
        ans |= dfs(nums, s + 1, e, sum - nums[s], m, dp);
      }
      if sum - nums[e] >= m {
        ans |= dfs(nums, s, e - 1, sum - nums[e], m, dp);
      }

      dp[s][e] = if ans { 2 } else { 1 };
      ans
    }

    dfs(&nums, 0, nums.len() - 1, total, m, &mut dp)
  }
}
