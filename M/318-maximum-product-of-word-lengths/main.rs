struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn max_product(words: Vec<String>) -> i32 {
    let mut buf: HashMap<i32, i32> = HashMap::new();
    words.iter().for_each(|word| {
      let mut mask: i32 = 0;
      word.as_bytes().iter().for_each(|&c| {
        mask |= 1 << (c - b'a') as i32;
      });
      buf.insert(
        mask,
        std::cmp::max(*buf.get(&mask).unwrap_or(&0), (word.len() as i32)),
      );
    });
    let mut max: i32 = 0;
    for (k1, v1) in buf.iter() {
      for (k2, v2) in buf.iter() {
        if k1 & k2 == 0 {
          max = std::cmp::max(max, v1 * v2);
        }
      }
    }
    max
  }
}
