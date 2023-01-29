struct Solution {}

impl Solution {
  pub fn count_asterisks(s: String) -> i32 {
    let mut ret: i32 = 0;
    let mut cnt: i32 = 0;
    for b in s.chars() {
      if b == '|' {
        cnt += 1;
      } else if b == '*' && cnt % 2 == 0 {
        ret += 1;
      }
    }
    ret
  }
}
