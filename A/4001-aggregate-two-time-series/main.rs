impl Solution {
  pub fn aggregate_time_series(series1: Vec<Vec<i32>>, series2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut line: Vec<i32> = Vec::new();
    line.extend(series1.iter().map(|s| s[0]));
    line.extend(series2.iter().map(|s| s[0]));
    line.sort_unstable();
    line.dedup();

    let mut idx1: usize = 0;
    let mut idx2: usize = 0;
    let mut ans: Vec<Vec<i32>> = vec![];
    for &l in &line {
      let mut v = 0i32;
      if idx1 < series1.len() {
        v += series1[idx1][1];
      }
      if idx2 < series2.len() {
        v += series2[idx2][1];
      }
      ans.push(vec![l, v]);
      if idx1 < series1.len() && l == series1[idx1][0] {
        idx1 += 1;
      }
      if idx2 < series2.len() && l == series2[idx2][0] {
        idx2 += 1;
      }
    }
    ans
  }
}
