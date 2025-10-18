impl Solution {
  pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mn = nums.iter().min().unwrap() - k;
    let mx = nums.iter().max().unwrap() + k;
    let mut idx: usize = 0;
    let mut ans: i32 = 0;
    let mut i: i32 = mn;
    while i <= mx {
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
