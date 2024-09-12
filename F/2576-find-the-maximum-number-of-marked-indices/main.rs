struct Solution {}

impl Solution {
  pub fn max_num_of_marked_indices(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut start: usize = 0;
    let mut end: usize = nums.len() / 2;
    let mut ans: i32 = 0;
    while start < nums.len() / 2 && end < nums.len() {
      if nums[start] * 2 <= nums[end] {
        start += 1;
        end += 1;
        ans += 2;
      } else {
        end += 1;
      }
    }
    ans
  }
}
