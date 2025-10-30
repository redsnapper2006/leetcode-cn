impl Solution {
  pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
    let mut aggr: Vec<(i64, i64)> = vec![(0, 0); 100001];
    arr.iter().enumerate().for_each(|(idx, &v)| {
      let v = v as usize;
      aggr[v] = (aggr[v].0 + 1, aggr[v].1 + idx as i64 + 1);
    });

    let mut ans: Vec<i64> = vec![];
    let mut sum: Vec<(i64, i64)> = vec![(0, 0); 100001];
    arr.iter().enumerate().for_each(|(idx, &v)| {
      let v = v as usize;
      let idx2 = idx as i64 + 1;
      sum[v] = (sum[v].0 + 1, sum[v].1 + idx2);

      ans.push(
        sum[v].0 * idx2 - sum[v].1 + aggr[v].1
          - sum[v].1
          - (aggr[v].0 - sum[v].0) * idx2,
      );
    });
    ans
  }
}
