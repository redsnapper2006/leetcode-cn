impl Solution {
  pub fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut nums = nums;
    let k = k as usize;
    nums.sort_by(|a, b| b.cmp(a));
    nums.dedup();
    nums[0..k.min(nums.len())].to_vec()
  }
}
