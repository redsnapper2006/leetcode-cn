struct Solution {}

impl Solution {
  pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![0; 121];
    let mut sum: Vec<i32> = vec![0; 121];
    ages.iter().for_each(|&age| {
      dp[age as usize] += 1;
    });
    (1..121).for_each(|idx| {
      sum[idx] = sum[idx - 1] + dp[idx];
    });
    let mut ans: i32 = 0;
    ages.iter().for_each(|&age| {
      let min = (age >> 1) + 7;
      if min < age {
        ans += sum[age as usize] - sum[min as usize] - 1;
      }
    });
    ans
  }
}
