impl Solution {
  pub fn min_cost(
    start_pos: Vec<i32>, home_pos: Vec<i32>, row_costs: Vec<i32>, col_costs: Vec<i32>,
  ) -> i32 {
    (start_pos[0].min(home_pos[0])..=start_pos[0].max(home_pos[0]))
      .fold(0, |ans, r| ans + row_costs[r as usize])
      + (start_pos[1].min(home_pos[1])..=start_pos[1].max(home_pos[1]))
        .fold(0, |ans, c| ans + col_costs[c as usize])
      - row_costs[start_pos[0] as usize]
      - col_costs[start_pos[1] as usize]
  }
}
