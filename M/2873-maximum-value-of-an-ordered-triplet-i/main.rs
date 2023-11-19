struct Solution {}

impl Solution {
  pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let mut buf: Vec<[i32; 2]> = vec![[0; 2]; nums.len()];

    let mut max: i32 = 0;
    (0..nums.len()).for_each(|idx| {
      buf[idx][0] = max;
      if max < nums[idx] {
        max = nums[idx];
      }
    });
    max = 0;
    (0..nums.len()).rev().for_each(|idx| {
      buf[idx][1] = max;
      if max < nums[idx] {
        max = nums[idx];
      }
    });

    let mut ret: i64 = 0;
    (0..nums.len()).for_each(|i| {
      if (buf[i][0] as i64 - nums[i] as i64) * buf[i][1] as i64 > 0 {
        ret = std::cmp::max(ret, (buf[i][0] as i64 - nums[i] as i64) * buf[i][1] as i64);
      }
    });
    ret
  }
}
