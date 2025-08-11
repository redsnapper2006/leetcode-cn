use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
impl Solution {
  pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let M: i64 = 1000000007;
    let n = n as usize;
    let mut edge: HashMap<usize, Vec<(usize, i64)>> = HashMap::new();
    roads.iter().for_each(|road| {
      edge
        .entry(road[0] as usize)
        .and_modify(|x| x.push((road[1] as usize, road[2] as i64)))
        .or_insert(vec![(road[1] as usize, road[2] as i64)]);
      edge
        .entry(road[1] as usize)
        .and_modify(|x| x.push((road[0] as usize, road[2] as i64)))
        .or_insert(vec![(road[0] as usize, road[2] as i64)]);
    });
    let mut dp: Vec<(i64, i64)> = vec![(i64::MAX, 0); n];
    dp[0] = (0, 1);
    // distance, index, prev
    let mut heap: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, 0, 0)));

    while heap.len() > 0 {
      let (dist, cur, _prev) = heap.pop().unwrap().0;
      if cur == n - 1 || dist > dp[cur].0 {
        continue;
      }

      for nxt in edge.get(&cur).unwrap().iter() {
        if dist + nxt.1 <= dp[nxt.0].0 {
          if dist + nxt.1 < dp[nxt.0].0 {
            dp[nxt.0] = (dist + nxt.1, dp[cur].1 % M);
            heap.push(Reverse((dist + nxt.1, nxt.0, cur)));
          } else {
            dp[nxt.0].1 = (dp[nxt.0].1 + dp[cur].1) % M;
          }
        }
      }
    }
    dp[n - 1].1 as i32
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::count_paths(
      7,
      vec![
        vec![0, 6, 7],
        vec![0, 1, 2],
        vec![1, 2, 3],
        vec![1, 3, 3],
        vec![6, 3, 3],
        vec![3, 5, 1],
        vec![6, 5, 1],
        vec![2, 5, 1],
        vec![0, 4, 5],
        vec![4, 6, 2]
      ]
    )
  );
}
