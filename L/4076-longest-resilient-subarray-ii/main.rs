impl Solution {
  pub fn resilient_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut m = nums[0] % k;
    let mut cnt = 0i32;
    let mut ans = 1i32;
    for i in 0..nums.len() {
      if nums[i] % k == m {
        cnt += 1;
        if ((cnt - 1) * m) % k == 0 {
          ans = ans.max(cnt);
        }
      } else {
        m = nums[i] % k;
        cnt = 1;
      }
    }
    ans
  }
}
