struct Solution {}

impl Solution {
  pub fn min_flips(target: String) -> i32 {
    let t = target.as_bytes();
    let mut state: u8 = '0' as u8;
    let mut cnt: i32 = 0;
    for i in 0..t.len() {
      if t[i] == state {
        continue;
      }
      cnt += 1;
      if state == '1' as u8 {
        state = '0' as u8;
      } else {
        state = '1' as u8;
      }
    }
    cnt
  }
}

fn main() {
  println!("{}", Solution::min_flips(String::from("1000")));
}
