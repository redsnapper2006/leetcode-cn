impl Solution {
  pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
    let mut rows: Vec<i64> = vec![0; grid.len()];
    let mut cols: Vec<i64> = vec![0; grid[0].len()];
    let mut sum: i64 = 0;

    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        let v = grid[r][c] as i64;
        rows[r] += v;
        cols[c] += v;
        sum += v;
      }
    }

    for r in 1..grid.len() {
      rows[r] += rows[r - 1];
    }
    for c in 1..grid[0].len() {
      cols[c] += cols[c - 1];
    }

    sum % 2 == 0
      && (match rows.binary_search(&(sum / 2)) {
        Ok(ov) => true,
        Err(ev) => false,
      } || match cols.binary_search(&(sum / 2)) {
        Ok(ov) => true,
        Err(ev) => false,
      })
  }
}
