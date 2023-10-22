struct Solution {}

impl Solution {
  pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
    let mut satisfaction = satisfaction;
    satisfaction.sort();
    let mut max: i32 = 0;
    let mut times: i32 = 1;
    let mut sum: i32 = 0;
    let mut aggr: i32 = 0;
    satisfaction.iter().rev().for_each(|x| {
      sum += x;
      aggr += sum;
      max = max.max(aggr);
    });
    max
  }
}
