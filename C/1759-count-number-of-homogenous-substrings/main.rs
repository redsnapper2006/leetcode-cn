struct Solution {}

impl Solution {
  pub fn count_homogenous(s: String) -> i32 {
    let mut buf: Vec<i64> = Vec::new();

    let mut b: u8 = s.as_bytes()[0];
    let mut cnt: i64 = 0;
    for c in s.as_bytes() {
      if *c == b {
        cnt += 1;
      } else {
        buf.push(cnt);
        b = *c;
        cnt = 1;
      }
    }
    buf.push(cnt);
    let mut ret: i64 = 0;
    for mut v in buf {
      ret += v * (v + 1) / 2;
      ret %= 1000000007;
    }
    ret as i32 % 1000000007
  }
}
