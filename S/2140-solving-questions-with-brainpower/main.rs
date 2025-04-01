impl Solution {
  pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let mut dp: Vec<Vec<i64>> = vec![vec![0; questions.len() + 1]; 2];

    let mut ans: i64 = 0;
    (0..questions.len()).for_each(|idx| {
      let q_idx = idx + 1;
      dp[1][q_idx] = dp[0][q_idx].max(dp[1][idx]);
      dp[0][q_idx] = dp[0][q_idx].max(dp[1][idx]) + questions[idx][0] as i64;
      ans = ans.max(dp[0][q_idx]).max(dp[1][q_idx]);
      let next = q_idx + questions[idx][1] as usize + 1;
      if next < dp[0].len() {
        dp[0][next] = dp[0][next].max(dp[0][q_idx]);
      }
    });
    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]])
  );

  println!(
    "{}",
    Solution::most_points(vec![
      vec![1, 1],
      vec![2, 2],
      vec![3, 3],
      vec![4, 4],
      vec![5, 5]
    ])
  );

  println!(
    "{}",
    Solution::most_points(vec![
      vec![12, 46],
      vec![78, 19],
      vec![63, 15],
      vec![79, 62],
      vec![13, 10]
    ])
  );
}
