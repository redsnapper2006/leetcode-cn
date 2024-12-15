struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn min_set_size(arr: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    arr.iter().for_each(|&v| {
      m.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });
    let mut v = Vec::from_iter(m.values());
    v.sort_unstable();
    let mut idx: usize = v.len() - 1;
    let mut s: i32 = 0;
    while idx > 0 {
      s += v[idx];
      if s * 2 >= arr.len() as i32 {
        break;
      }
      idx -= 1;
    }
    (v.len() - idx) as i32
  }
}
