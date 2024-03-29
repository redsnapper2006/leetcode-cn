impl Solution {
  pub fn maximum_length_substring(s: String) -> i32 {
    let mut buf: Vec<i32> = vec![0; 26];

    let mut max: usize = 0;
    let mut start: usize = 0;
    let bb = s.as_bytes();
    (0..bb.len()).for_each(|bidx| {
      let idx = (bb[bidx] - b'a') as usize;
      buf[idx] += 1;
      if buf[idx] > 2 {
        while bb[start] != bb[bidx] {
          buf[(bb[start] - b'a') as usize] -= 1;
          start += 1;
        }
        buf[(bb[start] - b'a') as usize] -= 1;
        start += 1;
      }
      max = max.max(bidx - start + 1);
    });

    max as _
  }
}
