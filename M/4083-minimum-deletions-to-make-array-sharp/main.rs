impl Solution {
  pub fn min_deletions(nums: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let mut base = nums[0];
    for i in 1..nums.len() {
      if nums[i] == base {
        ans += 1;
      } else {
        base = nums[i];
      }
    }
    ans
  }
}
