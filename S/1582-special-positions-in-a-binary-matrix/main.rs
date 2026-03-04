impl Solution {
  pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let mut rows: Vec<(i32, usize)> = vec![(0, mat[0].len()); mat.len()];
    let mut cols: Vec<i32> = vec![0; mat[0].len()];
    for r in 0..mat.len() {
      let mut cc: usize = mat[0].len();
      for c in 0..mat[0].len() {
        rows[r].0 += mat[r][c];
        cols[c] += mat[r][c];
        if mat[r][c] == 1 {
          cc = c;
        }
      }
      if rows[r].0 == 1 {
        rows[r].1 = cc;
      }
    }

    rows.iter().fold(0, |sum, row| {
      if row.0 != 1 {
        sum
      } else {
        sum
          + if row.1 == mat[0].len() || cols[row.1] != 1 {
            0
          } else {
            1
          }
      }
    })
  }
}
