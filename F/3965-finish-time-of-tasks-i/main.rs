impl Solution {
  pub fn finish_time(n: i32, edges: Vec<Vec<i32>>, base_time: Vec<i32>) -> i64 {
    let mut nodes: Vec<Vec<usize>> = vec![vec![]; n as usize];
    edges.iter().for_each(|edge| {
      nodes[edge[0] as usize].push(edge[1] as usize);
    });

    fn recur(node: usize, nodes: &Vec<Vec<usize>>, base: &Vec<i32>) -> i64 {
      if nodes[node].is_empty() {
        return base[node] as i64;
      }

      let mut mx: i64 = i64::MIN;
      let mut mn: i64 = i64::MAX;
      for &nxt in &nodes[node] {
        let v = recur(nxt, nodes, base);
        mx = mx.max(v);
        mn = mn.min(v);
      }

      (mx << 1) - mn + base[node] as i64
    }

    recur(0, &nodes, &base_time)
  }
}
