impl Solution {
  pub fn minimum_buckets(hamsters: String) -> i32 {
    let mut bb = hamsters.as_bytes().to_vec();
    let mut ans: i32 = 0;
    for i in 0..bb.len() {
      if bb[i] == b'H' {
        if i > 0 && bb[i - 1] == b'F' {
          continue;
        }
        if i < bb.len() - 1 && bb[i + 1] == b'.' {
          bb[i + 1] = b'F';
          ans += 1;
        } else if i > 0 && bb[i - 1] == b'.' {
          ans += 1;
        } else {
          return -1;
        }
      }
    }
    ans
  }
}
