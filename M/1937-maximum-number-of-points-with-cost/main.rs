impl Solution {
  pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
    let mut dp: Vec<i64> = vec![0; points[0].len()];
    for i in 0..points.len() {
      for j in 0..points[0].len() {
        dp[j] += points[i][j] as i64;
      }
      let mut mx: i64 = 0;
      let mut left: Vec<i64> = vec![0; points[0].len()];
      for j in 0..points[0].len() {
        mx = dp[j].max(mx - 1);
        left[j] = mx;
      }
      mx = 0;
      let mut right: Vec<i64> = vec![0; points[0].len()];
      for j in (0..points[0].len()).rev() {
        mx = dp[j].max(mx - 1);
        right[j] = mx;
      }
      for j in 0..points[0].len() {
        dp[j] = left[j].max(right[j]);
      }
    }
    dp.iter().max().unwrap()
  }
}
