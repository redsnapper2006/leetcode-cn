
impl Solution {
  pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut mat = mat;
    let rn = mat.len();
    let cn = mat[0].len();
    (0..rn).rev().for_each(|i| {
      let mut buf = vec![];
      let mut col_idx = 0;
      for j in i..rn {
        if col_idx < cn {
          buf.push(mat[j][col_idx]);
          col_idx += 1;
        }
      }
      buf.sort();
      let mut col_idx = 0;
      let mut idx = 0;
      for j in i..rn {
        if col_idx < cn {
          mat[j][col_idx] = buf[idx];
          col_idx += 1;
          idx += 1;
        }
      }
    });
    (1..cn).for_each(|i| {
      let mut buf = vec![];
      let mut row_idx = 0;
      for j in i..cn {
        if row_idx < rn {
          buf.push(mat[row_idx][j]);
          row_idx += 1;
        }
      }
      buf.sort();
      let mut row_idx = 0;
      let mut idx = 0;
      for j in i..cn {
        if row_idx < rn {
          mat[row_idx][j] = buf[idx];
          row_idx += 1;
          idx += 1;
        }
      }
    });
    mat
  }
}
