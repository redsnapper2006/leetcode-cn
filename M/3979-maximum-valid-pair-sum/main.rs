impl Solution {
  pub fn max_valid_pair_sum(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut mx = 0i32;
    let mut ans = 0i32;
    for i in k..nums.len() {
      mx = mx.max(nums[i - k]);
      ans = ans.max(mx + nums[i]);
    }
    ans
  }
}
