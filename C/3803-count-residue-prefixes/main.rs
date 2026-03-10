impl Solution {
  pub fn residue_prefixes(s: String) -> i32 {
    let mut ch: Vec<i32> = vec![0; 26];
    let bb = s.as_bytes().to_vec();
    let mut cnt: usize = 0;
    let mut ans: i32 = 0;
    for i in 0..bb.len() {
      let idx = (bb[i] - b'a') as usize;
      bb[idx] += 1;
      if bb[idx] == 1 {
        cnt += 1;
      }
      if (i + 1) % 3 == cnt {
        ans += 1;
      }
    }
    ans
  }
}
