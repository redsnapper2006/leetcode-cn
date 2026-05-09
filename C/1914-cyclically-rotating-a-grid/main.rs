impl Solution {
  pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
    let times = grid.len().min(grid[0].len()) / 2;

    for i in (0..times) {
      let mut temp: Vec<i32> = vec![];
      for ii in (i..grid[0].len() - i) {
        temp.push(grid[i][ii]);
      }
      for ii in (i + 1..grid.len() - i) {
        temp.push(grid[ii][grid[0].len() - 1 - i]);
      }
      for ii in (i..grid[0].len() - 1 - i).rev() {
        temp.push(grid[grid.len() - 1 - i][ii]);
      }
      for ii in (i + 1..grid.len() - 1 - i).rev() {
        temp.push(grid[ii][i]);
      }
      let mut offset = k as usize % temp.len();
      for ii in (i..grid[0].len() - i) {
        ans[i][ii] = temp[offset];
        offset = (offset + 1) % temp.len();
      }
      for ii in (i + 1..grid.len() - i) {
        ans[ii][grid[0].len() - 1 - i] = temp[offset];
        offset = (offset + 1) % temp.len();
      }
      for ii in (i..grid[0].len() - 1 - i).rev() {
        ans[grid.len() - 1 - i][ii] = temp[offset];
        offset = (offset + 1) % temp.len();
      }
      for ii in (i + 1..grid.len() - 1 - i).rev() {
        ans[ii][i] = temp[offset];
        offset = (offset + 1) % temp.len();
      }
    }
    ans
  }
}
