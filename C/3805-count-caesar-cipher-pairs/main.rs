use std::collections::HashMap;

impl Solution {
  pub fn count_pairs(words: Vec<String>) -> i64 {
    let mut m: HashMap<String, i64> = HashMap::new();
    for w in &words {
      let bb = w.as_bytes();
      let diff = bb[0] as i32 - b'a' as i32;
      *m.entry(
        String::from_utf8(bb.iter().map(|&b| ((b as i32 - diff + 26) % 26) as u8).collect())
          .unwrap(),
      )
      .or_insert(0) += 1;
    }
    m.iter().fold(0i64, |ans, (_, v)| ans + v * (v - 1) / 2)
  }

  pub fn count_pairs2(words: Vec<String>) -> i64 {
    let mut m: HashMap<Vec<i32>, i64> = HashMap::new();
    for w in &words {
      let bb = w.as_bytes();
      let diff = (1..bb.len()).map(|i| (bb[i] as i32 - bb[i - 1] as i32 + 26) % 26).collect();
      *m.entry(diff).or_insert(0) += 1;
    }
    m.iter().fold(0i64, |ans, (_, v)| ans + v * (v - 1) / 2)
  }
}
