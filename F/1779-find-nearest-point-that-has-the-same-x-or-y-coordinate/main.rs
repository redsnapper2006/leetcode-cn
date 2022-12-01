struct Solution {}

impl Solution {
  pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    let mut ret: i32 = -1;
    let mut manhan: i32 = 1 << 31 - 1;
    for (i, point) in points.iter().enumerate() {
      if point[0] != x && point[1] != y {
        continue;
      }
      let distance = (point[0] - x).abs() + (point[1] - y).abs();
      if distance < manhan {
        manhan = distance;
        ret = i as i32;
      }
    }
    ret
  }
}
