impl Solution {
  pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let mut nums = nums.iter().map(|x| (x * x) as i64).collect::<Vec<i64>>();
    nums.sort_unstable();

    let half = nums.len() / 2;
    nums
      .iter()
      .enumerate()
      .fold(0, |sum, (idx, &n)| sum + if idx < half { -n } else { n })
  }
}
