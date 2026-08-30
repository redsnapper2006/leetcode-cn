impl Solution {
  pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let (mut mn, mut mn_idx, mut mx, mut mx_idx) = (i32::MAX, 0usize, i32::MIN, 0usize);
    for i in 0..n {
      if mn > nums[i] {
        mn = nums[i];
        mn_idx = i;
      }
      if mx < nums[i] {
        mx = nums[i];
        mx_idx = i;
      }
    }

    let left = mn_idx.min(mx_idx);
    let right = mn_idx.max(mx_idx);
    (left + 1 + n - right).min(right + 1).min(n - left) as i32
  }
}
