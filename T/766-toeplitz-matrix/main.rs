struct Solution {}

impl Solution {
  pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let row = matrix.len();
    let col = matrix[0].len();
    let mut buf = vec![0; row + col - 1];
    for i in 0..matrix.len() {
      buf[row - 1 - i] = matrix[i][0];
    }
    for i in 1..matrix[0].len() {
      buf[row - 1 + i] = matrix[0][i];
    }
    for i in 0..matrix.len() {
      for j in 0..matrix[0].len() {
        if matrix[i][j] != buf[row - 1 + j - i] {
          return false;
        }
      }
    }
    true
  }
}

fn main() {
  println!("{}", Solution::is_toeplitz_matrix(vec![vec![0; 5]]));
}
