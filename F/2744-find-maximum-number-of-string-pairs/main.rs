struct Solution {}

impl Solution {
  pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
    let mut map = std::collections::HashMap::new();
    for word in words {
      let mut chars = word.chars().collect::<Vec<char>>();
      chars.sort();
      let key = chars.into_iter().collect::<String>();
      *map.entry(key).or_insert(0) += 1;
    }
    map.values().map(|&v| v / 2).sum()
  }
}
