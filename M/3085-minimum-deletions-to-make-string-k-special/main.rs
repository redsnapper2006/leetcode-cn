use std::collections::HashMap;
impl Solution {
  pub fn minimum_deletions(word: String, k: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; 26];
    word.as_bytes().iter().for_each(|b| {
      buf[(b - b'a') as usize] += 1;
    });
    buf.sort_unstable();
    let mut left: Vec<i32> = vec![0; 26];
    let mut right: Vec<i32> = vec![0; 26];

    (0..26).for_each(|idx| {
      let mut v: i32 = 0;
      if idx > 0 {
        v = left[idx - 1];
      }
      left[idx] = v + buf[idx];

      let mut vv: i32 = 0;
      if idx > 0 {
        vv = right[buf.len() - idx];
      }
      right[buf.len() - 1 - idx] = vv + buf[buf.len() - 1 - idx];
    });

    let mut ans: i32 = word.len() as i32 + 1;
    (0..26).for_each(|idx| {
      let mut lv: i32 = 0;
      if idx > 0 {
        lv = left[idx - 1];
      }
      let target = buf[idx] + k;
      let ll = match buf.binary_search(&target) {
        Ok(ov) => ov,
        Err(ev) => ev,
      };
      let mut rv: i32 = 0;
      if ll < buf.len() {
        rv = right[ll] - (buf.len() - ll) as i32 * target;
      }
      ans = ans.min(lv + rv);
    });
    ans
  }
}
