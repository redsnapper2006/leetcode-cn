impl Solution {
  pub fn max_depth_after_split(seq: String) -> Vec<i32> {
    let mut ans = Vec::with_capacity(seq.len());
    let mut d = 0i32;
    for b in seq.bytes() {
      if b == b'(' {
        ans.push(d & 1);
        d += 1;
      } else {
        d -= 1;
        ans.push(d & 1);
      }
    }
    ans
  }
}
