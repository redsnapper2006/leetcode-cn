struct Solution {}

impl Solution {
  pub fn find_value_of_partition(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ans: i32 = i32::MAX;
    (1..nums.len()).for_each(|idx| {
      ans = ans.min(nums[idx] - nums[idx - 1]);
    });
    ans
  }
}
