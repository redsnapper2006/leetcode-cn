impl Solution {
  pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut row: Vec<i32> = vec![0; rows];
    let mut col: Vec<i32> = vec![0; cols];
    for (i, r) in grid.iter().enumerate() {
      for (j, &v) in r.iter().enumerate() {
        row[i] += v;
        col[j] += v;
      }
    }

    let mut total: i32 = 0;
    for (i, r) in grid.iter().enumerate() {
      for (j, &v) in r.iter().enumerate() {
        if v == 1 && (row[i] >= 2 || col[j] >= 2) {
          total += 1;
        }
      }
    }
    total
  }
}
