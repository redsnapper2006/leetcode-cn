impl Solution {
  pub fn max_difference(s: String) -> i32 {
    let mut buf: Vec<i32> = vec![0; 26];
    s.as_bytes().iter().for_each(|b| {
      buf[(b - b'a') as usize] += 1;
    });

    let mut a1: i32 = 0;
    let mut a2: i32 = i32::MAX;
    (0..26).for_each(|idx| {
      if buf[idx] == 0 {
        return;
      }
      if buf[idx] % 2 == 1 && buf[idx] > a1 {
        a1 = buf[idx];
      }
      if buf[idx] % 2 == 0 && buf[idx] < a2 {
        a2 = buf[idx];
      }
    });

    a1 - a2
  }
}
