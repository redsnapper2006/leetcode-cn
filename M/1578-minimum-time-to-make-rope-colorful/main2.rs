
impl Solution {
  pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let bb = colors.as_bytes().to_vec();
    let mut base : u8 = bb[0];
    let mut t : i32 = needed_time[0];
    let mut ans : i32 = 0;
    for i in 1..bb.len() {
      if bb[i] == base {
        ans += needed_time[i].min(t);
        t = t.max(needed_time[i]);
      } else {
        t = needed_time[i];
      }
      base = bb[i];
    }
    ans
  }
}
