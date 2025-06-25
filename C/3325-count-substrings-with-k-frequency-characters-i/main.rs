impl Solution {
  pub fn number_of_substrings(s: String, k: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; 26];
    let bb = s.as_bytes().to_vec();

    let mut ans: i32 = 0;
    let mut start: usize = 0;
    for i in 0..bb.len() {
      let off = (bb[i] - b'a') as usize;
      buf[off] += 1;
      if buf[off] == k {
        let right = (bb.len() - i) as i32;
        while buf[off] == k {
          ans += right;
          buf[(bb[start] - b'a') as usize] -= 1;
          start += 1;
        }
      }
    }

    ans
  }
}
