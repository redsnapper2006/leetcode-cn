struct Solution {}

impl Solution {
  pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp: Vec<i32> = vec![0; target as usize + 1];

    dp[0] = 1;

    (1..=target).for_each(|i| {
      nums.iter().for_each(|&num| {
        if i >= num {
          dp[i as usize] += dp[(i - num) as usize];
        }
      });
    });

    dp[target as usize]
  }
}
