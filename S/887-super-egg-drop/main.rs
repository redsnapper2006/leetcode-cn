struct Solution {}

impl Solution {
  pub fn super_egg_drop(k: i32, n: i32) -> i32 {
    let n = n as usize;
    let mut dp: Vec<Vec<i32>> = vec![vec![0; k as usize + 1]; n + 1];
    let mut i: usize = 1;
    while i <= n {
      (1..=k as usize).for_each(|j| {
        dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1] + 1;
      });
      if dp[i][k as usize] >= n as i32 {
        return i as i32;
      }

      i += 1;
    }
    -1
  }
}
