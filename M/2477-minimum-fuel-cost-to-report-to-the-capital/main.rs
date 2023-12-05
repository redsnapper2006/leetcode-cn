struct Solution {}

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
  fn dfs(
    idx: i32,
    buf: &HashMap<i32, Vec<i32>>,
    visited: &mut HashSet<i32>,
    seats: i32,
    res: &mut i64,
  ) -> i64 {
    let mut node: i64 = 0;
    buf[&idx].iter().for_each(|&next| {
      if visited.contains(&next) {
        return;
      }
      visited.insert(next);
      node += Self::dfs(next, buf, visited, seats, res);
    });
    node += 1;
    if idx != 0 {
      *res += (node + seats as i64 - 1) / seats as i64;
    }
    node
  }

  pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let mut buf: HashMap<i32, Vec<i32>> = HashMap::new();
    roads.iter().for_each(|r| {
      let (a, b) = (r[0], r[1]);
      buf.entry(a).or_insert(Vec::new()).push(b);
      buf.entry(b).or_insert(Vec::new()).push(a);
    });
    let mut visited: HashSet<i32> = HashSet::new();
    visited.insert(0);

    if !buf.contains_key(&0) {
      return 0;
    }
    let mut res: i64 = 0;
    Self::dfs(0, &buf, &mut visited, seats, &mut res);
    res
  }
}

fn main() {
  println!(
    "{}",
    Solution::minimum_fuel_cost(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5)
  );
  println!(
    "{}",
    Solution::minimum_fuel_cost(
      vec![
        vec![3, 1],
        vec![3, 2],
        vec![1, 0],
        vec![0, 4],
        vec![0, 5],
        vec![4, 6]
      ],
      2
    )
  );
}
