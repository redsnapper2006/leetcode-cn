impl Solution {
  pub fn count_key_changes(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut cnt: i32 = 0;
    (1..bb.len()).for_each(|idx| {
      cnt += if bb[idx] & 31 != bb[idx - 1] & 31 {
        1
      } else {
        0
      };
    });
    cnt
  }

  pub fn count_key_changes2(s: String) -> i32 {
    let bb = s.to_lowercase().as_bytes().to_vec();

    let mut base = bb[0] - b'a';
    let mut cnt: i32 = 0;
    (1..bb.len()).for_each(|idx| {
      if base != bb[idx] - b'a' {
        cnt += 1;
        base = bb[idx] - b'a';
      }
    });
    cnt
  }
}
