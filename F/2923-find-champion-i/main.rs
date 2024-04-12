struct Solution {}

impl Solution {
  pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
    grid
      .iter()
      .map(|x| x.into_iter().sum())
      .collect::<Vec<i32>>()
      .iter()
      .enumerate()
      .filter(|(idx, v)| **v == grid.len() as i32 - 1)
      .map(|(idx, _)| idx as i32)
      .collect::<Vec<i32>>()[0]
  }
}
