struct Solution {}

impl Solution {
  pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let sum = nums.iter().sum::<i32>();
    let s = sum - target.abs();
    if s < 0 || s % 2 == 1 {
      return 0;
    }
    let m = s / 2;
    let mut dp: Vec<Vec<i32>> = vec![vec![-1; m as usize + 1]; nums.len()];
    fn dfs(i: i32, c: i32, nums: &Vec<i32>, dp: &mut Vec<Vec<i32>>) -> i32 {
      if i < 0 {
        if c == 0 {
          return 1;
        }
        return 0;
      }
      if dp[i as usize][c as usize] != -1 {
        return dp[i as usize][c as usize];
      }
      let mut v: i32 = 0;
      if c < nums[i as usize] {
        v = dfs(i - 1, c, nums, dp);
      } else {
        v = dfs(i - 1, c - nums[i as usize], nums, dp) + dfs(i - 1, c, nums, dp);
      }
      dp[i as usize][c as usize] = v;
      v
    }
    dfs(nums.len() as i32 - 1, m, &nums, &mut dp)
  }
}

fn main() {
  println!("{}", Solution::find_target_sum_ways(vec![1, 2, 3, 4], 5));
}
