impl Solution {
  pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
    let buf = (0..nums.len() - 1)
      .map(|i| nums[i + 1] - nums[i])
      .collect::<Vec<i32>>();

    let mut max = 0;
    let mut idx: usize = 0;
    let mut cnt: i32 = 0;
    let mut sign: i32 = 1;
    while idx < buf.len() {
      if buf[idx] == sign {
        cnt += 1;
        sign *= -1;
      } else {
        if cnt > max {
          max = cnt;
        }
        if buf[idx] == 1 {
          cnt = 1;
          sign = -1;
        } else {
          cnt = 0;
          sign = 1;
        }
      }
      idx += 1;
    }
    if cnt > max {
      max = cnt;
    }
    if max == 0 {
      -1
    } else {
      max + 1
    }
  }
}
