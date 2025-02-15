struct Solution {}

impl Solution {
  pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    (0..grid[0].len())
      .map(|col| {
        let mut row: usize = 0;
        let mut col2 = col as i32;
        while row < grid.len() {
          let prev = col2;
          col2 += grid[row][col2 as usize];
          if col2 < 0
            || col2 >= grid[0].len() as i32
            || grid[row][prev as usize] != grid[row][col2 as usize]
          {
            return -1;
          }
          row += 1;
        }
        col2
      })
      .collect::<Vec<i32>>()
  }
}
