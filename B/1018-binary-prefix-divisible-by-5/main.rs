impl Solution {
  pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    nums
      .iter()
      .fold((vec![], 0), |mut ag, n| {
        ag.1 = (ag.1 * 2 + n) % 5;
        ag.0.push(ag.1 == 0);
        ag
      })
      .0
  }
}
