struct Solution {}

impl Solution {
  pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
    let mut buf: [i32; 26] = [-1; 26];
    s.as_bytes()
      .iter()
      .enumerate()
      .map(|(idx, b)| {
        let offset = (b - 'a' as u8) as usize;
        if buf[offset] > -1 && idx as i32 - buf[offset] - 1 != distance[offset] {
          return false;
        }

        buf[offset] = idx as i32;
        true
      })
      .collect::<Vec<bool>>()
      .iter()
      .all(|&x| x)
  }
}
