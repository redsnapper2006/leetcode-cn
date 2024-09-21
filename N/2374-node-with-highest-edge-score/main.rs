use std::collections::HashMap;
impl Solution {
  pub fn edge_score(edges: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, usize> = HashMap::new();

    edges.iter().enumerate().for_each(|(idx, &v)| {
      m.entry(v).and_modify(|x| *x += idx).or_insert(idx);
    });

    let mut ans: i32 = 0;
    let mut max: usize = 0;
    m.iter().for_each(|(&k, &v)| {
      if v > max || v == max && ans > k {
        ans = k;
        max = v;
      }
    });
    ans
  }
}
