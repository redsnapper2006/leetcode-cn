struct Solution {}

impl Solution {
  pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
    let mut dp: Vec<i64> = vec![0; next_visit.len()];

    (1..next_visit.len()).for_each(|i| {
      dp[i] = (2 + dp[i - 1] * 2 - dp[next_visit[i - 1] as usize] + 1_000_000_007) % 1_000_000_007;
    });

    dp[next_visit.len() - 1] as _
  }
}
