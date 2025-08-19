impl Solution {
  pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];
    queries.iter().for_each(|query| {
      dp[query[0] as usize][query[1] as usize] += 1;
      if query[3] < n as i32 - 1 {
        dp[query[0] as usize][query[3] as usize + 1] -= 1;
      }
      if query[2] < n as i32 - 1 {
        dp[query[2] as usize + 1][query[1] as usize] -= 1;
      }
      if query[2] < n as i32 - 1 && query[3] < n as i32 - 1 {
        dp[query[2] as usize + 1][query[3] as usize + 1] += 1;
      }
    });

    for i in 0..n {
      for j in 0..n {
        dp[i][j] += if j > 0 { dp[i][j - 1] } else { 0 } + if i > 0 { dp[i - 1][j] } else { 0 }
          - if i > 0 && j > 0 { dp[i - 1][j - 1] } else { 0 };
      }
    }
    dp
  }

  pub fn range_add_queries2(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];
    queries.iter().for_each(|query| {
      for r in query[0]..=query[2] {
        dp[r as usize][query[1] as usize] += 1;
        if query[3] < n as i32 - 1 {
          dp[r as usize][query[3] as usize + 1] -= 1;
        }
      }
    });

    for i in 0..n {
      for j in 1..n {
        dp[i][j] += dp[i][j - 1];
      }
    }
    dp
  }
}
