impl Solution {
  pub fn limit_occurrences(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut idx: usize = 0;
    let mut base: i32 = 0;
    let mut cnt: i32 = 0;
    let size = nums.len();
    for i in 0..size {
      if nums[i] != base {
        base = nums[i];
        cnt = 1;
      } else {
        cnt += 1;
      }
      if cnt <= k {
        nums[idx] = nums[i];
        idx += 1;
      }
    }
    nums.resize(idx, 0);
    nums
  }
}
