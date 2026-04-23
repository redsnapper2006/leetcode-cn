use std::collections::HashMap;

impl Solution {
  pub fn prefix_connected(words: Vec<String>, k: i32) -> i32 {
    let k = k as usize;
    let mut m: HashMap<String, i32> = HashMap::new();

    words.iter().for_each(|w| {
      if w.len() < k {
        return;
      }
      *m.entry(String::from_utf8(w.as_bytes().to_vec()[0..k].to_vec()).unwrap()).or_insert(0) += 1;
    });

    let mut ans: i32 = 0;
    for (_, v) in m {
      ans += if v > 1 { 1 } else { 0 };
    }
    ans
  }
}
