impl Solution {
  pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let mut nums = nums;
    nums.sort_unstable_by_key(|&n| n.abs());

    let half = nums.len() / 2;
    -nums[..half].iter().map(|&x| (x as i64).pow(2)).sum::<i64>()
      + nums[half..].iter().map(|&x| (x as i64).pow(2)).sum::<i64>()
  }
}
