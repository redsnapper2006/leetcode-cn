impl Solution {
  pub fn is_array_special(nums: Vec<i32>) -> bool {
    let mut idx: usize = 1;
    while idx < nums.len() {
      if nums[idx] % 2 == nums[idx - 1] % 2 {
        return false;
      }
      idx += 1;
    }
    true
  }
}
