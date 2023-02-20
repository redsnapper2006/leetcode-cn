struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
    if suits.iter().all(|&x| x == suits[0]) {
      return "Flush".to_string();
    }
    let mut m: HashMap<i32, i32> = HashMap::new();
    for r in ranks {
      let mut v = m.entry(r).or_insert(0);
      *v += 1;
    }
    let values = m.into_values().collect::<Vec<i32>>();
    if values.iter().any(|&x| x >= 3) {
      return "Three of a Kind".to_string();
    }
    if values.iter().any(|&x| x == 2) {
      return "Pair".to_string();
    }
    "High Card".to_string()
  }
}
