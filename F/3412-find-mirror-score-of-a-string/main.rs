impl Solution {
  pub fn calculate_score(s: String) -> i64 {
    let mut p: Vec<Vec<usize>> = vec![vec![]; 26];

    s.as_bytes().into_iter().enumerate().fold(0, |ans, (idx, b)| {
      let offset = (b - b'a') as usize;
      if let Some(prev) = p[25 - offset].pop() {
        ans + (idx - prev) as i64
      } else {
        p[offset].push(idx);
        ans
      }
    })
  }
}
