struct Solution {}

impl Solution {
  pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
    nums
      .iter()
      .fold(
        (Vec::new(), 0 as i64, 0),
        |(mut aggr_vec, aggr_sum, max), &v| {
          aggr_vec.push(aggr_sum + (max.max(v) + v) as i64);
          (aggr_vec, aggr_sum + (max.max(v) + v) as i64, max.max(v))
        },
      )
      .0
  }
}
