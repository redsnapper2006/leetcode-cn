impl Solution {
  pub fn min_changes2(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();

    let mut ans: i32 = 0;
    for i in (0..bb.len()).step_by(2) {
      if bb[i] != bb[i + 1] {
        ans += 1;
      }
    }
    ans
  }

  pub fn min_changes(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();
    (0..bb.len()).step_by(2).fold(0, |sum, idx| {
      sum + if bb[idx] != bb[idx + 1] { 1 } else { 0 }
    })
  }
}
