struct Solution {}

impl Solution {
  pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut left: usize = 0;
    let mut right: usize = mat[0].len() - 1;

    while left < right {
      let mid = left + (right - left) / 2;

      let mut max: i32 = mat[0][mid];
      let mut max_idx: usize = 0;
      (1..mat.len()).for_each(|row| {
        if max < mat[row][mid] {
          max = mat[row][mid];
          max_idx = row;
        }
      });
      if mid == 0 {
        if mat[max_idx][mid + 1] < max {
          return vec![max_idx as i32, 0];
        } else {
          left = 1;
        }
      } else {
        if mat[max_idx][mid + 1] < max && mat[max_idx][mid - 1] < max {
          return vec![max_idx as i32, mid as i32];
        } else if mat[max_idx][mid + 1] > max && mat[max_idx][mid - 1] < max {
          left = mid + 1;
        } else {
          right = mid - 1;
        }
      }
    }
    let mut max: i32 = mat[0][left];
    let mut max_idx: usize = 0;
    (1..mat.len()).for_each(|row| {
      if max < mat[row][left] {
        max = mat[row][left];
        max_idx = row;
      }
    });
    vec![max_idx as i32, left as i32]
  }
}
