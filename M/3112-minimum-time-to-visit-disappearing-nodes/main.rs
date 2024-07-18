struct Solution {}

use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
  pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
    let mut m: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    edges.iter().for_each(|edge| {
      let e1 = edge[0];
      let e2 = edge[1];
      let w = edge[2];
      m.entry(e1)
        .and_modify(|x| x.push((e2, w)))
        .or_insert(vec![(e2, w)]);
      m.entry(e2)
        .and_modify(|x| x.push((e1, w)))
        .or_insert(vec![(e1, w)]);
    });

    let mut ans: Vec<i32> = vec![-1; n as usize];
    ans[0] = 0;
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back((0, 0));
    while q.len() > 0 {
      let (candi, w) = q.pop_front().unwrap();
      if w > ans[candi as usize] {
        continue;
      }
      match m.get(&candi) {
        Some(vv) => {
          vv.iter().for_each(|(nn, next_w)| {
            let next = *nn as usize;
            if w + *next_w < disappear[next] && (ans[next] < 0 || w + *next_w < ans[next]) {
              ans[next] = w + *next_w;
              q.push_back((*nn, w + *next_w));
            }
          });
        }
        _ => (),
      };
    }
    ans
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::minimum_time(
      3,
      vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
      vec![1, 1, 5]
    )
  );
  println!(
    "{:?}",
    Solution::minimum_time(
      3,
      vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
      vec![1, 3, 5]
    )
  );
  println!(
    "{:?}",
    Solution::minimum_time(
      7,
      vec![vec![1, 4, 3], vec![3, 4, 2], vec![2, 5, 5], vec![3, 3, 10]],
      vec![10, 1, 13, 1, 7, 1, 19]
    )
  );
}
