impl Solution {
  pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    for i in 1..triangle.len() {
      for j in 0..triangle[i].len() {
        triangle[i][j] += (if j > 0 {
          triangle[i - 1][j - 1]
        } else {
          i32::MAX
        })
        .min(if j < triangle[i - 1].len() {
          triangle[i - 1][j]
        } else {
          i32::MAX
        });
      }
    }
    *triangle[triangle.len() - 1].iter().min().unwrap()
  }
}
