use std::collections::HashMap;
impl Solution {
  pub fn max_probability(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start: i32,
    end: i32,
  ) -> f64 {
    let mut visit: HashMap<i32, f64> = HashMap::new();
    let mut edge_map: HashMap<i32, Vec<(i32, f64)>> = HashMap::new();

    edges.iter().enumerate().for_each(|(idx, edge)| {
      let mut v = edge_map.entry(edge[0]).or_insert(Vec::new());
      v.push((edge[1], succ_prob[idx]));
      let mut v1 = edge_map.entry(edge[1]).or_insert(Vec::new());
      v1.push((edge[0], succ_prob[idx]));
    });

    let mut queue: Vec<i32> = vec![start];
    visit.insert(start, 1.0);

    while queue.len() > 0 {
      let mut t: Vec<i32> = Vec::new();
      let mut m: HashMap<i32, f64> = HashMap::new();
      queue.iter().for_each(|current| {
        if !edge_map.contains_key(&current) {
          return;
        }

        edge_map[&current].iter().for_each(|v| {
          if !visit.contains_key(&v.0) || visit[&v.0] < visit[&current] * v.1 {
            visit.insert(v.0, visit[&current] * v.1);
            t.push(v.0);
          }
        });
      });
      queue = t;
    }

    *visit.entry(end).or_insert(0.0)
  }
}
