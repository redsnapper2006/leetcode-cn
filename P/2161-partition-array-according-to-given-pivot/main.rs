impl Solution {
  pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![pivot; nums.len()];

    let mut left: usize = 0;
    let mut right: usize = nums.len() - 1;

    for i in 0..nums.len() {
      if nums[i] < pivot {
        ans[left] = nums[i];
        left += 1;
      } else if nums[i] > pivot {
        ans[right] = nums[i];
        right -= 1;
      }
    });
    ans[right + 1..].reverse();
    ans
  }
}
