impl Solution {
  pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    (0..n).for_each(|r| {
      let (mut s, mut e) = (0, n - 1);
      while s < e {
        (matrix[r][s], matrix[r][e]) = (matrix[r][e], matrix[r][s]);
        s += 1;
        e -= 1;
      }
    });
    (0..n).for_each(|r| {
      (0..n).for_each(|c| {
        if r + c >= n - 1 {
          return;
        }
        (matrix[r][c], matrix[n - 1 - c][n - 1 - r]) = (matrix[n - 1 - c][n - 1 - r], matrix[r][c]);
      });
    });
  }
}
