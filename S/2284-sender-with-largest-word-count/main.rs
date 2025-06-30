use std::cmp::Ordering;
use std::collections::HashMap;
impl Solution {
  pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
    let mut m: HashMap<String, usize> = HashMap::new();

    for i in 0..messages.len() {
      let ml = messages[i].split(" ").collect::<Vec<_>>().len();
      m.entry(senders[i].clone())
        .and_modify(|x| *x += ml)
        .or_insert(ml);
    }

    let mut ans: String = "".to_string();
    let mut l: usize = 0;
    for (k, v) in m {
      if v > l || v == l && k.cmp(&ans) == Ordering::Greater {
        l = v;
        ans = k.clone();
      }
    }
    ans
  }
}
