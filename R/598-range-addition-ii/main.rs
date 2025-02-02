struct Solution {}

impl Solution {
  pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
    let mut ops = ops;
    ops.push(vec![m, n]);
    ops.iter().min_by(|x, y| x[0].cmp(&y[0])).unwrap()[0]
      * ops.iter().min_by(|x, y| x[1].cmp(&y[1])).unwrap()[1]
  }
}
