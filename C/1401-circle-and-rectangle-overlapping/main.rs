struct Solution {}

impl Solution {
  pub fn check_overlap(
    radius: i32,
    x_center: i32,
    y_center: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
  ) -> bool {
    let mut dist: i32 = 0;
    if (x_center < x1) || (x_center > x2) {
      dist += ((x1 - x_center) * (x1 - x_center)).min((x2 - x_center) * (x2 - x_center));
    }
    if (y_center < y1) || (y_center > y2) {
      dist += ((y1 - y_center) * (y1 - y_center)).min((y2 - y_center) * (y2 - y_center));
    }
    dist <= radius * radius
  }
}
