impl Solution {
  pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let mut prev: i32 = -1;
    for i in 0..nums.len() {
      if nums[i] == 0 {
        continue;
      }
      if prev != -1 && i as i32 - prev < k + 1 {
        return false;
      }
      prev = i as i32;
    }
    true
  }
}
