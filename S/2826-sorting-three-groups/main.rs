struct Solution {}

impl Solution {
  pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; 3]; nums.len()];

    buf[0][0] = 1;
    buf[0][1] = 1;
    buf[0][2] = 1;
    buf[0][nums[0] as usize - 1] = 0;
    (1..nums.len()).for_each(|idx| {
      buf[idx][0] = buf[idx - 1][0];
      if nums[idx] != 1 {
        buf[idx][0] += 1;
      }
      buf[idx][1] = buf[idx - 1][1].min(buf[idx - 1][0]);
      if nums[idx] != 2 {
        buf[idx][1] += 1;
      }
      buf[idx][2] = buf[idx - 1][2].min(buf[idx - 1][1]).min(buf[idx - 1][0]);
      if nums[idx] != 3 {
        buf[idx][2] += 1;
      }
    });

    buf[nums.len() - 1][2]
      .min(buf[nums.len() - 1][1])
      .min(buf[nums.len() - 1][0])
  }
}
