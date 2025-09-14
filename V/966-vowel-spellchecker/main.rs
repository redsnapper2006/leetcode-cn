use std::collections::HashMap;
impl Solution {
  pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    let mut m0: HashMap<String, String> = HashMap::new();
    let mut m1: HashMap<String, String> = HashMap::new();
    let mut m2: HashMap<String, String> = HashMap::new();

    wordlist.iter().for_each(|word| {
      m0.insert(word.clone(), word.clone());
      if !m1.contains_key(&word.to_lowercase()) {
        m1.insert(word.to_lowercase(), word.clone());
      }
      let binding = word.clone().to_lowercase();
      let bb = binding.as_bytes();
      let mut t: Vec<u8> = vec![];
      for b in bb.iter() {
        if "aeiou".contains(*b as char) {
          t.push(b'a');
        } else {
          t.push(*b);
        }
      }
      let tw = String::from_utf8(t).unwrap();
      // println!("{}", tw);
      if !m2.contains_key(&tw) {
        m2.insert(tw, word.clone());
      }
    });

    queries
      .iter()
      .map(|q| {
        if m0.contains_key(q) {
          return q.clone();
        } else if m1.contains_key(&q.to_lowercase()) {
          return m1.get(&q.to_lowercase()).unwrap().clone();
        } else {
          let binding = q.to_lowercase();
          let bb = binding.as_bytes();
          let mut t: Vec<u8> = vec![];
          for b in bb.iter() {
            if "aeiou".contains(*b as char) {
              t.push(b'a');
            } else {
              t.push(*b);
            }
          }
          let tw = String::from_utf8(t).unwrap();
          if m2.contains_key(&tw) {
            m2.get(&tw).unwrap().clone()
          } else {
            "".to_string()
          }
        }
      })
      .collect::<Vec<String>>()
  }
}
