impl Solution {
  pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut idx: usize = 0;
    let mut ans: i32 = 0;
    let mut i: i32 = nums[0] - k;
    while i <= nums[nums.len() - 1] + k {
      while nums[idx] + k < i {
        idx += 1;
      }
      if (i - nums[idx]).abs() <= k {
        ans += 1;
        idx += 1;
        i += 1;
        if idx == nums.len() {
          break;
        }
      } else {
        i = nums[idx] - k;
      }
    }
    ans
  }
}
