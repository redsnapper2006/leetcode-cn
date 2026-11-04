impl Solution {
  pub fn peak_valley_difference(nums: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    for i in 1..nums.len() - 1 {
      if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
        ans += nums[i];
      } else if nums[i] < nums[i - 1] && nums[i] < nums[i + 1] {
        ans -= nums[i];
      }
    }
    ans
  }
}
