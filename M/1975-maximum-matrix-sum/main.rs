impl Solution {
  pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut ans: i64 = 0;
    let mut cnt: i64 = 0;
    let mut mn: i64 = i64::MAX;
    for r in 0..matrix.len() {
      for c in 0..matrix[0].len() {
        let v = matrix[r][c].abs() as i64;
        ans += v;
        cnt += if matrix[r][c] <= 0 { 1 } else { 0 };
        mn = mn.min(v);
      }
    }
    ans - if cnt % 2 == 1 { mn * 2 } else { 0 }
  }
}
