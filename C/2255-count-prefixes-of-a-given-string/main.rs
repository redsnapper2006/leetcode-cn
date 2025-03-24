struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
    let mut m: HashMap<String, i32> = HashMap::new();
    words.iter().for_each(|word| {
      m.entry(word.clone()).and_modify(|x| *x += 1).or_insert(1);
    });

    let mut idx: usize = 1;
    let mut ans: i32 = 0;
    while idx <= s.len() {
      ans += m.get(&s[..idx]).or(Some(&0)).unwrap();
      idx += 1;
    }
    ans
  }
}
