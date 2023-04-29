struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn odd_string(words: Vec<String>) -> String {
    let mut m :HashMap<Vec<i32>,Vec<usize>> = HashMap::new();
    let b = words.iter().enumerate().for_each(|(index,word)|{
      let bb = word.as_bytes().to_vec();
      let r = (1..word.len()).map(|idx| {
        (bb[idx] - bb[idx-1]) as i32
      }).collect::<Vec<i32>>();
      let mut v =  m.entry(r).or_insert(Vec::new());
      v.push(index);
    });

    let mut idx : usize = words.len();
    m.iter().for_each(|(k,v)|{
      if v.len() == 1 {
        idx = v[0];
      }
    });
    words[idx].clone()
  }
}
