impl Solution {
  pub fn min_moves2(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len();
    let median = *nums.select_nth_unstable(n / 2).1;
    nums.iter().fold(0, |ans, &v| ans + (v - median).abs())
  }
}
