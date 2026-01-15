impl Solution {
  pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    points
      .iter()
      .fold((0, &points[0]), |(sum, prev), point| {
        (
          sum + (point[0] - prev[0]).abs().max((point[1] - prev[1]).abs()),
          point,
        )
      })
      .0
  }
}
