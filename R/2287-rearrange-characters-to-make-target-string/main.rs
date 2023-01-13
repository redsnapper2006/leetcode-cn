struct Solution {}

impl Solution {
  pub fn rearrange_characters(s: String, target: String) -> i32 {
    let mut sb: [i32; 26] = [0; 26];
    let mut tb: [i32; 26] = [0; 26];

    s.as_bytes()
      .iter()
      .for_each(|b| sb[(b - 'a' as u8) as usize] += 1);
    target
      .as_bytes()
      .iter()
      .for_each(|b| tb[(b - 'a' as u8) as usize] += 1);

    (0..26).fold(s.len() as i32 / target.len() as i32, |acc, d| {
      match (sb[d], tb[d]) {
        (_, 0) => acc,
        (x, y) => {
          if x / y < acc {
            x / y
          } else {
            acc
          }
        }
      }
    })
  }
}
