struct Solution {}

impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
      return nums[0];
    }
    let mut buf: Vec<i32> = vec![0; nums.len()];

    buf[0] = nums[0];
    let mut max: i32 = nums[0];
    (1..nums.len()).for_each(|idx| {
      if buf[idx - 1] + nums[idx] > nums[idx] {
        buf[idx] = buf[idx - 1] + nums[idx];
      } else {
        buf[idx] = nums[idx];
      }
      if buf[idx] > max {
        max = buf[idx];
      }
    });

    max
  }
}
