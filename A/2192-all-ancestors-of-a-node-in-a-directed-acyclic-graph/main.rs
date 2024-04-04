struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut visited: Vec<bool> = vec![false; n as usize];
    let mut egress: HashMap<i32, Vec<i32>> = HashMap::new();

    edges.iter().for_each(|edge| {
      egress.entry(edge[1]).or_insert(Vec::new()).push(edge[0]);
    });

    let mut ret: Vec<Vec<i32>> = vec![Vec::new(); n as usize];

    fn dfs(
      idx: i32,
      egress: &HashMap<i32, Vec<i32>>,
      visited: &mut Vec<bool>,
      ret: &mut Vec<Vec<i32>>,
    ) -> Vec<i32> {
      if visited[idx as usize] || !egress.contains_key(&idx) {
        visited[idx as usize] = true;
        let mut t = ret[idx as usize].clone();
        t.push(idx);
        return t;
      }
      let mut aggr: Vec<i32> = Vec::new();
      egress.get(&idx).unwrap().iter().for_each(|&parent| {
        let mut par_ret = dfs(parent, egress, visited, ret);
        aggr.append(&mut par_ret);
      });
      visited[idx as usize] = true;
      aggr.sort();
      aggr.dedup();
      ret[idx as usize] = aggr;
      let mut t = ret[idx as usize].clone();
      t.push(idx);
      t
    }

    (0..n).for_each(|idx| {
      dfs(idx, &egress, &mut visited, &mut ret);
    });
    ret
  }
}
