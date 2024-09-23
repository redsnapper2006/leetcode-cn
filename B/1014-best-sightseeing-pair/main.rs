struct Solution {}

impl Solution {
  pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut prev: i32 = values[0];
    let mut ans: i32 = 0;
    (1..values.len()).for_each(|idx| {
      prev -= 1;
      ans = ans.max(prev + values[idx]);
      prev = prev.max(values[idx]);
    });
    ans
  }
}
