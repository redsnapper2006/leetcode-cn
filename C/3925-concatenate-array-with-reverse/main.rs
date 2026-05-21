impl Solution {
  pub fn concat_with_reverse(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = nums.clone();
    (0..nums.len()).rev().for_each(|idx| {
      ans.push(nums[idx]);
    });
    ans
  }
}
