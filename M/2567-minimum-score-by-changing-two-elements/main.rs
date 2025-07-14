impl Solution {
  pub fn minimize_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ans: i32 = i32::MAX;
    for i in 0..=2 {
      ans = ans.min(nums[nums.len() - 3 + i] - nums[i]);
    }
    ans
  }
}
