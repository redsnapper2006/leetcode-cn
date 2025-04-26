impl Solution {
  pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
    let mut min: i32 = i32::MAX;
    cost
      .iter()
      .map(|&v| {
        min = min.min(v);
        min
      })
      .collect::<Vec<i32>>()
  }
}
