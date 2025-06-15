impl Solution {
  pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut buf: Vec<i32> = vec![0];
    nums.iter().for_each(|&v| {
      sum += v;
      buf.push(sum);
    });

    let mut min: i32 = i32::MAX;
    (1..buf.len()).for_each(|idx| {
      (l..=r).for_each(|diff| {
        if idx as i32 - diff >= 0
          && buf[idx] - buf[(idx as i32 - diff) as usize] > 0
          && buf[idx] - buf[(idx as i32 - diff) as usize] < min
        {
          min = buf[idx] - buf[(idx as i32 - diff) as usize];
        }
      })
    });
    if min == i32::MAX {
      -1
    } else {
      min
    }
  }
}
