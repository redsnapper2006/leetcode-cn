use std::collections::HashSet;
impl Solution {
  pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let mut buf: Vec<i32> = vec![-1; graph.len()];

    fn dfs(idx: usize, buf: &mut Vec<i32>, graph: &Vec<Vec<i32>>, color: i32) -> bool {
      if buf[idx] != -1 {
        if buf[idx] != color {
          return false;
        }
        return true;
      }

      buf[idx] = color;
      for n in graph[idx].iter() {
        if !dfs(*n as usize, buf, graph, 1 - color) {
          return false;
        }
      }
      true
    }

    for idx in (0..graph.len()) {
      let mut color = buf[idx];
      if color == -1 {
        color = 0;
      }
      if !dfs(idx, &mut buf, &graph, color) {
        return false;
      }
    }

    true
  }
}
