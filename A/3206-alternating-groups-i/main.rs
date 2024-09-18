struct Solution {}

impl Solution {
  pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
    (0..colors.len())
      .map(|idx| {
        if colors[idx % colors.len()] != colors[(idx + 1) % colors.len()]
          && colors[idx % colors.len()] == colors[(idx + 2) % colors.len()]
        {
          1
        } else {
          0
        }
      })
      .collect::<Vec<i32>>()
      .iter()
      .sum()
  }
}
