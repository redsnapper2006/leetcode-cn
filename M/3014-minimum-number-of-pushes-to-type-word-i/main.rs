struct Solution {}

impl Solution {
  pub fn minimum_pushes(word: String) -> i32 {
    let mut buf: Vec<i32> = vec![0; 26];
    word.as_bytes().iter().for_each(|&b| {
      buf[(b - b'a') as usize] += 1;
    });
    buf.sort();
    let mut cnt: i32 = 0;
    buf
      .iter()
      .rev()
      .enumerate()
      .for_each(|(idx, &v)| cnt += v * (idx as i32 / 8 + 1));
    cnt
  }
}
