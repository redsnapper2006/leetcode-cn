impl Solution {
  pub fn count_dominant_elements(nums: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let n = nums.len();
    for i in 0..n {
      if nums[i] * 2
        >= if i == 0 { nums[i] } else { nums[i - 1] }
          + if i == n - 1 { nums[i] } else { nums[i + 1] }
      {
        ans += 1;
      }
    }
    ans
  }
}
