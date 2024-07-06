struct Solution {}

impl Solution {
  pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
    let mut start: usize = 0;
    let mut end: usize = 1;
    let mut ans: i64 = 0;
    while end < nums.len() {
      if nums[end] == nums[end - 1] {
        let n = (end - start) as i64;
        ans += n * (n + 1) / 2;
        start = end;
      }
      end += 1;
    }
    let n = (end - start) as i64;
    ans + n * (n + 1) / 2
  }
}
