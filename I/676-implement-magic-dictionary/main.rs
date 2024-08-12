struct MagicDictionary {
  d: Vec<String>,
}

impl MagicDictionary {
  fn new() -> Self {
    MagicDictionary { d : Vec::new()}
  }

  fn build_dict(&mut self, dictionary: Vec<String>) {
    self.d = dictionary;
  }

  fn search(&self, search_word: String) -> bool {
    let mut idx : usize = 0;
    while idx < self.d.len() {
      let candi = &self.d[idx];
      if candi.len() == search_word.len() {
        let mut offset : usize = 0;
        let mut cnt : i32 = 0;
        while offset < candi.len() {
          if candi.as_bytes()[offset] != search_word.as_bytes()[offset] {
            cnt += 1;
            if cnt > 1 {
              break;
            }
          }
          offset+=1;
        }
        if cnt == 1 {
          return true;
        }
      }
      idx+=1;
    }
    false
  }
}

fn main() {
  let mut obj = MagicDictionary::new();
  obj.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
  println!(
    "{} {} {} {}",
    obj.search("hello".to_string()),
    obj.search("hhllo".to_string()),
    obj.search("leetcoded".to_string()),
    obj.search("leetcodf".to_string())
  );
}
