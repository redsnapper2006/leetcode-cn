struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn similar_pairs(words: Vec<String>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    words
      .iter()
      .map(|word| {
        let mut v: i32 = 0;
        word.as_bytes().iter().for_each(|b| {
          v |= (1 << (b - 'a' as u8));
        });
        let mut c = m.entry(v).or_insert(0);
        *c += 1;
        *c - 1
      })
      .collect::<Vec<i32>>()
      .iter()
      .sum()
  }
}
