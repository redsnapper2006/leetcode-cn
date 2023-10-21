struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let mut visited: Vec<bool> = vec![false; n as usize];
    let mut color: i32 = 0;
    let mut color_count: Vec<i32> = Vec::new();

    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for edge in edges.iter() {
      let a = edge[0];
      let b = edge[1];
      graph.entry(a).or_insert(vec![]).push(b);
      graph.entry(b).or_insert(vec![]).push(a);
    }

    (0..n).for_each(|idx| {
      if visited[idx as usize] {
        return;
      }

      color += 1;
      let mut stack: Vec<i32> = vec![idx];
      let mut count: i32 = 0;
      while stack.len() > 0 {
        let node = stack.pop().unwrap();
        if visited[node as usize] {
          continue;
        }
        visited[node as usize] = true;
        count += 1;

        match graph.get(&node) {
          None => continue,
          Some(neighbors) => {
            neighbors.iter().for_each(|neighbor| {
              if !visited[*neighbor as usize] {
                stack.push(*neighbor);
              }
            });
          }
        }
      }
      color_count.push(count);
    });

    let mut result: i64 = 0;
    let mut sum: i64 = 0;
    (0..color_count.len()).for_each(|i| {
      result += color_count[i] as i64 * sum;
      sum += color_count[i] as i64;
    });

    result
  }
}
