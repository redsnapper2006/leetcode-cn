struct Solution {}

impl Solution {
  pub fn sum_counts(nums: Vec<i32>) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; 101]; nums.len() + 1];

    (0..nums.len()).for_each(|idx| {
      (0..101).for_each(|c| {
        buf[idx + 1][c] = buf[idx][c];
      });

      buf[idx + 1][nums[idx] as usize] += 1;
    });

    let mut ret: i32 = 0;
    (0..buf.len()).for_each(|idx| {
      (idx + 1..buf.len()).for_each(|idx2| {
        let mut sum: i32 = 0;
        (0..101).for_each(|v| {
          if buf[idx2][v] - buf[idx][v] > 0 {
            sum += 1;
          }
        });
        ret += sum * sum;
        ret %= 1000000007;
      });
    });
    ret
  }
}
