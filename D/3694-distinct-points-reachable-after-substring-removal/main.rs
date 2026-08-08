use std::collections::HashSet;

impl Solution {
  pub fn distinct_points(s: String, k: i32) -> i32 {
    let k = k as usize;
    let bb = s.as_bytes().to_vec();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut buf: Vec<(i32, i32)> = vec![];
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..bb.len() {
      match bb[i] {
        b'U' => y += 1,
        b'D' => y -= 1,
        b'L' => x -= 1,
        _ => x += 1,
      };
      buf.push((x, y));

      if i >= k - 1 {
        let diff = (
          x - if i >= k { buf[i - k].0 } else { 0 },
          y - if i >= k { buf[i - k].1 } else { 0 },
        );
        set.insert(diff);
      }
    }
    set.len() as i32
  }
}
