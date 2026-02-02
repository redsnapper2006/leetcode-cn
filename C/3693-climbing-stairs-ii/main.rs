impl Solution {
  pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![0; costs.len()];
    for i in 0..costs.len() {
      let p1 = if i > 2 { dp[i - 3] + 9 } else { 9 };
      let p2 = if i > 1 { dp[i - 2] + 4 } else { 4 };
      let p3 = if i > 0 { dp[i - 1] + 1 } else { 1 };
      dp[i] = p1.min(p2).min(p3) + costs[i];
    }
    dp[dp.len() - 1]
  }
}
