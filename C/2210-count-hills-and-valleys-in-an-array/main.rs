struct Solution {}

impl Solution {
  pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
    let mut buf: Vec<i32> = vec![nums[0]];

    (1..nums.len()).for_each(|idx| {
      if nums[idx] != nums[idx - 1] {
        buf.push(nums[idx]);
      }
    });

    (1..buf.len() - 1)
      .map(|idx| {
        if buf[idx] > buf[idx - 1] && buf[idx] > buf[idx + 1]
          || buf[idx] < buf[idx - 1] && buf[idx] < buf[idx + 1]
        {
          1
        } else {
          0
        }
      })
      .collect::<Vec<i32>>()
      .iter()
      .sum()
  }
}
