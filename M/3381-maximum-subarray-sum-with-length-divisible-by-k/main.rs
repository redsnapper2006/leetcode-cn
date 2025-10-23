impl Solution {
  pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut buf: Vec<i64> = vec![0; k];
    let mut sum: i64 = 0;
    for i in 0..k - 1 {
      sum += nums[i] as i64;
      buf[(i + 1) % k] = sum;
    }
    let mut ans: i64 = i64::MIN;
    for i in k - 1..nums.len() {
      sum += nums[i] as i64;
      ans = ans.max(sum - buf[(i + 1) % k]);
      buf[(i + 1) % k] = buf[(i + 1) % k].min(sum);
    }
    ans
  }
}
