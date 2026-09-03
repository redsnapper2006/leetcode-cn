impl Solution {
  pub fn minimum_swaps(nums: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0usize, nums.len() - 1);
    let mut ans: i32 = 0;
    while left < right {
      while left < right && nums[right] == 0 {
        right -= 1;
      }
      while left < right && nums[left] != 0 {
        left += 1;
      }
      ans += if left < right { 1 } else { 0 };
      left += 1;
      if right != 0 {
        right -= 1;
      }
    }
    ans
  }
}
