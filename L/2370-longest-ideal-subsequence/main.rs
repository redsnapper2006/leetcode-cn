struct Solution {}

impl Solution {
  pub fn longest_ideal_string(s: String, k: i32) -> i32 {
    let mut buf: Vec<[i32; 26]> = vec![[0; 26]; s.len()];

    s.as_bytes()
      .iter()
      .enumerate()
      .for_each(|(idx, b)| match idx {
        0 => buf[idx][(b - 'a' as u8) as usize] = 1,
        _ => {
          (0..26).for_each(|offset| {
            buf[idx][offset] = buf[idx - 1][offset];
          });
          let offset = (b - 'a' as u8) as i32;
          (0..=k).for_each(|dis| {
            let bd = offset - dis as i32;
            let ad = offset + dis as i32;
            if bd >= 0 && bd < 26 && buf[idx][offset as usize] < buf[idx - 1][bd as usize] + 1 {
              buf[idx][offset as usize] = buf[idx - 1][bd as usize] + 1;
            }
            if ad >= 0 && ad < 26 && buf[idx][offset as usize] < buf[idx - 1][ad as usize] + 1 {
              buf[idx][offset as usize] = buf[idx - 1][ad as usize] + 1;
            }
          });
        }
      });
    buf[s.len() - 1].to_vec().into_iter().max().unwrap()
  }
}
