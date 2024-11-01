struct Solution {}

impl Solution {
  pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
    let len = energy_drink_a.len();
    let mut dp: Vec<Vec<i64>> = vec![vec![0; 2]; len];
    dp[0][0] = energy_drink_a[0] as i64;
    dp[0][1] = energy_drink_b[0] as i64;
    (1..len).for_each(|idx| {
      let oa = dp[idx - 1][0];
      let ob = dp[idx - 1][1];
      dp[idx][0] = ob.max(oa + energy_drink_a[idx] as i64);
      dp[idx][1] = oa.max(ob + energy_drink_b[idx] as i64);
    });
    dp[len - 1][0].max(dp[len - 1][1])
  }
}
