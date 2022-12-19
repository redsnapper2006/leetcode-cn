struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut edge_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for edge in edges {
      edge_map
        .entry(edge[0])
        .and_modify(|x| (*x).push(edge[1]))
        .or_insert(vec![edge[1]]);
      edge_map
        .entry(edge[1])
        .and_modify(|x| (*x).push(edge[0]))
        .or_insert(vec![edge[0]]);
    }

    let mut visited: HashMap<i32, i32> = HashMap::new();
    let mut candis: Vec<i32> = vec![source];
    while candis.len() > 0 {
      let mut t: Vec<i32> = Vec::new();
      for candi in candis {
        if candi == destination {
          return true;
        }
        visited.insert(candi, 1);
        let next = edge_map.get(&candi).unwrap();
        for n in next {
          if visited.contains_key(&n) {
            continue;
          }
          t.push(*n);
        }
      }
      candis = t;
    }
    false
  }
}
