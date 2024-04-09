impl Solution {
  pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut ret: i32 = 1;

    let mut idx: usize = 1;
    let mut cnt: i32 = 1;
    while idx < nums.len() {
      if nums[idx] > nums[idx - 1] {
        cnt += 1;
      } else {
        ret = ret.max(cnt);
        cnt = 1;
      }
      idx += 1;
    }
    ret = ret.max(cnt);
    idx = 1;
    cnt = 1;
    while idx < nums.len() {
      if nums[idx] < nums[idx - 1] {
        cnt += 1;
      } else {
        ret = ret.max(cnt);
        cnt = 1;
      }
      idx += 1;
    }
    ret = ret.max(cnt);
    ret
  }
}
