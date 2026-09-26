use std::collections::HashMap;
impl Solution {
  pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    let mut m: HashMap<String, Vec<u8>> = knowledge
      .into_iter()
      .map(|x| (x[0].clone(), x[1].as_bytes().to_vec()))
      .collect();

    let bb = s.as_bytes().to_vec();
    let mut ans: Vec<u8> = vec![];
    let mut prev = 0usize;
    for i in 0..bb.len() {
      match bb[i] {
        b'(' => prev = ans.len(),
        b')' => {
          let sub = m
            .get(&String::from_utf8(ans[prev..].to_vec()).unwrap())
            .unwrap_or(&vec![b'?'])
            .clone();
          ans.resize(prev, b'?');
          ans.extend(sub);
        }
        _ => ans.push(bb[i]),
      }
    }

    String::from_utf8(ans).unwrap()
  }
}
