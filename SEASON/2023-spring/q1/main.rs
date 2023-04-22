struct Solution {}

impl Solution {
  pub fn supply_wagon(supplies: Vec<i32>) -> Vec<i32> {
    let mut len: usize = supplies.len() / 2;
    let mut m = supplies;

    while m.len() > len {
      let mut min_idx: usize = 0;
      let mut min: i32 = m[0] + m[1];
      (0..m.len() - 1).for_each(|idx| {
        if m[idx] + m[idx + 1] < min {
          min_idx = idx;
          min = m[idx] + m[idx + 1];
        }
      });
      m[min_idx] = min;
      m.remove(min_idx + 1);
    }
    m
  }
}
