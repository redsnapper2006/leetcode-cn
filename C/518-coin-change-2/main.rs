struct Solution {}

impl Solution {
  pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![0; amount as usize + 1];
    dp[0] = 1;

    coins.iter().for_each(|&c| {
      (c..=amount).for_each(|i| {
        dp[i as usize] += dp[(i - c) as usize];
      });
    });

    dp[amount as usize]
  }
}
