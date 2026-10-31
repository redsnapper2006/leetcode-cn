impl Solution {
  pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
    const M: i32 = 1_000_000_007;
    let (m, n) = (grid.len(), grid[0].len());

    let mut prev = vec![(0i32, 0i32); n];
    let mut curr = vec![(0i32, 0i32); n];
    for c in 1..n {
      prev[c].0 = 1;
      if grid[0][c] == 1 {
        break;
      }
    }

    let mut row_hit = false;
    for r in 1..m {
      curr[0] = (0, 1);
      if grid[r - 1][0] == 1 || row_hit {
        curr[0] = (0, 0);
        row_hit = true;
      }
      for c in 1..n {
        let (la, lb) = curr[c - 1];
        let left = lb + if grid[r][c - 1] == 0 { la } else { 0 };

        let (ua, ub) = prev[c];
        let up = ua + if grid[r - 1][c] == 0 { ub } else { 0 };

        curr[c] = (left % M, up % M);
      }
      std::mem::swap(&mut prev, &mut curr);
    }

    (prev[n - 1].0 + prev[n - 1].1) % M
  }
}
