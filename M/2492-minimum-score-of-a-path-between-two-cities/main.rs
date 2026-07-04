use std::collections::{HashMap, VecDeque};

impl Solution {
  pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut edge: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    roads.iter().for_each(|road| {
      edge.entry(road[0]).or_insert(HashMap::new()).insert(road[1], road[2]);
      edge.entry(road[1]).or_insert(HashMap::new()).insert(road[0], road[2]);
    });

    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(1);

    let mut visit: Vec<bool> = vec![false; n as usize + 1];
    let mut ans: i32 = i32::MAX;
    while let Some(node) = q.pop_front() {
      visit[node as usize] = true;
      if !edge.contains_key(&node) {
        continue;
      }

      for (&nxt, &dist) in edge.get(&node).unwrap() {
        ans = ans.min(dist);
        if !visit[nxt as usize] {
          q.push_back(nxt);
        }
      }
    }
    ans
  }
}
