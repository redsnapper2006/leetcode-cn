impl Solution {
  pub fn max_vowels(s: String, k: i32) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut mx: i32 = 0;
    let mut sum: i32 = 0;
    for idx in 0..bb.len() {
      let b = bb[idx];
      sum += if b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u' {
        1
      } else {
        0
      };

      if idx >= k as usize {
        let b = bb[idx - k as usize];
        sum -= if b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u' {
          1
        } else {
          0
        };
      }
      mx = mx.max(sum);
    }

    mx
  }
}
