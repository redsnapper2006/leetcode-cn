use std::collections::HashMap;

impl Solution {
  pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
    if changed.len() % 2 != 0 {
      return Vec::new();
    }

    let mut changed = changed;
    changed.sort();

    let mut ret: Vec<i32> = Vec::new();
    let mut m: HashMap<i32, i32> = HashMap::new();
    changed.iter().for_each(|&v| {
      let c = m.entry(v).or_insert(0);
      if *c > 0 {
        *c -= 1;
        return;
      }
      *m.entry(v * 2).or_insert(0) += 1;
      ret.push(v);
    });

    for (_, v) in m {
      if v > 0 {
        return Vec::new();
      }
    }
    ret
  }
}
