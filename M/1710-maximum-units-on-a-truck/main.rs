struct Solution {}

use std::cmp::Ordering;
impl Solution {
  pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut boxes = box_types.clone();
    boxes.sort_by(|a, b| b[1].cmp(&a[1]));

    let mut trunk: i32 = 0;
    let mut sum: i32 = 0;
    for boxs in boxes {
      let mut v = boxs[0];
      if trunk + v > truck_size {
        v = truck_size - trunk;
      }

      trunk += v;
      sum += boxs[1] * v;
      if trunk >= truck_size {
        break;
      }
    }
    sum
  }
}
