struct Solution {}

impl Solution {
  pub fn two_egg_drop(n: i32) -> i32 {
    (((n * 8 + 1) as f64).sqrt().ceil() / 2 as f64) as i32
  }

  pub fn two_egg_drop2(n: i32) -> i32 {
    let mut dp: Vec<i32> = vec![-1; n as usize + 1];

    fn dfs(i: i32, dp: &mut Vec<i32>) -> i32 {
      if dp[i as usize] != -1 {
        return dp[i as usize];
      }

      if i == 0 {
        dp[i as usize] = 0;
        return 0;
      }

      let mut min: i32 = i32::MAX;
      (1..=i).for_each(|j| {
        min = min.min(j.max(dfs(i - j, dp) + 1));
      });

      dp[i as usize] = min;
      min
    }
    dfs(i, &mut dp)
  }
}
