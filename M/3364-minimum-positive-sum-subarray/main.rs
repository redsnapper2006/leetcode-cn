impl Solution {
  pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let (l, r) = (l as usize, r as usize);
    let n = nums.len();
    let mut sum: i32 = 0;
    let mut buf: Vec<i32> = vec![0];
    nums.iter().for_each(|&v| {
      sum += v;
      buf.push(sum);
    });

    let mut min: i32 = i32::MAX;
    (1..buf.len()).for_each(|idx| {
      (l..=r).for_each(|diff| {
        let offset = idx - diff;
        if idx >= diff && buf[idx] - buf[offset] > 0 && buf[idx] - buf[offset] < min {
          min = buf[idx] - buf[offset];
        }
      })
    });
    if min == i32::MAX { -1 } else { min }
  }
}
