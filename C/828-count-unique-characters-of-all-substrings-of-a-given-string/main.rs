struct Solution {}

impl Solution {
  pub fn unique_letter_string(s: String) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![Vec::new(); 26];

    (0..26).for_each(|idx| {
      buf[idx].push(-1);
    });
    s.as_bytes().iter().enumerate().for_each(|(i, &b)| {
      let offset = (b - b'A') as usize;
      buf[offset].push(i as i32);
    });

    (0..26).for_each(|idx| {
      buf[idx].push(s.len() as i32);
    });

    let mut ret: i32 = 0;
    (0..26).for_each(|idx| {
      if buf[idx].len() <= 2 {
        return;
      }
      let mut j: usize = 1;
      while j < buf[idx].len() - 1 {
        ret += (buf[idx][j] - buf[idx][j - 1]) * (buf[idx][j + 1] - buf[idx][j]);
        j += 1;
      }
    });
    ret
  }
}
