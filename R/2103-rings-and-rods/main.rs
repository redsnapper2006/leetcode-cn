struct Solution {}

impl Solution {
  pub fn count_points(rings: String) -> i32 {
    let mut buf = vec![vec![0; 3]; 10];
    let bb = rings.as_bytes();
    (0..bb.len()).step_by(2).for_each(|idx| {
      let idx1 = match bb[idx] {
        b'R' => 0,
        b'G' => 1,
        _ => 2,
      };
      let offset = (bb[idx + 1] - b'0') as usize;
      buf[offset][idx1] += 1;
    });

    buf
      .iter()
      .filter(|b| b[0] > 0 && b[1] > 0 && b[2] > 0)
      .count() as i32
  }
}
