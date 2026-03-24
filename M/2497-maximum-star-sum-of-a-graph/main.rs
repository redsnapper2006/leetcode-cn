use std::collections::HashMap;

impl Solution {
  pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();

    edges.iter().for_each(|edge| {
      m.entry(edge[0]).or_insert(vec![]).push(vals[edge[1] as usize]);
      m.entry(edge[1]).or_insert(vec![]).push(vals[edge[0] as usize]);
    });

    let mut ans: i32 = i32::MIN;
    for i in 0..vals.len() {
      let ii = i as i32;
      if !m.contains_key(&ii) {
        ans = ans.max(vals[i]);
        continue;
      }
      let mut e = m.get(&ii).unwrap().clone();
      e.sort_unstable();
      let mut s: usize = 0;
      let mut sum: i32 = vals[i];
      while s < k as usize && s < e.len() && e[e.len() - 1 - s] > 0 {
        sum += e[e.len() - 1 - s];
        s += 1;
      }
      ans = ans.max(sum);
    }
    ans
  }
}
