struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    for (idx, kn) in key_name.iter().enumerate() {
      let mut t = m.entry(kn.clone()).or_insert(Vec::new());
      t.push(key_time[idx].clone());
    }

    let mut ret: Vec<String> = Vec::new();
    for (k, v) in m.iter_mut() {
      v.sort();
      if (2..v.len())
        .map(|idx| {
          let t1h: i32 = v[idx - 2].get(0..2).unwrap().parse().unwrap();
          let t1m: i32 = v[idx - 2].get(3..).unwrap().parse().unwrap();
          let t2h: i32 = v[idx].get(0..2).unwrap().parse().unwrap();
          let t2m: i32 = v[idx].get(3..).unwrap().parse().unwrap();
          if t1h == t2h || t2h - t1h == 1 && t2m <= t1m {
            return true;
          }
          false
        })
        .any(|x| x)
      {
        ret.push(k.to_string());
      }
    }
    ret.sort();
    ret
  }
}
