impl Solution {
  pub fn append_characters(s: String, t: String) -> i32 {
    let sb = s.as_bytes().to_vec();
    let tb = t.as_bytes().to_vec();

    let mut idx: usize = 0;
    for b in &sb {
      if idx >= tb.len() {
        break;
      }
      if *b == tb[idx] {
        idx += 1;
      }
    }

    (tb.len() - idx) as _
  }
}
