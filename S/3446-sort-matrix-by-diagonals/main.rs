impl Solution {
  pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut grid = grid;
    (0..grid.len()).for_each(|r| {
      let mut row = r;
      let mut buf: Vec<i32> = vec![];
      for col in 0..grid[0].len() {
        if row >= grid.len() {
          break;
        }
        buf.push(grid[row][col]);
        row += 1;
      }
      buf.sort_unstable();
      row = r;
      for col in 0..grid[0].len() {
        if row >= grid.len() {
          break;
        }
        grid[row][col] = buf[buf.len() - 1 - col];
        row += 1;
      }
    });

    (1..grid[0].len()).for_each(|c| {
      let mut col = c;
      let mut buf: Vec<i32> = vec![];
      for row in 0..grid.len() {
        if col >= grid[0].len() {
          break;
        }
        buf.push(grid[row][col]);
        col += 1;
      }
      buf.sort_unstable();
      col = c;
      for row in 0..grid.len() {
        if col >= grid[0].len() {
          break;
        }
        grid[row][col] = buf[row];
        col += 1;
      }
    });
    grid
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::sort_matrix(vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]])
  );
}
