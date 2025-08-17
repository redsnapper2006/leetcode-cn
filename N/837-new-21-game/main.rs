impl Solution {
  pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    let n = n as usize;
    let k = k as usize;
    let max_pts = max_pts as usize;
    let mut dp: Vec<f64> = vec![0.0; (k + max_pts)];
    let mut sum: f64 = 0.0;
    for i in k..k + max_pts {
      dp[i] = if i <= n { 1.0 } else { 0.0 };
      sum += dp[i];
    }
    for i in (0..k).rev() {
      dp[i] = sum / (max_pts as f64);
      sum -= dp[i + max_pts] - dp[i];
    }
    dp[0]
  }
}
