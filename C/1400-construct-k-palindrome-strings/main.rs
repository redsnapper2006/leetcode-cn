struct Solution {}

impl Solution {
  pub fn can_construct(s: String, k: i32) -> bool {
    let mut buf: Vec<i32> = vec![0; 26];
    s.as_bytes().iter().for_each(|&b| {
      buf[(b - 'a' as u8) as usize] += 1;
    });

    let mut odd: i32 = 0;
    buf.iter().for_each(|&v| {
      if v % 2 == 1 {
        odd += 1;
      }
    });
    (s.len() as i32) >= k && odd <= k
  }
}
