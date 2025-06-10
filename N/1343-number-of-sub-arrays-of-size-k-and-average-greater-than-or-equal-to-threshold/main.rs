impl Solution {
  pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let mut buf: Vec<i32> = vec![];

    let mut sum: i32 = 0;
    arr.iter().for_each(|&v| {
      sum += v;
      buf.push(sum);
    });

    let mut ans: i32 = 0;
    (k as usize - 1..buf.len()).for_each(|idx| {
      ans += if buf[idx]
        - (if (idx as i32) < k {
          0
        } else {
          buf[idx - k as usize]
        })
        >= k * threshold
      {
        1
      } else {
        0
      };
    });
    ans
  }
}
