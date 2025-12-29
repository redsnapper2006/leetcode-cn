use std::collections::HashMap;

impl Solution {
  pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
    let mut m: HashMap<u8, HashMap<u8, Vec<u8>>> = HashMap::new();
    allowed.iter().for_each(|allow| {
      let bb = allow.as_bytes().to_vec();
      m.entry(bb[0])
        .or_insert(HashMap::new())
        .entry(bb[1])
        .or_insert(vec![])
        .push(bb[2]);
    });

    fn build(stack: &Vec<u8>, m: &HashMap<u8, HashMap<u8, Vec<u8>>>) -> Vec<Vec<u8>> {
      let mut ret: Vec<Vec<u8>> = vec![vec![]];

      for i in 0..stack.len() - 1 {
        if !m.contains_key(&stack[i]) || !m.get(&stack[i]).unwrap().contains_key(&stack[i + 1]) {
          return vec![];
        }
        let mut temp: Vec<Vec<u8>> = vec![];
        for n in m.get(&stack[i]).unwrap().get(&stack[i + 1]).unwrap() {
          for r in &ret {
            let mut t = r.clone();
            t.push(*n);
            temp.push(t);
          }
        }
        ret = temp;
      }

      ret
    }

    fn dfs(stack: &Vec<u8>, m: &HashMap<u8, HashMap<u8, Vec<u8>>>) -> bool {
      if stack.len() == 1 {
        return true;
      }

      for n in build(stack, m) {
        if dfs(&n, m) {
          return true;
        }
      }

      false
    }
    dfs(&bottom.as_bytes().to_vec(), &m)
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::pyramid_transition(
      "BCD".to_string(),
      vec![
        "BCC".to_string(),
        "CDE".to_string(),
        "CEA".to_string(),
        "FFF".to_string()
      ]
    )
  );

  println!(
    "{}",
    Solution::pyramid_transition(
      "AAAA".to_string(),
      vec![
        "AAB".to_string(),
        "AAC".to_string(),
        "BCD".to_string(),
        "DEF".to_string(),
        "BBE".to_string()
      ]
    )
  );
}
