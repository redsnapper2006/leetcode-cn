impl Solution {
  pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
    mat
      .iter()
      .enumerate()
      .map(|(idx, row)| (row.iter().sum(), idx as i32))
      .collect::<Vec<(i32, i32)>>()
      .iter()
      .fold(vec![-1, 0], |aggr, (c, i)| {
        if c > aggr[1] || c == aggr[1] && i < aggr[0] {
          vec![i, c]
        } else {
          aggr
        }
      })
  }
}
