struct Solution {}

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
  pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut rs: HashSet<i32> = HashSet::new();

    (0..restricted.len()).for_each(|idx| {
      rs.insert(restricted[idx]);
    });
    edges.iter().for_each(|edge| {
      let e1 = edge[0];
      let e2 = edge[1];

      let ee1 = m.entry(e1).or_insert(Vec::new());
      ee1.push(e2);
      let ee2 = m.entry(e2).or_insert(Vec::new());
      ee2.push(e1);
    });

    let mut q: VecDeque<i32> = VecDeque::new();
    let mut v: HashSet<i32> = HashSet::new();
    q.push_back(0);

    let mut cnt: i32 = 0;
    while q.len() > 0 {
      let c = q.pop_front().unwrap();
      cnt += 1;
      v.insert(c);

      let nc = m.entry(c).or_insert(Vec::new());
      nc.iter().for_each(|&n| {
        if v.contains(&n) || rs.contains(&n) {
          return;
        }
        q.push_back(n);
      });
    }

    cnt
  }
}

fn main() {
  println!(
    "{}",
    Solution::reachable_nodes(5, vec![vec![1, 2], vec![2, 3]], Vec::new())
  );
}
