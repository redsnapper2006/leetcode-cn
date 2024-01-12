struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let mut m1: HashMap<String, i32> = HashMap::new();
    let mut m2: HashMap<String, i32> = HashMap::new();
    words1.iter().for_each(|w| {
      let count = m1.entry(w.to_string()).or_insert(0);
      *count += 1;
    });
    words2.iter().for_each(|w| {
      let count = m2.entry(w.to_string()).or_insert(0);
      *count += 1;
    });
    m1.iter()
      .filter(|(k, v)| **v == 1 && *m2.entry(k.to_string()).or_insert(0) == 1)
      .count() as i32
  }
}
