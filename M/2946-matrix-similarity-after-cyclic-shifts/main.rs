impl Solution {
  pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
    let col_cnt = mat[0].len();
    let k = k as usize % col_cnt;
    let mut row: usize = 0;
    while row < mat.len() {
      let kk = if row % 2 == 0 { col_cnt - k } else { k };
      let mut col: usize = 0;
      while col < mat[0].len() {
        if mat[row][col] != mat[row][((col + kk) % col_cnt)] {
          return false;
        }
        col += 1;
      }

      row += 1;
    }
    true
  }
}
