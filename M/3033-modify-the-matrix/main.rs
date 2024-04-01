impl Solution {
  pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut maxs: Vec<i32> = Vec::new();
    (0..matrix[0].len()).for_each(|col| {
      let mut max: i32 = -1;
      (0..matrix.len()).for_each(|row| {
        max = max.max(matrix[row][col]);
      });
      maxs.push(max);
    });

    let mut matrix = matrix;
    (0..matrix.len()).for_each(|row| {
      (0..matrix[0].len()).for_each(|col| {
        if matrix[row][col] == -1 {
          matrix[row][col] = maxs[col];
        }
      });
    });
    matrix
  }
}
