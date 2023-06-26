impl Solution {
  pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut buf: Vec<(i32, usize)> = mat
      .iter()
      .enumerate()
      .map(|(idx, row)| (row.iter().sum(), idx))
      .collect::<Vec<(i32, usize)>>();
    buf.sort_by(|a, b| {
      if a.0 == b.0 {
        a.1.cmp(&b.1)
      } else {
        b.0.cmp(&a.0)
      }
    });
    vec![buf[0].1 as i32, buf[0].0]
  }
}
