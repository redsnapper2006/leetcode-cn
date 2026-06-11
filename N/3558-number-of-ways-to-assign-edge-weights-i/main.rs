use std::collections::HashMap;

impl Solution {
  pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
    const M: i32 = 1000000007;
    let mut edges = edges;
    for e in &mut edges {
      e.sort_unstable();
    }
    edges.sort_unstable();

    let mut mx: usize = 0;
    let mut m: HashMap<i32, usize> = HashMap::new();
    edges.iter().for_each(|edge| {
      let r = *m.entry(edge[0]).or_insert(0);
      let c = *m.entry(edge[1]).or_insert(0);
      let c = c.max(r + 1);
      mx = mx.max(c);
      m.insert(edge[1], c);
    });

    let mut ans: i32 = 1;
    for i in 1..mx {
      ans = (ans * 2) % M;
    }
    ans
  }
}
