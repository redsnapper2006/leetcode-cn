struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn longest_word(words: Vec<String>) -> String {
    let mut dict: HashSet<String> = HashSet::new();
    words.iter().for_each(|word| {
      dict.insert(word.to_string());
    });

    fn dfs(word: &str, dict: &mut HashSet<String>) -> bool {
      if word.len() == 0 {
        return true;
      }

      for i in 1..=word.len() {
        if dict.contains(&word[..i]) && dfs(&word[i..], dict) {
          return true;
        }
      }
      false
    }

    let mut res: String = "".to_string();
    words.iter().for_each(|word| {
      dict.remove(word);
      if dfs(word, &mut dict) {
        if word.len() > res.len() || (word.len() == res.len() && *word < res) {
          res = word.to_string();
        }
      }
      dict.insert(word.to_string());
    });

    res
  }
}

fn main() {
  println!(
    "{}",
    Solution::longest_word(vec![
      "qlmql".to_string(),
      "qlmqlmqqlqmqqlq".to_string(),
      "mqqlqmqqlqmqqlq".to_string(),
      "mqqlq".to_string(),
      "mqqlqlmlsmqq".to_string(),
      "qmlmmmmsm".to_string(),
      "lmlsmqq".to_string(),
      "slmsqq".to_string(),
      "mslqssl".to_string(),
      "mqqlqmqqlq".to_string()
    ])
  );
}
