struct Solution {}

impl Solution {
  pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
    let mut reward_values = reward_values;
    reward_values.sort_unstable();
    reward_values.dedup();

    let mx = reward_values[reward_values.len() - 1];
    if reward_values.len() > 1 && reward_values[reward_values.len() - 2] == mx - 1 {
      return 2 * mx - 1;
    }
    let mut dp: Vec<bool> = vec![false; 2 * mx as usize];
    dp[0] = true;
    reward_values.iter().for_each(|&v| {
      (v..2 * v).rev().for_each(|candi| {
        dp[candi as usize] |= dp[(candi - v) as usize];
      });
    });

    let mut idx = 2 * mx - 1;
    while idx > 0 {
      if dp[idx as usize] {
        return idx;
      }
      idx -= 1;
    }
    0
  }
}
