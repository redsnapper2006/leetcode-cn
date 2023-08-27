struct Solution {}

impl Solution {
  pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut pp = intervals.clone();
    pp.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut start = pp[0][0];
    let mut end = pp[0][1];
    (1..pp.len()).for_each(|idx| {
      if pp[idx][0] > end {
        res.push(vec![start, end]);
        start = pp[idx][0];
        end = pp[idx][1];
      } else if pp[idx][1] > end {
        end = pp[idx][1];
      }
    });
    res.push(vec![start, end]);
    res
  }
}
