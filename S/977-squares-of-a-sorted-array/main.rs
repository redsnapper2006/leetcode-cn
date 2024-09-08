struct Solution {}

impl Solution {
  pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; nums.len()];
    let mut start: usize = 0;
    let mut end: usize = nums.len() - 1;
    let mut idx: usize = nums.len() - 1;
    while start <= end {
      let mut v = nums[end] * nums[end];
      if nums[start] * nums[start] < nums[end] * nums[end] {
        end -= 1;
      } else {
        v = nums[start] * nums[start];
        start += 1;
      };
      ans[idx] = v;
      idx -= 1;
    }
    ans
  }
}
