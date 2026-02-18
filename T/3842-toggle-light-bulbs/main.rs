use std::collections::HashMap;

impl Solution {
  pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    bulbs.iter().for_each(|&v| {
      *m.entry(v).or_insert(0) += 1;
    });

    let mut ans: Vec<i32> = vec![];
    for (k, v) in m.iter() {
      if v % 2 == 1 {
        ans.push(*k);
      }
    }
    ans.sort_unstable();
    ans
  }
}
