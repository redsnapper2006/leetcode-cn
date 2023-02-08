struct Solution {}

struct Hier {
  c: HashMap<String, Hier>,
}

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
  pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    let mut root: Hier = Hier { c: HashMap::new() };
    let mut is_dir: HashSet<String> = HashSet::new();
    for f in &folder {
      let children: Vec<&str> = f.split("/").collect();
      let mut p: &mut Hier = &mut root;
      let mut path: String = "".to_string();
      for c in children.get(1..).unwrap() {
        path += &("/".to_string() + &c.to_string());

        let v = p
          .c
          .entry(path.clone())
          .or_insert(Hier { c: HashMap::new() });
        p = v;
      }
      is_dir.insert(f.to_string());
    }

    let mut ret: Vec<String> = Vec::new();

    unsafe {
      let mut candi: Vec<*const Hier> = vec![&root];
      while candi.len() > 0 {
        let mut t: Vec<*const Hier> = Vec::new();
        for c in candi {
          for (k, v) in &(*c).c {
            if !is_dir.contains(&k.clone()) {
              t.push(v);
            } else {
              ret.push(k.clone());
            }
          }
        }
        candi = t;
      }
    }

    ret
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::remove_subfolders(vec![
      "/a".to_string(),
      "/a/b".to_string(),
      "/c/d".to_string(),
      "/c/d/e".to_string(),
      "/c/f".to_string()
    ])
  );

  println!(
    "{:?}",
    Solution::remove_subfolders(vec![
      "/a".to_string(),
      "/a/b/c".to_string(),
      "/a/b/d".to_string()
    ])
  );

  println!(
    "{:?}",
    Solution::remove_subfolders(vec![
      "/a/b/c".to_string(),
      "/a/b/ca".to_string(),
      "/a/b/d".to_string()
    ])
  );
}
