impl Solution {
  pub fn maximum_median_sum(nums: Vec<i32>) -> i64 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut sum: i64 = 0;
    for i in (0..nums.len() / 3) {
      sum += nums[nums.len() - 2 * (i + 1)];
    }
    sum
  }
}
