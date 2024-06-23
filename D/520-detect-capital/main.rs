struct Solution {}

impl Solution {
  pub fn detect_capital_use(word: String) -> bool {
    let bb = word.as_bytes().to_vec();
    let mut cnt: i32 = 0;
    let mut cap: bool = false;
    bb.iter().enumerate().for_each(|(idx, &b)| {
      if b >= b'A' && b <= b'Z' {
        cnt += 1;
      }
      if idx == 0 && b >= b'A' && b <= b'Z' {
        cap = true;
      }
    });
    cnt == bb.len() as i32 || cnt == 0 || bb.len() > 1 && cnt == 1 && cap
  }
}
