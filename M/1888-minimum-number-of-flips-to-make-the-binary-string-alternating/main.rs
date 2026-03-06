impl Solution {
  pub fn min_flips(s: String) -> i32 {
    let bb = s.as_bytes();

    let mut ans: i32 = bb.len() as i32;
    let mut cnt: i32 = 0;
    for i in 0..bb.len() * 2 - 1 {
      if bb[i % bb.len()] - b'0' != (i % 2) as u8 {
        cnt += 1;
      }

      if i < bb.len() - 1 {
        continue;
      }
      ans = ans.min(cnt).min(bb.len() as i32 - cnt);
      if (bb[i + 1 - bb.len()] - b'0') as i32 != (i + 1 - bb.len()) as i32 % 2 {
        cnt -= 1;
      }
    }
    ans
  }
}
