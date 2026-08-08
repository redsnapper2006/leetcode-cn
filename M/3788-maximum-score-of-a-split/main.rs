impl Solution {
  pub fn maximum_score(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let total = nums.iter().map(|&x| x as i64).sum::<i64>();
    let mut suffix_sum: i64 = 0;
    let mut suffix_min: i64 = i64::MAX;
    let mut ans: i64 = i64::MIN;
    for i in (1..n).rev() {
      suffix_sum += nums[i] as i64;
      suffix_min = suffix_min.min(nums[i] as i64);
      ans = ans.max(total - suffix_sum - suffix_min);
    }

    ans
  }
}
