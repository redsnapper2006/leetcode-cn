struct Solution {}

impl Solution {
  pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    queries
      .iter()
      .map(|q| {
        points
          .iter()
          .filter(|p| (p[0] - q[0]) * (p[0] - q[0]) + (p[1] - q[1]) * (p[1] - q[1]) <= q[2] * q[2])
          .count() as i32
      })
      .collect()
  }
}
