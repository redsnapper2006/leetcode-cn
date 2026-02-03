impl Solution {
  pub fn minimum_cost(nums: Vec<i32>) -> i32 {
    let (mn1, mn2) = (1..nums.len()).fold((i32::MAX, i32::MAX), |(mn1, mn2), idx| {
      match (mn1 >= nums[idx], mn2 >= nums[idx]) {
        (true, true) => (mn2, nums[idx]),
        (true, false) => (nums[idx], mn2),
        (_, _) => (mn1, mn2),
      }
    });

    nums[0] + mn1 + mn2
  }

  pub fn minimum_cost2(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut sum: i32 = nums[0];
    nums.remove(0);
    nums.sort();
    sum + nums[0] + nums[1]
  }
}

struct Solution {}
