impl Solution {
  pub fn count_rotations(s: String, k: i32) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut score = 0i32;
    for i in 1..bb.len() {
      if bb[i] == bb[i - 1] {
        score += 1;
      }
    }
    let mut ans = if score == k { 1 } else { 0 };
    for i in 1..bb.len() {
      if bb[i] == bb[i - 1] {
        score -= 1;
      }
      if bb[i - 1] == bb[(i + bb.len() - 2) % bb.len()] {
        score += 1;
      }
      if score == k {
        ans += 1;
      }
    }
    ans
  }
}
