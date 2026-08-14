impl Solution {
  pub fn maximum_length_substring(s: String) -> i32 {
    let mut cnt: [i32; 26] = [0i32; 26];

    let mut max_len: usize = 0;
    let mut start: usize = 0;
    let bb = s.as_bytes();
    for bidx in 0..bb.len() {
      let idx = (bb[bidx] - b'a') as usize;
      cnt[idx] += 1;
      if cnt[idx] > 2 {
        while bb[start] != bb[bidx] {
          cnt[(bb[start] - b'a') as usize] -= 1;
          start += 1;
        }
        cnt[idx] -= 1;
        start += 1;
      }
      max_len = max_len.max(bidx - start + 1);
    }

    max_len as _
  }
}
