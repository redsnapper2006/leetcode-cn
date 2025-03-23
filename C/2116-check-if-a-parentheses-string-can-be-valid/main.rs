impl Solution {
  pub fn can_be_valid(s: String, locked: String) -> bool {
    let mut mn: i32 = 0;
    let mut mx: i32 = 0;
    let sbb = s.as_bytes().to_vec();
    let lbb = locked.as_bytes().to_vec();

    let mut idx: usize = 0;
    while idx < sbb.len() {
      if lbb[idx] == b'1' {
        let d = if sbb[idx] == b'(' { 1 } else { -1 };
        mx += d;
        if mx < 0 {
          return false;
        }
        mn += d;
      } else {
        mx += 1;
        mn -= 1;
      }
      if mn < 0 {
        mn = 1;
      }
      idx += 1;
    }
    mn == 0
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::can_be_valid(
      "((()(()()))()((()()))))()((()(()".to_string(),
      "10111100100101001110100010001001".to_string()
    )
  )
}
