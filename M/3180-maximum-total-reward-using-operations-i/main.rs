struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
    let mut reward_values = reward_values;
    reward_values.sort_unstable();
    reward_values.dedup();

    let mx = *reward_values.iter().max().unwrap();
    let mut dp: Vec<bool> = vec![false; 2 * mx as usize];
    dp[0] = true;

    reward_values.iter().for_each(|&v| {
      (v..2 * v).rev().for_each(|c| {
        dp[c as usize] |= dp[(c - v) as usize];
      });
    });

    let mut idx: usize = 2 * mx as usize - 1;
    while idx > 0 {
      if dp[idx] {
        return idx as i32;
      }
      idx -= 1;
    }
    0
  }

  pub fn max_total_reward2(reward_values: Vec<i32>) -> i32 {
    let mut reward_values = reward_values;
    reward_values.sort_unstable();
    reward_values.dedup();

    let mut dp: Vec<HashMap<i32, i32>> = vec![HashMap::new(); reward_values.len()];
    let mut mx: i32 = 0;
    fn dfs(reward_values: &Vec<i32>, idx: usize, sum: i32, dp: &mut Vec<HashMap<i32, i32>>) -> i32 {
      if idx >= reward_values.len() {
        return 0;
      }

      if dp[idx].contains_key(&sum) {
        return *dp[idx].get(&sum).unwrap();
      }

      let mut ret: i32 = 0;
      if sum < reward_values[idx] {
        let t = dfs(reward_values, idx + 1, sum + reward_values[idx], dp);
        ret = ret.max(reward_values[idx] + t);
      }

      ret = ret.max(dfs(reward_values, idx + 1, sum, dp));
      dp[idx].insert(sum, ret);
      ret
    };

    dfs(&reward_values, 0, 0, &mut dp);

    *dp[0].get(&0).unwrap()
  }
}
