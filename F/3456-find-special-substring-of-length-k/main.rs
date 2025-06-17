impl Solution {
  pub fn has_special_substring(s: String, k: i32) -> bool {
    let mut buf: Vec<i32> = vec![0; 26];

    let bb = s.as_bytes().to_vec();
    for idx in (0..bb.len()) {
      buf[(bb[idx] - b'a') as usize] += 1;
      if idx < k as usize - 1 {
        continue;
      }
      let mut valid: bool = true;
      if idx >= k as usize {
        buf[(bb[idx - (k as usize)] - b'a') as usize] -= 1;
      }

      (0..26).for_each(|idx| {
        if buf[idx] == 0 {
          return;
        }
        if buf[idx] != k {
          valid = false;
        }
      });

      if idx >= (k as usize) && bb[idx] == bb[idx - k as usize] {
        valid = false;
      }
      if idx < bb.len() - 1 && bb[idx] == bb[idx + 1] {
        valid = false;
      }
      if valid {
        return true;
      }
    }
    false
  }
}
