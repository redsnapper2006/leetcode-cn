struct Solution {}

use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
  pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut dp: Vec<i32> = vec![0; n];
    (0..n).for_each(|idx| {
      dp[idx] = idx as i32;
    });

    let mut edge: Vec<Vec<i32>> = vec![Vec::new(); n];
    let mut ans: Vec<i32> = Vec::new();
    queries.iter().for_each(|query| {
      let q1 = query[1] as usize;
      let q0 = query[0] as usize;
      edge[q1].push(query[0]);

      if dp[q0] + 1 < dp[q1] {
        dp[q1] = dp[q0] + 1;
        (q1 + 1..n).for_each(|idx| {
          dp[idx] = dp[idx].min(dp[idx - 1] + 1);
          edge[idx].iter().for_each(|&idx2| {
            dp[idx] = dp[idx].min(dp[idx2 as usize] + 1);
          });
        });
      }

      ans.push(dp[n - 1]);
    });

    ans
  }

  pub fn shortest_distance_after_queries2(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut short: Vec<HashSet<i32>> = vec![HashSet::new(); n as usize];
    (0..n - 1).for_each(|idx| {
      short[idx as usize].insert(idx + 1);
    });

    let mut ans: Vec<i32> = Vec::new();
    let mut vis: Vec<i32> = vec![-1; n as usize];
    queries.iter().enumerate().for_each(|(i, query)| {
      short[query[0] as usize].insert(query[1]);

      let mut q: VecDeque<(i32, i32)> = VecDeque::new();
      q.push_back((0, 0));
      while q.len() > 0 {
        let (src, step) = q.pop_front().unwrap();
        if src == n - 1 {
          ans.push(step);
          break;
        }

        short[src as usize].iter().for_each(|&k| {
          if vis[k as usize] != i as i32 {
            vis[k as usize] = i as i32;

            q.push_back((k, step + 1));
          }
        });
      }
    });

    ans
  }
}
