struct Solution {}

impl Solution {
  pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut start: i32 = new_interval[0];
    let mut end: i32 = new_interval[1];
    let mut idx: usize = 0;
    while idx < intervals.len() && intervals[idx][1] < start {
      res.push(intervals[idx].clone());
      idx += 1;
    }
    while idx < intervals.len() && intervals[idx][0] <= end {
      if intervals[idx][0] < start {
        start = intervals[idx][0];
      }
      if intervals[idx][1] > end {
        end = intervals[idx][1];
      }
      idx += 1;
    }
    res.push(vec![start, end]);
    while idx < intervals.len() {
      res.push(intervals[idx].clone());
      idx += 1;
    }

    res
  }
}
