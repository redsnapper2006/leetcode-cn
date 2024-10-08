struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut m: HashMap<String, i32> = HashMap::new();
    paths.iter().for_each(|path| {
      let a = path[0].clone();
      let b = path[1].clone();
      m.entry(a).and_modify(|x| *x += 1).or_insert(1);
      m.entry(b).and_modify(|x| *x += 1).or_insert(0);
    });

    let mut ans: String = "".to_string();
    m.into_iter().for_each(|(k, v)| {
      if v == 0 {
        ans = k.clone();
      }
    });
    ans
  }
}
