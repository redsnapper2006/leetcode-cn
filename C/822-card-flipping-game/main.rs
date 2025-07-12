use std::collections::HashSet;

impl Solution {
  pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
    let mut bann: HashSet<i32> = HashSet::new();
    let mut exist: HashSet<i32> = HashSet::new();
    let mut max: i32 = 0;
    let mut min: i32 = i32::MAX;
    for i in 0..fronts.len() {
      exist.insert(fronts[i]);
      exist.insert(backs[i]);
      max = max.max(fronts[i]).max(backs[i]);
      min = min.min(fronts[i]).min(backs[i]);
      if fronts[i] == backs[i] {
        bann.insert(fronts[i]);
      }
    }
    let mut v: i32 = min;
    while v <= max {
      if exist.contains(&v) && !bann.contains(&v) {
        return v;
      }
      v += 1;
    }
    0
  }
}
