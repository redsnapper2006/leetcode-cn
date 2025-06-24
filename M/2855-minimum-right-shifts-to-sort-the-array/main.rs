impl Solution {
  pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
    let mut cnt: i32 = 0;
    let mut base: i32 = nums[0];
    let mut off: usize = nums.len();
    (1..nums.len()).for_each(|idx| {
      if base > nums[idx] {
        cnt += 1;
        off = idx;
      }
      base = nums[idx];
    });
    if cnt > 1 || cnt != 0 && nums[nums.len() - 1] > nums[0] {
      -1
    } else {
      (nums.len() - off) as i32
    }
  }
}
