impl Solution {
  pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; k as usize + 1];
    let mut ans: usize = 0;
    (0..nums.len()).rev().for_each(|idx| {
      if nums[idx] > k || buf[nums[idx] as usize] != 0 {
        return;
      }
      buf[nums[idx] as usize] = 1;
      ans = ans.min(idx);
    });
    (nums.len() - ans) as _
  }
}
