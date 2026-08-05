use std::collections::VecDeque;

impl Solution {
  pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let k = k as usize;
    let mut edge: Vec<Vec<usize>> = vec![vec![]; n];
    let mut ingress: Vec<usize> = vec![0; n];
    for inv in invocations {
      let (inv0, inv1) = (inv[0] as usize, inv[1] as usize);
      edge[inv0].push(inv1);
      ingress[inv1] += 1;
    }

    let mut q: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n];
    q.push_back(k);
    visited[k] = true;

    while let Some(cur) = q.pop_front() {
      for &nxt in &edge[cur] {
        ingress[nxt] -= 1;
        if !visited[nxt] {
          visited[nxt] = true;
          q.push_back(nxt);
        }
      }
    }

    let keep: bool = (0..n).any(|i| visited[i] && ingress[i] > 0);

    let mut ans: Vec<i32> = vec![];
    for i in 0..n {
      if keep || !visited[i] {
        ans.push(i as i32);
      }
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]])
  );
}
