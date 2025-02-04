struct Solution {}

impl Solution {
  pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut even: usize = 0;
    let mut odd: usize = 1;

    while even < nums.len() && odd < nums.len() {
      while even < nums.len() && nums[even] % 2 == 0 {
        even += 2;
      }
      while odd < nums.len() && nums[odd] % 2 == 1 {
        odd += 2;
      }
      if even >= nums.len() || odd >= nums.len() {
        break;
      }
      let t = nums[even];
      nums[even] = nums[odd];
      nums[odd] = t;
      even += 2;
      odd += 2;
    }
    nums
  }
}
