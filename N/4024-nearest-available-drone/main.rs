impl Solution {
  pub fn nearest_drone(drones: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
    drones
      .iter()
      .enumerate()
      .fold((-1i32, i32::MAX), |(idx, dist), (ii, d)| {
        let di = (d[0] - target[0]).abs() + (d[1] - target[1]).abs();
        if di <= d[2] && di < dist {
          (ii as i32, di)
        } else {
          (idx, dist)
        }
      })
      .0
  }
}
