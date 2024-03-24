struct Solution {}

impl Solution {
  pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp: Vec<i32> = vec![0; amount as usize + 1];
    dp[0] = 1;
    (0..amount as usize).for_each(|v| {
      if dp[v] == 0 {
        return;
      }
      coins.iter().for_each(|&c| {
        if c <= amount
          && c + v as i32 <= amount
          && (dp[c as usize + v] == 0 || dp[c as usize + v] > dp[v] + 1)
        {
          dp[c as usize + v] = dp[v] + 1;
        }
      });
    });

    dp[amount as usize] - 1
  }
}
