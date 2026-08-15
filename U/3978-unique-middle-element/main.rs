impl Solution {
  pub fn is_middle_element_unique(nums: Vec<i32>) -> bool {
    nums.iter().fold(0, |cnt, &v| {
      cnt + if v == nums[nums.len() / 2] { 1 } else { 0 }
    }) == 1
  }
}
