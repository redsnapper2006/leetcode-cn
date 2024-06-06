impl Solution {
  pub fn minimum_steps(s: String) -> i64 {
    s.as_bytes()
      .iter()
      .fold((0, 0), |(ans, cnt), &b| match b {
        b'1' => (ans, cnt + 1),
        _ => (ans + cnt, cnt),
      })
      .0
  }

  pub fn minimum_steps2(s: String) -> i64 {
    let bb = s.as_bytes().to_vec();
    let mut ans: i64 = 0;
    let mut start: i32 = 0;
    let mut end: i32 = bb.len() as i32 - 1;

    while start < end {
      if bb[start as usize] == b'0' {
        start += 1;
        continue;
      }
      if bb[end as usize] == b'1' {
        end -= 1;
        continue;
      }
      ans += (end - start) as i64;
      start += 1;
      end -= 1;
    }

    ans
  }
}
