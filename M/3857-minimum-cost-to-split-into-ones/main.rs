impl Solution {
  pub fn min_cost(n: i32) -> i32 {
    if n <= 2 {
      return n - 1;
    }

    let n = n as usize;
    let mut dp: Vec<i32> = vec![0; (n + 1) / 2 + 1];
    dp[1] = 0;
    dp[2] = 1;
    for i in 3..=(n + 1) / 2 {
      let left = i / 2;
      let right = i - left;
      dp[i] = left as i32 * right as i32 + dp[left] + dp[right];
    }
    (n / 2) as i32 * (n - n / 2) as i32 + dp[n / 2] + dp[n - n / 2]
  }
}
