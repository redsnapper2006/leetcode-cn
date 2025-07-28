use std::collections::HashSet;
struct MagicDictionary {
  s: HashSet<String>,
}

impl MagicDictionary {
  fn new() -> Self {
    MagicDictionary { s: HashSet::new() }
  }

  fn build_dict(&mut self, dictionary: Vec<String>) {
    dictionary.iter().for_each(|dict| {
      let bb = dict.as_bytes().to_vec();

      for i in 0..bb.len() {
        for j in 0..26 {
          let r = b'a' + j as u8;
          if r == bb[i] {
            continue;
          }
          let mut t = bb.clone();
          t[i] = r;
          self.s.insert(String::from_utf8(t).unwrap());
        }
      }
    });
  }

  fn search(&self, search_word: String) -> bool {
    self.s.contains(&search_word)
  }
}
