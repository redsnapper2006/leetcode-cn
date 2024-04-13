use std::collections::HashMap;
impl Solution {
  pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut egress: HashMap<i32, i32> = HashMap::new();

    edges.iter().for_each(|edge| {
      *egress.entry(edge[1]).or_insert(0) += 1;
    });

    let mut buf: Vec<i32> = vec![0; n as usize];

    (0..n).for_each(|v| {
      buf[v as usize] = *egress.entry(v).or_insert(0);
    });
    let parent = buf
      .into_iter()
      .enumerate()
      .filter(|(idx, x)| *x == 0)
      .map((|(idx, x)| idx as i32))
      .collect::<Vec<i32>>();

    if parent.len() > 1 {
      -1
    } else {
      parent[0]
    }
  }
}
