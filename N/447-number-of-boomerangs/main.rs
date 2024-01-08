struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    let mut count: i32 = 0;

    for i in 0..points.len() {
      let mut map: HashMap<i32, i32> = HashMap::new();
      for j in 0..points.len() {
        if i == j {
          continue;
        }
        let counter = map
          .entry((points[i][0] - points[j][0]).pow(2) + (points[i][1] - points[j][1]).pow(2))
          .or_insert(0);
        *counter += 1;
      }
      for (_, v) in map.iter() {
        count += v * (v - 1);
      }
    }

    count
  }
}
