struct Solution {}

impl Solution {
  pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
    let sum_grid: Vec<Vec<i32>> = grid
      .iter()
      .map(|row| {
        let mut sum: i32 = 0;
        let mut sum_row: Vec<i32> = Vec::new();
        for i in 0..row.len() {
          sum += row[i];
          sum_row.push(sum);
        }
        sum_row
      })
      .collect::<Vec<Vec<i32>>>();
    // println!("{:?}", sum_grid);
    (2..sum_grid.len())
      .map(|row| {
        let r = row;
        let size = sum_grid[r].len();
        (2..size)
          .map(|c| {
            let mut left: i32 = 0;
            if c > 2 {
              left = sum_grid[r - 2][c - 3] + sum_grid[r][c - 3];
            }
            sum_grid[r - 2][c] + sum_grid[r][c] + grid[r - 1][c - 1] - left
          })
          .collect::<Vec<i32>>()
      })
      .flatten()
      .max()
      .unwrap()
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_sum(vec![
      vec![6, 2, 1, 3],
      vec![4, 2, 1, 5],
      vec![9, 2, 8, 7],
      vec![4, 1, 2, 9]
    ])
  );
}
