struct Solution {}

impl Solution {
  pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut x: Vec<i32> = points.iter().map(|v| v[0]).collect::<Vec<i32>>();
    x.sort();
    *(0..x.len())
      .map(|i| match i {
        0 => 0,
        _ => x[i] - x[i - 1],
      })
      .collect::<Vec<i32>>()
      .iter()
      .max()
      .unwrap() as i32
  }
}
