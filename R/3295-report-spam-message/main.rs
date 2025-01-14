struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
    let mut m: HashMap<String, i32> = HashMap::new();
    banned_words.iter().for_each(|word| {
      m.insert(word.clone(), 1);
    });

    message
      .iter()
      .map(|msg| if m.contains_key(msg) { 1 } else { 0 })
      .sum::<i32>()
      > 1
  }
}
